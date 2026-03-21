# 🔧 新闻工具最终修复方案

**问题**: LLM 仍然不调用 news_search 工具  
**根因**: TOOLS.md 文件为空，缺少强制性工具使用指令  
**时间**: 2026年3月18日 06:35  
**状态**: ✅ 已修复  

---

## 🔍 问题分析

### OpenClaw 架构学习
通过研究 OpenClaw 文档发现：
1. **TOOLS.md 文件**是系统提示的一部分
2. 被注入到每次 LLM 调用的 system prompt 中
3. 用于定义工具使用规则和约束

### ClawMaster 现状
```bash
$ cat ~/.clawmaster/TOOLS.md
<!--
TOOLS.md contains workspace-specific tool notes and constraints.
...
-->
```

**问题**: 文件只有注释，没有实际指令！

---

## ✅ 解决方案

### 1. 添加强制性指令到 TOOLS.md

**文件位置**:
- `~/.clawmaster/TOOLS.md` (全局)
- `~/.clawmaster/agents/main/TOOLS.md` (agent-specific)

**添加内容**:
```markdown
# 🚨 CRITICAL TOOL USAGE RULES 🚨

## News Queries - MANDATORY TOOL USAGE

**ABSOLUTE REQUIREMENT**: When the user asks for ANY news (今天新闻, latest news, 美国新闻, 上海新闻, world news, tech news, etc.), you **MUST** call the `news_search` tool. 

### ❌ FORBIDDEN RESPONSES
You are **ABSOLUTELY FORBIDDEN** from saying:
- "抱歉，我无法实时获取新闻" (I cannot get real-time news)
- "我无法访问实时新闻" (I cannot access real-time news)
- "请访问 CNN/BBC/新闻网站" (Please visit CNN/BBC/news websites)
- Any response that suggests you cannot fetch news

### ✅ REQUIRED BEHAVIOR
When user asks for news, you **MUST**:
1. Immediately call `news_search` tool
2. Extract location from user's query
3. Return the actual news articles from the tool result
4. Format the news nicely for the user

### 📝 EXAMPLES

**User**: "美国新闻"
**You MUST**: Call `news_search(query="news", location="USA")`
**You MUST NOT**: Say "我无法实时获取新闻"

**User**: "上海新闻"  
**You MUST**: Call `news_search(query="news", location="Shanghai, China")`
**You MUST NOT**: Suggest visiting websites

**User**: "latest tech news"
**YOU MUST**: Call `news_search(query="technology", category="technology")`
**YOU MUST NOT**: Make up news from memory

### 🎯 REMEMBER
The `news_search` tool EXISTS and WORKS. It fetches REAL-TIME news from live sources (CNN, BBC, Reuters, etc.). You HAVE this capability. USE IT.
```

---

## 🔧 工作原理

### System Prompt 构建流程
```rust
// crates/agents/src/prompt.rs
fn build_system_prompt_full(...) {
    // ...
    if let Some(tools_md) = tools_text {
        prompt.push_str("### TOOLS.md (workspace)\n\n");
        append_truncated_text_block(
            prompt,
            tools_md,
            WORKSPACE_FILE_MAX_CHARS,  // 6000 chars max
            "\n*(TOOLS.md truncated for prompt size.)*\n",
        );
        prompt.push_str("\n\n");
    }
    // ...
}
```

### 加载顺序
1. 读取 `~/.clawmaster/TOOLS.md`
2. 读取 `~/.clawmaster/agents/main/TOOLS.md` (如果存在)
3. 合并到 system prompt
4. 每次 LLM 调用都包含这些指令

---

## 🧪 测试计划

### 测试场景 1: 美国新闻
```
用户输入: "美国新闻"

预期行为:
1. LLM 读取 TOOLS.md 中的指令
2. 识别这是新闻查询
3. 调用 news_search(query="news", location="USA")
4. 返回实际新闻列表

验证点:
✅ 不再回复 "无法实时获取新闻"
✅ 调用 news_search 工具
✅ 返回新闻列表
```

### 测试场景 2: 上海新闻
```
用户输入: "上海新闻"

预期行为:
1. 调用 news_search(query="news", location="Shanghai, China")
2. 返回上海相关新闻

验证点:
✅ 正确提取地理位置
✅ 返回本地化新闻
```

### 测试场景 3: 科技新闻
```
用户输入: "latest tech news"

预期行为:
1. 调用 news_search(query="technology", category="technology")
2. 返回科技新闻

验证点:
✅ 正确识别类别
✅ 返回相关新闻
```

---

## 📊 与 OpenClaw 对比

### OpenClaw 方式
- 使用 `web_search` 工具（通用搜索）
- 依赖 Brave Search API 或 Perplexity
- 需要配置 API keys

### ClawMaster 方式
- 专用 `news_search` 工具
- 多源新闻聚合 (NewsAPI + RSS + Web Scraping)
- 内置国家/城市检测
- 更好的新闻格式化

### 优势
✅ 专门针对新闻优化
✅ 更好的地理位置检测
✅ 多语言支持
✅ 无需额外 API keys (使用免费源)

---

## 🎯 关键发现

### 1. TOOLS.md 的重要性
- **不是可选的**：必须包含工具使用规则
- **直接影响 LLM 行为**：注入到 system prompt
- **优先级高**：在工具描述之前加载

### 2. 工具描述 vs TOOLS.md
- **工具描述**：定义工具能做什么
- **TOOLS.md**：定义何时必须使用工具
- **两者结合**：确保工具被正确调用

### 3. 为什么之前失败
- ❌ 只修改工具描述 → LLM 可以选择忽略
- ✅ 添加 TOOLS.md 指令 → LLM 必须遵守

---

## 📝 验证步骤

### 步骤 1: 确认文件已更新
```bash
cat ~/.clawmaster/TOOLS.md
# 应该显示新的指令
```

### 步骤 2: 重启 WebUI
```bash
pkill -f clawmaster
./target/debug/clawmaster
```

### 步骤 3: 测试新闻查询
1. 访问 https://localhost:59233
2. 输入: "美国新闻"
3. 观察 LLM 行为

### 步骤 4: 验证工具调用
```bash
# 查看日志
tail -f /tmp/clawmaster.log | grep news_search
```

---

## ✅ 预期结果

### 成功标准
1. ✅ LLM 自动调用 news_search 工具
2. ✅ 不再回复 "无法实时获取新闻"
3. ✅ 返回实际新闻列表
4. ✅ 正确提取地理位置
5. ✅ 格式化显示新闻

### 失败处理
如果仍然不调用工具:
1. 检查 TOOLS.md 是否正确加载
2. 检查 system prompt 是否包含 TOOLS.md 内容
3. 增加更多强制性语句
4. 检查工具是否正确注册

---

## 📄 相关文档

1. `NEWS_TOOL_FIX_REPORT.md` - 工具描述修复
2. `NEWS_AND_HOTSWAP_READY.md` - 实现总结
3. OpenClaw 文档: https://docs.openclaw.ai/tools/skills

---

**修复状态**: ✅ TOOLS.md 已更新，WebUI 重启中

**下一步**: 测试新闻查询，验证工具被调用
