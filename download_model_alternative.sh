#!/bin/bash
# 备用下载方案：使用 wget 或 aria2c

set -e

MODEL_PATH="$HOME/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"
MODEL_URL="https://huggingface.co/bartowski/Meta-Llama-3.1-8B-Instruct-GGUF/resolve/main/Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf"

echo "🔄 备用下载方案"
echo ""

# 创建目录
mkdir -p "$HOME/.clawmaster/models"

# 方法 1: 使用 wget（支持断点续传）
if command -v wget &> /dev/null; then
    echo "📥 使用 wget 下载（支持断点续传）..."
    wget -c -O "$MODEL_PATH" \
        --show-progress \
        --tries=10 \
        --wait=5 \
        --timeout=300 \
        "$MODEL_URL"
    
    if [ $? -eq 0 ]; then
        echo "✅ 下载完成！"
        ls -lh "$MODEL_PATH"
        exit 0
    fi
fi

# 方法 2: 使用 aria2c（多线程下载）
if command -v aria2c &> /dev/null; then
    echo "📥 使用 aria2c 下载（多线程）..."
    aria2c -x 8 -s 8 -k 1M \
        --retry-wait=5 \
        --max-tries=10 \
        --timeout=300 \
        -d "$HOME/.clawmaster/models" \
        -o "llama-3.1-8b-instruct-q4_k_m.gguf" \
        "$MODEL_URL"
    
    if [ $? -eq 0 ]; then
        echo "✅ 下载完成！"
        ls -lh "$MODEL_PATH"
        exit 0
    fi
fi

# 方法 3: 使用 curl 分段下载
echo "📥 使用 curl 分段下载..."
TEMP_DIR="$HOME/.clawmaster/models/temp"
mkdir -p "$TEMP_DIR"

# 获取文件大小
FILE_SIZE=$(curl -sI "$MODEL_URL" | grep -i content-length | awk '{print $2}' | tr -d '\r')
echo "文件大小: $FILE_SIZE bytes"

# 分段大小 (100MB)
CHUNK_SIZE=104857600
CHUNKS=$((FILE_SIZE / CHUNK_SIZE + 1))

echo "分成 $CHUNKS 段下载..."

for i in $(seq 0 $((CHUNKS - 1))); do
    START=$((i * CHUNK_SIZE))
    END=$((START + CHUNK_SIZE - 1))
    
    if [ $END -gt $FILE_SIZE ]; then
        END=$FILE_SIZE
    fi
    
    echo "下载段 $((i + 1))/$CHUNKS (${START}-${END})..."
    
    curl -L -r "${START}-${END}" \
        --retry 5 \
        --retry-delay 5 \
        -o "${TEMP_DIR}/chunk_${i}" \
        "$MODEL_URL"
    
    if [ $? -ne 0 ]; then
        echo "❌ 段 $i 下载失败"
        exit 1
    fi
done

# 合并文件
echo "合并文件..."
cat ${TEMP_DIR}/chunk_* > "$MODEL_PATH"
rm -rf "$TEMP_DIR"

echo "✅ 下载完成！"
ls -lh "$MODEL_PATH"
