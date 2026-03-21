#!/bin/bash
# WASM 工具测试 - 完整输出版本
# 显示每个测试的完整输入和输出

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
WHITE='\033[1;37m'
NC='\033[0m'
BOLD='\033[1m'

echo -e "${BOLD}${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${CYAN}  WASM 工具完整输出测试${NC}"
echo -e "${BOLD}${CYAN}  显示每个测试的输入命令和完整返回结果${NC}"
echo -e "${BOLD}${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""

TOTAL=0
PASSED=0

run_test() {
    local num=$1
    local name=$2
    local desc=$3
    local test_name=$4
    
    ((TOTAL++))
    
    echo ""
    echo -e "${BOLD}${MAGENTA}┌─────────────────────────────────────────────────────────┐${NC}"
    echo -e "${BOLD}${MAGENTA}│ 测试 #${num}: ${name} - ${desc}${NC}"
    echo -e "${BOLD}${MAGENTA}└─────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    local cmd="cargo test --package clawmaster-tools --lib ${test_name} -- --nocapture --test-threads=1 2>&1"
    
    echo -e "${YELLOW}➤ 执行命令:${NC}"
    echo -e "  ${CYAN}${cmd}${NC}"
    echo ""
    
    echo -e "${GREEN}✓ 完整输出:${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    local output
    output=$(eval "$cmd" | grep -v "Compiling\|Finished\|Running\|Blocking waiting")
    
    echo "$output"
    
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    
    if echo "$output" | grep -q "test result: ok"; then
        echo -e "${GREEN}${BOLD}✅ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}${BOLD}❌ FAIL${NC}"
    fi
}

# 1. WASM 引擎测试
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${WHITE}  1️⃣  WASM 引擎核心功能测试${NC}"
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"

run_test 1 "wasm_engine" "组件编译缓存" \
    "wasm_engine::tests::compile_component_round_trip_and_cache_hit"

run_test 2 "wasm_limits" "燃料耗尽检测" \
    "wasm_limits::wasm_tests::fuel_exhaustion_returns_error"

# 2. calc 计算器测试
echo ""
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${WHITE}  2️⃣  calc 计算器工具测试${NC}"
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"

run_test 3 "calc" "运算符优先级" \
    "calc::tests::evaluates_operator_precedence"

run_test 4 "calc" "除零拒绝" \
    "calc::tests::rejects_division_by_zero"

run_test 5 "calc" "浮点数支持" \
    "calc::tests::supports_floating_point_results"

# 3. web_fetch 安全测试
echo ""
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${WHITE}  3️⃣  web_fetch SSRF 安全防护测试${NC}"
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"

run_test 6 "web_fetch" "阻止 localhost" \
    "web_fetch::tests::test_ssrf_blocks_localhost_url"

run_test 7 "web_fetch" "阻止私有 IP" \
    "web_fetch::tests::test_ssrf_blocks_private_ip"

run_test 8 "web_fetch" "UTF-8 边界处理" \
    "web_fetch::tests::test_truncation_utf8_boundary"

# 4. location 位置工具
echo ""
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${WHITE}  4️⃣  location 位置工具测试${NC}"
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"

run_test 9 "location" "精确位置模式" \
    "location::tests::precision_defaults_to_precise"

run_test 10 "location" "浏览器位置获取" \
    "location::tests::browser_location_success"

# 总结
echo ""
echo ""
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}${WHITE}  📊 测试总结${NC}"
echo -e "${BOLD}${WHITE}═══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BOLD}总测试数:${NC} ${TOTAL}"
echo -e "${BOLD}${GREEN}通过:${NC} ${PASSED}"
echo -e "${BOLD}${RED}失败:${NC} $((TOTAL - PASSED))"

if [ $PASSED -eq $TOTAL ]; then
    echo ""
    echo -e "${GREEN}${BOLD}🎉 所有测试通过！${NC}"
else
    echo ""
    echo -e "${YELLOW}${BOLD}⚠️  有测试失败${NC}"
fi
