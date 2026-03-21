#!/bin/bash
# 下载 Llama 3.1 8B Instruct 模型

set -e

echo "🚀 开始下载 Llama 3.1 8B Instruct 模型"
echo ""

# 模型信息
MODEL_NAME="llama-3.1-8b-instruct-q4_k_m"
MODEL_FILE="${MODEL_NAME}.gguf"
MODEL_DIR="$HOME/.clawmaster/models"
MODEL_PATH="${MODEL_DIR}/${MODEL_FILE}"

# HuggingFace 下载链接（使用 bartowski 的量化版本）
HF_REPO="bartowski/Meta-Llama-3.1-8B-Instruct-GGUF"
HF_FILE="Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"
HF_URL="https://huggingface.co/${HF_REPO}/resolve/main/${HF_FILE}"

echo "📦 模型信息:"
echo "  名称: Llama 3.1 8B Instruct"
echo "  量化: Q4_K_M (约 4.9GB)"
echo "  来源: ${HF_REPO}"
echo "  目标: ${MODEL_PATH}"
echo ""

# 创建模型目录
mkdir -p "${MODEL_DIR}"

# 检查是否已存在
if [ -f "${MODEL_PATH}" ]; then
    echo "⚠️  模型文件已存在: ${MODEL_PATH}"
    read -p "是否重新下载? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "✅ 使用现有模型"
        exit 0
    fi
    rm -f "${MODEL_PATH}"
fi

# 下载模型
echo "⬇️  开始下载 (约 4.9GB，可能需要几分钟)..."
echo ""

# 使用 curl 下载，显示进度
if command -v curl &> /dev/null; then
    curl -L -o "${MODEL_PATH}" \
        --progress-bar \
        --retry 3 \
        --retry-delay 5 \
        "${HF_URL}"
elif command -v wget &> /dev/null; then
    wget -O "${MODEL_PATH}" \
        --show-progress \
        --tries=3 \
        --wait=5 \
        "${HF_URL}"
else
    echo "❌ 错误: 需要 curl 或 wget"
    exit 1
fi

# 验证下载
if [ ! -f "${MODEL_PATH}" ]; then
    echo "❌ 下载失败"
    exit 1
fi

FILE_SIZE=$(du -h "${MODEL_PATH}" | cut -f1)
echo ""
echo "✅ 下载完成!"
echo "  文件大小: ${FILE_SIZE}"
echo "  路径: ${MODEL_PATH}"
echo ""

# 创建配置建议
echo "📝 配置建议:"
echo ""
echo "修改 ~/.clawmaster/clawmaster.toml (或 ~/.config/clawmaster/clawmaster.toml):"
echo ""
cat << 'EOF'
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
# 或直接指定路径:
# model_path = "$HOME/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"
gpu_layers = 33  # Metal GPU 加速 (macOS)
temperature = 0.7
context_size = 8192
EOF
echo ""

echo "🎯 下一步:"
echo "1. 修改配置文件"
echo "2. 重启 ClawMaster"
echo "3. 测试新闻工具"
echo ""
echo "🚀 完成!"
