# 🌐 代理下载指南

**问题**: SSL 连接失败 - "Unable to establish SSL connection"

---

## 🎯 快速解决方案

### 方案 1: 配置代理后下载（推荐）

```bash
# 1. 设置代理（根据您的代理端口修改）
export https_proxy=http://127.0.0.1:7890
export http_proxy=http://127.0.0.1:7890

# 2. 使用脚本下载
cd /Users/arksong/ClawMaster
./download_with_proxy.sh
```

**常见代理端口**:
- Clash: 7890
- V2Ray: 1080
- Shadowsocks: 1080
- 其他: 查看您的代理软件设置

---

### 方案 2: 使用 HuggingFace 镜像（国内推荐）

```bash
# 使用 HuggingFace 镜像
export HF_ENDPOINT=https://hf-mirror.com

# 安装 huggingface-cli
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

### 方案 3: 使用 ModelScope（阿里云镜像）

```bash
# 安装
pip3 install modelscope

# 下载
python3 << 'EOF'
from modelscope import snapshot_download
import shutil
from pathlib import Path

# 下载模型
model_dir = snapshot_download(
    'LLM-Research/Meta-Llama-3.1-8B-Instruct-GGUF',
    cache_dir='~/.cache/modelscope'
)

# 找到 GGUF 文件并复制
import os
for root, dirs, files in os.walk(model_dir):
    for file in files:
        if file.endswith('Q4_K_M.gguf'):
            src = os.path.join(root, file)
            dst = Path.home() / '.clawmaster' / 'models' / 'llama-3.1-8b-instruct-q4_k_m.gguf'
            dst.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, dst)
            print(f"✅ 模型已复制到: {dst}")
            break
EOF
```

---

### 方案 4: 使用 wget 忽略 SSL（不推荐，但可用）

```bash
mkdir -p ~/.clawmaster/models && \
wget -c -O ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf \
  --show-progress \
  --tries=10 \
  --wait=5 \
  --no-check-certificate \
  "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
```

---

### 方案 5: 浏览器下载（最可靠）

1. 访问: https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/tree/main
2. 找到文件: `Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf`
3. 点击下载（浏览器会使用系统代理）
4. 下载完成后移动文件:
   ```bash
   mv ~/Downloads/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
      ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
   ```

---

## 🔍 检查代理配置

### 查看当前代理设置

```bash
echo "HTTP_PROXY: $HTTP_PROXY"
echo "HTTPS_PROXY: $HTTPS_PROXY"
echo "http_proxy: $http_proxy"
echo "https_proxy: $https_proxy"
```

### 测试代理是否工作

```bash
# 测试 HTTP 连接
curl -I --proxy http://127.0.0.1:7890 https://www.google.com

# 或
wget --spider --proxy=on --https-proxy=http://127.0.0.1:7890 https://www.google.com
```

---

## 💡 我的推荐

### 如果在国内

**最简单**: 使用 HuggingFace 镜像

```bash
export HF_ENDPOINT=https://hf-mirror.com
pip3 install huggingface-hub
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False
mv ~/.clawmaster/models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
   ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

### 如果有代理

**配置代理后使用脚本**:

```bash
export https_proxy=http://127.0.0.1:7890
export http_proxy=http://127.0.0.1:7890
cd /Users/arksong/ClawMaster
./download_with_proxy.sh
```

### 如果都不行

**浏览器下载**: 最可靠，会自动使用系统代理

---

## ✅ 下载完成后

### 1. 验证文件

```bash
ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
# 应该约 4.9GB
```

### 2. 配置 ClawMaster

```bash
# 查找配置文件
find ~ -name "clawmaster.toml" 2>/dev/null

# 编辑配置
# [providers.local-llm]
# enabled = true
# model_id = "llama-3.1-8b-instruct-q4_k_m"
# gpu_layers = 33
```

### 3. 重启测试

```bash
pkill -9 -f clawmaster
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &
sleep 10
```

### 4. 测试工具调用

访问 https://localhost:59233，输入：`美国新闻`

### 5. 验证日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(tool_mode|native_tools|tool_calls_count)"
```

**期望**: `tool_calls_count = 1` 🎉

---

**选择一个方案开始下载吧！** 🚀
