#!/bin/bash
# ClawMaster CLI 自动化测试套件
# 执行预定义的测试场景，生成详细报告

set -e

CLAWMASTER="../target/debug/clawmaster"
LOG_DIR="./test_logs"
REPORT_FILE="./test_report_$(date +%Y%m%d_%H%M%S).md"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 创建日志目录
mkdir -p "$LOG_DIR"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}ClawMaster CLI 自动化测试套件${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# 初始化报告
cat > "$REPORT_FILE" << 'EOF'
# ClawMaster CLI 自动化测试报告

**执行时间**: $(date)

---

## 测试场景

EOF

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 执行测试
run_test() {
    local test_name="$1"
    local test_message="$2"
    local expected_behavior="$3"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    echo -e "${YELLOW}[测试 ${TOTAL_TESTS}] ${test_name}${NC}"
    echo "输入: ${test_message}"
    
    local log_file="${LOG_DIR}/test_${TOTAL_TESTS}.log"
    local start_time=$(date +%s%3N)
    
    # 执行测试
    if $CLAWMASTER agent --message "$test_message" --log-level debug 2>&1 > "$log_file"; then
        local end_time=$(date +%s%3N)
        local duration=$((end_time - start_time))
        
        # 分析结果
        local has_tool_call=$(grep -c "executing tool" "$log_file" || echo "0")
        local has_error=$(grep -c "ERROR\|error\|failed" "$log_file" || echo "0")
        
        if [ "$has_error" -eq 0 ]; then
            PASSED_TESTS=$((PASSED_TESTS + 1))
            echo -e "${GREEN}✓ 通过${NC} (${duration}ms)"
            
            # 写入报告
            cat >> "$REPORT_FILE" << EOF

### ✅ ${test_name}

**输入**: ${test_message}  
**预期**: ${expected_behavior}  
**结果**: 通过  
**响应时间**: ${duration}ms  
**工具调用**: ${has_tool_call} 次  

EOF
        else
            FAILED_TESTS=$((FAILED_TESTS + 1))
            echo -e "${RED}✗ 失败${NC} (发现错误)"
            
            # 写入报告
            cat >> "$REPORT_FILE" << EOF

### ❌ ${test_name}

**输入**: ${test_message}  
**预期**: ${expected_behavior}  
**结果**: 失败  
**错误**: 检测到错误日志  

EOF
        fi
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo -e "${RED}✗ 失败${NC} (执行错误)"
        
        # 写入报告
        cat >> "$REPORT_FILE" << EOF

### ❌ ${test_name}

**输入**: ${test_message}  
**预期**: ${expected_behavior}  
**结果**: 失败  
**错误**: 命令执行失败  

EOF
    fi
    
    echo ""
    sleep 1
}

# ========================================
# 测试场景定义
# ========================================

echo -e "${BLUE}开始执行测试...${NC}"
echo ""

# 场景 1: 中文新闻查询
run_test "中文新闻查询" \
    "今天有什么中国新闻？" \
    "调用 news_search 工具，location=China"

# 场景 2: 英文新闻查询
run_test "英文新闻查询" \
    "What's the latest news in USA?" \
    "调用 news_search 工具，location=USA"

# 场景 3: 特定类别新闻
run_test "科技新闻查询" \
    "给我看看美国的科技新闻" \
    "调用 news_search 工具，location=USA, category=technology"

# 场景 4: 身份问答（中文）
run_test "身份问答（中文）" \
    "你是谁？" \
    "直接回答，不调用工具"

# 场景 5: 身份问答（英文）
run_test "身份问答（英文）" \
    "What can you do?" \
    "直接回答，不调用工具"

# 场景 6: 简单计算
run_test "简单计算" \
    "计算 123 + 456" \
    "调用 calc 工具或直接回答"

# 场景 7: 复杂表达式
run_test "复杂表达式" \
    "Calculate (15 + 25) * 3" \
    "调用 calc 工具或直接回答"

# 场景 8: 城市新闻查询
run_test "城市新闻查询" \
    "我想了解一下今天上海的新闻" \
    "调用 news_search 工具，location=Shanghai, China"

# 场景 9: 模糊查询
run_test "模糊新闻查询" \
    "有什么新闻吗？" \
    "调用 news_search 工具，使用默认参数"

# 场景 10: 体育新闻
run_test "体育新闻查询" \
    "Show me sports news from USA" \
    "调用 news_search 工具，location=USA, category=sports"

# ========================================
# 生成测试摘要
# ========================================

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}测试完成${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${GREEN}总测试数: ${TOTAL_TESTS}${NC}"
echo -e "${GREEN}通过: ${PASSED_TESTS}${NC}"
echo -e "${RED}失败: ${FAILED_TESTS}${NC}"

if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${YELLOW}通过率: ${PASS_RATE}%${NC}"
fi

echo ""
echo -e "${GREEN}报告已生成: ${REPORT_FILE}${NC}"
echo -e "${GREEN}日志目录: ${LOG_DIR}${NC}"

# 写入摘要到报告
cat >> "$REPORT_FILE" << EOF

---

## 测试摘要

| 指标 | 数值 |
|------|------|
| 总测试数 | ${TOTAL_TESTS} |
| 通过 | ${PASSED_TESTS} |
| 失败 | ${FAILED_TESTS} |
| 通过率 | ${PASS_RATE}% |

**执行完成时间**: $(date)

EOF

echo ""
