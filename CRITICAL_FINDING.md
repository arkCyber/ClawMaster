# 🚨 关键发现：LLM 明确拒绝使用工具

**时间**: 2026年3月18日 06:53  
**严重性**: 高  

---

## 💥 问题确认

### LLM 的实际回复
```
抱歉，我无法直接使用 `news_search` 工具。
但我可以建议你使用一些可靠的新闻网站，
如 CNN、The New York Times 或 BBC 获取最新的美国新闻。
```

### 日志证据
```
tool_calls_count=0
iterations=1
response=抱歉，我无法直接使用 `news_search` 工具...
```

**结论**: 
- ❌ LLM **看到了** news_search 工具
- ❌ LLM **选择不调用**它
- ❌ LLM 反而建议用户访问网站

---

## 🔍 根本原因分析

### 问题不在于：
- ✅ 工具注册 - news_search 已注册
- ✅ TOOLS.md 加载 - 文件已正确加载
- ✅ System Prompt 注入 - TOOLS.md 在 prompt 中

### 问题在于：
❌ **LLM 认为它"无法"使用工具**

这说明：
1. LLM 看到了工具列表
2. LLM 看到了 TOOLS.md 的强制性指令
3. **但 LLM 仍然认为它没有权限/能力使用工具**

---

## 💡 可能的原因

### 原因 1: System Prompt 中有冲突的指令
**假设**: 在 TOOLS.md 之前或之后，有其他指令告诉 LLM "你不能使用工具"

**验证方法**: 获取完整 system prompt，搜索：
- "cannot use tools"
- "不能使用工具"
- "无法调用"
- 任何限制性语句

### 原因 2: 工具调用格式不匹配
**假设**: LLM 期望某种格式的工具调用，但当前配置不匹配

**关键配置**: `native_tools`
- `true`: 使用 API 的 function calling
- `false`: 使用文本格式 ```tool_call```

**当前状态**: 未知，需要检查

### 原因 3: TOOLS.md 位置问题
**假设**: TOOLS.md 在 system prompt 的开头，LLM 在看到工具列表后"忘记"了

**当前顺序**:
```
1. 基础指令
2. Workspace Files (包含 TOOLS.md)
3. 工具列表
4. 其他指令
```

**问题**: TOOLS.md 在工具列表**之前**，LLM 可能在看到工具时已经忘记了强制性指令。

### 原因 4: 模型本身的限制
**假设**: Qwen 2.5 Coder 14B 虽然看到工具，但不理解如何调用

**证据**: 
- 日志显示 `tool_calls_count=0`
- LLM 明确说"无法直接使用"
- 这不是"忘记调用"，而是"认为不能调用"

---

## 🔧 解决方案

### 方案 1: 调整 TOOLS.md 位置 ⭐ 推荐
将 TOOLS.md 移到工具列表**之后**，作为最后的强调：

```rust
// 修改 build_system_prompt_full 顺序
append_available_tools_section(...);  // 先显示工具
append_workspace_files_section(...);  // 然后强调必须使用
```

**理由**: LLM 在看到工具列表后，立即看到"你必须使用这些工具"的指令。

### 方案 2: 在工具列表前添加强制性前缀
在 `append_available_tools_section` 中添加：

```rust
prompt.push_str("## Available Tools\n\n");
prompt.push_str("🚨 CRITICAL: When user asks for news, YOU MUST call news_search. ");
prompt.push_str("DO NOT say you cannot get news. The tool exists and works.\n\n");
// 然后列出工具...
```

### 方案 3: 修改工具描述本身
在 `news_tool.rs` 的 `description()` 中添加更直接的指令：

```rust
"🚨 YOU HAVE PERMISSION TO USE THIS TOOL 🚨\n\
 When user asks for news, call this tool immediately.\n\
 Do NOT say 'I cannot use this tool' - that is FALSE.\n\
 You CAN and MUST use this tool for news queries.\n\n\
 [现有描述...]"
```

### 方案 4: 检查并修复 native_tools 配置
确保 `native_tools` 设置正确：

```bash
# 检查当前配置
grep -r "native_tools" ~/.clawmaster/
```

如果是 `false`，可能需要改为 `true`，或者确保文本格式的工具调用提示正确。

---

## 🧪 立即测试

### 测试 A: 检查 native_tools
```bash
curl -s -k https://localhost:59233/api/gon | jq '.native_tools'
```

### 测试 B: 获取完整 System Prompt
需要找到正确的 API 或在代码中添加日志输出完整 prompt。

### 测试 C: 测试其他工具
```
请使用 calc 工具计算 2+2
```

如果 calc 也不能调用，说明是通用的工具调用问题。
如果 calc 能调用，说明问题特定于 news_search。

---

## 📊 下一步行动

### 立即执行
1. ✅ 确认问题：LLM 拒绝使用工具
2. ⏳ 检查 native_tools 配置
3. ⏳ 实施方案 1：调整 TOOLS.md 位置
4. ⏳ 实施方案 2：添加工具列表前缀
5. ⏳ 重新测试

### 如果仍然失败
1. 实施方案 3：修改工具描述
2. 获取完整 system prompt 人工检查
3. 测试其他工具验证通用性
4. 考虑使用 API 模型测试

---

**关键发现**: LLM 不是"忘记"调用工具，而是"认为不能"调用工具。这是一个更深层的问题，需要修改 system prompt 的结构或内容。
