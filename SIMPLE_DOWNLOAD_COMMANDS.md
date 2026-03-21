# 🚀 最简单的下载命令

**问题**: SSL 连接失败，未检测到代理

**解决**: 使用 HuggingFace 镜像（国内可用）

---

## ✅ 一键下载（复制粘贴执行）

```bash
# 设置镜像
export HF_ENDPOINT=https://hf-mirror.com

# 安装工具（使用 --user 避免权限问题）
pip3 install --user huggingface-hub

# 下载模型
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False

# 重命名文件
mv ~/.clawmaster/models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
   ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf

# 验证
ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
```

---

## 🔄 备选方案：忽略 SSL 证书

如果上面的方法不行，使用 wget 忽略 SSL：

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

## 📝 下载完成后的配置

### 1. 查找配置文件

```bash
find ~ -name "clawmaster.toml" 2>/dev/null
```

### 2. 编辑配置

修改找到的配置文件：

```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

### 3. 重启 ClawMaster

```bash
pkill -9 -f clawmaster
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &
sleep 10
```

### 4. 测试

访问: https://localhost:59233  
输入: `美国新闻`

### 5. 验证日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(tool_mode|native_tools|tool_calls_count)"
```

**期望**: `tool_calls_count = 1` 🎉

---

**现在执行第一组命令开始下载！** 🚀
