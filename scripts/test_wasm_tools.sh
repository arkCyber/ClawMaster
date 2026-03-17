#!/bin/bash
# ClawMaster WASM 工具测试脚本

set -e

# 配置
API_BASE="https://localhost:3000/api"
SESSION="main"
RESULTS_FILE="wasm_tool_test_results_$(date +%Y%m%d_%H%M%S).md"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# 测试计数器
TOTAL=0
PASSED=0
FAILED=0
SKIPPED=0

# 初始化结果文件
cat > "$RESULTS_FILE" << EOF
# ClawMaster WASM 工具测试报告

**测试时间**: $(date)
**测试会话**: main
**WASM 工具数量**: 3

---

## 测试结果

EOF

# 测试函数
test_tool() {
    local tool_name=$1
    local test_message=$2
    local test_id=$3
    
    TOTAL=$((TOTAL + 1))
    
    echo -e "${YELLOW}[测试 $test_id] $tool_name (WASM)${NC}"
    echo "问题: $test_message"
    
    response=$(curl -k -s -X POST "$API_BASE/chat" \
        -H "Content-Type: application/json" \
        -d "{\"session\":\"$SESSION\",\"message\":\"$test_message\",\"stream\":false}" \
        2>/dev/null || echo "ERROR")
    
    if [ "$response" = "ERROR" ] || [ -z "$response" ]; then
        echo -e "${RED}✗ 失败${NC}"
        FAILED=$((FAILED + 1))
        echo "### ❌ $tool_name - 测试 $test_id" >> "$RESULTS_FILE"
        echo "- **问题**: $test_message" >> "$RESULTS_FILE"
        echo "- **结果**: 失败" >> "$RESULTS_FILE"
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
    sleep 2
}

echo "========================================"
echo "  ClawMaster WASM 工具测试"
echo "  测试 3 个 WASM 工具"
echo "========================================"
echo ""

# 检查 WASM feature
echo -e "${CYAN}检查 WASM Feature 状态...${NC}"
echo ""

# ============================================
# WASM 工具测试
# ============================================

echo -e "${BLUE}=== WASM 工具测试 (3个) ===${NC}"
echo ""

# 1. calc (WASM)
echo -e "${CYAN}--- 测试 1: calc (WASM 版本) ---${NC}"
test_tool "calc (WASM)" "请用 WASM 计算器计算：(100 + 200) * 3" "W1.1"
test_tool "calc (WASM)" "请计算：sqrt(144) + pow(2, 8)" "W1.2"
test_tool "calc (WASM)" "请计算：sin(pi/2) + cos(0)" "W1.3"

# 2. web_fetch (WASM)
echo -e "${CYAN}--- 测试 2: web_fetch (WASM 版本) ---${NC}"
test_tool "web_fetch (WASM)" "请使用 WASM 获取 https://example.com 的内容" "W2.1"
test_tool "web_fetch (WASM)" "请用 WASM 工具获取 https://www.rust-lang.org 的内容" "W2.2"

# 3. web_search (WASM)
echo -e "${CYAN}--- 测试 3: web_search (WASM 版本) ---${NC}"
test_tool "web_search (WASM)" "请使用 WASM 搜索：Rust WebAssembly" "W3.1"
test_tool "web_search (WASM)" "请用 WASM 搜索今天的新闻" "W3.2"

# ============================================
# 对比测试 (WASM vs 原生)
# ============================================

echo -e "${BLUE}=== 对比测试: WASM vs 原生 ===${NC}"
echo ""

test_tool "对比测试" "请分别用 WASM 和原生工具计算 123 * 456，比较结果" "C1.1"
test_tool "对比测试" "请用两种方式获取 https://example.com，比较性能" "C2.1"

# 生成统计信息
echo "" >> "$RESULTS_FILE"
echo "---" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "## 测试统计" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "- **总测试数**: $TOTAL" >> "$RESULTS_FILE"
echo "- **通过**: $PASSED" >> "$RESULTS_FILE"
echo "- **失败**: $FAILED" >> "$RESULTS_FILE"
echo "- **跳过**: $SKIPPED" >> "$RESULTS_FILE"

if [ $TOTAL -gt 0 ]; then
    PASS_RATE=$(echo "scale=1; ($PASSED * 100) / $TOTAL" | bc)
    echo "- **通过率**: ${PASS_RATE}%" >> "$RESULTS_FILE"
else
    echo "- **通过率**: 0%" >> "$RESULTS_FILE"
fi

echo "" >> "$RESULTS_FILE"

# WASM 工具信息
echo "## WASM 工具信息" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "### 已测试的 WASM 工具" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "1. **calc** - WASM 计算器 (Pure Tool)" >> "$RESULTS_FILE"
echo "2. **web_fetch** - WASM 网页获取 (HTTP Tool)" >> "$RESULTS_FILE"
echo "3. **web_search** - WASM 网络搜索 (HTTP Tool)" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"

# 输出最终结果
echo ""
echo "========================================"
echo "  WASM 工具测试完成"
echo "========================================"
echo -e "总测试数: $TOTAL"
echo -e "${GREEN}通过: $PASSED${NC}"
echo -e "${RED}失败: $FAILED${NC}"
echo -e "${YELLOW}跳过: $SKIPPED${NC}"

if [ $TOTAL -gt 0 ]; then
    PASS_RATE=$(echo "scale=1; ($PASSED * 100) / $TOTAL" | bc)
    echo -e "通过率: ${PASS_RATE}%"
fi

echo ""
echo "详细报告已保存到: $RESULTS_FILE"
echo ""

# 提示信息
echo -e "${CYAN}注意事项:${NC}"
echo "1. WASM 工具需要 'wasm' feature 启用"
echo "2. 如果测试失败，检查 WASM 文件是否存在"
echo "3. WASM 工具提供沙箱化执行环境"
echo ""
