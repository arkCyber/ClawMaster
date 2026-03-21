# 🎯 正确的模型信息

---

## 📦 Llama 3.1 8B Instruct Q4_K_M

### 模型详情

**HuggingFace 仓库**: `bartowski/Meta-Llama-3.1-8B-Instruct-GGUF`

**文件名**: `Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf`

**直接下载链接**:
```
https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf
```

**大小**: 约 4.9GB (5,268,000,000 bytes)

**量化**: Q4_K_M（4-bit 量化，中等质量）

**本地保存路径**: `~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf`

**ClawMaster 配置中的 model_id**: `llama-3.1-8b-instruct-q4_k_m`

---

## 🚀 推荐下载方法（不使用 Ollama）

### 方法 1: 使用 Python 脚本（推荐）⭐⭐⭐⭐⭐

**优点**: 支持断点续传，显示进度，可中断后继续

```bash
# 安装依赖
pip3 install requests tqdm

# 下载
cd /Users/arksong/ClawMaster
python3 download_with_python.py
```

**如果中断**: 直接重新运行脚本，会自动继续下载

---

### 方法 2: 使用 wget（如果已安装）⭐⭐⭐⭐

```bash
# 检查是否有 wget
which wget

# 如果没有，安装
brew install wget

# 下载（支持断点续传）
mkdir -p ~/.clawmaster/models
wget -c -O ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf \
  --show-progress \
  --tries=10 \
  --wait=5 \
  --timeout=300 \
  "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
```

---

### 方法 3: 使用 HuggingFace CLI⭐⭐⭐⭐

```bash
# 安装
pip3 install huggingface-hub

# 下载
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False

# 重命名文件
mv ~/.clawmaster/models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
   ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

---

### 方法 4: 使用浏览器下载⭐⭐⭐

1. 访问: https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/tree/main
2. 找到文件: `Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf`
3. 点击下载
4. 下载完成后移动到: `~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf`

---

## 🔍 验证下载

### 检查文件大小

```bash
ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

**应该显示**: 约 4.9GB

### 检查文件完整性（可选）

```bash
# 如果下载页面提供了 SHA256
# shasum -a 256 ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

---

## ⚙️ 配置 ClawMaster

### 1. 查找配置文件

```bash
find ~ -name "clawmaster.toml" 2>/dev/null
```

可能的位置：
- `~/.clawmaster/clawmaster.toml`
- `~/.config/clawmaster/clawmaster.toml`

### 2. 修改配置

编辑配置文件，找到 `[providers.local-llm]` 部分：

```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
gpu_layers = 33  # macOS Metal GPU 加速
temperature = 0.7
context_size = 8192
max_tokens = 2048
```

**重要**: `model_id` 必须与文件名匹配（不含 .gguf 后缀）

### 3. 重启 ClawMaster

```bash
# 停止旧进程
pkill -9 -f clawmaster

# 启动新进程
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &

# 等待启动
sleep 10

# 检查状态
tail -20 /tmp/clawmaster_llama31.log | grep "listening"
```

---

## 🧪 测试

### 1. 访问 WebUI

https://localhost:59233

### 2. 输入测试查询

```
美国新闻
```

### 3. 查看日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(model=|tool_mode|native_tools|tool_calls_count)"
```

### 4. 预期结果

**成功的标志**:
```
model="local-llm::llama-3.1-8b-instruct-q4_k_m"
tool_mode = Text
native_tools = false
tool_calls_count = 1
```

**如果 `tool_calls_count = 1` 或更多** → 🎉 **成功！**

---

## 📝 其他可用的 Llama 3.1 8B 模型

如果上面的模型下载失败，可以尝试这些替代品：

### 选项 A: TheBloke 的版本

```
https://huggingface.co/TheBloke/Llama-3.1-8B-Instruct-GGUF
文件: llama-3.1-8b-instruct.Q4_K_M.gguf
```

### 选项 B: lmstudio-community 的版本

```
https://huggingface.co/lmstudio-community/Meta-Llama-3.1-8B-Instruct-GGUF
文件: Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf
```

### 选项 C: 使用更小的模型（如果下载太慢）

**Llama 3.2 3B Instruct** (约 2GB):
```
https://huggingface.co/bartowski/Llama-3.2-3B-Instruct-GGUF
文件: Llama-3.2-3B-Instruct-Q4_K_M.gguf
```

---

## 💡 故障排除

### 问题 1: 下载速度太慢

**解决**: 使用 Python 脚本，可以中断后继续

### 问题 2: 连接重置

**解决**: 使用支持断点续传的工具（wget、Python 脚本、HuggingFace CLI）

### 问题 3: 磁盘空间不足

**检查**:
```bash
df -h ~/.clawmaster/models
```

需要至少 5GB 空闲空间

### 问题 4: 模型加载失败

**检查文件大小**:
```bash
ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

如果小于 4.5GB，说明下载不完整，需要重新下载

---

## 🎯 推荐流程

1. **使用 Python 脚本下载**（最可靠）:
   ```bash
   pip3 install requests tqdm
   python3 download_with_python.py
   ```

2. **配置 ClawMaster**:
   ```bash
   find ~ -name "clawmaster.toml" 2>/dev/null
   # 编辑找到的文件，设置 model_id = "llama-3.1-8b-instruct-q4_k_m"
   ```

3. **重启并测试**:
   ```bash
   pkill -9 -f clawmaster
   ./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &
   ```

4. **验证**:
   - 访问 https://localhost:59233
   - 输入"美国新闻"
   - 检查日志中的 `tool_calls_count`

---

**现在开始下载吧！** 🚀
