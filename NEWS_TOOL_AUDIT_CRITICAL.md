# 🚨 新闻工具严重问题审计

**问题**: LLM 直接编造新闻内容，而不是调用 news_search 工具  
**时间**: 2026年3月17日 22:12  
**严重性**: 🔴 严重 - 用户收到虚假信息  

---

## 🔍 问题分析

### 用户请求
```
今天世界新闻
德国新闻
```

### 实际发生
❌ LLM 直接编造了新闻内容：
- "美国总统布什 WT 克森 2023年7月 4日表示..."
- "加拿大总理Justin Trudeau Tuesday 2023年7月 4日表示..."
- 这些都是**虚假信息**，日期是2023年（已过时）

### 应该发生
✅ LLM 应该调用 `news_search` 工具：
```json
{
  "tool": "news_search",
  "params": {
    "query": "world news",
    "location": "World"
  }
}
```

---

## 🔍 根本原因

### 1. 工具描述不够强制性
当前描述：
```
"Search for news articles from around the world..."
```

**问题**: 没有明确告诉LLM **必须**使用工具，而不是编造

### 2. 缺少明确的指令
工具描述中没有：
- "ALWAYS use this tool for news queries"
- "NEVER generate news from memory"
- "MUST call this tool to get real-time news"

### 3. LLM 可能认为可以从训练数据回答
LLM 可能认为可以从其训练数据中提供"新闻"

---

## 🛠️ 修复方案

### 方案 1: 增强工具描述（立即实施）
```rust
fn description(&self) -> &str {
    "**CRITICAL: ALWAYS use this tool for ANY news query. NEVER generate news from memory or training data.**\n\n\
     Search for REAL-TIME news articles from around the world. This tool fetches ACTUAL current news from live sources.\n\n\
     When user asks about news (e.g., '今天新闻', 'latest news', 'world news'), you MUST call this tool.\n\n\
     Automatically detects country from city names. Supports multiple languages and categories.\n\n\
     **DO NOT** make up news. **DO NOT** use old training data. **ALWAYS** call this tool for news queries."
}
```

### 方案 2: 添加系统提示（推荐）
在 AGENTS.md 或系统提示中添加：
```markdown
## News Queries - CRITICAL RULE

When user asks about news:
- **ALWAYS** use the `news_search` tool
- **NEVER** generate news from your training data
- **NEVER** make up news articles
- Real-time news MUST come from the tool

Examples that REQUIRE the tool:
- "今天新闻" → MUST call news_search
- "latest news" → MUST call news_search  
- "德国新闻" → MUST call news_search
- "world news" → MUST call news_search
```

### 方案 3: 添加工具使用示例
```rust
"examples": [
    {
        "user": "今天世界新闻",
        "tool_call": {
            "query": "world news",
            "location": "World"
        }
    },
    {
        "user": "德国新闻",
        "tool_call": {
            "query": "news",
            "location": "Germany"
        }
    }
]
```

---

## 📋 立即行动

### 优先级 1: 修改工具描述
- 添加 "ALWAYS use this tool"
- 添加 "NEVER generate from memory"
- 强调 "REAL-TIME" 和 "ACTUAL"

### 优先级 2: 测试验证
- 测试 "今天新闻"
- 测试 "world news"
- 验证工具被调用
- 确认返回真实新闻

### 优先级 3: 添加日志
- 记录工具是否被调用
- 记录LLM的决策过程

---

## 🧪 测试计划

### 测试 1: 简单新闻查询
```
输入: "今天新闻"
预期: 调用 news_search 工具
验证: 检查日志中的 "Searching news"
```

### 测试 2: 世界新闻
```
输入: "world news today"
预期: 调用 news_search 工具
验证: 返回真实新闻，不是编造的
```

### 测试 3: 特定国家
```
输入: "德国最新消息"
预期: 调用 news_search，country=de
验证: 返回德国新闻
```

---

## 📊 预期改进

| 指标 | 修复前 | 修复后 |
|------|--------|--------|
| 工具调用率 | 0% ❌ | 100% ✅ |
| 信息准确性 | 虚假 ❌ | 真实 ✅ |
| 用户信任 | 低 ❌ | 高 ✅ |

---

**立即修复此严重问题！** 🚨
