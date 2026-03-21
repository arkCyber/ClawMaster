#!/bin/bash
# 完整的自动测试流程：下载模型、配置、测试

set -e

echo "🚀 Llama 3.1 8B 完整测试流程"
echo "========================================"
echo ""

# 步骤 1: 检查模型
echo "📦 步骤 1/6: 检查模型文件"
MODEL_PATH="$HOME/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"

if [ -f "$MODEL_PATH" ]; then
    FILE_SIZE=$(du -h "$MODEL_PATH" | cut -f1)
    echo "  ✅ 模型已存在: $FILE_SIZE"
else
    echo "  ⚠️  模型不存在，开始下载..."
    echo ""
    
    # 下载模型
    ./download_llama31_8b.sh
    
    if [ ! -f "$MODEL_PATH" ]; then
        echo "  ❌ 下载失败"
        exit 1
    fi
fi
echo ""

# 步骤 2: 查找配置文件
echo "⚙️  步骤 2/6: 查找配置文件"
CONFIG_FILE=""
for path in "$HOME/.clawmaster/clawmaster.toml" "$HOME/.config/clawmaster/clawmaster.toml"; do
    if [ -f "$path" ]; then
        CONFIG_FILE="$path"
        echo "  ✅ 找到配置: $CONFIG_FILE"
        break
    fi
done

if [ -z "$CONFIG_FILE" ]; then
    echo "  ⚠️  配置文件不存在，创建默认配置"
    mkdir -p "$HOME/.clawmaster"
    CONFIG_FILE="$HOME/.clawmaster/clawmaster.toml"
    cat > "$CONFIG_FILE" << 'EOF'
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
gpu_layers = 33
temperature = 0.7
context_size = 8192
EOF
    echo "  ✅ 创建配置: $CONFIG_FILE"
fi
echo ""

# 步骤 3: 更新配置
echo "📝 步骤 3/6: 更新模型配置"
if grep -q "llama-3.1-8b" "$CONFIG_FILE"; then
    echo "  ✅ 配置已包含 Llama 3.1 8B"
else
    echo "  ⚠️  需要手动更新配置文件"
    echo ""
    echo "  请编辑: $CONFIG_FILE"
    echo "  修改 [providers.local-llm] 部分为:"
    echo ""
    echo "  [providers.local-llm]"
    echo "  enabled = true"
    echo "  model_id = \"llama-3.1-8b-instruct-q4_k_m\""
    echo "  gpu_layers = 33"
    echo ""
    read -p "  配置完成后按 Enter 继续..."
fi
echo ""

# 步骤 4: 重新编译
echo "🔨 步骤 4/6: 重新编译"
cargo build -p clawmaster 2>&1 | tail -5
echo "  ✅ 编译完成"
echo ""

# 步骤 5: 重启服务
echo "🔄 步骤 5/6: 重启 ClawMaster"
pkill -9 -f clawmaster 2>/dev/null || true
sleep 2

LOG_FILE="/tmp/clawmaster_llama31_test.log"
./target/debug/clawmaster > "$LOG_FILE" 2>&1 &
PID=$!

echo "  进程 ID: $PID"
echo "  日志文件: $LOG_FILE"
echo "  等待启动..."
sleep 10

if ! pgrep -f clawmaster > /dev/null; then
    echo "  ❌ 启动失败"
    echo ""
    echo "  查看日志:"
    tail -50 "$LOG_FILE"
    exit 1
fi

echo "  ✅ 服务已启动"
echo ""

# 步骤 6: 等待测试
echo "🧪 步骤 6/6: 测试"
echo "========================================"
echo ""
echo "📱 请在 WebUI 中测试:"
echo "  1. 访问: https://localhost:59233"
echo "  2. 输入: 美国新闻"
echo "  3. 观察是否调用工具"
echo ""
echo "⏱️  等待 30 秒后自动分析日志..."
echo ""

# 等待用户测试
for i in {30..1}; do
    echo -ne "  倒计时: $i 秒\r"
    sleep 1
done
echo ""
echo ""

# 分析日志
echo "📊 日志分析"
echo "========================================"
echo ""

echo "1️⃣  模型加载信息:"
tail -200 "$LOG_FILE" | grep -E "loading local LLM model|model=" | tail -3
echo ""

echo "2️⃣  Tool Mode 配置:"
tail -200 "$LOG_FILE" | grep -E "resolved effective tool mode" | tail -2
echo ""

echo "3️⃣  请求配置:"
tail -200 "$LOG_FILE" | grep -E "tool mode configuration for this request" | tail -2
echo ""

echo "4️⃣  工具调用结果:"
tail -200 "$LOG_FILE" | grep -E "tool_calls_count" | tail -3
echo ""

# 判断结果
TOOL_CALLS=$(tail -200 "$LOG_FILE" | grep "tool_calls_count" | tail -1 | grep -o "tool_calls_count=[0-9]*" | cut -d= -f2 || echo "0")

echo "========================================"
if [ "$TOOL_CALLS" -gt 0 ]; then
    echo "🎉 成功！工具被调用了 $TOOL_CALLS 次"
    echo ""
    echo "✅ Llama 3.1 8B 工具调用正常工作！"
else
    echo "❌ 失败：工具未被调用 (tool_calls_count=$TOOL_CALLS)"
    echo ""
    echo "🔍 可能的原因:"
    echo "  1. native_tools 仍然是 true（应该是 false）"
    echo "  2. 模型没有输出 tool_call 格式"
    echo "  3. System prompt 没有正确传递"
    echo ""
    echo "📝 下一步:"
    echo "  1. 检查完整日志: tail -100 $LOG_FILE"
    echo "  2. 查看 native_tools 的值"
    echo "  3. 考虑使用 API 模型（Claude/GPT-4）"
fi
echo ""

# 提供完整日志路径
echo "📄 完整日志: $LOG_FILE"
echo ""
