# ClawMaster 工具调度机制详解

**文档版本**: 1.0  
**创建时间**: 2026年3月16日 21:44  
**适用版本**: ClawMaster v1.0+

---

## 📋 目录

1. [核心问题解答](#核心问题解答)
2. [工具调度架构](#工具调度架构)
3. [统一调度机制](#统一调度机制)
4. [WASM vs 原生工具](#wasm-vs-原生工具)
5. [工具注册流程](#工具注册流程)
6. [工具调用流程](#工具调用流程)
7. [实际案例分析](#实际案例分析)

---

## 核心问题解答

### ❓ 在对话中，软件是如何使用工具的？

**答案**: ClawMaster 使用 **统一的 `AgentTool` trait** 来调度所有工具，无论是 WASM 工具还是原生工具。

### ❓ 是使用 WASM 工具还是其他的？

**答案**: **两者都使用**！ClawMaster 同时支持：
- **WASM 工具** (3个): calc, web_fetch, web_search
- **原生工具** (29+个): exec, browser, memory_* 等

### ❓ 能够融合在一起，统一调动吗？

**答案**: **完全可以！** 所有工具通过 `ToolRegistry` 统一管理和调度，AI 在对话中可以**无缝调用任何工具**，无需区分 WASM 或原生。

---

## 工具调度架构

### 核心设计理念

```
┌─────────────────────────────────────────────────────────┐
│                    AI 对话系统                           │
│              (不关心工具的实现方式)                       │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                  ToolRegistry                            │
│              (统一工具注册表)                             │
│                                                          │
│  • 统一接口: AgentTool trait                             │
│  • 透明调度: 不区分工具来源                              │
│  • 动态管理: 运行时注册/注销                             │
└─────────────────────────────────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │ WASM 工具    │ │ 原生工具     │ │ MCP 工具     │
    │              │ │              │ │              │
    │ • calc       │ │ • exec       │ │ • 外部服务   │
    │ • web_fetch  │ │ • browser    │ │ • 动态加载   │
    │ • web_search │ │ • memory_*   │ │              │
    └──────────────┘ └──────────────┘ └──────────────┘
```

---

## 统一调度机制

### 1. AgentTool Trait (统一接口)

所有工具必须实现这个 trait：

```rust
#[async_trait]
pub trait AgentTool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn warmup(&self) -> Result<()>;
    async fn execute(&self, params: serde_json::Value) -> Result<serde_json::Value>;
}
```

**关键点**:
- ✅ **统一接口**: 所有工具都实现相同的方法
- ✅ **异步执行**: 支持高并发调用
- ✅ **类型安全**: 通过 JSON schema 验证参数
- ✅ **透明调度**: 调用者不需要知道工具的实现方式

### 2. ToolRegistry (工具注册表)

```rust
pub struct ToolRegistry {
    tools: HashMap<String, ToolEntry>,
}

struct ToolEntry {
    tool: Arc<dyn AgentTool>,
    source: ToolSource,  // Builtin | Wasm | Mcp
}
```

**功能**:
- ✅ 统一存储所有工具
- ✅ 按名称索引
- ✅ 记录工具来源
- ✅ 支持动态注册/注销

### 3. ToolSource (工具来源标记)

```rust
pub enum ToolSource {
    Builtin,                          // 原生工具
    Wasm { component_hash: [u8; 32] }, // WASM 工具
    Mcp { server: String },            // MCP 工具
}
```

**用途**:
- 📊 **监控和调试**: 知道工具来自哪里
- 🔍 **过滤和管理**: 可以批量操作特定来源的工具
- 📈 **统计分析**: 追踪不同类型工具的使用情况

---

## WASM vs 原生工具

### 对比表

| 特性 | WASM 工具 | 原生工具 | 统一调度 |
|------|----------|---------|---------|
| **接口** | AgentTool trait | AgentTool trait | ✅ 完全一致 |
| **注册** | `register_wasm()` | `register()` | ✅ 统一注册表 |
| **调用** | `execute()` | `execute()` | ✅ 相同方法 |
| **调度** | ToolRegistry | ToolRegistry | ✅ 透明调度 |
| **AI 感知** | 不可见 | 不可见 | ✅ 完全透明 |

### 工具列表对比

#### WASM 工具 (3个)
```
1. calc (WASM)       - 沙箱化数学计算
2. web_fetch (WASM)  - 沙箱化网页获取
3. web_search (WASM) - 沙箱化网络搜索
```

#### 原生工具 (29+个)
```
1. exec              - 命令执行
2. calc              - 原生计算器
3. process           - 进程管理
4. browser           - 浏览器自动化
5. web_fetch         - 原生网页获取
6. web_search        - 原生网络搜索
7. memory_*          - 记忆管理
8. sessions_*        - 会话管理
... 等 29+ 个工具
```

### 重要发现

#### ⚠️ 工具名称冲突处理

对于 `calc`, `web_fetch`, `web_search` 这三个工具：

**注册顺序**:
```rust
// 1. 先注册原生 calc
tool_registry.register(Box::new(CalcTool::new()));

// 2. 如果启用 wasm feature，注册 WASM 工具
#[cfg(feature = "wasm")]
{
    register_wasm_tools(&mut tool_registry, ...);
    // 这会注册 WASM 版本的 calc, web_fetch, web_search
}

// 3. 后注册原生 web_fetch 和 web_search
if let Some(t) = WebSearchTool::from_config(...) {
    tool_registry.register(Box::new(t));
}
if let Some(t) = WebFetchTool::from_config(...) {
    tool_registry.register(Box::new(t));
}
```

**结果**: 
- ✅ **后注册的工具会覆盖先注册的同名工具**
- ✅ 最终使用的是 **原生版本** (因为原生工具后注册)
- ✅ WASM 工具作为**备用选项**存在

---

## 工具注册流程

### 完整注册代码 (server.rs)

```rust
// 创建工具注册表
let mut tool_registry = ToolRegistry::new();

// === 第 1 步: 注册核心原生工具 ===
tool_registry.register(Box::new(exec_tool));
tool_registry.register(Box::new(CalcTool::new()));

// === 第 2 步: 注册 WASM 工具 (如果启用) ===
#[cfg(feature = "wasm")]
{
    if let Err(e) = register_wasm_tools(
        &mut tool_registry,
        &wasm_limits,
        epoch_interval_ms,
        fetch_timeout_secs,
        fetch_cache_ttl_minutes,
        search_timeout_secs,
        search_cache_ttl_minutes,
        brave_api_key,
    ) {
        warn!(%e, "wasm tool registration failed");
    }
}

// === 第 3 步: 注册其他原生工具 ===
tool_registry.register(Box::new(process_tool));
tool_registry.register(Box::new(sandbox_packages_tool));
tool_registry.register(Box::new(cron_tool));
tool_registry.register(Box::new(SendMessageTool::new(...)));
tool_registry.register(Box::new(SendImageTool::new(...)));

// === 第 4 步: 条件注册 (根据配置) ===
if let Some(t) = WebSearchTool::from_config(...) {
    tool_registry.register(Box::new(t));
}
if let Some(t) = WebFetchTool::from_config(...) {
    tool_registry.register(Box::new(t));
}
if let Some(t) = BrowserTool::from_config(...) {
    tool_registry.register(Box::new(t));
}

// === 第 5 步: 注册记忆工具 ===
if let Some(ref mm) = memory_manager {
    tool_registry.register(Box::new(MemorySearchTool::new(mm)));
    tool_registry.register(Box::new(MemoryGetTool::new(mm)));
    tool_registry.register(Box::new(MemorySaveTool::new(mm)));
}

// === 第 6 步: 注册会话工具 ===
tool_registry.register(Box::new(SessionsListTool::new(...)));
tool_registry.register(Box::new(SessionsCreateTool::new(...)));
// ... 更多会话工具

// === 第 7 步: 注册其他工具 ===
tool_registry.register(Box::new(TaskListTool::new(...)));
tool_registry.register(Box::new(LocationTool::new(...)));
tool_registry.register(Box::new(ShowMapTool::new(...)));
// ... 更多工具
```

### WASM 工具注册细节

```rust
pub fn register_wasm_tools(
    registry: &mut ToolRegistry,
    wasm_limits: &WasmToolLimits,
    epoch_interval_ms: u64,
    // ... 其他参数
) -> Result<()> {
    let wasm_engine = Arc::new(WasmComponentEngine::new(None)?);
    
    // 1. 注册 calc (Pure Tool)
    if let Ok(calc_bytes) = calc_component_bytes() {
        let (fuel, memory) = wasm_limits.resolve_store_limits("calc");
        let runner = WasmToolRunner::new(
            Arc::clone(&wasm_engine),
            calc_bytes.as_ref(),
            fuel,
            memory,
            Duration::from_secs(2),
            epoch_interval_ms,
        )?;
        let hash = runner.component_hash();
        registry.register_wasm(Box::new(runner), hash);
    }
    
    // 2. 注册 web_fetch (HTTP Tool)
    if let Ok(fetch_bytes) = web_fetch_component_bytes() {
        let http_host = HttpHostImpl::new(...)?;
        let runner = WasmToolRunner::new_http(
            Arc::clone(&wasm_engine),
            fetch_bytes.as_ref(),
            fuel,
            memory,
            Duration::from_secs(5),
            epoch_interval_ms,
            http_host,
        )?;
        let hash = runner.component_hash();
        let cached = CachingWasmToolRunner::new(
            Arc::new(runner),
            hash,
            cache_ttl,
        );
        registry.register_wasm(Box::new(cached), hash);
    }
    
    // 3. 注册 web_search (HTTP Tool with API Key)
    // ... 类似 web_fetch
    
    Ok(())
}
```

---

## 工具调用流程

### 1. AI 对话中的工具调用

```
用户: "请计算 123 * 456"
  │
  ▼
┌─────────────────────────────────┐
│  AI 模型 (Claude/GPT)            │
│  - 理解用户意图                  │
│  - 决定使用 calc 工具            │
│  - 生成工具调用请求              │
└─────────────────────────────────┘
  │
  ▼
┌─────────────────────────────────┐
│  Chat Runtime                    │
│  - 解析工具调用                  │
│  - 查找工具: "calc"              │
└─────────────────────────────────┘
  │
  ▼
┌─────────────────────────────────┐
│  ToolRegistry                    │
│  - registry.get("calc")          │
│  - 返回 Arc<dyn AgentTool>       │
└─────────────────────────────────┘
  │
  ▼
┌─────────────────────────────────┐
│  AgentTool::execute()            │
│  - 执行工具逻辑                  │
│  - 返回 JSON 结果                │
└─────────────────────────────────┘
  │
  ▼
┌─────────────────────────────────┐
│  AI 模型                         │
│  - 接收工具结果                  │
│  - 生成自然语言回复              │
└─────────────────────────────────┘
  │
  ▼
用户: "结果是 56088"
```

### 2. 工具调用代码示例

```rust
// AI 决定调用工具
let tool_call = ToolCall {
    name: "calc".to_string(),
    parameters: json!({
        "expression": "123 * 456"
    }),
};

// 从注册表获取工具
if let Some(tool) = tool_registry.get(&tool_call.name) {
    // 执行工具 (无论是 WASM 还是原生)
    let result = tool.execute(tool_call.parameters).await?;
    
    // 返回结果给 AI
    // result: {"value": 56088}
}
```

### 3. 透明调度的关键

```rust
// AI 和调用者完全不需要知道工具的实现方式
// 以下代码对 WASM 和原生工具完全相同：

let tool = registry.get("calc").unwrap();
let result = tool.execute(params).await?;

// 无论 calc 是:
// - WASM 版本 (WasmToolRunner)
// - 原生版本 (CalcTool)
// 调用方式完全一样！
```

---

## 实际案例分析

### 案例 1: 计算工具调用

**用户请求**: "请计算 (100 + 200) * 3"

**调用流程**:
```
1. AI 解析: 需要使用 calc 工具
2. 查找工具: registry.get("calc")
3. 返回: 原生 CalcTool (因为后注册覆盖了 WASM 版本)
4. 执行: CalcTool::execute({"expression": "(100 + 200) * 3"})
5. 结果: {"value": 900}
6. AI 回复: "计算结果是 900"
```

**如果只启用 WASM**:
```
1-2. 相同
3. 返回: WasmToolRunner (WASM calc)
4. 执行: WasmToolRunner::execute(...)
   - 加载 WASM 组件
   - 设置燃料和内存限制
   - 在沙箱中执行
   - 返回结果
5-6. 相同
```

### 案例 2: 网页获取

**用户请求**: "请获取 https://example.com 的内容"

**调用流程**:
```
1. AI 解析: 需要使用 web_fetch 工具
2. 查找工具: registry.get("web_fetch")
3. 返回: 原生 WebFetchTool (后注册)
4. 执行: WebFetchTool::execute({"url": "https://example.com"})
5. 结果: {"content": "...", "status": 200}
6. AI 回复: "网页内容是..."
```

### 案例 3: 多工具协作

**用户请求**: "请搜索 Rust 相关信息，然后保存到记忆中"

**调用流程**:
```
1. AI 决定: 需要两个工具
   - web_search: 搜索信息
   - memory_save: 保存记忆

2. 第一步: 调用 web_search
   registry.get("web_search").execute({"query": "Rust"})
   
3. 第二步: 调用 memory_save
   registry.get("memory_save").execute({
       "content": "搜索结果...",
       "tags": ["rust", "programming"]
   })
   
4. AI 回复: "已搜索 Rust 相关信息并保存到记忆中"
```

**关键点**: 
- ✅ AI 可以**连续调用多个工具**
- ✅ 工具之间可以**传递数据**
- ✅ 完全**透明调度**，不区分工具类型

---

## 统一调度的优势

### 1. 对 AI 模型透明

```
AI 只需要知道:
- 工具名称 (name)
- 工具描述 (description)
- 参数格式 (parameters_schema)

AI 不需要知道:
- 工具是 WASM 还是原生
- 工具的实现细节
- 工具的资源限制
```

### 2. 灵活的工具替换

```rust
// 可以轻松替换工具实现
// 例如: 从原生切换到 WASM

// 原来: 使用原生 calc
registry.register(Box::new(CalcTool::new()));

// 现在: 使用 WASM calc
registry.register_wasm(Box::new(wasm_calc), hash);

// AI 调用代码完全不需要改变！
```

### 3. 动态工具管理

```rust
// 运行时添加工具
registry.register(Box::new(new_tool));

// 运行时删除工具
registry.unregister("old_tool");

// 批量删除 MCP 工具
registry.unregister_mcp();

// AI 自动感知工具变化
```

### 4. 多来源工具融合

```
ToolRegistry 可以同时管理:
- Builtin 工具 (29+个)
- WASM 工具 (3个)
- MCP 工具 (动态加载)

所有工具统一调度，无缝协作！
```

---

## 配置和控制

### 1. 启用/禁用 WASM 工具

**通过 Feature Flag**:
```bash
# 启用 WASM
cargo build --features wasm

# 禁用 WASM (默认)
cargo build
```

**效果**:
```rust
#[cfg(feature = "wasm")]
{
    // 这段代码只在启用 wasm feature 时编译
    register_wasm_tools(&mut tool_registry, ...);
}
```

### 2. WASM 工具资源限制

**配置文件** (`clawmaster.toml`):
```toml
[wasm_tool_limits]
default_memory = 16777216  # 16 MB
default_fuel = 1000000

[wasm_tool_limits.tool_overrides.calc]
fuel = 100000
memory = 2097152  # 2 MB
```

**代码应用**:
```rust
let (fuel, memory) = wasm_limits.resolve_store_limits("calc");
// calc: fuel=100000, memory=2MB
// 其他: fuel=1000000, memory=16MB
```

### 3. 工具优先级控制

**通过注册顺序**:
```rust
// 方案 1: 优先使用原生工具
register_native_tools();  // 后注册
register_wasm_tools();    // 先注册

// 方案 2: 优先使用 WASM 工具
register_wasm_tools();    // 后注册
register_native_tools();  // 先注册

// 后注册的工具会覆盖同名工具
```

---

## 总结

### 核心答案

#### 1. 如何使用工具？
**通过统一的 `AgentTool` trait 和 `ToolRegistry`**
- 所有工具实现相同接口
- 统一注册表管理
- 透明调度执行

#### 2. 使用 WASM 还是原生？
**两者都使用，但原生工具优先**
- WASM 工具: 3 个 (calc, web_fetch, web_search)
- 原生工具: 29+ 个
- 同名工具: 原生版本覆盖 WASM 版本

#### 3. 能否统一调动？
**完全可以！**
- ✅ 统一接口 (AgentTool trait)
- ✅ 统一注册 (ToolRegistry)
- ✅ 统一调用 (execute 方法)
- ✅ 透明调度 (AI 无感知)
- ✅ 无缝协作 (多工具组合)

### 架构优势

1. **灵活性**: 可以轻松添加/删除/替换工具
2. **可扩展性**: 支持多种工具来源 (Builtin, WASM, MCP)
3. **安全性**: WASM 工具提供沙箱隔离
4. **性能**: 原生工具提供最佳性能
5. **透明性**: AI 和调用者无需关心实现细节

### 最佳实践

1. **默认使用原生工具** - 性能最佳
2. **WASM 作为备选** - 提供额外安全层
3. **统一接口设计** - 便于工具替换
4. **动态工具管理** - 运行时灵活调整
5. **资源限制配置** - 防止工具滥用

---

**文档结论**: ClawMaster 通过精心设计的统一调度机制，实现了 WASM 工具和原生工具的完美融合，为 AI 对话系统提供了强大、灵活、安全的工具调用能力！🎉
