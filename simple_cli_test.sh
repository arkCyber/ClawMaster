#!/bin/bash
# 简单 CLI 测试脚本 - 使用默认配置

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     ClawMaster 简单 CLI 测试（3个工具，9个场景）           ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 测试 1: calc
echo -e "${BLUE}[1/9] 测试 calc - 简单算术${NC}"
echo -e "${YELLOW}输入: 计算 123 + 456${NC}"
cargo run --bin clawmaster -- agent --message "计算 123 + 456" 2>&1 | tee test_calc_1.log
echo ""

echo -e "${BLUE}[2/9] 测试 calc - 复杂表达式${NC}"
echo -e "${YELLOW}输入: Calculate (15 + 25) * 3${NC}"
cargo run --bin clawmaster -- agent --message "Calculate (15 + 25) * 3" 2>&1 | tee test_calc_2.log
echo ""

echo -e "${BLUE}[3/9] 测试 calc - 幂运算${NC}"
echo -e "${YELLOW}输入: What is 2 to the power of 10?${NC}"
cargo run --bin clawmaster -- agent --message "What is 2 to the power of 10?" 2>&1 | tee test_calc_3.log
echo ""

# 测试 2: task_list
echo -e "${BLUE}[4/9] 测试 task_list - 添加任务${NC}"
echo -e "${YELLOW}输入: Add a task: Review code changes${NC}"
cargo run --bin clawmaster -- agent --message "Add a task: Review code changes" 2>&1 | tee test_task_1.log
echo ""

echo -e "${BLUE}[5/9] 测试 task_list - 列出任务${NC}"
echo -e "${YELLOW}输入: Show me my tasks${NC}"
cargo run --bin clawmaster -- agent --message "Show me my tasks" 2>&1 | tee test_task_2.log
echo ""

echo -e "${BLUE}[6/9] 测试 task_list - 完成任务${NC}"
echo -e "${YELLOW}输入: Mark task 1 as complete${NC}"
cargo run --bin clawmaster -- agent --message "Mark task 1 as complete" 2>&1 | tee test_task_3.log
echo ""

# 测试 3: sessions_list
echo -e "${BLUE}[7/9] 测试 sessions_list - 列出会话${NC}"
echo -e "${YELLOW}输入: List all sessions${NC}"
cargo run --bin clawmaster -- agent --message "List all sessions" 2>&1 | tee test_sessions_1.log
echo ""

echo -e "${BLUE}[8/9] 测试 sessions_list - 活跃会话${NC}"
echo -e "${YELLOW}输入: Show me active sessions${NC}"
cargo run --bin clawmaster -- agent --message "Show me active sessions" 2>&1 | tee test_sessions_2.log
echo ""

echo -e "${BLUE}[9/9] 测试 sessions_list - 查找会话${NC}"
echo -e "${YELLOW}输入: Find session named 'main'${NC}"
cargo run --bin clawmaster -- agent --message "Find session named 'main'" 2>&1 | tee test_sessions_3.log
echo ""

echo -e "${GREEN}测试完成！${NC}"
echo ""
echo -e "${CYAN}日志文件已保存：${NC}"
ls -lh test_*.log
