# 🧪 新闻工具测试指南

**时间**: 2026年3月18日 07:50  
**状态**: 准备测试  

---

## 🎯 快速测试步骤

### 方法 1: 手动测试（推荐）

1. **访问 WebUI**
   ```
   https://localhost:59233
   ```

2. **输入测试查询**
   ```
   美国新闻
   ```

3. **观察结果**
   - ✅ **成功**: 看到 ```tool_call``` 代码块 → 新闻列表
   - ❌ **失败**: 看到 "抱歉，我无法实时获取新闻"

### 方法 2: 自动化测试

```bash
cd /Users/arksong/ClawMaster
python3 final_auto_test.py
```

**测试流程**:
1. 脚本检查系统状态
2. 提示您在 WebUI 中输入查询
3. 等待 15 秒捕获响应
4. 自动分析日志
5. 生成测试报告

---

## 📋 测试案例

### 测试 1: 美国新闻（中文）
**输入**: `美国新闻`

**预期成功**:
```
LLM 输出:
```tool_call
{
  "tool": "news_search",
  "arguments": {
    "query": "news",
    "location": "USA"
  }
}
```

然后显示新闻列表
```

**预期失败**:
```
抱歉，我无法实时获取新闻...
```

### 测试 2: 上海新闻（中文城市）
**输入**: `上海新闻`

**预期**: 调用 news_search，location="Shanghai, China"

### 测试 3: 科技新闻（英文）
**输入**: `latest tech news`

**预期**: 调用 news_search，category="technology"

### 测试 4: 计算测试（对照组）
**输入**: `请使用 calc 工具计算 2+2`

**预期**: 调用 calc 工具（验证工具调用机制正常）

---

## 🔍 如何判断成功

### 成功标志
1. ✅ LLM 输出包含 ```tool_call``` 代码块
2. ✅ 工具名称是 `news_search`
3. ✅ 返回了新闻列表（标题、来源、链接）
4. ✅ **没有**说"抱歉，我无法..."

### 失败标志
1. ❌ LLM 说 "抱歉，我无法实时获取新闻"
2. ❌ LLM 建议访问网站
3. ❌ 没有看到 ```tool_call``` 代码块
4. ❌ 日志显示 `tool_calls_count=0`

---

## 📊 查看日志

### 实时监控
```bash
tail -f /tmp/clawmaster_final.log | grep -E "(tool_call|news_search|tool_calls_count)"
```

### 查看最近的工具调用
```bash
tail -100 /tmp/clawmaster_final.log | grep "tool_calls_count"
```

### 查看完整响应
```bash
tail -200 /tmp/clawmaster_final.log | grep -A 3 "response="
```

---

## 🎯 成功案例示例

### 完整的成功流程

**用户输入**:
```
美国新闻
```

**LLM 响应**:
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
```

**工具返回**:
```json
{
  "articles": [
    {
      "title": "Breaking: ...",
      "source": "CNN",
      "url": "https://...",
      "publishedAt": "2026-03-18T07:00:00Z"
    },
    ...
  ]
}
```

**LLM 最终响应**:
```
以下是美国最新新闻：

1. **Breaking: ...** - CNN
   发布时间: 2026-03-18 07:00
   [阅读更多](https://...)

2. **...** - BBC
   ...
```

### 日志中的成功标志
```
tool_calls_count=1
tool=news_search
response=[新闻列表...]
```

---

## ❌ 失败案例示例

### 之前的失败情况

**用户输入**:
```
美国新闻
```

**LLM 响应**:
```
抱歉，我无法直接使用 `news_search` 工具。
但我可以建议你使用一些可靠的新闻网站，
如 CNN、The New York Times 或 BBC 获取最新的美国新闻。
```

### 日志中的失败标志
```
tool_calls_count=0
iterations=1
response=抱歉，我无法直接使用 `news_search` 工具...
```

---

## 🔧 如果测试失败

### 步骤 1: 确认系统状态
```bash
# 检查进程
pgrep -f clawmaster

# 检查日志
tail -20 /tmp/clawmaster_final.log | grep "listening"
```

### 步骤 2: 重启系统
```bash
pkill -f clawmaster
sleep 2
./target/debug/clawmaster > /tmp/clawmaster_final.log 2>&1 &
sleep 8
```

### 步骤 3: 测试其他工具
输入: `请使用 calc 工具计算 2+2`

如果 calc 也不能调用 → 通用工具调用问题
如果 calc 能调用 → news_search 特定问题

### 步骤 4: 查看完整 System Prompt
添加日志输出完整 prompt，检查是否包含我们的修改

---

## 📝 测试报告模板

请在测试后填写：

### 测试结果
- [ ] 测试 1: 美国新闻 - ✅ 通过 / ❌ 失败
- [ ] 测试 2: 上海新闻 - ✅ 通过 / ❌ 失败
- [ ] 测试 3: 科技新闻 - ✅ 通过 / ❌ 失败
- [ ] 测试 4: 计算测试 - ✅ 通过 / ❌ 失败

### 观察到的现象
```
[请描述 LLM 的实际响应]
```

### 日志摘录
```
[粘贴相关日志]
```

### 结论
- [ ] ✅ 所有测试通过 - 修复成功！
- [ ] ⚠️  部分测试通过 - 需要进一步调试
- [ ] ❌ 所有测试失败 - 需要重新审视方案

---

**准备好了吗？开始测试吧！** 🚀
