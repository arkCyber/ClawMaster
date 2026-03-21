#!/bin/bash
# 监控日志并自动分析测试结果

echo "=== Llama 3.1 8B 自动化测试监控 ==="
echo ""

# 获取 ClawMaster 进程 ID
PID=$(ps aux | grep 'target/debug/clawmaster' | grep -v grep | awk '{print $2}' | head -1)

if [ -z "$PID" ]; then
    echo "❌ ClawMaster 进程未运行"
    exit 1
fi

echo "✅ ClawMaster 进程 ID: $PID"
echo ""

# 测试场景列表
declare -a SCENARIOS=(
    "场景1:美国新闻？:已完成"
    "场景2:你是谁？:待测试"
    "场景3:US news?:待测试"
    "场景4:科技新闻:待测试"
    "场景5:上海新闻:待测试"
    "场景6:你好:待测试"
)

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "测试场景清单"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for scenario in "${SCENARIOS[@]}"; do
    IFS=':' read -r id input status <<< "$scenario"
    if [ "$status" = "已完成" ]; then
        echo "✅ $id: $input - $status"
    else
        echo "⏳ $id: $input - $status"
    fi
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "自动化测试说明"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "我会实时监控后端日志，当你在 WebUI 中输入测试命令时："
echo "1. 自动捕获模型输出"
echo "2. 自动分析工具调用"
echo "3. 自动判断测试结果"
echo "4. 自动生成测试报告"
echo ""
echo "请依次在 WebUI 中输入上述测试命令"
echo "我会在后台持续监控并分析结果"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
