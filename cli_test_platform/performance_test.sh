#!/bin/bash
# ClawMaster CLI 性能测试工具
# 测试响应时间、Token 使用、迭代次数等性能指标

set -e

CLAWMASTER="../target/debug/clawmaster"
LOG_DIR="./test_logs"
PERF_REPORT="./performance_report_$(date +%Y%m%d_%H%M%S).md"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

mkdir -p "$LOG_DIR"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}ClawMaster CLI 性能测试${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# 初始化报告
cat > "$PERF_REPORT" << 'EOF'
# ClawMaster CLI 性能测试报告

**执行时间**: $(date)

---

## 性能指标

EOF

# 性能测试
run_performance_test() {
    local test_name="$1"
    local test_message="$2"
    
    echo -e "${YELLOW}[性能测试] ${test_name}${NC}"
    
    local log_file="${LOG_DIR}/perf_${test_name// /_}.log"
    local start_time=$(date +%s%3N)
    
    # 执行测试
    $CLAWMASTER agent --message "$test_message" --log-level debug 2>&1 > "$log_file"
    
    local end_time=$(date +%s%3N)
    local duration=$((end_time - start_time))
    
    # 提取性能数据
    local iterations=$(grep -c "iteration=" "$log_file" || echo "0")
    local input_tokens=$(grep "input_tokens=" "$log_file" | tail -1 | sed 's/.*input_tokens=\([0-9]*\).*/\1/' || echo "0")
    local output_tokens=$(grep "output_tokens=" "$log_file" | tail -1 | sed 's/.*output_tokens=\([0-9]*\).*/\1/' || echo "0")
    local tool_calls=$(grep -c "executing tool" "$log_file" || echo "0")
    
    echo -e "${GREEN}响应时间: ${duration}ms${NC}"
    echo -e "${GREEN}迭代次数: ${iterations}${NC}"
    echo -e "${GREEN}输入 Tokens: ${input_tokens}${NC}"
    echo -e "${GREEN}输出 Tokens: ${output_tokens}${NC}"
    echo -e "${GREEN}工具调用: ${tool_calls}${NC}"
    echo ""
    
    # 写入报告
    cat >> "$PERF_REPORT" << EOF

### ${test_name}

**输入**: ${test_message}

| 指标 | 数值 |
|------|------|
| 响应时间 | ${duration}ms |
| 迭代次数 | ${iterations} |
| 输入 Tokens | ${input_tokens} |
| 输出 Tokens | ${output_tokens} |
| 工具调用 | ${tool_calls} |

EOF
}

# 执行性能测试
run_performance_test "新闻查询" "今天有什么中国新闻？"
run_performance_test "身份问答" "你是谁？"
run_performance_test "计算功能" "计算 123 + 456"
run_performance_test "复杂查询" "给我看看美国的科技新闻"

echo -e "${GREEN}性能测试完成${NC}"
echo -e "${GREEN}报告已生成: ${PERF_REPORT}${NC}"
echo ""
