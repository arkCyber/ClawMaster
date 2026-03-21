# 真实 WebUI 测试结果分析

**测试时间**: 2026-03-19 09:09  
**测试输入**: "美国新闻？"  
**模型**: mistral-7b-q5_k_m

---

## 🔍 实际模型输出分析

从后端日志中提取的模型响应：

```
I am arkSong, a helpful assistant with tool-calling capabilities.

To provide you with the latest American news, I will call the news_search tool 
to fetch the articles for you. Here's the tool call: news_search(query="news", 
location="USA")

(Wait for the response from the news_search tool and provide the articles to 
the user once received).

Here's an example of a news article from a reputable source:

---
**Title:** White House Announces New Plan to Boost American Innovation
**Source:** CNN
**Summary:** The Biden administration has unveiled a new plan...
---

**Title:** NASA Prepares for Artemis I Moon Launch
**Source:** NBC News
**Summary:** NASA is gearing up for the launch...
---

**Title:** Tesla Model Y Sets New Record...
**Source:** The New York Times
**Summary:** According to recent sales data...
---
```

---

## ❌ 问题诊断

### 问题 1: 模型还在说 "I will call"
**现象**: 模型输出了 "I will call the news_search tool"

**原因**: 
1. 解释性短语检测**应该生效**，但模型没有输出 fenced block 格式
2. 模型输出的是 inline code 格式：`news_search(query="news", location="USA")`
3. 不是标准的 ````tool_call` fenced block

**结论**: 模型没有遵循系统提示词的格式要求

### 问题 2: 模型提供了训练数据中的旧新闻
**现象**: 模型编造了 3 条新闻示例（Biden、NASA、Tesla）

**原因**:
1. 没有真正调用 `news_search` 工具
2. 模型从训练数据中提取了旧新闻
3. 这些新闻**没有时间戳**（因为是编造的）

**结论**: 工具调用完全失败

### 问题 3: 没有时间戳
**现象**: 新闻内容中没有 "时间:" 字段

**原因**: 因为这些是模型编造的示例，不是真实工具调用的结果

---

## 🎯 根本原因

### 模型输出格式不符合预期

**预期格式**:
```
```tool_call
{"tool": "news_search", "arguments": {"query": "news", "location": "USA"}}
```
```

**实际输出**:
```
news_search(query="news", location="USA")
```

### 为什么会这样？

1. **模型能力限制**: Mistral 7B Instruct 可能不够强大，无法完全遵循复杂的格式要求
2. **系统提示词不够强**: 虽然我们添加了很多指令，但模型仍然选择用自然语言解释
3. **Grammar 约束未生效**: 本地 LLM 的 GBNF grammar 可能没有正确约束输出格式

---

## 🔧 需要的修复

### 修复 1: 检查 Grammar 约束
**位置**: `crates/providers/src/local_llm/mod.rs:297-300`

需要确认：
- Grammar sampler 是否正确启用
- GBNF 规则是否正确生成
- 模型是否支持 grammar 约束

### 修复 2: 增强系统提示词
需要更明确地告诉模型：
- 不要说 "I will call"
- 不要提供示例新闻
- 必须输出 fenced block 格式

### 修复 3: 添加后处理逻辑
如果模型输出了 inline code 格式的工具调用，需要：
1. 检测 `news_search(...)` 模式
2. 解析参数
3. 转换为标准工具调用

---

## 📊 测试结果总结

| 检查项 | 预期 | 实际 | 状态 |
|--------|------|------|------|
| 直接输出工具调用 | ✅ | ❌ 输出了解释 | 失败 |
| 不包含 "I will call" | ✅ | ❌ 包含 | 失败 |
| 调用真实工具 | ✅ | ❌ 编造新闻 | 失败 |
| 包含时间戳 | ✅ | ❌ 无时间戳 | 失败 |
| 使用中文回答 | ✅ | ❌ 使用英文 | 失败 |

**成功率**: 0/5 (0%)

---

## 🚨 关键发现

### 我们的修复**部分有效**

1. **解释性短语检测**: ✅ 代码逻辑正确
2. **文本比例检查**: ✅ 代码逻辑正确
3. **时间戳支持**: ✅ 代码已实现

### 但是模型行为**没有改变**

问题不在我们的解析逻辑，而在于：
1. 模型没有输出正确的格式
2. 模型没有遵循系统提示词
3. 需要更强的约束机制

---

## 💡 建议的下一步

### 短期修复（立即可做）

1. **添加 inline code 解析器**
   - 检测 `tool_name(arg1="value1", arg2="value2")` 模式
   - 解析为标准工具调用
   - 位置: `crates/agents/src/tool_parsing.rs`

2. **增强 Grammar 约束**
   - 确保 GBNF grammar 正确应用
   - 强制模型输出 fenced block 格式
   - 位置: `crates/providers/src/local_llm/tool_grammar.rs`

3. **简化系统提示词**
   - 使用更简单、更直接的指令
   - 减少复杂的示例
   - 使用模型更容易理解的语言

### 中期改进（需要更多工作）

1. **切换到更强的模型**
   - Llama 3.1 8B Instruct
   - Mistral 7B v0.3（更新版本）
   - Claude 3.5 Sonnet（API）

2. **实现多轮对话修正**
   - 检测到格式错误时，自动重试
   - 提供格式修正提示

3. **添加工具调用验证**
   - 在执行前验证工具调用格式
   - 自动修正常见错误

---

## 结论

虽然我们的**代码修复是正确的**，但由于**模型能力限制**，实际效果不理想。

**核心问题**: Mistral 7B Instruct 模型不够强大，无法完全遵循复杂的工具调用格式要求。

**推荐方案**: 
1. 添加 inline code 解析器（快速修复）
2. 切换到更强的模型（长期方案）
3. 增强 Grammar 约束（中期方案）
