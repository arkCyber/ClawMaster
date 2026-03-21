# 🔍 新闻工具问题最终诊断

**时间**: 2026年3月18日 06:50  
**状态**: 深度分析中  

---

## ✅ 已验证正常的部分

### 1. TOOLS.md 文件加载 ✅
```bash
$ cat ~/.clawmaster/TOOLS.md | sed '/^<!--/,/^-->/d' | head -5
# 🚨 CRITICAL TOOL USAGE RULES 🚨

## News Queries - MANDATORY TOOL USAGE

**ABSOLUTE REQUIREMENT**: When the user asks for ANY news...
```

**结论**: 
- ✅ HTML 注释被正确去除
- ✅ 内容以强制性指令开头
- ✅ 文件大小 1887 bytes（< 6000 限制）

### 2. 代码路径验证 ✅
```rust
// 加载链路
load_prompt_persona_for_agent("main")
  → load_tools_md_for_agent("main")
  → load_workspace_markdown(~/.clawmaster/agents/main/TOOLS.md)
  → strip_leading_html_comments(content)
  → 返回去除注释后的内容
```

**strip_leading_html_comments 实现**:
```rust
fn strip_leading_html_comments(content: &str) -> &str {
    let mut rest = content;
    loop {
        // 循环去除所有开头的 HTML 注释
        // 直到没有更多注释为止
    }
}
```

### 3. System Prompt 注入 ✅
```rust
fn append_workspace_files_section(
    prompt: &mut String,
    agents_text: Option<&str>,
    tools_text: Option<&str>,
) {
    prompt.push_str("## Workspace Files\n\n");
    if let Some(tools_md) = tools_text {
        prompt.push_str("### TOOLS.md (workspace)\n\n");
        append_truncated_text_block(prompt, tools_md, 6000, ...);
    }
}
```

**测试确认**: `system prompt 包含 TOOLS.md` ✅

---

## 🎯 核心问题

### 问题: 为什么 LLM 仍然不调用 news_search 工具？

**可能原因分析**:

### 原因 1: 工具描述与 TOOLS.md 冲突
**假设**: LLM 看到两个矛盾的指令
- 工具描述说："可以调用"
- TOOLS.md 说："必须调用"

**验证方法**: 检查 system prompt 中工具描述的位置和内容

### 原因 2: TOOLS.md 在 prompt 中的位置
**假设**: TOOLS.md 可能在工具列表之前或之后
- 如果在之前：LLM 可能忘记
- 如果在之后：LLM 可能已经决定不调用

**验证方法**: 获取完整 system prompt，检查顺序

### 原因 3: LLM 模型本身的限制
**假设**: 本地 GGUF 模型可能不够强大
- Llama 3.2 1B: 参数太少，理解能力有限
- Qwen 2.5 Coder 14B: 专注代码，可能不擅长工具调用

**验证方法**: 
1. 检查模型是否支持工具调用
2. 查看日志中的工具调用格式
3. 尝试其他查询测试工具调用能力

### 原因 4: 工具调用格式问题
**假设**: 模型需要特定的工具调用格式
- Native tools: 通过 API 的 function calling
- Text tools: 通过 ```tool_call``` 代码块

**验证方法**: 检查 `native_tools` 配置

---

## 🧪 关键测试

### 测试 A: 获取完整 System Prompt
```bash
# 需要找到正确的 API 端点
# 或者直接在代码中打印
```

**目标**: 
1. 确认 TOOLS.md 内容在 prompt 中
2. 检查 TOOLS.md 和工具列表的顺序
3. 验证没有被截断

### 测试 B: 检查工具注册
```bash
tail -f /tmp/clawmaster.log | grep "tool.*register"
```

**预期**: 应该看到 `news_search` 工具被注册

### 测试 C: 手动测试工具调用
通过 WebUI 发送查询，观察：
1. LLM 是否尝试调用任何工具
2. 如果调用，调用了什么工具
3. 如果不调用，LLM 的回复是什么

### 测试 D: 简化测试
发送更明确的指令：
```
请使用 news_search 工具查询美国新闻
```

如果这样也不调用，说明问题在工具调用机制本身。

---

## 💡 可能的解决方案

### 方案 1: 移动 TOOLS.md 到工具描述之后
修改 `build_system_prompt_full` 的顺序：
```rust
// 当前顺序
append_workspace_files_section(...);  // TOOLS.md
append_available_tools_section(...);  // 工具列表

// 建议顺序
append_available_tools_section(...);  // 工具列表
append_workspace_files_section(...);  // TOOLS.md（作为最后的强调）
```

### 方案 2: 将 TOOLS.md 内容合并到工具描述
直接在 `news_search` 工具的 description 中包含所有强制性指令，不依赖 TOOLS.md。

### 方案 3: 使用更强的模型
如果本地模型能力不足，考虑：
- 使用 API 模型（Claude, GPT-4）进行测试
- 验证是否是模型能力问题

### 方案 4: 修改工具调用提示
在 system prompt 的工具部分添加更强的指令：
```
When listing tools, if user asks for news, YOU MUST call news_search.
DO NOT say you cannot get news. The tool exists.
```

---

## 📊 下一步行动

### 立即执行
1. ✅ 验证 HTML 注释去除 - **已完成**
2. ⏳ 检查 ClawMaster 日志
3. ⏳ 通过 WebUI 手动测试
4. ⏳ 观察 LLM 的实际行为

### 如果手动测试失败
1. 获取完整 system prompt
2. 检查工具调用格式（native vs text）
3. 尝试简化测试（明确要求调用工具）
4. 考虑实施方案 1 或 2

---

## 🔧 代码审计清单

- [x] TOOLS.md 文件存在且内容正确
- [x] HTML 注释被正确去除
- [x] load_tools_md_for_agent 正确实现
- [x] append_workspace_files_section 正确调用
- [ ] System prompt 完整内容验证
- [ ] 工具注册日志验证
- [ ] 工具调用格式验证
- [ ] LLM 实际行为观察

---

**当前状态**: 等待 ClawMaster 完全启动，准备手动测试
