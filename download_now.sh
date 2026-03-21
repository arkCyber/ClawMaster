#!/bin/bash
# 最简单的下载方案 - 使用 HuggingFace 镜像

set -e

echo "🚀 使用 HuggingFace 镜像下载"
echo "========================================"
echo ""

# 设置镜像
export HF_ENDPOINT=https://hf-mirror.com

echo "✅ 使用镜像: $HF_ENDPOINT"
echo ""

# 检查 huggingface-cli
if ! command -v huggingface-cli &> /dev/null; then
    echo "📦 安装 huggingface-hub..."
    pip3 install huggingface-hub
fi

echo "📥 开始下载 Llama 3.1 8B (约 4.9GB)..."
echo ""

# 下载
huggingface-cli download \
  bartowski/Meta-Llama-3.1-8B-Instruct-GGUF \
  Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
  --local-dir ~/.clawmaster/models \
  --local-dir-use-symlinks False

# 重命名
if [ -f ~/.clawmaster/models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf ]; then
    mv ~/.clawmaster/models/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf \
       ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
    
    echo ""
    echo "✅ 下载完成！"
    ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf
    
    echo ""
    echo "📝 下一步:"
    echo "1. 查找配置: find ~ -name 'clawmaster.toml' 2>/dev/null"
    echo "2. 修改配置: model_id = \"llama-3.1-8b-instruct-q4_k_m\""
    echo "3. 重启服务: pkill -9 -f clawmaster && ./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &"
else
    echo "❌ 下载失败"
    exit 1
fi
