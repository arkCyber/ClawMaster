#!/bin/bash
# 简化的全面功能测试脚本

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

GATEWAY_URL="https://localhost:59233"
LOG_DIR="test_results_$(date +%Y%m%d_%H%M%S)"
PASSED=0
FAILED=0
TOTAL=0

mkdir -p "$LOG_DIR"

echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}   ClawMaster 全面功能测试                                  ${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""

# 测试函数
test_tool() {
    local num=$1
    local name="$2"
    local msg="$3"
    
    ((TOTAL++))
    echo -e "${CYAN}[$num] $name${NC}"
    echo "  输入: $msg"
    
    local log="$LOG_DIR/${num}_${name}.log"
    
    if CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" timeout 30s ./target/release/clawmaster agent --message "$msg" > "$log" 2>&1; then
        echo -e "  ${GREEN}✅ 通过${NC}"
        ((PASSED++))
    else
        echo -e "  ${RED}❌ 失败${NC}"
        ((FAILED++))
    fi
    echo ""
}

# 运行测试
echo "开始测试..."
echo ""

# 基础工具测试
test_tool 1 "calc_加法" "计算 123 + 456"
test_tool 2 "calc_乘法" "Calculate 15 * 8"
test_tool 3 "calc_幂运算" "What is 2^10?"

test_tool 4 "task_添加" "Add a task: Test ClawMaster"
test_tool 5 "task_列表" "List all tasks"
test_tool 6 "task_完成" "Complete first task"

test_tool 7 "session_列表" "List sessions"
test_tool 8 "session_创建" "Create new session"
test_tool 9 "session_历史" "Show session history"

test_tool 10 "memory_保存" "Remember: I like Rust"
test_tool 11 "memory_搜索" "What do you remember?"
test_tool 12 "memory_查询" "Search for Rust"

test_tool 13 "web_search" "Search for Rust tutorials"
test_tool 14 "web_fetch" "Fetch https://www.rust-lang.org"
test_tool 15 "news_search" "Find latest AI news"

# 显示结果
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}测试完成！${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "总测试数: $TOTAL"
echo -e "${GREEN}通过: $PASSED${NC}"
echo -e "${RED}失败: $FAILED${NC}"
echo "通过率: $((PASSED * 100 / TOTAL))%"
echo ""
echo "日志目录: $LOG_DIR"
echo ""

# 生成报告
cat > "$LOG_DIR/report.md" << EOF
# ClawMaster 测试报告

**时间**: $(date '+%Y-%m-%d %H:%M:%S')
**总测试**: $TOTAL
**通过**: $PASSED
**失败**: $FAILED
**通过率**: $((PASSED * 100 / TOTAL))%

## 测试详情

查看日志目录: $LOG_DIR
EOF

echo "报告已生成: $LOG_DIR/report.md"
