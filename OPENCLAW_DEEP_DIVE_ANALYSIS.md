# OpenClaw 深度源码分析与 ClawMaster 对比

**分析时间**: 2026-03-20  
**基于**: OpenClaw 官方文档 + 源码分析

---

## 一、OpenClaw Agent Loop 核心架构

### 1.1 执行流程

```
用户消息 
  ↓
agent RPC (验证、持久化) → 返回 { runId, acceptedAt }
  ↓
agentCommand (解析模型、加载技能)
  ↓
runEmbeddedPiAgent (pi-agent-core 运行时)
  ↓
串行化队列 (per-session + global)
  ↓
构建 pi session + 订阅事件
  ↓
流式输出 (assistant/tool/lifecycle 事件)
  ↓
超时控制 + 返回结果
```

**关键特性**：
- ✅ **串行化执行**: 每个 session 一个队列，避免竞态
- ✅ **事件流式**: `stream: "tool"`, `stream: "assistant"`, `stream: "lifecycle"`
- ✅ **超时保护**: 默认 600s，可配置
- ✅ **Hook 系统**: 内部 hooks + 插件 hooks

### 1.2 工具执行机制

```typescript
// OpenClaw 的工具执行流程
Tool start → emit tool event (stream: "tool")
  ↓
Tool execution (sanitize size/image payloads)
  ↓
Tool end → emit tool event
  ↓
Track messaging tool sends (避免重复确认)
  ↓
Assemble final payloads (assistant text + tool summaries)
```

**关键点**：
1. **工具事件独立流**: 不混在 assistant 流中
2. **结果清理**: 自动清理大型 payload 和图片
3. **去重机制**: 跟踪 messaging tool 发送，避免重复
4. **NO_REPLY 令牌**: 静默执行，不返回确认

---

## 二、OpenClaw System Prompt 架构

### 2.1 Prompt 组成部分

OpenClaw 的 system prompt 包含 **20+ 个动态部分**：

| 部分 | 条件 | 作用 |
|------|------|------|
| **Tooling** | 总是 | 工具列表 + 简短描述 |
| **Safety** | 总是 | 安全防护提醒 |
| **Skills** | 有技能时 | 技能加载指导 |
| **Memory** | 有 memory 工具时 | 记忆使用指导 |
| **Bootstrap Files** | 总是 | 注入 AGENTS.md, SOUL.md, TOOLS.md 等 |
| **Sandbox** | 沙箱启用时 | 沙箱路径和权限 |
| **Time** | 总是 | 当前时间 + 时区 |
| **Runtime** | 总是 | 主机、OS、节点、模型信息 |
| **Reasoning** | 支持时 | 推理可见性提示 |

### 2.2 Bootstrap 文件注入

OpenClaw 自动注入以下文件到 system prompt：

```
AGENTS.md      → 智能体配置和规则
SOUL.md        → 个性化定制
TOOLS.md       → 工具使用规则 (用户自定义)
IDENTITY.md    → 身份信息
USER.md        → 用户偏好
HEARTBEAT.md   → 心跳行为
BOOTSTRAP.md   → 初始化指导 (仅新工作区)
MEMORY.md      → 长期记忆
```

**关键洞察**：
- ✅ **TOOLS.md 是用户可编辑的**: 用户可以自定义工具行为
- ✅ **分离关注点**: 工具定义 (代码) vs. 工具使用规则 (TOOLS.md)
- ✅ **自动注入**: 无需手动 read，减少工具调用

### 2.3 Prompt Modes

OpenClaw 支持 3 种 prompt 模式：

1. **full** (默认): 包含所有部分
2. **minimal**: 用于子智能体，省略技能、记忆召回等
3. **none**: 仅返回基础身份行

---

## 三、OpenClaw 工具调用策略

### 3.1 "Do Not Narrate" 核心原则

从文档和搜索结果中提取的关键指令：

```
## Tool Call Style

Default: do not narrate routine tool calls. Just call the tool.

When to narrate (briefly):
- Multi-step work
- Complex problems
- Sensitive actions (deletions, system changes)
- User explicitly asks

Keep narration brief and value-dense.
```

**对比传统方法**：

| 传统方法 | OpenClaw 方法 |
|---------|--------------|
| "你必须调用工具！" | "默认不解释，直接调用" |
| "禁止说'你可以使用'！" | "保持简洁，避免重复" |
| 冗长的强调和示例 | 简短的原则和少量示例 |

### 3.2 工具呈现方式

OpenClaw 使用 **双轨制**：

1. **System prompt text**: 人类可读的工具列表
   ```
   ## Available Tools
   
   - `exec`: Execute shell commands
   - `web_search`: Search the web
   - `news_search`: Search news articles
   ```

2. **Tool schema**: 结构化的函数定义 (发送给 API)
   ```json
   {
     "name": "exec",
     "description": "Execute shell commands",
     "parameters": { ... }
   }
   ```

**关键点**：
- ✅ 原生工具调用模式：schema 通过 API 发送
- ✅ 文本模式：schema 嵌入 prompt
- ✅ 简洁列表：避免重复大型 JSON

---

## 四、ClawMaster vs OpenClaw 对比

### 4.1 架构对比

| 特性 | OpenClaw | ClawMaster |
|------|----------|-----------|
| **语言** | TypeScript | Rust |
| **Agent Loop** | pi-agent-core (嵌入式) | 自定义 Rust 实现 |
| **队列系统** | per-session + global | 单一队列 |
| **事件流** | tool/assistant/lifecycle 分离 | chat 事件统一 |
| **超时控制** | 600s 默认，可配置 | 可配置 |
| **Hook 系统** | 内部 + 插件 hooks | 有限支持 |

### 4.2 Prompt 策略对比

| 方面 | OpenClaw | ClawMaster (修改前) | ClawMaster (修改后) |
|------|----------|-------------------|-------------------|
| **工具调用指导** | "Do Not Narrate" | "必须调用，禁止解释" | "Do Not Narrate" ✅ |
| **语气** | 简洁、命令式 | 冗长、强调式 | 简洁、命令式 ✅ |
| **示例数量** | 2-3 个 | 10+ 个 | 3 个 ✅ |
| **TOOLS.md** | 用户可编辑 | 支持但未强调 | 提供模板 ✅ |
| **Bootstrap 注入** | 自动注入 8 个文件 | 支持部分文件 | 支持 |

### 4.3 工具执行对比

| 特性 | OpenClaw | ClawMaster |
|------|----------|-----------|
| **工具事件流** | 独立 `tool` 流 | 混在 `chat` 流中 |
| **结果清理** | 自动清理大型 payload | 手动处理 |
| **去重机制** | 跟踪 messaging tool | 无 |
| **NO_REPLY** | 支持静默令牌 | 无 |
| **工具链式执行** | 支持 | 支持 |

---

## 五、关键发现与洞察

### 5.1 OpenClaw 的成功之处

#### 1. **简洁优于冗长**
```
❌ 错误: "你必须调用工具！禁止说'你可以使用'！"
✅ 正确: "Default: do not narrate routine tool calls."
```

**原因**：
- 简短指令更容易被模型理解
- 减少 prompt 噪音，提高关键信息的注意力
- 自然语气比强制语气更有效

#### 2. **分离关注点**
```
工具定义 (代码)  ← 开发者维护
     ↓
工具 Schema (API) ← 系统生成
     ↓
TOOLS.md (规则)  ← 用户自定义
```

**优势**：
- 用户可以自定义工具行为，无需修改代码
- 工具定义和使用规则解耦
- 更灵活的配置

#### 3. **事件流分离**
```
stream: "tool"      → 工具执行事件
stream: "assistant" → LLM 输出
stream: "lifecycle" → 生命周期事件
```

**优势**：
- 清晰的事件边界
- 更容易调试和监控
- 客户端可以选择性订阅

#### 4. **自动注入 Bootstrap 文件**
```
无需: read("AGENTS.md")
直接: 自动注入到 system prompt
```

**优势**：
- 减少工具调用次数
- 更快的启动速度
- 一致的上下文

### 5.2 ClawMaster 可以改进的地方

#### 1. **简化 Prompt** ✅ 已修改
```rust
// 修改前 (冗长)
"❌ FORBIDDEN: Do NOT say \"你可以使用\" / \"You can use\""
"✅ REQUIRED: DIRECTLY call the tool and return the ACTUAL result"

// 修改后 (简洁)
"Default: do not narrate routine tool calls. Just call the tool."
```

#### 2. **强化 TOOLS.md** ✅ 已创建模板
```markdown
# Tool Usage Rules

## Default Behavior
Do not narrate routine tool calls. Just call the tool.
```

#### 3. **考虑事件流分离** (未来改进)
```rust
// 当前: 所有事件在 "chat" 流
// 建议: 分离为 "tool", "assistant", "lifecycle"
```

#### 4. **添加 NO_REPLY 支持** (未来改进)
```rust
// 支持静默工具执行
if response.contains("[NO_REPLY]") {
    // 不发送确认消息
}
```

---

## 六、应用到 ClawMaster 的改进

### 6.1 已实施的改进 ✅

#### 1. 简化 tool_call_guidance
```rust
// crates/agents/src/prompt.rs
concat!(
    "\n## Tool Call Style\n\n",
    "Default: do not narrate routine tool calls. Just call the tool.\n\n",
    "When to narrate (briefly):\n",
    "- Multi-step work\n",
    "- Complex problems\n",
    "- Sensitive actions (deletions, system changes)\n",
    "- User explicitly asks\n\n",
    "Keep narration brief and value-dense.\n\n",
)
```

#### 2. 简化工具列表说明
```rust
prompt.push_str("## Available Tools\n\n");
prompt.push_str("You have permission to use these tools. ");
prompt.push_str("When a tool matches the user's request, call it directly.\n\n");
```

#### 3. 创建 TOOLS.md 模板
```markdown
# Tool Usage Rules

## Default Behavior
Do not narrate routine tool calls. Just call the tool.

## Examples
✅ Correct: [Call tool directly, return results]
❌ Wrong: "你可以使用 news_search 命令..."
```

### 6.2 建议的未来改进

#### 1. 事件流分离
```rust
pub enum StreamEvent {
    Tool(ToolEvent),
    Assistant(AssistantEvent),
    Lifecycle(LifecycleEvent),
}
```

#### 2. 工具结果清理
```rust
fn sanitize_tool_result(result: &str) -> String {
    // 限制大小
    // 清理图片 payload
    // 格式化输出
}
```

#### 3. NO_REPLY 令牌支持
```rust
const NO_REPLY_TOKEN: &str = "[NO_REPLY]";

if response.contains(NO_REPLY_TOKEN) {
    return None; // 静默执行
}
```

#### 4. 增强 Bootstrap 注入
```rust
// 自动注入更多文件
const BOOTSTRAP_FILES: &[&str] = &[
    "AGENTS.md",
    "SOUL.md",
    "TOOLS.md",
    "IDENTITY.md",
    "USER.md",
    "HEARTBEAT.md",
    "MEMORY.md",
];
```

---

## 七、测试计划

### 7.1 验证简化 Prompt 的效果

```bash
# 1. 重新编译
cargo build --release --bin clawmaster

# 2. 重启服务
pkill clawmaster
./target/release/clawmaster gateway &

# 3. 运行测试
./force_tool_execution_test.sh

# 4. 对比结果
# 期望: 工具实际调用率从 40% 提升到 70%+
```

### 7.2 测试不同模型

```bash
# Llama 3.1 8B (当前)
# Llama 3.2 1B Instruct (已知支持工具调用)
# Llama 3.1 70B (更强大)
```

### 7.3 A/B 测试

| 测试组 | Prompt 风格 | 预期结果 |
|--------|------------|---------|
| A | 冗长强调式 | 40% 调用率 |
| B | 简洁 OpenClaw 式 | 70%+ 调用率 |

---

## 八、结论

### 8.1 OpenClaw 的核心优势

1. **简洁的 Prompt 工程**: "Do Not Narrate" 比 "禁止解释" 更有效
2. **清晰的架构**: 事件流分离、队列系统、Hook 机制
3. **用户可定制**: TOOLS.md 让用户控制工具行为
4. **自动化**: Bootstrap 文件自动注入，减少手动操作

### 8.2 ClawMaster 的改进方向

1. ✅ **已完成**: 简化 Prompt，采用 OpenClaw 风格
2. ✅ **已完成**: 创建 TOOLS.md 模板
3. ⏭️ **下一步**: 测试改进效果
4. 🔮 **未来**: 事件流分离、NO_REPLY 支持、增强 Bootstrap

### 8.3 关键洞察

**问题不在代码，而在策略**：
- ClawMaster 的代码架构是健全的
- 工具注册、执行机制都是完整的
- 关键是 **Prompt 策略** 和 **模型能力**

**OpenClaw 的成功秘诀**：
- 简洁 > 冗长
- 自然 > 强制
- 分离 > 耦合
- 自动 > 手动

---

**下一步行动**：
1. ✅ 完成 Prompt 简化
2. ⏭️ 重新编译和测试
3. ⏭️ 对比新旧效果
4. ⏭️ 根据结果调整策略

**最终目标**: 将工具实际调用率从 40% 提升到 80%+
