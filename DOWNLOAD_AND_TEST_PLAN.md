# 🎯 下载 Llama 3.1 8B 并自动测试

**时间**: 2026年3月18日 08:20  
**目标**: 使用更强的模型解决工具调用问题  

---

## 📦 推荐模型：Llama 3.1 8B Instruct

### 为什么选择这个模型？

**优势**:
1. ✅ **原生支持工具调用** - 在 function calling 数据上训练过
2. ✅ **性能强大** - 8B 参数，理解能力强
3. ✅ **速度适中** - 比 14B 快，比 1B 强
4. ✅ **多语言支持** - 中英文都很好
5. ✅ **广泛验证** - 社区广泛使用，工具调用可靠

**规格**:
- 参数: 8B
- 量化: Q4_K_M
- 大小: 约 4.9GB
- Context: 128K tokens
- 速度: 在 M 系列芯片上约 20-30 tok/s

---

## 🚀 快速开始

### 方法 1: 使用下载脚本（推荐）

```bash
cd /Users/arksong/ClawMaster
./download_llama31_8b.sh
```

脚本会：
1. 自动创建模型目录
2. 从 HuggingFace 下载模型
3. 显示下载进度
4. 提供配置建议

### 方法 2: 使用 Ollama

```bash
# 安装 Ollama (如果还没有)
brew install ollama

# 下载模型
ollama pull llama3.1:8b

# Ollama 会自动管理模型
```

### 方法 3: 手动下载

访问: https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF

下载: `Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf`

保存到: `~/.clawmaster/models/`

---

## ⚙️ 配置

### 查找配置文件

```bash
# 可能的位置
find ~ -name "clawmaster.toml" 2>/dev/null
```

常见位置：
- `~/.clawmaster/clawmaster.toml`
- `~/.config/clawmaster/clawmaster.toml`

### 修改配置

```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
# 或使用 Ollama
# model_id = "llama3.1:8b"

# GPU 加速 (macOS Metal)
gpu_layers = 33

# 生成参数
temperature = 0.7
context_size = 8192
max_tokens = 2048
```

---

## 🔄 重启和测试

### 1. 重新编译（如果修改了代码）

```bash
cd /Users/arksong/ClawMaster
cargo build -p clawmaster
```

### 2. 停止旧进程

```bash
pkill -9 -f clawmaster
```

### 3. 启动新进程

```bash
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &
```

### 4. 等待启动

```bash
sleep 10
tail -20 /tmp/clawmaster_llama31.log | grep "listening"
```

### 5. 测试

访问: https://localhost:59233

输入: `美国新闻`

### 6. 查看日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(tool_mode|native_tools|tool_calls_count|resolved effective)"
```

---

## 📊 预期结果

### 成功的标志

**日志应该显示**:
```
resolved effective tool mode
  effective_mode = Text
  declared_mode = Some(Text)
  supports_tools = true

tool mode configuration
  tool_mode = Text
  native_tools = false

streaming LLM response complete
  tool_calls_count = 1
```

**WebUI 应该显示**:
```
```tool_call
{"tool": "news_search", "arguments": {"query": "news", "location": "USA"}}
```
```

然后返回实际的新闻列表。

### 失败的标志

**如果仍然 `tool_calls_count = 0`**:
- 说明 Llama 3.1 8B 也不调用工具
- 需要检查 prompt 是否正确传递
- 可能需要切换到 API 模型

---

## 🔍 调试检查清单

- [ ] 模型文件已下载（约 4.9GB）
- [ ] 配置文件已修改
- [ ] ClawMaster 已重启
- [ ] 日志显示 `model="local-llm::llama-3.1-8b-instruct-q4_k_m"`
- [ ] 日志显示 `tool_mode = Text`
- [ ] 日志显示 `native_tools = false`
- [ ] 测试查询已发送
- [ ] 检查 `tool_calls_count`

---

## 🎯 自动测试脚本

创建完整的自动测试：

```bash
#!/bin/bash
# 完整的模型切换和测试流程

echo "🔄 完整测试流程"
echo ""

# 1. 检查模型
echo "1. 检查模型文件..."
if [ ! -f ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf ]; then
    echo "❌ 模型不存在，请先运行 ./download_llama31_8b.sh"
    exit 1
fi
echo "✅ 模型文件存在"
echo ""

# 2. 重新编译
echo "2. 重新编译..."
cargo build -p clawmaster 2>&1 | tail -5
echo "✅ 编译完成"
echo ""

# 3. 重启服务
echo "3. 重启 ClawMaster..."
pkill -9 -f clawmaster
sleep 2
./target/debug/clawmaster > /tmp/clawmaster_test.log 2>&1 &
sleep 10
echo "✅ 服务已启动"
echo ""

# 4. 等待用户测试
echo "4. 请在 WebUI 中输入: 美国新闻"
echo "   访问: https://localhost:59233"
echo ""
read -p "测试完成后按 Enter 继续..."

# 5. 分析日志
echo ""
echo "5. 分析日志..."
echo ""
echo "=== Tool Mode 配置 ==="
tail -200 /tmp/clawmaster_test.log | grep -E "(resolved effective|tool mode configuration)" | tail -5
echo ""
echo "=== 工具调用结果 ==="
tail -200 /tmp/clawmaster_test.log | grep -E "tool_calls_count" | tail -3
echo ""

# 6. 判断结果
TOOL_CALLS=$(tail -200 /tmp/clawmaster_test.log | grep "tool_calls_count" | tail -1 | grep -o "tool_calls_count=[0-9]*" | cut -d= -f2)

if [ "$TOOL_CALLS" -gt 0 ]; then
    echo "🎉 成功！工具被调用了 $TOOL_CALLS 次"
else
    echo "❌ 失败：工具未被调用"
    echo ""
    echo "可能的原因："
    echo "1. native_tools 仍然是 true"
    echo "2. Llama 3.1 8B 也不支持工具调用"
    echo "3. Prompt 没有正确传递"
fi
```

---

## 💡 备选方案

### 如果 Llama 3.1 8B 仍然失败

#### 选项 A: 使用 API 模型

**Claude 3.5 Sonnet** (最推荐):
```toml
[providers.anthropic]
enabled = true
api_key = "sk-ant-..."
model = "claude-3-5-sonnet-20241022"

[providers.local-llm]
enabled = false
```

**GPT-4**:
```toml
[providers.openai]
enabled = true
api_key = "sk-..."
model = "gpt-4-turbo"

[providers.local-llm]
enabled = false
```

#### 选项 B: 检查 System Prompt

输出完整的 system prompt 到文件：
```rust
// 在 build_system_prompt_full 最后
std::fs::write("/tmp/system_prompt.txt", &prompt).ok();
```

#### 选项 C: 强制 Text 模式

修改 `effective_tool_mode`:
```rust
fn effective_tool_mode(provider: &dyn LlmProvider) -> ToolMode {
    // 强制所有本地模型使用 Text 模式
    if provider.name() == "local-llm" {
        return ToolMode::Text;
    }
    // ... 原有逻辑
}
```

---

## 📝 总结

### 当前计划
1. ✅ 添加详细日志
2. ⏳ 下载 Llama 3.1 8B
3. ⏳ 配置并重启
4. ⏳ 自动测试
5. ⏳ 分析结果

### 预期时间
- 下载: 5-10 分钟（取决于网速）
- 配置: 2 分钟
- 测试: 5 分钟
- **总计**: 约 15-20 分钟

---

**现在开始下载模型！** 🚀

```bash
cd /Users/arksong/ClawMaster
./download_llama31_8b.sh
```
