#!/bin/bash
# ClawMaster 工具交互式测试脚本
# 在终端中逐个显示测试问题，等待用户在 WebUI 中测试

set -e

# 颜色输出
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

# 测试问题数组
declare -a tests=(
    "exec|请执行命令：echo 'Hello from ClawMaster'"
    "exec|请列出当前目录的文件和文件夹"
    "calc|请计算：(123 + 456) * 789"
    "calc|请计算 2 的 10 次方"
    "web_search|请搜索：Rust 编程语言最新特性"
    "web_fetch|请获取 https://example.com 的内容"
    "browser|请打开浏览器访问 https://www.wikipedia.org"
    "memory_save|请记住：今天完成了 ClawMaster 工具测试"
    "memory_search|请搜索关于'测试'的记忆"
    "sessions_list|请列出所有活跃的会话"
    "task_list|请显示当前的任务列表"
    "location|上海天气？"
    "show_map|请显示上海的地图"
)

echo -e "${BLUE}========================================"
echo "  ClawMaster 工具交互式测试"
echo "========================================${NC}"
echo ""
echo -e "${YELLOW}使用说明：${NC}"
echo "1. 在浏览器中打开 https://localhost:3000"
echo "2. 每次显示测试问题后，复制到 WebUI 对话窗口"
echo "3. 观察 AI 的响应和工具调用"
echo "4. 按回车继续下一个测试"
echo ""
echo -e "${CYAN}按回车开始测试...${NC}"
read

test_num=1
total=${#tests[@]}

for test in "${tests[@]}"; do
    IFS='|' read -r tool question <<< "$test"
    
    clear
    echo -e "${BLUE}========================================"
    echo "  测试进度: $test_num / $total"
    echo "========================================${NC}"
    echo ""
    echo -e "${GREEN}工具: $tool${NC}"
    echo ""
    echo -e "${YELLOW}测试问题：${NC}"
    echo -e "${CYAN}$question${NC}"
    echo ""
    echo "---"
    echo ""
    echo -e "${YELLOW}请将上述问题复制到 WebUI 对话窗口${NC}"
    echo ""
    echo -e "按回车继续下一个测试..."
    read
    
    test_num=$((test_num + 1))
done

clear
echo -e "${GREEN}========================================"
echo "  所有测试问题已显示完毕！"
echo "========================================${NC}"
echo ""
echo "请在 WebUI 中查看测试结果"
echo ""
