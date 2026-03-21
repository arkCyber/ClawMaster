#!/bin/bash
# 快速测试脚本 - 测试 3 个工具作为示例

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║     ClawMaster 快速测试示例（3个工具，9个场景）            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 测试 1: calc
echo -e "${BLUE}[1/9] 测试 calc - 简单算术${NC}"
echo -e "${YELLOW}输入: 计算 123 + 456${NC}"
cargo run --release --bin clawmaster -- agent --message "计算 123 + 456"
echo ""

echo -e "${BLUE}[2/9] 测试 calc - 复杂表达式${NC}"
echo -e "${YELLOW}输入: Calculate (15 + 25) * 3${NC}"
cargo run --release --bin clawmaster -- agent --message "Calculate (15 + 25) * 3"
echo ""

echo -e "${BLUE}[3/9] 测试 calc - 幂运算${NC}"
echo -e "${YELLOW}输入: What is 2 to the power of 10?${NC}"
cargo run --release --bin clawmaster -- agent --message "What is 2 to the power of 10?"
echo ""

# 测试 2: task_list
echo -e "${BLUE}[4/9] 测试 task_list - 添加任务${NC}"
echo -e "${YELLOW}输入: Add a task: Review code changes${NC}"
cargo run --release --bin clawmaster -- agent --message "Add a task: Review code changes"
echo ""

echo -e "${BLUE}[5/9] 测试 task_list - 列出任务${NC}"
echo -e "${YELLOW}输入: Show me my tasks${NC}"
cargo run --release --bin clawmaster -- agent --message "Show me my tasks"
echo ""

echo -e "${BLUE}[6/9] 测试 task_list - 完成任务${NC}"
echo -e "${YELLOW}输入: Mark task 1 as complete${NC}"
cargo run --release --bin clawmaster -- agent --message "Mark task 1 as complete"
echo ""

# 测试 3: sessions_list
echo -e "${BLUE}[7/9] 测试 sessions_list - 列出会话${NC}"
echo -e "${YELLOW}输入: List all sessions${NC}"
cargo run --release --bin clawmaster -- agent --message "List all sessions"
echo ""

echo -e "${BLUE}[8/9] 测试 sessions_list - 活跃会话${NC}"
echo -e "${YELLOW}输入: Show me active sessions${NC}"
cargo run --release --bin clawmaster -- agent --message "Show me active sessions"
echo ""

echo -e "${BLUE}[9/9] 测试 sessions_list - 查找会话${NC}"
echo -e "${YELLOW}输入: Find session named 'main'${NC}"
cargo run --release --bin clawmaster -- agent --message "Find session named 'main'"
echo ""

echo -e "${GREEN}快速测试完成！${NC}"
