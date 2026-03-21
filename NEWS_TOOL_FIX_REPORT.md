# 🔧 新闻工具修复报告

**问题**: LLM 不调用新闻工具，直接回复"无法实时获取新闻"  
**时间**: 2026年3月17日 23:50  
**状态**: ✅ 已修复  

---

## 🐛 问题分析

### 用户查询
- "美国新闻" → LLM 回复: "抱歉，我无法实时获取新闻..."
- "上海新闻" → LLM 回复: "抱歉，我无法实时获取新闻..."

### 根因
虽然新闻工具已注册且描述中包含"CRITICAL"和"MANDATORY"，但：
1. LLM 仍然选择不调用工具
2. 工具描述不够强制性
3. 缺少明确的"禁止"语句

---

## ✅ 修复方案

### 1. 增强工具描述
**文件**: `crates/tools/src/news_tool.rs`

#### 修复前
```rust
fn description(&self) -> &str {
    "**CRITICAL: ALWAYS use this tool for ANY news query...**"
}
```

#### 修复后
```rust
fn description(&self) -> &str {
    "🚨 **MANDATORY TOOL - YOU MUST USE THIS FOR ALL NEWS QUERIES** 🚨\n\n\
     ⚠️ **CRITICAL INSTRUCTION**: If user asks ANYTHING about news (今天新闻, latest news, 上海新闻, 美国新闻, world news, etc.), \
     you are FORBIDDEN from answering without calling this tool first. You CANNOT say 'I cannot get real-time news' - that is FALSE. \
     This tool EXISTS to fetch real-time news. USE IT.\n\n\
     **WHAT THIS TOOL DOES**:\n\
     - Fetches REAL-TIME news from live internet sources (CNN, BBC, Reuters, etc.)\n\
     - Supports ALL countries: China (cn), USA (us), Germany (de), Japan (jp), UK (gb), France (fr), Korea (kr), etc.\n\
     - Supports ALL cities: Shanghai (上海), Beijing (北京), New York, Tokyo (東京), London, Paris, Berlin, etc.\n\
     - Returns actual news articles with titles, descriptions, sources, and links\n\n\
     **MANDATORY USAGE EXAMPLES**:\n\
     - User: '美国新闻' → YOU MUST call: news_search(query='news', location='USA')\n\
     - User: '上海新闻' → YOU MUST call: news_search(query='news', location='Shanghai, China')\n\
     - User: 'latest tech news' → YOU MUST call: news_search(query='technology', category='technology')\n\
     - User: 'world news' → YOU MUST call: news_search(query='world news')\n\n\
     **ABSOLUTELY FORBIDDEN**:\n\
     ❌ Saying 'I cannot get real-time news' - THIS IS A LIE\n\
     ❌ Suggesting to visit websites manually - USE THIS TOOL INSTEAD\n\
     ❌ Making up news from memory\n\
     ❌ Using old training data\n\
     ❌ Answering news queries without calling this tool\n\n\
     **IF USER ASKS FOR NEWS, YOUR ONLY VALID RESPONSE IS TO CALL THIS TOOL IMMEDIATELY.**"
}
```

### 关键改进
1. ✅ **视觉强调**: 使用 🚨 和 ⚠️ emoji
2. ✅ **明确禁止**: "FORBIDDEN from answering without calling this tool"
3. ✅ **纠正错误**: "You CANNOT say 'I cannot get real-time news' - that is FALSE"
4. ✅ **具体示例**: 提供中英文查询示例
5. ✅ **强制指令**: "YOUR ONLY VALID RESPONSE IS TO CALL THIS TOOL"

---

## 🧪 测试计划

### 测试场景 1: 美国新闻
```
用户输入: "美国新闻"

预期行为:
1. LLM 调用 news_search 工具
2. 参数: query="news", location="USA"
3. 返回实际新闻列表
4. 格式化显示给用户

禁止行为:
❌ 回复 "无法实时获取新闻"
❌ 建议访问网站
❌ 从记忆生成新闻
```

### 测试场景 2: 上海新闻
```
用户输入: "上海新闻"

预期行为:
1. LLM 调用 news_search 工具
2. 参数: query="news", location="Shanghai, China"
3. 返回上海相关新闻
4. 格式化显示

禁止行为:
❌ 回复 "无法实时获取新闻"
```

### 测试场景 3: 科技新闻
```
用户输入: "latest tech news"

预期行为:
1. LLM 调用 news_search 工具
2. 参数: query="technology", category="technology"
3. 返回科技新闻
4. 格式化显示
```

---

## 📊 验证检查清单

### 编译验证
- [ ] cargo build -p clawmaster-tools 成功
- [ ] 无编译错误
- [ ] 无编译警告

### 运行时验证
- [ ] WebUI 启动成功
- [ ] 工具已注册
- [ ] 工具描述正确加载

### 功能验证
- [ ] "美国新闻" 调用工具
- [ ] "上海新闻" 调用工具
- [ ] 返回实际新闻
- [ ] 格式化正确

---

## 🔍 调试信息

### 检查工具注册
```bash
# 查看日志确认工具已注册
grep "news_search" /tmp/clawmaster.log

# 预期输出:
# INFO tool registered: news_search
```

### 检查工具调用
```bash
# 查看工具调用日志
grep "tool_call.*news" /tmp/clawmaster.log

# 预期输出:
# INFO tool_call: news_search args={"query":"news","location":"USA"}
```

---

## 📝 测试步骤

### 步骤 1: 重新编译
```bash
cargo build -p clawmaster-tools
```

### 步骤 2: 重启 WebUI
```bash
pkill -f clawmaster
./target/debug/clawmaster
```

### 步骤 3: 测试查询
1. 访问 https://localhost:59233
2. 输入: "美国新闻"
3. 观察 LLM 是否调用工具
4. 验证返回新闻列表

### 步骤 4: 验证日志
```bash
# 查看工具调用
tail -f /tmp/clawmaster.log | grep news
```

---

## ✅ 预期结果

### 成功标准
1. ✅ LLM 自动调用 news_search 工具
2. ✅ 不再回复 "无法实时获取新闻"
3. ✅ 返回实际新闻列表
4. ✅ 格式化显示正确

### 失败处理
如果仍然不调用工具:
1. 检查工具是否正确注册
2. 检查 LLM 系统提示
3. 检查工具描述是否正确加载
4. 增加更多强制性语句

---

**修复状态**: ✅ 代码已修复，等待测试验证
