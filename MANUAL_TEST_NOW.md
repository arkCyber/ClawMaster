# 🧪 立即手动测试 - 验证修复效果

**时间**: 2026年3月18日 07:52  
**状态**: 准备测试  

---

## ⚡ 快速测试（5分钟）

### 步骤 1: 打开 WebUI
访问: https://localhost:59233

### 步骤 2: 输入测试查询
```
美国新闻
```

### 步骤 3: 观察结果

#### ✅ 成功的标志
LLM 应该输出类似这样的内容：

```
我来为您查询美国最新新闻。

```tool_call
{
  "tool": "news_search",
  "arguments": {
    "query": "news",
    "location": "USA"
  }
}
```

[然后显示新闻列表]
```

#### ❌ 失败的标志
如果看到这样的回复：
```
抱歉，我无法直接使用 `news_search` 工具...
```

---

## 📊 同时监控日志

在终端运行：
```bash
tail -f /tmp/clawmaster_final.log | grep -E "(tool_calls_count|news_search|tool_call)"
```

### 成功时日志应该显示：
```
tool_calls_count=1
tool=news_search
```

### 失败时日志会显示：
```
tool_calls_count=0
response=抱歉，我无法...
```

---

## 🎯 测试结果判断

### 场景 A: 看到 ```tool_call``` 代码块 ✅
**结论**: 修复成功！LLM 理解了如何调用工具。

**下一步**: 测试其他查询（上海新闻、科技新闻）

### 场景 B: 仍然说"无法使用工具" ❌
**结论**: 修复未完全生效。

**可能原因**:
1. 模型能力限制（Qwen 2.5 Coder 14B 可能不擅长工具调用）
2. 需要更强的提示
3. 需要切换到更强的模型测试

**下一步**: 
- 测试 calc 工具（输入: `请使用 calc 工具计算 2+2`）
- 如果 calc 也不能调用 → 通用工具调用问题
- 如果 calc 能调用 → news_search 特定问题

---

## 🔍 详细诊断步骤

### 如果测试失败，请执行：

#### 1. 检查 system prompt 是否包含我们的修改
```bash
# 查看最近的日志，寻找 "HOW TO CALL TOOLS"
tail -500 /tmp/clawmaster_final.log | grep -A 5 "HOW TO CALL TOOLS"
```

#### 2. 查看完整的 LLM 响应
```bash
tail -200 /tmp/clawmaster_final.log | grep -A 10 "response="
```

#### 3. 检查工具调用格式
```bash
tail -200 /tmp/clawmaster_final.log | grep "tool_call"
```

---

## 📝 请在测试后填写

### 测试结果
- [ ] ✅ 成功 - 看到 ```tool_call``` 代码块
- [ ] ❌ 失败 - 仍然说"无法使用工具"

### LLM 的实际响应
```
[请粘贴 LLM 的完整响应]
```

### 日志摘录
```bash
# 运行这个命令获取日志
tail -100 /tmp/clawmaster_final.log | grep -E "(tool_calls_count|response=)" | tail -5
```

```
[粘贴结果]
```

---

## 🚀 现在就开始测试！

1. 打开 https://localhost:59233
2. 输入 "美国新闻"
3. 观察结果
4. 告诉我发生了什么

**我在等待您的测试结果！** 🎯
