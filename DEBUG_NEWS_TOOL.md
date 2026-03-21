# 🔍 新闻工具调试报告

**时间**: 2026年3月18日 06:45  
**状态**: 正在调试  

---

## ✅ 已确认工作的部分

### 1. TOOLS.md 文件
- ✅ `~/.clawmaster/TOOLS.md` 存在 (1887 bytes)
- ✅ `~/.clawmaster/agents/main/TOOLS.md` 存在 (1887 bytes)
- ✅ 包含 15 处 "news" 关键词
- ✅ 内容正确（强制性新闻工具指令）

### 2. 加载逻辑
```rust
// crates/config/src/loader.rs:568
pub fn load_tools_md_for_agent(agent_id: &str) -> Option<String> {
    let agent_path = agent_workspace_dir(agent_id).join("TOOLS.md");
    load_workspace_markdown(agent_path).or_else(load_tools_md)
}

// crates/config/src/loader.rs:876
fn load_workspace_markdown(path: PathBuf) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    let trimmed = strip_leading_html_comments(&content).trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
```

**工作原理**:
1. 读取 TOOLS.md 文件
2. 去除 HTML 注释（`<!-- ... -->`）
3. Trim 空白字符
4. 返回内容

### 3. System Prompt 注入
- ✅ 测试确认：system prompt 包含 "TOOLS.md"
- ✅ 代码路径：`crates/agents/src/prompt.rs:560-588`

```rust
fn append_workspace_files_section(
    prompt: &mut String,
    agents_text: Option<&str>,
    tools_text: Option<&str>,
) {
    if agents_text.is_none() && tools_text.is_none() {
        return;
    }
    
    prompt.push_str("## Workspace Files\n\n");
    if let Some(tools_md) = tools_text {
        prompt.push_str("### TOOLS.md (workspace)\n\n");
        append_truncated_text_block(
            prompt,
            tools_md,
            WORKSPACE_FILE_MAX_CHARS,  // 6000 chars
            "\n*(TOOLS.md truncated for prompt size.)*\n",
        );
        prompt.push_str("\n\n");
    }
}
```

### 4. 调用链
```
load_prompt_persona_for_agent("main")
  ↓
load_tools_md_for_agent("main")
  ↓
load_workspace_markdown(~/.clawmaster/agents/main/TOOLS.md)
  ↓
strip_leading_html_comments(content)
  ↓
返回: "# 🚨 CRITICAL TOOL USAGE RULES 🚨\n\n## News Queries..."
  ↓
build_system_prompt_with_session_runtime(...)
  ↓
append_workspace_files_section(..., tools_text)
  ↓
System Prompt 包含 TOOLS.md 内容
```

---

## ❓ 待验证的问题

### 问题 1: HTML 注释去除
**代码**: `strip_leading_html_comments()`

**当前 TOOLS.md 开头**:
```markdown
<!--
TOOLS.md contains workspace-specific tool notes and constraints.
...
-->

# 🚨 CRITICAL TOOL USAGE RULES 🚨
```

**问题**: 
- HTML 注释是否被正确去除？
- 去除后的内容是否以 `# 🚨 CRITICAL TOOL USAGE RULES 🚨` 开头？

**验证方法**:
```rust
// 检查 strip_leading_html_comments 实现
// 确保它正确处理多行 HTML 注释
```

### 问题 2: 内容截断
**限制**: `WORKSPACE_FILE_MAX_CHARS = 6000`

**当前文件大小**: 1887 bytes

**状态**: ✅ 不会被截断（远小于 6000）

### 问题 3: LLM 是否真的看到了指令
**测试方法**:
1. 获取完整的 system prompt
2. 搜索 "MANDATORY TOOL USAGE"
3. 搜索 "FORBIDDEN RESPONSES"
4. 确认这些关键词在 prompt 中

**当前状态**: 
- ✅ system prompt 包含 "TOOLS.md"
- ❓ 但不确定完整内容是否正确

---

## 🧪 下一步测试

### 测试 1: 验证 HTML 注释去除
```bash
# 检查去除注释后的内容
cat ~/.clawmaster/TOOLS.md | sed '/^<!--/,/^-->/d' | head -5
```

**预期输出**:
```
# 🚨 CRITICAL TOOL USAGE RULES 🚨

## News Queries - MANDATORY TOOL USAGE
```

### 测试 2: 获取完整 System Prompt
```bash
# 通过 API 获取
curl -k -X POST https://localhost:59233/api/chat.system_prompt \
    -H "Content-Type: application/json" \
    -d '{"_session_key":"test"}' | jq -r '.prompt' > /tmp/system_prompt.txt

# 检查关键词
grep -n "MANDATORY TOOL USAGE" /tmp/system_prompt.txt
grep -n "FORBIDDEN RESPONSES" /tmp/system_prompt.txt
grep -n "news_search" /tmp/system_prompt.txt
```

### 测试 3: 实际发送新闻查询
```bash
# 通过 WebSocket 或正确的 API 端点发送
# 观察 LLM 是否调用 news_search 工具
```

### 测试 4: 检查日志
```bash
# 启动时带日志
./target/debug/clawmaster > /tmp/clawmaster.log 2>&1 &

# 发送查询后检查
tail -f /tmp/clawmaster.log | grep -E "(news|tool_call|TOOLS)"
```

---

## 🔍 可能的问题

### 假设 1: HTML 注释未正确去除
**症状**: TOOLS.md 内容被加载，但以 `<!--` 开头  
**影响**: LLM 可能将整个内容视为注释  
**验证**: 检查 `strip_leading_html_comments` 实现  

### 假设 2: 内容被截断
**症状**: 只有部分 TOOLS.md 被加载  
**影响**: 关键的 "FORBIDDEN" 部分可能丢失  
**验证**: 检查 prompt 长度和截断逻辑  

### 假设 3: LLM 忽略了指令
**症状**: 即使看到指令，LLM 仍然不调用工具  
**影响**: 需要更强的强制性语句  
**验证**: 获取完整 prompt 并人工检查  

### 假设 4: 工具未正确注册
**症状**: news_search 工具不在可用工具列表中  
**影响**: LLM 无法调用  
**验证**: 检查工具注册日志  

---

## 📊 测试结果

### 自动测试脚本结果
```
✅ TOOLS.md 文件存在
✅ 进程运行中
✅ API 可访问
✅ system prompt 包含 TOOLS.md
❌ chat.send API 返回 "not found"
⚠️  日志文件不存在（使用了错误的路径）
```

### 关键发现
1. **TOOLS.md 正确加载** - 文件存在且大小正确
2. **System Prompt 包含 TOOLS.md** - 注入成功
3. **API 端点问题** - chat.send 可能不是正确的端点
4. **日志路径** - 需要使用正确的日志输出

---

## 🎯 下一步行动

### 立即执行
1. ✅ 重启 ClawMaster 并输出到 /tmp/clawmaster.log
2. ⏳ 检查工具注册日志
3. ⏳ 通过 WebUI 手动测试新闻查询
4. ⏳ 观察日志中的工具调用
5. ⏳ 获取完整 system prompt 验证内容

### 如果仍然失败
1. 检查 `strip_leading_html_comments` 实现
2. 增加更多强制性语句
3. 将 TOOLS.md 指令移到工具描述中
4. 考虑使用 AGENTS.md 替代

---

**当前状态**: 正在重启 ClawMaster 并收集日志
