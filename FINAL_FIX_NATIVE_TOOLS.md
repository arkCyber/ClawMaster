# 🎯 最终修复：强制 Text 模式

**时间**: 2026年3月18日 08:18  
**问题**: `native_tools=true` 但应该是 `false`  

---

## 🔍 问题分析

### 当前状态
```
model="local-llm::llama-3.2-1b-q4_k_m"
native_tools=true  ← 问题在这里
tool_calls_count=0
```

### 代码已经设置了 Text 模式
`crates/providers/src/local_gguf/mod.rs`:
```rust
fn tool_mode(&self) -> Option<ToolMode> {
    Some(ToolMode::Text)  // 已经设置
}
```

### 但是 effective_tool_mode 可能有问题
`crates/chat/src/lib.rs` 中的 `effective_tool_mode` 函数可能：
1. 检查 `supports_tools()` 返回 `true`
2. 忽略了 `tool_mode()` 的 `Text` 设置
3. 默认使用 Native 模式

---

## 🔧 解决方案

### 修改 effective_tool_mode 逻辑

确保当 `tool_mode()` 返回 `Some(ToolMode::Text)` 时，
`native_tools` 应该是 `false`。

**预期逻辑**:
```rust
fn effective_tool_mode(provider: &dyn LlmProvider) -> ToolMode {
    let declared_mode = provider.tool_mode();
    let supports_tools = provider.supports_tools();

    match declared_mode {
        Some(ToolMode::Text) => ToolMode::Text,  // 强制 Text
        Some(ToolMode::Native) => ToolMode::Native,
        Some(ToolMode::Off) => ToolMode::Off,
        Some(ToolMode::Auto) | None => {
            if supports_tools {
                ToolMode::Native  // 默认 Native
            } else {
                ToolMode::Text
            }
        }
    }
}
```

然后：
```rust
let native_tools = effective_mode == ToolMode::Native;
```

---

## 🎯 快速验证

### 检查当前逻辑
```bash
grep -A 30 "fn effective_tool_mode" /Users/arksong/ClawMaster/crates/chat/src/lib.rs
```

### 如果逻辑正确但仍然 native_tools=true
可能是：
1. 缓存问题 - 需要完全重新编译
2. 配置覆盖 - 检查配置文件
3. 其他地方设置了 native_tools

---

## 💡 临时解决方案

如果修改 effective_tool_mode 太复杂，可以：

### 方案 A: 在 LocalGgufProvider 中强制返回 false
```rust
fn supports_tools(&self) -> bool {
    false  // 临时强制 Text 模式
}
```

这样 `effective_tool_mode` 会返回 `ToolMode::Text`。

### 方案 B: 修改 build_system_prompt_full
强制使用 Text 模式的 prompt：
```rust
let native_tools = false;  // 强制 Text 模式
```

---

**建议**: 先检查 effective_tool_mode 的实现，确认逻辑是否正确。
