# 🎯 重大突破！找到根本原因

**时间**: 2026年3月18日 07:50  
**状态**: 问题根源已定位  

---

## 💡 关键发现

### LocalGgufProvider 使用 Text 模式
```rust
// crates/providers/src/local_gguf/mod.rs
fn tool_mode(&self) -> Option<ToolMode> {
    Some(ToolMode::Text)  // ← 关键！
}

fn supports_tools(&self) -> bool {
    true
}
```

### effective_tool_mode 逻辑
```rust
// crates/chat/src/lib.rs:5626-5666
fn effective_tool_mode(provider: &dyn LlmProvider) -> ToolMode {
    let declared_mode = provider.tool_mode();  // Some(ToolMode::Text)
    let supports_tools = provider.supports_tools();  // true
    
    match declared_mode {
        Some(ToolMode::Text) => ToolMode::Text,  // ← 走这里
        // ...
    }
}
```

**结论**: 
- ✅ LocalGgufProvider 明确返回 `ToolMode::Text`
- ✅ 因此 `native_tools = false`
- ✅ 工具调用应该使用文本格式：```tool_call```

---

## 🔍 Text 模式的工具调用格式

### LLM 需要输出的格式
```
```tool_call
{
  "name": "news_search",
  "arguments": {
    "query": "news",
    "location": "USA"
  }
}
```
```

### System Prompt 应该包含的指导
```rust
fn tool_call_guidance(model_id: Option<&str>) -> String {
    // 应该告诉 LLM 如何使用 ```tool_call``` 格式
}
```

---

## 🚨 问题所在

### LLM 可能不理解 ```tool_call``` 格式
**原因**:
1. Qwen 2.5 Coder 14B 可能没有在这种格式上训练
2. System prompt 中的工具调用指导可能不够清晰
3. LLM 看到工具列表，但不知道如何调用

### 证据
```
LLM 响应: "抱歉，我无法直接使用 `news_search` 工具..."
```

这说明：
- ✅ LLM 看到了工具
- ✅ LLM 知道工具的名字
- ❌ LLM 不知道如何调用它

---

## 🔧 解决方案

### 方案 1: 增强 tool_call_guidance ⭐ 推荐
在 `crates/agents/src/prompt.rs` 中修改 `tool_call_guidance`:

```rust
fn tool_call_guidance(model_id: Option<&str>) -> String {
    format!(
        "\n## How to Call Tools\n\n\
         When you need to use a tool, output a code block with the exact format:\n\n\
         \\```tool_call\n\
         {{\n\
           \"name\": \"tool_name\",\n\
           \"arguments\": {{\n\
             \"param1\": \"value1\",\n\
             \"param2\": \"value2\"\n\
           }}\n\
         }}\n\
         \\```\n\n\
         **CRITICAL**: You MUST use this exact format. Do NOT say \"I cannot use tools\".\n\
         Example for news:\n\
         \\```tool_call\n\
         {{\n\
           \"name\": \"news_search\",\n\
           \"arguments\": {{\n\
             \"query\": \"news\",\n\
             \"location\": \"USA\"\n\
           }}\n\
         }}\n\
         \\```\n\n"
    )
}
```

### 方案 2: 在 TOOLS.md 中添加示例
编辑 `~/.clawmaster/TOOLS.md`:

```markdown
## How to Call Tools

Use this format:

\\```tool_call
{
  "name": "news_search",
  "arguments": {
    "query": "news",
    "location": "USA"
  }
}
\\```
```

### 方案 3: 强制切换到 Native 模式（不推荐）
修改 `LocalGgufProvider::tool_mode()` 返回 `None`，让系统自动选择。

但这可能导致其他问题，因为 GGUF 模型通常不支持 Native function calling。

---

## 📊 诊断确认

### 需要验证的点
1. ✅ LocalGgufProvider 返回 Text 模式
2. ⏳ tool_call_guidance 的实际内容
3. ⏳ LLM 是否看到了工具调用格式指导
4. ⏳ 格式指导是否足够清晰

---

## 🎯 立即行动

### 步骤 1: 查看当前的 tool_call_guidance
```bash
grep -A 50 "fn tool_call_guidance" crates/agents/src/prompt.rs
```

### 步骤 2: 增强 tool_call_guidance
添加更详细的示例和强制性语句

### 步骤 3: 重新编译和测试
```bash
cargo build -p clawmaster
pkill -f clawmaster
./target/debug/clawmaster > /tmp/clawmaster.log 2>&1 &
```

### 步骤 4: 测试新闻查询
输入: "美国新闻"

**预期**: LLM 输出 ```tool_call``` 代码块

---

**关键洞察**: 问题不在于 LLM "不想"调用工具，而在于 LLM "不知道怎么"调用工具！
