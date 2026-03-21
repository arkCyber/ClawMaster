#!/bin/bash
# 强制工具执行测试 - 验证 LLM 实际调用工具而不是解释

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

GATEWAY_URL="https://localhost:50699"
LOG_DIR="force_tool_test_$(date +%Y%m%d_%H%M%S)"
PASSED=0
FAILED=0
TOTAL=0

mkdir -p "$LOG_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   强制工具执行测试 - 验证实际调用而非解释                ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 测试函数 - 验证工具是否被实际调用
test_tool_execution() {
    local num=$1
    local tool="$2"
    local cmd="$3"
    local expected_pattern="$4"  # 期望的响应模式（不应该是"你可以使用"）
    
    ((TOTAL++))
    
    echo -e "${CYAN}[${num}] ${tool}${NC}"
    echo -e "  ${YELLOW}命令:${NC} ${cmd}"
    
    local log="$LOG_DIR/${num}_${tool}.log"
    
    # 运行测试
    CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" ./target/release/clawmaster agent --message "$cmd" > "$log" 2>&1 &
    local pid=$!
    
    # 等待最多30秒
    local elapsed=0
    while [ $elapsed -lt 30 ]; do
        if ! kill -0 $pid 2>/dev/null; then
            break
        fi
        sleep 1
        ((elapsed++))
    done
    
    if kill -0 $pid 2>/dev/null; then
        kill -9 $pid 2>/dev/null
        wait $pid 2>/dev/null
        echo -e "  ${YELLOW}⏱️  超时${NC}"
        return
    fi
    
    wait $pid
    local exit_code=$?
    
    if [ $exit_code -ne 0 ]; then
        echo -e "  ${RED}❌ 失败 (exit: ${exit_code})${NC}"
        ((FAILED++))
        return
    fi
    
    # 提取响应
    local response=$(grep "✅ 响应:" "$log" -A 10 | tail -10 | tr '\n' ' ')
    
    # 检查是否是解释性回答（错误）
    if echo "$response" | grep -qi "你可以使用\|可以用\|使用以下命令\|Here's how\|You can use"; then
        echo -e "  ${RED}❌ 失败 - LLM只是解释而非执行${NC}"
        echo -e "  ${RED}响应:${NC} ${response:0:150}..."
        ((FAILED++))
    # 检查是否包含期望的模式（正确）
    elif echo "$response" | grep -qi "$expected_pattern"; then
        echo -e "  ${GREEN}✅ 通过 - 工具被实际调用${NC}"
        echo -e "  ${GREEN}响应:${NC} ${response:0:150}..."
        ((PASSED++))
    else
        echo -e "  ${YELLOW}⚠️  不确定 - 需要人工检查${NC}"
        echo -e "  ${YELLOW}响应:${NC} ${response:0:150}..."
    fi
    
    echo ""
}

echo -e "${GREEN}开始强制工具执行测试...${NC}"
echo ""

# 测试 1: calc - 应该返回计算结果，不是解释
test_tool_execution 1 "calc" "计算 123 + 456" "579"

# 测试 2: calc - 复杂运算
test_tool_execution 2 "calc" "计算 2 的 10 次方" "1024"

# 测试 3: news_search - 应该返回实际新闻，不是解释
test_tool_execution 3 "news_search" "搜索最新科技新闻" "news\|新闻\|article\|文章"

# 测试 4: memory_save - 应该确认保存，不是解释
test_tool_execution 4 "memory_save" "记住：我喜欢 Rust" "保存\|saved\|记住了\|remembered"

# 测试 5: memory_search - 应该返回实际记忆内容
test_tool_execution 5 "memory_search" "我喜欢什么编程语言" "Rust"

# 测试 6: exec - 应该返回实际命令输出
test_tool_execution 6 "exec" "列出当前目录的文件" "Cargo.toml\|crates\|target"

# 测试 7: task_list - 应该返回实际任务列表
test_tool_execution 7 "task_list" "显示所有任务" "task\|任务\|list"

# 测试 8: sessions_list - 应该返回实际会话列表
test_tool_execution 8 "sessions_list" "显示所有会话" "main\|session\|会话"

# 测试 9: web_search - 应该返回搜索结果
test_tool_execution 9 "web_search" "搜索 Rust 教程" "rust\|tutorial\|教程"

# 测试 10: browser - 应该执行浏览器操作
test_tool_execution 10 "browser" "打开 https://www.rust-lang.org" "opened\|打开\|browser"

# 显示结果
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}测试完成！${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "总测试数: ${TOTAL}"
echo -e "${GREEN}通过（工具被调用）: ${PASSED}${NC}"
echo -e "${RED}失败（只是解释）: ${FAILED}${NC}"
if [ $TOTAL -gt 0 ]; then
    echo "实际执行率: $((PASSED * 100 / TOTAL))%"
fi
echo ""
echo "日志目录: ${LOG_DIR}"
echo ""

if [ $FAILED -gt 0 ]; then
    echo -e "${RED}⚠️  发现问题：LLM 没有实际调用工具，只是在解释如何使用${NC}"
    echo -e "${YELLOW}建议：检查 system prompt 是否包含正确的工具调用指导${NC}"
    exit 1
fi

exit 0
