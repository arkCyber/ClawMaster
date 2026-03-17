# ClawMaster WASM 工具信息

**发现时间**: 2026年3月16日 21:35  
**WASM 工具数量**: **3 个**

---

## 📋 WASM 工具清单

ClawMaster 包含 **3 个 WASM (WebAssembly) 工具**：

### 1. **calc** - WASM 计算器
- **类型**: Pure Tool (纯工具，无 HTTP 能力)
- **文件**: `clawmaster_wasm_calc.wasm`
- **功能**: 数学计算
- **超时**: 2 秒
- **燃料限制**: 100,000 (默认配置)
- **内存限制**: 2 MB (默认配置)

### 2. **web_fetch** - WASM 网页获取
- **类型**: HTTP Tool (HTTP 工具，有网络能力)
- **文件**: `clawmaster_wasm_web_fetch.wasm`
- **功能**: 获取网页内容
- **超时**: 可配置 (默认 30 秒)
- **缓存 TTL**: 可配置 (默认 60 分钟)
- **燃料限制**: 1,000,000 (默认)
- **内存限制**: 16 MB (默认)

### 3. **web_search** - WASM 网络搜索
- **类型**: HTTP Tool (HTTP 工具，有网络能力)
- **文件**: `clawmaster_wasm_web_search.wasm`
- **功能**: 网络搜索
- **超时**: 可配置 (默认 30 秒)
- **缓存 TTL**: 可配置 (默认 60 分钟)
- **API Key**: 支持 Brave Search API
- **燃料限制**: 1,000,000 (默认)
- **内存限制**: 16 MB (默认)

---

## 🏗️ WASM 架构

### 工具类型

1. **Pure Tool** (纯工具)
   - 无网络访问
   - 纯计算功能
   - 示例: calc

2. **HTTP Tool** (HTTP 工具)
   - 支持 HTTP 请求
   - 主机端 HTTP 处理
   - 示例: web_fetch, web_search

### 加载机制 (三层解析)

1. **Debug 文件系统** (仅 debug 构建)
   - 从 `target/wasm32-wasip2/release/` 读取
   - 用于迭代开发

2. **外部共享目录**
   - `share_dir()/wasm/` 
   - 用于打包部署

3. **嵌入式回退** (需要 `embedded-wasm` feature)
   - 编译到二进制中
   - `include_bytes!` 宏

---

## ⚙️ WASM 配置

### 资源限制

```rust
WasmToolLimits {
    default_memory: 16_777_216,  // 16 MB
    default_fuel: 1_000_000,
    overrides: {
        "calc": {
            fuel: 100_000,
            memory: 2_097_152  // 2 MB
        }
    }
}
```

### Epoch 间隔
- 默认: 100ms
- 用于超时控制

---

## 🔧 技术细节

### 运行时组件

1. **WasmComponentEngine**
   - Wasmtime 引擎封装
   - 组件模型支持

2. **WasmToolRunner**
   - AgentTool 适配器
   - 执行 WASM 组件

3. **WasmResourceLimiter**
   - 内存和表增长限制
   - 防止资源耗尽

4. **EpochTicker**
   - 超时控制
   - 后台线程递增 epoch

5. **CachingWasmToolRunner**
   - 结果缓存
   - TTL 管理

### 安全特性

- **燃料限制**: 防止无限循环
- **内存限制**: 防止内存耗尽
- **超时控制**: Epoch-based 超时
- **SSRF 防护**: HTTP 工具的 SSRF 检查
- **Secret Headers**: 主机端注入 API 密钥

---

## 📦 编译和部署

### 构建 WASM 工具
```bash
just wasm-tools
```

### 预编译 (AOT)
```bash
just wasm-precompile
```

### Feature Flags
- `wasm` - 启用 WASM 支持
- `embedded-wasm` - 嵌入 WASM 到二进制

---

## 🎯 与原生工具对比

| 特性 | WASM 工具 | 原生工具 |
|------|----------|---------|
| **隔离性** | ✅ 完全隔离 | ⚠️ 进程隔离 |
| **安全性** | ✅ 沙箱 | ⚠️ 依赖系统 |
| **性能** | ⚠️ 略慢 | ✅ 原生速度 |
| **可移植性** | ✅ 跨平台 | ⚠️ 平台相关 |
| **资源控制** | ✅ 精确限制 | ⚠️ 粗粒度 |
| **启动时间** | ✅ 快速 | ⚠️ 进程启动 |

---

## 🔍 WASM vs 原生工具

### WASM 版本
- `calc` (WASM) - 沙箱化计算
- `web_fetch` (WASM) - 沙箱化网页获取
- `web_search` (WASM) - 沙箱化搜索

### 原生版本
- `calc` (原生) - 直接计算
- `web_fetch` (原生) - 直接 HTTP 请求
- `web_search` (原生) - 直接搜索 API

### 注册逻辑
```rust
#[cfg(feature = "wasm")]
{
    if let Err(e) = clawmaster_tools::wasm_tool_runner::register_wasm_tools(
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
```

---

## 📝 测试注意事项

### WASM Feature 检查
WASM 工具需要 `wasm` feature 启用：
```bash
cargo build --features wasm
```

### 测试前提条件
1. ✅ WASM 工具已编译
2. ✅ `wasm` feature 已启用
3. ✅ 工具文件存在于正确位置

### 可能的测试问题
- ❌ Feature 未启用 → 工具不注册
- ❌ WASM 文件缺失 → 注册失败
- ❌ 资源限制过低 → 执行失败

---

## 🎯 测试策略

### 1. 检查 WASM Feature
```bash
cargo build --features wasm
```

### 2. 验证工具注册
检查日志中是否有 "wasm tool registration failed"

### 3. 测试 WASM 工具
- calc: 数学计算测试
- web_fetch: 网页获取测试
- web_search: 搜索功能测试

### 4. 对比原生工具
比较 WASM 版本和原生版本的结果

---

**总结**: ClawMaster 包含 **3 个 WASM 工具**，提供沙箱化的计算、网页获取和搜索功能。
