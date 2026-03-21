# 🚀 立即开始下载

您有 3 个简单的方法，选择一个执行即可：

---

## 方法 1: 使用 wget（最简单）⭐⭐⭐⭐⭐

**一键下载，支持断点续传**：

```bash
mkdir -p ~/.clawmaster/models && \
wget -c -O ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf \
  --show-progress \
  --tries=10 \
  --wait=5 \
  "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
```

**如果中断**: 重新运行相同命令，会自动继续下载

---

## 方法 2: 使用 Python 脚本（推荐）⭐⭐⭐⭐⭐

```bash
# 安装依赖
pip3 install requests tqdm

# 运行下载脚本
cd /Users/arksong/ClawMaster
python3 download_with_python.py
```

**优点**: 显示详细进度，支持断点续传

---

## 方法 3: 使用 HuggingFace CLI⭐⭐⭐⭐

```bash
# 安装
pip3 install huggingface-hub

# 下载
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False

# 重命名
mv ~/.clawmaster/models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
   ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

---

## 📊 下载信息

**模型**: Llama 3.1 8B Instruct Q4_K_M  
**大小**: 约 4.9GB  
**预计时间**: 5-15 分钟（取决于网速）  
**保存位置**: `~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf`

---

## ✅ 下载完成后

### 1. 验证文件

```bash
ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

应该显示约 4.9GB

### 2. 查找配置文件

```bash
find ~ -name "clawmaster.toml" 2>/dev/null
```

### 3. 修改配置

编辑找到的配置文件（例如 `~/.clawmaster/clawmaster.toml`）：

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
sleep 10
```

### 5. 测试

访问: https://localhost:59233  
输入: `美国新闻`

### 6. 查看日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(tool_mode|native_tools|tool_calls_count)"
```

**期望看到**:
- `tool_mode = Text`
- `native_tools = false`
- `tool_calls_count = 1` ← 成功！

---

## 💡 我的推荐

**使用 wget（最简单）**:

```bash
mkdir -p ~/.clawmaster/models && \
wget -c -O ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf \
  --show-progress \
  --tries=10 \
  --wait=5 \
  "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
```

**现在就开始吧！** 🚀
