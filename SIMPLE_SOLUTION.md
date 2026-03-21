# 🎯 最简单的解决方案

**直接下载失败了，使用 Ollama 是最简单的方法！**

---

## ✅ 推荐方案：使用 Ollama

### 步骤 1: 安装 Ollama

```bash
brew install ollama
```

### 步骤 2: 下载模型（自动断点续传）

```bash
ollama pull llama3.1:8b
```

**优点**:
- ✅ 自动处理下载
- ✅ 支持断点续传
- ✅ 自动管理模型
- ✅ 无需手动配置路径

### 步骤 3: 查找配置文件

```bash
find ~ -name "clawmaster.toml" 2>/dev/null
```

可能的位置：
- `~/.clawmaster/clawmaster.toml`
- `~/.config/clawmaster/clawmaster.toml`

### 步骤 4: 修改配置

编辑找到的配置文件，修改 `[providers.local-llm]` 部分：

```toml
[providers.local-llm]
enabled = true
model_id = "llama3.1:8b"
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

### 步骤 5: 重启 ClawMaster

```bash
pkill -9 -f clawmaster
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &
```

### 步骤 6: 等待启动

```bash
sleep 10
tail -20 /tmp/clawmaster_llama31.log | grep "listening"
```

### 步骤 7: 测试

1. 访问: https://localhost:59233
2. 输入: `美国新闻`
3. 观察是否调用工具

### 步骤 8: 查看日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(tool_mode|native_tools|tool_calls_count)"
```

---

## 📊 预期结果

**成功的标志**:
```
resolved effective tool mode
  effective_mode = Text
  native_tools = false

tool mode configuration for this request
  tool_mode = Text
  native_tools = false

streaming LLM response complete
  tool_calls_count = 1  ← 关键！
```

**如果看到 `tool_calls_count = 1` 或更多** → 🎉 **成功！**

---

## 🔄 备选方案

### 方案 A: 使用备用下载脚本

如果不想用 Ollama：

```bash
cd /Users/arksong/ClawMaster
./download_model_alternative.sh
```

这个脚本会尝试 wget、aria2c 或分段下载。

### 方案 B: 使用 HuggingFace CLI

```bash
pip install huggingface-hub

huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False
```

然后配置：
```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
```

### 方案 C: 使用 Claude API（最可靠）

如果本地模型都不行，使用 API：

```toml
[providers.anthropic]
enabled = true
api_key = "sk-ant-your-key-here"
model = "claude-3-5-sonnet-20241022"

[providers.local-llm]
enabled = false
```

---

## 🎯 我的建议

**立即执行**:

```bash
# 1. 安装 Ollama
brew install ollama

# 2. 下载模型
ollama pull llama3.1:8b

# 3. 查找配置
find ~ -name "clawmaster.toml" 2>/dev/null

# 4. 编辑配置（使用上面找到的路径）
# vim ~/.clawmaster/clawmaster.toml

# 5. 重启
pkill -9 -f clawmaster
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &

# 6. 等待并测试
sleep 10
echo "访问 https://localhost:59233 并输入'美国新闻'"
```

---

**Ollama 是最简单的方法，强烈推荐！** 🚀
