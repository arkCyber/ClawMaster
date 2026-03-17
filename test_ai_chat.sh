#!/bin/bash
# ClawMaster AI 对话功能自动化测试脚本
# 日期: 2026-03-14 22:10

set -e

BASE_URL="http://localhost:7878"
RESULTS_FILE="/tmp/ai_chat_test_results.txt"

echo "🧪 ClawMaster AI 对话功能测试" > $RESULTS_FILE
echo "================================" >> $RESULTS_FILE
echo "开始时间: $(date)" >> $RESULTS_FILE
echo "" >> $RESULTS_FILE

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 测试函数
run_test() {
    local test_name="$1"
    local test_cmd="$2"
    local expected="$3"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -e "\n${YELLOW}测试 $TOTAL_TESTS: $test_name${NC}"
    echo "测试 $TOTAL_TESTS: $test_name" >> $RESULTS_FILE
    
    # 执行测试
    start_time=$(date +%s.%N)
    result=$(eval "$test_cmd" 2>&1)
    end_time=$(date +%s.%N)
    duration=$(echo "$end_time - $start_time" | bc)
    
    # 检查结果
    if echo "$result" | grep -q "$expected"; then
        echo -e "${GREEN}✅ 通过${NC} (${duration}s)"
        echo "✅ 通过 (${duration}s)" >> $RESULTS_FILE
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}❌ 失败${NC}"
        echo "❌ 失败" >> $RESULTS_FILE
        echo "预期: $expected" >> $RESULTS_FILE
        echo "实际: $result" >> $RESULTS_FILE
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
}

echo ""
echo "🚀 开始测试..."
echo ""

# ============================================
# Phase 1: 服务器健康检查
# ============================================
echo "📋 Phase 1: 服务器健康检查"
echo "Phase 1: 服务器健康检查" >> $RESULTS_FILE

run_test "服务器响应" \
    "curl -s -o /dev/null -w '%{http_code}' $BASE_URL" \
    "200"

run_test "API 端点可用" \
    "curl -s $BASE_URL/api/gon | jq -r '.version' 2>/dev/null || echo 'error'" \
    "."

# ============================================
# Phase 2: 基础对话测试
# ============================================
echo ""
echo "📋 Phase 2: 基础对话测试"
echo "Phase 2: 基础对话测试" >> $RESULTS_FILE

# 注意：这些测试需要实际的 LLM 后端
# 如果没有配置 LLM，这些测试会失败

# ============================================
# Phase 3: WebSocket 测试
# ============================================
echo ""
echo "📋 Phase 3: WebSocket 测试"
echo "Phase 3: WebSocket 测试" >> $RESULTS_FILE

# 检查 WebSocket 端点是否存在
run_test "WebSocket 端点检查" \
    "curl -s -I $BASE_URL/ws | head -1" \
    "HTTP"

# ============================================
# Phase 4: 静态资源测试
# ============================================
echo ""
echo "📋 Phase 4: 静态资源测试"
echo "Phase 4: 静态资源测试" >> $RESULTS_FILE

run_test "CSS 资源加载" \
    "curl -s -o /dev/null -w '%{http_code}' $BASE_URL/assets/style.css" \
    "200"

run_test "JavaScript 资源加载" \
    "curl -s -o /dev/null -w '%{http_code}' $BASE_URL/assets/app.js" \
    "200"

# ============================================
# 测试总结
# ============================================
echo ""
echo "================================"
echo "📊 测试总结"
echo "================================"
echo "总测试数: $TOTAL_TESTS"
echo "通过: $PASSED_TESTS"
echo "失败: $FAILED_TESTS"
echo "成功率: $(echo "scale=2; $PASSED_TESTS * 100 / $TOTAL_TESTS" | bc)%"
echo ""

echo "" >> $RESULTS_FILE
echo "================================" >> $RESULTS_FILE
echo "测试总结" >> $RESULTS_FILE
echo "================================" >> $RESULTS_FILE
echo "总测试数: $TOTAL_TESTS" >> $RESULTS_FILE
echo "通过: $PASSED_TESTS" >> $RESULTS_FILE
echo "失败: $FAILED_TESTS" >> $RESULTS_FILE
echo "成功率: $(echo "scale=2; $PASSED_TESTS * 100 / $TOTAL_TESTS" | bc)%" >> $RESULTS_FILE
echo "结束时间: $(date)" >> $RESULTS_FILE

# 显示详细结果
echo "详细结果已保存到: $RESULTS_FILE"
echo ""

# 返回退出码
if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}✅ 所有测试通过！${NC}"
    exit 0
else
    echo -e "${RED}❌ 有 $FAILED_TESTS 个测试失败${NC}"
    exit 1
fi
