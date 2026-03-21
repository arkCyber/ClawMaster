#!/bin/bash
# ClawMaster WASM 工具综合测试
# 扩大测试范围，显示命令和结果

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'
BOLD='\033[1m'

# 配置
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="wasm_comprehensive_test_${TIMESTAMP}"
TEST_TIMEOUT=20

# 统计
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

mkdir -p "$RESULTS_DIR"

echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${CYAN}║  🧪 ClawMaster WASM 工具综合测试${NC}"
echo -e "${BOLD}${CYAN}║  实时显示命令和结果${NC}"
echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BOLD}测试时间:${NC} $(date)"
echo -e "${BOLD}结果目录:${NC} $RESULTS_DIR"
echo -e "${BOLD}测试模式:${NC} WASM 工具直接测试"
echo ""

# 打印函数
print_section() {
    echo ""
    echo -e "${BOLD}${MAGENTA}┌────────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${BOLD}${MAGENTA}│  $1${NC}"
    echo -e "${BOLD}${MAGENTA}└────────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
}

print_test() {
    local num=$1
    local tool=$2
    local desc=$3
    echo -e "${BOLD}${BLUE}━━━ 测试 #${num}: ${tool} ━━━${NC}"
    echo -e "${CYAN}📋 场景: ${desc}${NC}"
}

print_command() {
    local cmd=$1
    echo -e "${YELLOW}➤ 命令:${NC} ${cmd}"
}

print_result() {
    local status=$1
    local message=$2
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}${BOLD}✅ PASS${NC}: $message"
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}${BOLD}❌ FAIL${NC}: $message"
    fi
    echo ""
}

# 测试函数 - 直接调用 Rust 测试
test_wasm_tool() {
    local tool_name=$1
    local test_desc=$2
    local test_function=$3
    local expected_pattern=$4
    
    ((TOTAL_TESTS++))
    
    print_test "$TOTAL_TESTS" "$tool_name" "$test_desc"
    
    # 构建测试命令
    local test_cmd="cargo test --package clawmaster-tools --lib ${test_function} -- --nocapture --test-threads=1"
    
    print_command "$test_cmd"
    
    local output
    local exit_code
    local start_time=$(date +%s)
    
    # 执行测试并显示输出
    echo -e "${GREEN}✓ 输出:${NC}"
    if command -v gtimeout &> /dev/null; then
        output=$(gtimeout ${TEST_TIMEOUT}s bash -c "$test_cmd" 2>&1 || echo "COMMAND_ERROR")
    else
        output=$(bash -c "$test_cmd" 2>&1 || echo "COMMAND_ERROR")
    fi
    exit_code=$?
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    # 显示输出（过滤编译信息）
    echo "$output" | grep -v "^   Compiling" | grep -v "^    Finished" | grep -v "^     Running" | head -20 | sed 's/^/  │ /'
    if [ $(echo "$output" | wc -l) -gt 20 ]; then
        echo -e "  │ ${DIM}... (还有 $(($(echo "$output" | wc -l) - 20)) 行)${NC}"
    fi
    echo -e "${DIM}⏱  执行时间: ${duration}s${NC}"
    
    # 保存结果
    {
        echo "=== WASM Test #$TOTAL_TESTS: $tool_name ==="
        echo "Description: $test_desc"
        echo "Function: $test_function"
        echo "Duration: ${duration}s"
        echo "Output:"
        echo "$output"
        echo ""
    } >> "$RESULTS_DIR/detailed.log"
    
    # 验证结果
    if echo "$output" | grep -qi "test result: ok" || echo "$output" | grep -qi "$expected_pattern"; then
        print_result "PASS" "$tool_name - $test_desc (${duration}s)"
        ((PASSED_TESTS++))
        echo "PASS: $tool_name - $test_desc (${duration}s)" >> "$RESULTS_DIR/summary.txt"
    else
        print_result "FAIL" "$tool_name - 测试失败"
        ((FAILED_TESTS++))
        echo "FAIL: $tool_name - $test_desc" >> "$RESULTS_DIR/summary.txt"
        echo "$output" >> "$RESULTS_DIR/failures.txt"
    fi
}

# ============================================================================
# WASM 工具测试
# ============================================================================

print_section "1️⃣  WASM 引擎测试"

test_wasm_tool "wasm_engine" \
    "WASM 组件编译和缓存" \
    "wasm_engine::tests::compile_component_round_trip_and_cache_hit" \
    "test result: ok"

test_wasm_tool "wasm_engine" \
    "WASM 并发访问" \
    "wasm_engine::tests::compile_component_concurrent_access" \
    "test result: ok"

test_wasm_tool "wasm_engine" \
    "WASM 核心模块编译" \
    "wasm_engine::tests::compile_module_core_wasm" \
    "test result: ok"

test_wasm_tool "wasm_limits" \
    "WASM 燃料耗尽检测" \
    "wasm_limits::wasm_tests::fuel_exhaustion_returns_error" \
    "test result: ok"

test_wasm_tool "wasm_component" \
    "WASM HTTP 超时处理" \
    "wasm_component::tests::http_host_maps_timeout_errors" \
    "test result: ok"

# ============================================================================
# 2. calc 工具测试（通过 WASM）
# ============================================================================

print_section "2️⃣  calc 计算器工具测试"

test_wasm_tool "calc" \
    "基础加法运算" \
    "calc::tests" \
    "test result: ok"

# ============================================================================
# 3. web_fetch 工具测试
# ============================================================================

print_section "3️⃣  web_fetch 网页获取工具测试"

test_wasm_tool "web_fetch" \
    "SSRF 防护 - 阻止 localhost" \
    "web_fetch::tests::test_ssrf_blocks_localhost_url" \
    "test result: ok"

test_wasm_tool "web_fetch" \
    "SSRF 防护 - 阻止私有 IP" \
    "web_fetch::tests::test_ssrf_blocks_private_ip" \
    "test result: ok"

test_wasm_tool "web_fetch" \
    "SSRF 防护 - 阻止链路本地" \
    "web_fetch::tests::test_ssrf_blocks_link_local" \
    "test result: ok"

test_wasm_tool "web_fetch" \
    "白名单功能" \
    "web_fetch::tests::test_ssrf_check_allowlist_permits_private_ip" \
    "test result: ok"

test_wasm_tool "web_fetch" \
    "内容截断" \
    "web_fetch::tests::test_truncation" \
    "test result: ok"

test_wasm_tool "web_fetch" \
    "UTF-8 边界处理" \
    "web_fetch::tests::test_truncation_utf8_boundary" \
    "test result: ok"

# ============================================================================
# 4. web_search 工具测试
# ============================================================================

print_section "4️⃣  web_search 网页搜索工具测试"

test_wasm_tool "web_search" \
    "Brave 搜索响应解析" \
    "web_search::tests::test_brave_response_parsing" \
    "test result: ok"

test_wasm_tool "web_search" \
    "DuckDuckGo HTML 解析" \
    "web_search::tests::test_parse_duckduckgo_html_basic" \
    "test result: ok"

test_wasm_tool "web_search" \
    "Perplexity 响应解析" \
    "web_search::tests::test_perplexity_response_parsing" \
    "test result: ok"

test_wasm_tool "web_search" \
    "缓存命中和未命中" \
    "web_search::tests::test_cache_hit_and_miss" \
    "test result: ok"

# ============================================================================
# 5. 地图工具测试
# ============================================================================

print_section "5️⃣  map 地图工具测试"

test_wasm_tool "map" \
    "地图缩放限制" \
    "map::tests::execute_clamps_zoom" \
    "test result: ok"

test_wasm_tool "map" \
    "地图标签功能" \
    "map::tests::execute_includes_label_in_result" \
    "test result: ok"

test_wasm_tool "map" \
    "地图点位输入" \
    "map::tests::execute_supports_points_input" \
    "test result: ok"

# ============================================================================
# 6. 位置工具测试
# ============================================================================

print_section "6️⃣  location 位置工具测试"

test_wasm_tool "location" \
    "精确位置模式" \
    "location::tests::precision_defaults_to_precise" \
    "test result: ok"

test_wasm_tool "location" \
    "粗略位置模式" \
    "location::tests::precision_coarse_is_forwarded" \
    "test result: ok"

test_wasm_tool "location" \
    "浏览器位置获取" \
    "location::tests::browser_location_success" \
    "test result: ok"

test_wasm_tool "location" \
    "通道位置获取" \
    "location::tests::channel_location_success" \
    "test result: ok"

# ============================================================================
# 7. 进程和合约工具测试
# ============================================================================

print_section "7️⃣  process 和 contract 工具测试"

test_wasm_tool "process" \
    "进程列表（无沙箱）" \
    "process::tests::test_process_tool_list_no_sandbox" \
    "test result: ok"

test_wasm_tool "contract" \
    "合约超时强制执行" \
    "contract::tests::contract_timeout_is_enforced" \
    "test result: ok"

# ============================================================================
# 8. Agent 生成工具测试
# ============================================================================

print_section "8️⃣  spawn_agent Agent 生成工具测试"

test_wasm_tool "spawn_agent" \
    "超时取消长时间运行的 Agent" \
    "spawn_agent::tests::test_timeout_cancels_long_running_agent" \
    "test result: ok"

# ============================================================================
# 测试总结
# ============================================================================

print_section "📊 WASM 工具测试总结"

echo -e "${BOLD}${WHITE}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${WHITE}║                       测试结果统计                                 ║${NC}"
echo -e "${BOLD}${WHITE}╠════════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BOLD}${WHITE}║  总测试数: ${CYAN}${TOTAL_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  通过:     ${GREEN}${PASSED_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  失败:     ${RED}${FAILED_TESTS}${WHITE}                                                    ║${NC}"

if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${BOLD}${WHITE}║  通过率:   ${CYAN}${PASS_RATE}%${WHITE}                                                  ║${NC}"
fi

echo -e "${BOLD}${WHITE}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 生成报告
cat > "$RESULTS_DIR/WASM_TEST_REPORT.md" << EOF
# ClawMaster WASM 工具综合测试报告

**测试时间**: $(date)  
**测试模式**: WASM 工具直接测试

---

## 📊 测试统计

| 指标 | 数值 |
|------|------|
| **总测试数** | $TOTAL_TESTS |
| **通过** | $PASSED_TESTS |
| **失败** | $FAILED_TESTS |
| **通过率** | ${PASS_RATE}% |

---

## 🧪 测试覆盖

### 1. WASM 引擎测试（5 个）
- WASM 组件编译和缓存
- WASM 并发访问
- WASM 核心模块编译
- WASM 燃料耗尽检测
- WASM HTTP 超时处理

### 2. calc 计算器工具
- 基础运算测试

### 3. web_fetch 网页获取（6 个）
- SSRF 防护测试（3 个）
- 白名单功能
- 内容截断
- UTF-8 边界处理

### 4. web_search 网页搜索（4 个）
- Brave 搜索解析
- DuckDuckGo HTML 解析
- Perplexity 响应解析
- 缓存功能

### 5. map 地图工具（3 个）
- 缩放限制
- 标签功能
- 点位输入

### 6. location 位置工具（4 个）
- 精确位置模式
- 粗略位置模式
- 浏览器位置获取
- 通道位置获取

### 7. process 和 contract 工具（2 个）
- 进程列表
- 合约超时

### 8. spawn_agent Agent 生成（1 个）
- 超时取消

---

## 📝 测试详情

\`\`\`
$(cat "$RESULTS_DIR/summary.txt" 2>/dev/null || echo "无测试结果")
\`\`\`

---

## 🎯 结论

$(if [ $FAILED_TESTS -eq 0 ]; then
    echo "✅ **所有 WASM 工具测试通过！**"
    echo ""
    echo "WASM 工具系统已准备好用于生产环境。"
else
    echo "⚠️ **有 $FAILED_TESTS 个测试失败**"
    echo ""
    echo "请查看详细日志了解失败原因。"
fi)

---

**生成时间**: $(date)
EOF

echo -e "${BOLD}📁 生成的文件:${NC}"
echo -e "  • ${CYAN}$RESULTS_DIR/summary.txt${NC} - 测试摘要"
echo -e "  • ${CYAN}$RESULTS_DIR/detailed.log${NC} - 详细日志"
echo -e "  • ${CYAN}$RESULTS_DIR/WASM_TEST_REPORT.md${NC} - Markdown 报告"
if [ -f "$RESULTS_DIR/failures.txt" ]; then
    echo -e "  • ${CYAN}$RESULTS_DIR/failures.txt${NC} - 失败详情"
fi
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}${BOLD}🎉 所有 WASM 工具测试通过！${NC}"
    exit 0
else
    echo -e "${YELLOW}${BOLD}⚠️  有 $FAILED_TESTS 个测试失败${NC}"
    exit 1
fi
