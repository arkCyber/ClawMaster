#!/bin/bash
# ClawMaster CLI 全面测试脚本
# 通过 CLI 接口测试各项功能，观察终端输出

set -e

CLAWMASTER="./target/debug/clawmaster"
LOG_FILE="test_results_$(date +%Y%m%d_%H%M%S).log"

echo "========================================" | tee -a "$LOG_FILE"
echo "ClawMaster CLI 全面测试" | tee -a "$LOG_FILE"
echo "开始时间: $(date)" | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# 测试 1: 中文新闻查询
echo "【测试 1】中文新闻查询" | tee -a "$LOG_FILE"
echo "输入: 今天有什么中国新闻？" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "今天有什么中国新闻？" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 2: 英文新闻查询
echo "【测试 2】英文新闻查询" | tee -a "$LOG_FILE"
echo "输入: What's the latest news in Germany?" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "What's the latest news in Germany?" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 3: 特定类别新闻
echo "【测试 3】特定类别新闻查询" | tee -a "$LOG_FILE"
echo "输入: 给我看看美国的科技新闻" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "给我看看美国的科技新闻" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 4: 身份问答（中文）
echo "【测试 4】身份问答（中文）" | tee -a "$LOG_FILE"
echo "输入: 你是谁？" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "你是谁？" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 5: 身份问答（英文）
echo "【测试 5】身份问答（英文）" | tee -a "$LOG_FILE"
echo "输入: What can you do?" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "What can you do?" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 6: 计算功能
echo "【测试 6】计算功能" | tee -a "$LOG_FILE"
echo "输入: 计算 123 + 456" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "计算 123 + 456" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 7: 复杂查询（城市+国家）
echo "【测试 7】复杂查询（城市+国家）" | tee -a "$LOG_FILE"
echo "输入: 我想了解一下今天上海的新闻" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "我想了解一下今天上海的新闻" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 8: 模糊查询
echo "【测试 8】模糊查询" | tee -a "$LOG_FILE"
echo "输入: 有什么新闻吗？" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "有什么新闻吗？" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 9: 数学表达式
echo "【测试 9】数学表达式" | tee -a "$LOG_FILE"
echo "输入: Calculate (15 + 25) * 3" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "Calculate (15 + 25) * 3" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"
sleep 2

# 测试 10: 体育新闻
echo "【测试 10】体育新闻" | tee -a "$LOG_FILE"
echo "输入: Show me sports news from USA" | tee -a "$LOG_FILE"
echo "---" | tee -a "$LOG_FILE"
$CLAWMASTER agent --message "Show me sports news from USA" 2>&1 | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

echo "========================================" | tee -a "$LOG_FILE"
echo "测试完成时间: $(date)" | tee -a "$LOG_FILE"
echo "日志文件: $LOG_FILE" | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"
