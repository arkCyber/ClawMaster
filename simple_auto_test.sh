#!/bin/bash
# 简化的自动测试 - 通过日志分析验证修复效果

set -e

LOG_FILE="/tmp/clawmaster_final.log"
REPORT_FILE="/tmp/test_report_$(date +%s).txt"

echo "================================================================================"
echo "  新闻工具自动测试"
echo "================================================================================"
echo ""
echo "开始时间: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 检查系统状态
echo "🔍 检查系统状态..."
if ! pgrep -f clawmaster > /dev/null; then
    echo "❌ ClawMaster 未运行"
    exit 1
fi
echo "✅ ClawMaster 运行中"
echo ""

# 检查日志文件
if [ ! -f "$LOG_FILE" ]; then
    echo "❌ 日志文件不存在: $LOG_FILE"
    exit 1
fi
echo "✅ 日志文件存在"
echo ""

# 清空旧的测试标记
echo "=== AUTO TEST START ===" >> "$LOG_FILE"

echo "================================================================================"
echo "  测试说明"
echo "================================================================================"
echo ""
echo "此脚本将监控日志文件，等待您在 WebUI 中输入测试查询。"
echo ""
echo "请按以下步骤操作："
echo "1. 打开 WebUI: https://localhost:59233"
echo "2. 输入测试查询"
echo "3. 脚本会自动捕获并分析结果"
echo ""
echo "测试案例："
echo "  1. 美国新闻"
echo "  2. 请使用 calc 工具计算 2+2"
echo ""

# 函数：分析最近的日志
analyze_recent_logs() {
    local test_name="$1"
    local expected_tool="$2"
    
    echo "分析 '$test_name' 的结果..."
    
    # 获取最近100行日志
    local recent_logs=$(tail -100 "$LOG_FILE")
    
    # 检查 tool_calls_count
    local tool_count=$(echo "$recent_logs" | grep -o "tool_calls_count=[0-9]*" | tail -1 | cut -d= -f2)
    
    # 检查工具名称
    local tool_name=$(echo "$recent_logs" | grep -o "\"tool\":\s*\"[^\"]*\"" | tail -1 | cut -d'"' -f4)
    
    # 检查响应
    local response=$(echo "$recent_logs" | grep "response=" | tail -1)
    
    # 判断结果
    local passed=true
    local reasons=""
    
    if [ -z "$tool_count" ] || [ "$tool_count" = "0" ]; then
        passed=false
        reasons="${reasons}\n  ❌ 未检测到工具调用 (tool_calls_count=${tool_count:-0})"
    else
        echo "  ✅ 检测到 $tool_count 次工具调用"
    fi
    
    if [ -n "$tool_name" ]; then
        if [[ "$tool_name" == *"$expected_tool"* ]]; then
            echo "  ✅ 调用了正确的工具: $tool_name"
        else
            passed=false
            reasons="${reasons}\n  ❌ 调用了错误的工具: $tool_name (预期: $expected_tool)"
        fi
    fi
    
    # 检查禁止的文本
    if echo "$response" | grep -qi "抱歉\|无法\|cannot\|unable"; then
        passed=false
        reasons="${reasons}\n  ❌ 响应包含禁止的文本"
    fi
    
    # 输出结果
    echo ""
    if [ "$passed" = true ]; then
        echo "  ✅ 测试通过: $test_name"
    else
        echo "  ❌ 测试失败: $test_name"
        echo -e "$reasons"
    fi
    
    # 保存到报告
    {
        echo "----------------------------------------"
        echo "测试: $test_name"
        echo "预期工具: $expected_tool"
        echo "工具调用次数: ${tool_count:-0}"
        echo "实际调用工具: ${tool_name:-未知}"
        if [ "$passed" = true ]; then
            echo "结果: ✅ 通过"
        else
            echo "结果: ❌ 失败"
            echo -e "原因: $reasons"
        fi
        echo ""
    } >> "$REPORT_FILE"
    
    echo "$passed"
}

# 函数：等待新的日志条目
wait_for_activity() {
    local timeout=$1
    local start_line=$(wc -l < "$LOG_FILE")
    local elapsed=0
    
    echo "  ⏳ 等待活动（超时 ${timeout}秒）..."
    
    while [ $elapsed -lt $timeout ]; do
        sleep 2
        elapsed=$((elapsed + 2))
        
        local current_line=$(wc -l < "$LOG_FILE")
        if [ $current_line -gt $start_line ]; then
            local new_lines=$((current_line - start_line))
            echo "  📝 检测到 $new_lines 行新日志"
            return 0
        fi
        
        # 显示进度
        if [ $((elapsed % 10)) -eq 0 ]; then
            echo "  ⏱️  已等待 ${elapsed}秒..."
        fi
    done
    
    echo "  ⚠️  超时，未检测到活动"
    return 1
}

# 初始化报告
{
    echo "================================================================================"
    echo "  新闻工具测试报告"
    echo "================================================================================"
    echo ""
    echo "时间: $(date '+%Y-%m-%d %H:%M:%S')"
    echo ""
} > "$REPORT_FILE"

# 测试1: 美国新闻
echo "================================================================================"
echo "  测试 1/2: 美国新闻"
echo "================================================================================"
echo ""
echo "📝 请在 WebUI 中输入: 美国新闻"
echo ""

if wait_for_activity 60; then
    sleep 3  # 等待日志完全写入
    result1=$(analyze_recent_logs "美国新闻" "news_search")
else
    echo "  ❌ 测试超时"
    result1=false
fi

echo ""
echo "按 Enter 继续下一个测试..."
read

# 测试2: 计算测试
echo "================================================================================"
echo "  测试 2/2: 计算测试（对照组）"
echo "================================================================================"
echo ""
echo "📝 请在 WebUI 中输入: 请使用 calc 工具计算 2+2"
echo ""

if wait_for_activity 60; then
    sleep 3
    result2=$(analyze_recent_logs "计算测试" "calc")
else
    echo "  ❌ 测试超时"
    result2=false
fi

# 生成总结
echo ""
echo "================================================================================"
echo "  测试总结"
echo "================================================================================"
echo ""

total=2
passed=0
[ "$result1" = "true" ] && passed=$((passed + 1))
[ "$result2" = "true" ] && passed=$((passed + 1))

echo "总测试数: $total"
echo "通过: $passed ✅"
echo "失败: $((total - passed)) ❌"
echo "通过率: $(( passed * 100 / total ))%"
echo ""

{
    echo "================================================================================"
    echo "  总结"
    echo "================================================================================"
    echo ""
    echo "总测试数: $total"
    echo "通过: $passed"
    echo "失败: $((total - passed))"
    echo "通过率: $(( passed * 100 / total ))%"
    echo ""
} >> "$REPORT_FILE"

# 显示关键日志
echo "🔍 最近的关键日志:"
echo ""
tail -50 "$LOG_FILE" | grep -E "(tool_calls_count|tool.*:|response=)" | tail -10

echo ""
echo "📄 详细报告已保存: $REPORT_FILE"
echo ""

if [ $passed -eq $total ]; then
    echo "🎉 所有测试通过！新闻工具修复成功！"
    exit 0
elif [ $passed -gt 0 ]; then
    echo "⚠️  部分测试通过，需要进一步调试"
    exit 1
else
    echo "❌ 所有测试失败，需要重新审视修复方案"
    exit 1
fi
