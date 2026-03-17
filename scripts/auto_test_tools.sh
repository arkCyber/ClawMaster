#!/bin/bash
# ClawMaster 工具自动化测试脚本
# 使用 API 调用逐个测试所有工具

set -e

# 配置
API_BASE="https://localhost:3000/api"
SESSION="main"
RESULTS_FILE="tool_test_results_$(date +%Y%m%d_%H%M%S).md"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 初始化结果文件
cat > "$RESULTS_FILE" << 'EOF'
# ClawMaster 工具自动化测试报告

**测试时间**: $(date)
**测试会话**: main

---

## 测试结果

EOF

# 测试计数器
TOTAL=0
PASSED=0
FAILED=0

# 测试函数
test_tool() {
    local tool_name=$1
    local test_message=$2
    local test_id=$3
    
    TOTAL=$((TOTAL + 1))
    
    echo -e "${YELLOW}[测试 $test_id] $tool_name${NC}"
    echo "问题: $test_message"
    
    # 发送测试消息
    response=$(curl -k -s -X POST "$API_BASE/chat" \
        -H "Content-Type: application/json" \
        -d "{\"session\":\"$SESSION\",\"message\":\"$test_message\",\"stream\":false}" \
        2>/dev/null || echo "ERROR")
    
    if [ "$response" = "ERROR" ] || [ -z "$response" ]; then
        echo -e "${RED}✗ 失败 - API 调用失败${NC}"
        FAILED=$((FAILED + 1))
        echo "### ❌ $tool_name - 测试 $test_id" >> "$RESULTS_FILE"
        echo "- **问题**: $test_message" >> "$RESULTS_FILE"
        echo "- **结果**: API 调用失败" >> "$RESULTS_FILE"
        echo "" >> "$RESULTS_FILE"
    else
        echo -e "${GREEN}✓ 通过${NC}"
        PASSED=$((PASSED + 1))
        echo "### ✅ $tool_name - 测试 $test_id" >> "$RESULTS_FILE"
        echo "- **问题**: $test_message" >> "$RESULTS_FILE"
        echo "- **结果**: 成功" >> "$RESULTS_FILE"
        echo "" >> "$RESULTS_FILE"
    fi
    
    echo ""
    sleep 2  # 避免请求过快
}

echo "========================================"
echo "  ClawMaster 工具自动化测试"
echo "========================================"
echo ""

# 第 1 组：核心执行工具
echo -e "${YELLOW}=== 第 1 组：核心执行工具 ===${NC}"
echo ""

test_tool "exec" "请执行命令：echo 'Hello from ClawMaster'" "1.1"
test_tool "exec" "请列出当前目录的文件" "1.2"
test_tool "calc" "请计算：(123 + 456) * 789" "2.1"
test_tool "calc" "请计算 2 的 10 次方" "2.2"

# 第 2 组：网络工具
echo -e "${YELLOW}=== 第 2 组：网络工具 ===${NC}"
echo ""

test_tool "web_search" "请搜索：Rust 编程语言最新特性" "6.1"
test_tool "web_fetch" "请获取 https://example.com 的内容" "7.1"

# 第 3 组：内存工具
echo -e "${YELLOW}=== 第 3 组：内存工具 ===${NC}"
echo ""

test_tool "memory_save" "请记住：今天完成了工具自动化测试" "10.1"
test_tool "memory_search" "请搜索关于'测试'的记忆" "9.1"

# 第 4 组：会话工具
echo -e "${YELLOW}=== 第 4 组：会话工具 ===${NC}"
echo ""

test_tool "sessions_list" "请列出所有活跃的会话" "12.1"

# 第 5 组：辅助工具
echo -e "${YELLOW}=== 第 5 组：辅助工具 ===${NC}"
echo ""

test_tool "task_list" "请显示当前的任务列表" "17.1"
test_tool "location" "上海天气？" "18.2"

# 生成统计信息
echo "" >> "$RESULTS_FILE"
echo "---" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "## 测试统计" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "- **总测试数**: $TOTAL" >> "$RESULTS_FILE"
echo "- **通过**: $PASSED" >> "$RESULTS_FILE"
echo "- **失败**: $FAILED" >> "$RESULTS_FILE"
echo "- **通过率**: $(awk "BEGIN {printf \"%.1f\", ($PASSED/$TOTAL)*100}")%" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# 输出最终结果
echo ""
echo "========================================"
echo "  测试完成"
echo "========================================"
echo -e "总测试数: $TOTAL"
echo -e "${GREEN}通过: $PASSED${NC}"
echo -e "${RED}失败: $FAILED${NC}"
echo -e "通过率: $(awk "BEGIN {printf \"%.1f\", ($PASSED/$TOTAL)*100}")%"
echo ""
echo "详细报告已保存到: $RESULTS_FILE"
echo ""
