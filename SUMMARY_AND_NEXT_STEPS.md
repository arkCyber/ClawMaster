# 📋 新闻工具调试总结与下一步

**时间**: 2026年3月18日 07:45  
**状态**: 深度调试中  

---

## ✅ 已完成的工作

### 1. 完整的代码审计
- ✅ news_search 工具已注册（在 34 个工具中）
- ✅ TOOLS.md 文件正确加载（1887 bytes）
- ✅ HTML 注释正确去除
- ✅ System prompt 包含 TOOLS.md

### 2. 自动化测试框架
- ✅ 创建了 `auto_test_news.py`
- ✅ 5 个真实测试案例
- ✅ 自动捕获日志和响应
- ✅ 生成 JSON 测试报告

### 3. 第一轮修复（Prompt 结构优化）
**修改文件**: `crates/agents/src/prompt.rs`

**变更 1**: 调整 TOOLS.md 位置
```rust
// 旧顺序: TOOLS.md → 工具列表
// 新顺序: 工具列表 → TOOLS.md（紧跟强调）
```

**变更 2**: 添加工具权限前缀
```rust
prompt.push_str("🚨 **YOU HAVE FULL PERMISSION TO USE ALL TOOLS...");
```

**变更 3**: 强化标题
```rust
prompt.push_str("## 🚨 CRITICAL TOOL USAGE RULES 🚨\n\n");
```

### 4. 添加调试日志
**修改文件**: `crates/chat/src/lib.rs`

```rust
// 输出 system prompt 的关键部分
debug!("System prompt tools section: {}", tools_excerpt);
debug!("System prompt CRITICAL section: {}", critical_excerpt);
```

---

## 🚨 当前问题

### LLM 仍然拒绝使用工具
```
用户: "美国新闻"
LLM: "抱歉，我无法直接使用 `news_search` 工具..."
日志: tool_calls_count=0
```

**这表明**:
- 所有修复都未生效
- LLM 认为它"没有权限"使用工具
- 问题比 prompt 结构更深

---

## 🔍 关键发现

### native_tools 配置
```rust
// crates/chat/src/lib.rs:5699
let tool_mode = effective_tool_mode(&*provider);
let native_tools = matches!(tool_mode, ToolMode::Native);
```

**tool_mode 可能的值**:
- `ToolMode::Native` → native_tools = true
- `ToolMode::Text` → native_tools = false
- `ToolMode::Off` → tools_enabled = false

**关键问题**: 
- Qwen 2.5 Coder 14B 可能不支持 `Native` 模式
- 需要切换到 `Text` 模式

---

## 🎯 下一步行动

### 步骤 1: 检查 effective_tool_mode ⏳
查看调试日志中的：
```
resolved effective tool mode
native_tools = ?
```

### 步骤 2: 测试 calc 工具 ⏳
```bash
./test_calc_tool.sh
```

**在 WebUI 中输入**: "请使用 calc 工具计算 2+2"

**判断**:
- calc 能调用 → 问题在 news_search 描述
- calc 不能调用 → 问题在工具调用机制

### 步骤 3: 根据结果修复

#### 场景 A: calc 能调用，news_search 不能
**原因**: news_search 的描述或配置有问题

**修复**: 简化 news_search 的描述
```rust
fn description(&self) -> &str {
    "Search for real-time news articles. \
     Call this when user asks for news."
}
```

#### 场景 B: 所有工具都不能调用
**原因**: 工具调用机制问题

**修复方案 1**: 强制使用 Text 模式
```rust
// 在 run_with_tools 中
let native_tools = false; // 强制文本模式
```

**修复方案 2**: 检查 provider 的 supports_tools
```rust
// 在 effective_tool_mode 中
if !provider.supports_tools() {
    return ToolMode::Text; // 降级到文本模式
}
```

**修复方案 3**: 修改 SOUL.md
```markdown
## Tool Usage

You have access to tools. When a tool matches the user's request,
you MUST call it using the ```tool_call``` format.
```

---

## 📊 诊断矩阵

| 测试 | calc 能调用 | news_search 能调用 | 诊断 | 修复 |
|------|------------|-------------------|------|------|
| A | ✅ | ✅ | 问题已解决 | 无需修复 |
| B | ✅ | ❌ | news_search 特定问题 | 简化描述 |
| C | ❌ | ❌ | 工具调用机制问题 | 切换到 Text 模式 |
| D | ❌ | ✅ | 不太可能 | 检查 calc 配置 |

---

## 🔧 备选修复方案

### 方案 1: 强制 Text 模式（最激进）
```rust
// crates/chat/src/lib.rs:5699
let native_tools = false; // 强制使用文本模式
```

### 方案 2: 在基础 prompt 中添加工具指令
```rust
let mut prompt = String::from(if include_tools {
    "You are a helpful assistant with access to tools.\n\
     When a tool matches the user's request, you MUST call it.\n\
     Use the ```tool_call``` format to invoke tools.\n\n"
} else {
    "You are a helpful assistant.\n\n"
});
```

### 方案 3: 修改 news_tool 描述为最简单版本
```rust
fn description(&self) -> &str {
    "Search for news. Required for all news queries."
}
```

### 方案 4: 在 SOUL.md 中添加工具使用哲学
编辑 `~/.clawmaster/SOUL.md`:
```markdown
## Tool Philosophy

You have tools. Use them. When user asks for news, call news_search.
When user asks for calculation, call calc. Don't say you can't - you can.
```

---

## 📝 用户操作指南

### 现在请您执行：

1. **查看调试日志**
   ```bash
   tail -100 /tmp/clawmaster_debug.log | grep "tool mode"
   ```
   
   告诉我 `native_tools` 的值

2. **测试 calc 工具**
   - 访问 https://localhost:59233
   - 输入: "请使用 calc 工具计算 2+2"
   - 告诉我 LLM 的响应

3. **根据结果决定下一步**
   - 如果 calc 能调用 → 我们修复 news_search 描述
   - 如果 calc 不能调用 → 我们切换到 Text 模式

---

## 🎯 最终目标

让这个查询成功：
```
用户: "美国新闻"
LLM: [调用 news_search 工具]
工具: [返回新闻列表]
LLM: "以下是美国最新新闻：..."
```

---

**当前状态**: 等待用户测试 calc 工具并报告结果

**预计时间**: 5-10 分钟完成最终修复
