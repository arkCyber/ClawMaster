# ClawMaster 工具调用测试 - 当前状态总结

**更新时间**: 2026-03-20 20:30  
**测试环境**: ClawMaster v0.10.18 + Llama 3.1 8B Q4_K_M

---

## 当前问题诊断

### 1. 核心问题

**LLM 仍然在解释工具而不是调用工具**

示例输出：
```
用户: "列出当前目录的文件"
LLM: "你可以使用以下命令列出当前目录的文件：`sandobox ls`"
```

### 2. 已完成的工作

#### 代码修改
1. ✅ 修改了 `LocalGgufProvider::supports_tools()` 为 `false`
2. ✅ 修改了 `LazyLocalGgufProvider::supports_tools()` 为 `false`
3. ✅ 保持 `tool_mode()` 返回 `Some(ToolMode::Text)`
4. ✅ 采用了 OpenClaw 的简洁 "Do Not Narrate" prompt 风格

#### 测试结果
- **初始测试**: 10% 工具调用成功率（原生模式问题）
- **修改后测试**: 40% 工具调用成功率（部分改善）
- **真实场景测试**: 0% 成功率（所有测试超时，后端重启中）

### 3. 日志分析

从后端日志看到：
```
native_tools=true  ← 问题！应该是 false
tool_calls_count=0
```

**根本原因**: 虽然修改了代码，但系统仍然使用 `native_tools=true`，这意味着：
- LLM 被要求使用原生工具调用 API
- 但 Llama 3.1 8B 不支持原生工具调用
- 导致 LLM 只能用文本解释工具

### 4. OpenClaw 源码对比

从 OpenClaw 的 `system-prompt.ts` 学到：

#### Tool Call Style (第 462-470 行)
```typescript
"## Tool Call Style",
"Default: do not narrate routine, low-risk tool calls (just call the tool).",
"Narrate only when it helps: multi-step work, complex/challenging problems, sensitive actions (e.g., deletions), or when the user explicitly asks.",
"Keep narration brief and value-dense; avoid repeating obvious steps.",
```

**关键点**:
- 简洁、直接、命令式
- 不重复明显的步骤
- 只在必要时解释

#### ClawMaster 已对齐
我们的 prompt 已经采用了相同的风格，但问题不在 prompt，而在于：
1. **模型能力限制**: Llama 3.1 8B Q4_K_M 对工具调用支持有限
2. **配置问题**: `native_tools` 仍然是 `true`，需要确保是 `false`

---

## 下一步行动

### 方案 A: 验证配置生效（立即执行）

1. **检查编译后的二进制**
   ```bash
   # 确认修改已编译进去
   strings ./target/release/clawmaster | grep -i "supports_tools"
   ```

2. **检查运行时日志**
   ```bash
   # 查看 effective_tool_mode 的输出
   tail -f backend_fresh.log | grep "resolved effective tool mode"
   ```

3. **如果仍然是 native_tools=true**
   - 可能有缓存问题
   - 需要 `cargo clean` 后重新编译

### 方案 B: 强制文本模式（如果方案 A 失败）

修改 `crates/chat/src/lib.rs` 中的 `effective_tool_mode` 函数：

```rust
fn effective_tool_mode(provider: &dyn clawmaster_agents::model::LlmProvider) -> ToolMode {
    // 强制使用文本模式用于本地模型
    if provider.name() == "local-llm" {
        return ToolMode::Text;
    }
    
    // 原有逻辑...
}
```

### 方案 C: 切换到更好的模型（推荐）

如果上述方案仍然无效，根本解决方案是切换模型：

1. **Llama 3.2 3B Instruct** (本地，免费)
   ```bash
   ollama pull llama3.2:3b-instruct
   # 修改配置使用此模型
   ```

2. **Qwen 2.5 7B Instruct** (本地，更好)
   ```bash
   ollama pull qwen2.5:7b-instruct
   ```

3. **GPT-4** (云端，最佳)
   ```toml
   [chat]
   provider = "openai"
   model = "gpt-4"
   ```

---

## 测试计划

### 立即测试（方案 A）

1. 重新编译确保修改生效
2. 重启后端服务
3. 运行简单测试验证 `native_tools=false`
4. 如果成功，运行完整的真实场景测试

### 如果失败（方案 B）

1. 修改 `effective_tool_mode` 强制文本模式
2. 重新编译和测试
3. 验证工具调用成功率

### 最终方案（方案 C）

1. 切换到 Llama 3.2 3B Instruct 或更好的模型
2. 预期工具调用成功率提升到 60-80%

---

## 关键结论

**问题不在代码架构或 Prompt，而在于**:

1. **配置未生效**: `native_tools` 仍然是 `true`
2. **模型能力**: Llama 3.1 8B Q4_K_M 对工具调用支持有限

**解决路径**:
1. 确保文本模式配置生效（修改 `supports_tools` 为 `false`）
2. 如果仍然无效，强制文本模式
3. 最终方案：切换到支持工具调用的模型

**预期结果**:
- 文本模式生效后：40-50% 成功率
- 切换到更好模型后：60-90% 成功率

---

**生成时间**: 2026-03-20 20:30
