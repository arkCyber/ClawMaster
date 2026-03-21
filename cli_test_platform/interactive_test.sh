#!/bin/bash
# ClawMaster CLI 交互式测试平台
# 提供实时对话测试功能，无需启动 WebUI

set -e

CLAWMASTER="../target/debug/clawmaster"
LOG_DIR="./test_logs"
SESSION_ID="cli_test_$(date +%Y%m%d_%H%M%S)"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 创建日志目录
mkdir -p "$LOG_DIR"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}ClawMaster CLI 交互式测试平台${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${GREEN}会话 ID: ${SESSION_ID}${NC}"
echo -e "${GREEN}日志目录: ${LOG_DIR}${NC}"
echo ""
echo -e "${YELLOW}提示：${NC}"
echo "  - 输入消息进行测试"
echo "  - 输入 'quit' 或 'exit' 退出"
echo "  - 输入 'help' 查看帮助"
echo "  - 输入 'stats' 查看统计"
echo ""

# 统计变量
TOTAL_QUERIES=0
SUCCESSFUL_QUERIES=0
FAILED_QUERIES=0
TOTAL_TIME=0

# 帮助信息
show_help() {
    echo -e "${BLUE}可用命令：${NC}"
    echo "  help     - 显示此帮助信息"
    echo "  stats    - 显示测试统计"
    echo "  clear    - 清屏"
    echo "  quit     - 退出测试"
    echo ""
    echo -e "${BLUE}测试示例：${NC}"
    echo "  今天有什么中国新闻？"
    echo "  What's the latest news in USA?"
    echo "  计算 123 + 456"
    echo "  你是谁？"
    echo ""
}

# 显示统计
show_stats() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}测试统计${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo -e "${GREEN}总查询数: ${TOTAL_QUERIES}${NC}"
    echo -e "${GREEN}成功: ${SUCCESSFUL_QUERIES}${NC}"
    echo -e "${RED}失败: ${FAILED_QUERIES}${NC}"
    if [ $TOTAL_QUERIES -gt 0 ]; then
        AVG_TIME=$((TOTAL_TIME / TOTAL_QUERIES))
        echo -e "${YELLOW}平均响应时间: ${AVG_TIME}ms${NC}"
    fi
    echo ""
}

# 执行查询
execute_query() {
    local message="$1"
    local start_time=$(date +%s%3N)
    
    TOTAL_QUERIES=$((TOTAL_QUERIES + 1))
    
    echo -e "${YELLOW}[查询 #${TOTAL_QUERIES}]${NC} ${message}"
    echo ""
    
    # 执行命令并捕获输出
    local log_file="${LOG_DIR}/query_${TOTAL_QUERIES}.log"
    
    if $CLAWMASTER agent --message "$message" --log-level info 2>&1 | tee "$log_file"; then
        SUCCESSFUL_QUERIES=$((SUCCESSFUL_QUERIES + 1))
        echo -e "${GREEN}✓ 查询成功${NC}"
    else
        FAILED_QUERIES=$((FAILED_QUERIES + 1))
        echo -e "${RED}✗ 查询失败${NC}"
    fi
    
    local end_time=$(date +%s%3N)
    local duration=$((end_time - start_time))
    TOTAL_TIME=$((TOTAL_TIME + duration))
    
    echo -e "${BLUE}响应时间: ${duration}ms${NC}"
    echo ""
}

# 主循环
show_help

while true; do
    echo -n -e "${GREEN}> ${NC}"
    read -r input
    
    # 去除首尾空格
    input=$(echo "$input" | xargs)
    
    # 检查空输入
    if [ -z "$input" ]; then
        continue
    fi
    
    # 处理命令
    case "$input" in
        quit|exit)
            echo -e "${YELLOW}退出测试平台...${NC}"
            show_stats
            echo -e "${GREEN}日志已保存到: ${LOG_DIR}${NC}"
            exit 0
            ;;
        help)
            show_help
            ;;
        stats)
            show_stats
            ;;
        clear)
            clear
            echo -e "${BLUE}ClawMaster CLI 交互式测试平台${NC}"
            echo ""
            ;;
        *)
            execute_query "$input"
            ;;
    esac
done
