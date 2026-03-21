#!/bin/bash
# 快速测试脚本 - 验证修复后的功能

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     ClawMaster 快速测试（修复后，3个场景）                ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

LOG_DIR="quick_test_logs_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$LOG_DIR"

PASSED=0
FAILED=0
TOTAL=3

# 测试函数
run_test() {
    local test_num=$1
    local test_name="$2"
    local input="$3"
    local log_file="$LOG_DIR/${test_num}_${test_name}.log"
    
    echo -e "${BLUE}[${test_num}/${TOTAL}] 测试: ${test_name}${NC}"
    echo -e "${YELLOW}输入: ${input}${NC}"
    
    # 记录开始时间
    local start_time=$(date +%s)
    
    # 写入日志
    echo "=== Test: ${test_name} ===" > "$log_file"
    echo "Input: ${input}" >> "$log_file"
    echo "Time: $(date)" >> "$log_file"
    echo "" >> "$log_file"
    
    # 运行命令（后台进程 + 超时控制）
    cargo run --release --bin clawmaster -- agent --message "$input" >> "$log_file" 2>&1 &
    local cmd_pid=$!
    
    # 等待最多30秒
    local timeout=30
    local elapsed=0
    while kill -0 $cmd_pid 2>/dev/null && [ $elapsed -lt $timeout ]; do
        sleep 1
        ((elapsed++))
    done
    
    # 检查进程状态
    if kill -0 $cmd_pid 2>/dev/null; then
        # 超时
        kill -9 $cmd_pid 2>/dev/null
        wait $cmd_pid 2>/dev/null
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        echo -e "  ${YELLOW}⏱️  超时${NC} (${duration}s)"
        echo ""
    else
        # 完成
        wait $cmd_pid
        local exit_code=$?
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        if [ $exit_code -eq 0 ]; then
            echo -e "  ${GREEN}✅ 通过${NC} (${duration}s)"
            ((PASSED++))
        else
            echo -e "  ${RED}❌ 失败${NC} (exit code: ${exit_code}, ${duration}s)"
            ((FAILED++))
        fi
        echo ""
    fi
}

# 执行测试
run_test 1 "calc_简单算术" "计算 123 + 456"
run_test 2 "task_list_添加任务" "Add a task: Review code changes"
run_test 3 "sessions_list_列出会话" "List all sessions"

# 显示结果
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}测试完成！${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "总测试数: ${TOTAL}"
echo "通过: ${PASSED}"
echo "失败: ${FAILED}"
echo ""
echo "通过率: $((PASSED * 100 / TOTAL))%"
echo "日志目录: ${LOG_DIR}"
echo ""
