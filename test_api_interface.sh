#!/bin/bash
# ClawMaster API 测试脚本
# 通过 HTTP API 测试系统功能

set -e

API_URL="https://localhost:59233/api"
LOG_FILE="api_test_results_$(date +%Y%m%d_%H%M%S).log"

echo "========================================" | tee -a "$LOG_FILE"
echo "ClawMaster API 全面测试" | tee -a "$LOG_FILE"
echo "开始时间: $(date)" | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# 辅助函数：发送消息并观察响应
send_message() {
    local test_num=$1
    local test_name=$2
    local message=$3
    
    echo "【测试 ${test_num}】${test_name}" | tee -a "$LOG_FILE"
    echo "输入: ${message}" | tee -a "$LOG_FILE"
    echo "---" | tee -a "$LOG_FILE"
    
    # 使用 curl 发送消息到 API
    curl -k -X POST "${API_URL}/chat/message" \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"${message}\", \"session\": \"test\"}" \
        2>&1 | tee -a "$LOG_FILE"
    
    echo "" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"
    sleep 3
}

# 测试 1: 中文新闻查询
send_message 1 "中文新闻查询" "今天有什么中国新闻？"

# 测试 2: 英文新闻查询
send_message 2 "英文新闻查询" "What's the latest news in Germany?"

# 测试 3: 特定类别新闻
send_message 3 "特定类别新闻" "给我看看美国的科技新闻"

# 测试 4: 身份问答（中文）
send_message 4 "身份问答（中文）" "你是谁？"

# 测试 5: 身份问答（英文）
send_message 5 "身份问答（英文）" "What can you do?"

# 测试 6: 计算功能
send_message 6 "计算功能" "计算 123 + 456"

# 测试 7: 复杂查询
send_message 7 "复杂查询" "我想了解一下今天上海的新闻"

# 测试 8: 模糊查询
send_message 8 "模糊查询" "有什么新闻吗？"

# 测试 9: 数学表达式
send_message 9 "数学表达式" "Calculate (15 + 25) * 3"

# 测试 10: 体育新闻
send_message 10 "体育新闻" "Show me sports news from USA"

echo "========================================" | tee -a "$LOG_FILE"
echo "测试完成时间: $(date)" | tee -a "$LOG_FILE"
echo "日志文件: $LOG_FILE" | tee -a "$LOG_FILE"
echo "========================================" | tee -a "$LOG_FILE"
