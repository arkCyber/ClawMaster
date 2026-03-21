#!/bin/bash
# 使用代理下载 Llama 3.1 8B 模型

set -e

echo "🌐 配置代理下载 Llama 3.1 8B"
echo "========================================"
echo ""

# 模型信息
MODEL_URL="https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
MODEL_PATH="$HOME/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"

# 创建目录
mkdir -p "$HOME/.clawmaster/models"

# 检查代理配置
if [ -n "$HTTP_PROXY" ] || [ -n "$HTTPS_PROXY" ] || [ -n "$http_proxy" ] || [ -n "$https_proxy" ]; then
    echo "✅ 检测到代理配置:"
    echo "  HTTP_PROXY: ${HTTP_PROXY:-${http_proxy:-未设置}}"
    echo "  HTTPS_PROXY: ${HTTPS_PROXY:-${https_proxy:-未设置}}"
    echo ""
else
    echo "⚠️  未检测到代理配置"
    echo ""
    echo "如果需要代理，请先设置环境变量:"
    echo "  export https_proxy=http://127.0.0.1:7890"
    echo "  export http_proxy=http://127.0.0.1:7890"
    echo ""
    read -p "是否继续？(y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# 方法 1: 使用 wget（支持代理）
echo "📥 方法 1: 尝试使用 wget..."
if command -v wget &> /dev/null; then
    wget -c -O "$MODEL_PATH" \
        --show-progress \
        --tries=10 \
        --wait=5 \
        --timeout=300 \
        --no-check-certificate \
        "$MODEL_URL"
    
    if [ $? -eq 0 ]; then
        echo "✅ 下载完成！"
        ls -lh "$MODEL_PATH"
        exit 0
    else
        echo "⚠️  wget 下载失败，尝试其他方法..."
    fi
fi

# 方法 2: 使用 curl（支持代理）
echo ""
echo "📥 方法 2: 尝试使用 curl..."
if command -v curl &> /dev/null; then
    curl -L -o "$MODEL_PATH" \
        --progress-bar \
        --retry 10 \
        --retry-delay 5 \
        --max-time 3600 \
        --continue-at - \
        --insecure \
        "$MODEL_URL"
    
    if [ $? -eq 0 ]; then
        echo "✅ 下载完成！"
        ls -lh "$MODEL_PATH"
        exit 0
    else
        echo "⚠️  curl 下载失败"
    fi
fi

# 方法 3: 使用 Python
echo ""
echo "📥 方法 3: 尝试使用 Python..."
if command -v python3 &> /dev/null; then
    python3 << 'PYTHON_SCRIPT'
import os
import sys
import urllib.request
from pathlib import Path

MODEL_URL = "https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
MODEL_PATH = Path.home() / ".clawmaster" / "models" / "llama-3.1-8b-instruct-q4_k_m.gguf"

# 配置代理
proxy_handler = urllib.request.ProxyHandler({
    'http': os.environ.get('http_proxy', os.environ.get('HTTP_PROXY', '')),
    'https': os.environ.get('https_proxy', os.environ.get('HTTPS_PROXY', ''))
})
opener = urllib.request.build_opener(proxy_handler)
urllib.request.install_opener(opener)

print(f"下载到: {MODEL_PATH}")

try:
    # 下载
    urllib.request.urlretrieve(MODEL_URL, MODEL_PATH)
    print(f"✅ 下载完成: {MODEL_PATH.stat().st_size / (1024**3):.2f} GB")
except Exception as e:
    print(f"❌ 下载失败: {e}")
    sys.exit(1)
PYTHON_SCRIPT
    
    if [ $? -eq 0 ]; then
        echo "✅ 下载完成！"
        ls -lh "$MODEL_PATH"
        exit 0
    fi
fi

echo ""
echo "❌ 所有下载方法都失败了"
echo ""
echo "💡 建议:"
echo "1. 检查网络连接"
echo "2. 配置代理（如果在国内）"
echo "3. 尝试使用 HuggingFace 镜像"
echo "4. 手动从浏览器下载"
