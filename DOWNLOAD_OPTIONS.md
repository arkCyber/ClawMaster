# 📥 模型下载方案

**问题**: 直接下载 4.9GB 文件可能因网络问题失败

---

## 🎯 推荐方案（按优先级）

### 方案 1: 使用 Ollama（最简单）⭐⭐⭐⭐⭐

```bash
# 安装 Ollama
brew install ollama

# 下载模型（自动管理，支持断点续传）
ollama pull llama3.1:8b
```

**优点**:
- ✅ 自动断点续传
- ✅ 自动管理模型
- ✅ 稳定可靠
- ✅ 无需手动配置路径

**配置**:
```toml
[providers.local-llm]
enabled = true
model_id = "llama3.1:8b"
gpu_layers = 33
temperature = 0.7
```

---

### 方案 2: 使用备用下载脚本（支持断点续传）⭐⭐⭐⭐

```bash
cd /Users/arksong/ClawMaster
./download_model_alternative.sh
```

这个脚本会尝试：
1. wget（支持断点续传）
2. aria2c（多线程下载）
3. curl 分段下载

---

### 方案 3: 使用 HuggingFace CLI（官方工具）⭐⭐⭐⭐

```bash
# 安装
pip install huggingface-hub

# 下载
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False
```

---

### 方案 4: 使用 Git LFS（适合网络不稳定）⭐⭐⭐

```bash
# 安装 Git LFS
brew install git-lfs
git lfs install

# 克隆仓库（只下载这个文件）
cd ~/.clawmaster/models
GIT_LFS_SKIP_SMUDGE=1 git clone https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF
cd Meta-Llama-3.1-8B-Instruct-GGUF
git lfs pull --include="Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"

# 移动文件
mv Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf ../llama-3.1-8b-instruct-q4_k_m.gguf
cd ..
rm -rf Meta-Llama-3.1-8B-Instruct-GGUF
```

---

### 方案 5: 使用国内镜像（如果在中国）⭐⭐⭐⭐⭐

```bash
# 使用 ModelScope（阿里云）
# 安装
pip install modelscope

# 下载
python3 << 'EOF'
from modelscope import snapshot_download
model_dir = snapshot_download(
    'LLM-Research/Meta-Llama-3.1-8B-Instruct-GGUF',
    cache_dir='~/.clawmaster/models'
)
print(f"模型下载到: {model_dir}")
EOF
```

或使用 HuggingFace 镜像：
```bash
export HF_ENDPOINT=https://hf-mirror.com
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models
```

---

## 🔍 检查下载进度

### 如果使用 curl（后台下载）

```bash
# 查看进程
ps aux | grep curl

# 查看文件大小（应该逐渐增长）
watch -n 5 'ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf'

# 预期大小：约 4.9GB (5,268,000,000 bytes)
```

### 检查是否完整

```bash
# 文件应该约 4.9GB
du -h ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf

# 如果小于 4GB，说明下载未完成
```

---

## 💡 我的推荐

### 最简单：使用 Ollama

```bash
brew install ollama
ollama pull llama3.1:8b
```

然后修改配置：
```toml
[providers.local-llm]
enabled = true
model_id = "llama3.1:8b"
```

### 最可靠：使用 HuggingFace CLI

```bash
pip install huggingface-hub
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False
```

---

## 🚀 下载后的步骤

### 1. 验证文件

```bash
ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
# 应该显示约 4.9GB
```

### 2. 查找配置文件

```bash
find ~ -name "clawmaster.toml" 2>/dev/null
```

### 3. 修改配置

```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

### 4. 重启 ClawMaster

```bash
pkill -9 -f clawmaster
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &
```

### 5. 测试

访问: https://localhost:59233  
输入: `美国新闻`

### 6. 查看日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(tool_mode|native_tools|tool_calls_count)"
```

---

## 📊 预期结果

**成功标志**:
```
tool_mode = Text
native_tools = false
tool_calls_count = 1
```

**如果成功**: 🎉 问题解决！

**如果失败**: 考虑使用 Claude API（最可靠）

---

**选择一个方案开始下载吧！** 🚀
