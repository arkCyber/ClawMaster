#!/bin/bash
# ClawMaster CLI 测试平台演示脚本
# 快速演示测试平台的功能

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

clear

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}║          ClawMaster CLI 测试平台演示                       ║${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 检查 clawmaster 是否存在
if [ ! -f "../target/debug/clawmaster" ]; then
    echo -e "${RED}错误: ClawMaster 未编译${NC}"
    echo -e "${YELLOW}请先运行: cd .. && cargo build${NC}"
    exit 1
fi

echo -e "${BLUE}✓ ClawMaster 已就绪${NC}"
echo ""

# 显示菜单
show_menu() {
    echo -e "${YELLOW}请选择测试模式：${NC}"
    echo ""
    echo -e "  ${GREEN}1${NC} - 交互式测试 (实时对话)"
    echo -e "  ${GREEN}2${NC} - 自动化测试 (10个场景)"
    echo -e "  ${GREEN}3${NC} - 性能测试 (性能指标)"
    echo -e "  ${GREEN}4${NC} - 日志分析 (分析已有日志)"
    echo -e "  ${GREEN}5${NC} - 快速演示 (单个测试)"
    echo -e "  ${GREEN}0${NC} - 退出"
    echo ""
    echo -n -e "${CYAN}请输入选项 [0-5]: ${NC}"
}

# 快速演示
quick_demo() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}快速演示：新闻查询测试${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
    
    echo -e "${YELLOW}测试消息:${NC} 今天有什么中国新闻？"
    echo ""
    echo -e "${CYAN}正在执行...${NC}"
    echo ""
    
    # 创建临时日志目录
    mkdir -p ./test_logs
    
    # 执行测试
    if ../target/debug/clawmaster agent --message "今天有什么中国新闻？" --log-level info 2>&1 | tee ./test_logs/demo.log; then
        echo ""
        echo -e "${GREEN}✓ 测试成功完成${NC}"
        echo ""
        
        # 提取关键信息
        echo -e "${BLUE}关键信息：${NC}"
        
        TOOL_CALLS=$(grep -c "executing tool" ./test_logs/demo.log || echo "0")
        echo -e "  ${GREEN}工具调用:${NC} ${TOOL_CALLS} 次"
        
        if grep -q "Location extracted" ./test_logs/demo.log; then
            LOCATION=$(grep "Location extracted" ./test_logs/demo.log | head -1)
            echo -e "  ${GREEN}位置提取:${NC} ${LOCATION##*: }"
        fi
        
        if grep -q "Selected.*feeds" ./test_logs/demo.log; then
            FEEDS=$(grep "Selected.*feeds" ./test_logs/demo.log | head -1)
            echo -e "  ${GREEN}数据源:${NC} ${FEEDS##*: }"
        fi
    else
        echo ""
        echo -e "${RED}✗ 测试失败${NC}"
    fi
    
    echo ""
    echo -e "${YELLOW}日志已保存到: ./test_logs/demo.log${NC}"
    echo ""
}

# 主循环
while true; do
    show_menu
    read -r choice
    
    case $choice in
        1)
            echo ""
            echo -e "${GREEN}启动交互式测试...${NC}"
            echo ""
            ./interactive_test.sh
            ;;
        2)
            echo ""
            echo -e "${GREEN}启动自动化测试...${NC}"
            echo ""
            ./auto_test.sh
            ;;
        3)
            echo ""
            echo -e "${GREEN}启动性能测试...${NC}"
            echo ""
            ./performance_test.sh
            ;;
        4)
            echo ""
            echo -e "${GREEN}启动日志分析...${NC}"
            echo ""
            ./log_analyzer.sh
            ;;
        5)
            echo ""
            quick_demo
            echo -n -e "${CYAN}按 Enter 继续...${NC}"
            read
            clear
            ;;
        0)
            echo ""
            echo -e "${YELLOW}退出演示${NC}"
            echo ""
            exit 0
            ;;
        *)
            echo ""
            echo -e "${RED}无效选项，请重新选择${NC}"
            echo ""
            sleep 1
            clear
            ;;
    esac
done
