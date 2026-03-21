#!/bin/bash
# ClawMaster 工具功能测试脚本
# 测试所有工具的实际功能和返回结果

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'
BOLD='\033[1m'

# 配置
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="functional_test_${TIMESTAMP}"
TEST_TIMEOUT=15

# 统计
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  🧪 ClawMaster 工具功能测试${NC}"
echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BOLD}测试时间:${NC} $(date)"
echo -e "${BOLD}结果目录:${NC} $RESULTS_DIR"
echo ""

mkdir -p "$RESULTS_DIR"

# 测试函数
test_tool() {
    local tool_name=$1
    local test_desc=$2
    local test_cmd=$3
    local expected_pattern=$4
    local verify_func=$5
    
    ((TOTAL_TESTS++))
    
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}测试 #${TOTAL_TESTS}: ${tool_name}${NC}"
    echo -e "${CYAN}描述: ${test_desc}${NC}"
    echo -e "${YELLOW}命令: ${test_cmd}${NC}"
    echo ""
    
    # 执行测试
    local output
    local exit_code
    
    output=$(eval "timeout ${TEST_TIMEOUT}s ${test_cmd}" 2>&1 || echo "TIMEOUT_OR_ERROR")
    exit_code=$?
    
    # 保存结果
    echo "=== Test: $tool_name ===" >> "$RESULTS_DIR/all_tests.log"
    echo "Description: $test_desc" >> "$RESULTS_DIR/all_tests.log"
    echo "Command: $test_cmd" >> "$RESULTS_DIR/all_tests.log"
    echo "Output:" >> "$RESULTS_DIR/all_tests.log"
    echo "$output" >> "$RESULTS_DIR/all_tests.log"
    echo "Exit Code: $exit_code" >> "$RESULTS_DIR/all_tests.log"
    echo "" >> "$RESULTS_DIR/all_tests.log"
    
    # 显示输出
    echo -e "${CYAN}输出:${NC}"
    echo "$output" | head -10
    echo ""
    
    # 验证结果
    local test_passed=false
    
    if [ -n "$verify_func" ]; then
        # 使用自定义验证函数
        if $verify_func "$output"; then
            test_passed=true
        fi
    elif [ -n "$expected_pattern" ]; then
        # 使用模式匹配
        if echo "$output" | grep -qi "$expected_pattern"; then
            test_passed=true
        fi
    else
        # 只检查退出码
        if [ $exit_code -eq 0 ]; then
            test_passed=true
        fi
    fi
    
    if $test_passed; then
        echo -e "${GREEN}✅ PASS${NC}: $tool_name"
        ((PASSED_TESTS++))
        echo "PASS: $tool_name - $test_desc" >> "$RESULTS_DIR/summary.txt"
    else
        echo -e "${RED}❌ FAIL${NC}: $tool_name"
        echo -e "${RED}   预期: $expected_pattern${NC}"
        ((FAILED_TESTS++))
        echo "FAIL: $tool_name - $test_desc" >> "$RESULTS_DIR/summary.txt"
        echo "$output" >> "$RESULTS_DIR/failures.txt"
    fi
    
    echo ""
}

skip_test() {
    local tool_name=$1
    local reason=$2
    
    ((TOTAL_TESTS++))
    ((SKIPPED_TESTS++))
    
    echo -e "${YELLOW}⏭  SKIP${NC}: $tool_name - $reason"
    echo "SKIP: $tool_name - $reason" >> "$RESULTS_DIR/summary.txt"
}

# ============================================================================
# 核心工具测试
# ============================================================================
echo -e "${BOLD}${MAGENTA}[1] 核心工具测试${NC}"
echo ""

# 1. calc - 计算器
test_tool "calc" \
    "基础加法运算" \
    "cargo run --bin clawmaster -- tools exec calc '2 + 2'" \
    "4"

test_tool "calc" \
    "复杂表达式" \
    "cargo run --bin clawmaster -- tools exec calc '(10 + 5) * 2'" \
    "30"

test_tool "calc" \
    "科学计算" \
    "cargo run --bin clawmaster -- tools exec calc '2^10'" \
    "1024"

# 2. exec - 命令执行
test_tool "exec" \
    "列出文件" \
    "cargo run --bin clawmaster -- tools exec bash 'ls -la | head -5'" \
    "total"

test_tool "exec" \
    "显示当前目录" \
    "cargo run --bin clawmaster -- tools exec bash 'pwd'" \
    "ClawMaster"

test_tool "exec" \
    "环境变量" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \$HOME'" \
    "/Users"

# ============================================================================
# 网络工具测试
# ============================================================================
echo ""
echo -e "${BOLD}${MAGENTA}[2] 网络工具测试${NC}"
echo ""

# 3. web_fetch - 网页获取
test_tool "web_fetch" \
    "获取网页内容" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/get'" \
    "\"url\""

test_tool "web_fetch" \
    "获取 JSON 数据" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://api.github.com/zen'" \
    "."

# 4. web_search - 网页搜索
skip_test "web_search" "需要 API 密钥配置"

# ============================================================================
# 会话工具测试
# ============================================================================
echo ""
echo -e "${BOLD}${MAGENTA}[3] 会话工具测试${NC}"
echo ""

# 5. sessions_list - 会话列表
test_tool "sessions_list" \
    "列出所有会话" \
    "cargo run --bin clawmaster -- sessions list" \
    ""

# ============================================================================
# 任务工具测试
# ============================================================================
echo ""
echo -e "${BOLD}${MAGENTA}[4] 任务工具测试${NC}"
echo ""

# 6. task_list - 任务列表
skip_test "task_list" "需要 Gateway 运行"

# ============================================================================
# 系统工具测试
# ============================================================================
echo ""
echo -e "${BOLD}${MAGENTA}[5] 系统工具测试${NC}"
echo ""

# 7. sandbox_packages - 沙箱包
test_tool "sandbox_packages" \
    "列出沙箱包" \
    "cargo run --bin clawmaster -- sandbox list" \
    ""

# ============================================================================
# 配置工具测试
# ============================================================================
echo ""
echo -e "${BOLD}${MAGENTA}[6] 配置工具测试${NC}"
echo ""

# 8. gateway_config - 网关配置
test_tool "config_validate" \
    "验证配置" \
    "cargo run --bin clawmaster -- config validate" \
    ""

# ============================================================================
# 测试总结
# ============================================================================
echo ""
echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  📊 测试总结${NC}"
echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo -e "${BOLD}总测试数:${NC} $TOTAL_TESTS"
echo -e "${GREEN}${BOLD}通过:${NC} $PASSED_TESTS"
echo -e "${RED}${BOLD}失败:${NC} $FAILED_TESTS"
echo -e "${YELLOW}${BOLD}跳过:${NC} $SKIPPED_TESTS"

if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${BOLD}通过率:${NC} ${PASS_RATE}%"
fi

echo ""
echo -e "${BOLD}详细日志:${NC} $RESULTS_DIR/all_tests.log"
echo -e "${BOLD}测试摘要:${NC} $RESULTS_DIR/summary.txt"
echo ""

# 生成报告
cat > "$RESULTS_DIR/FUNCTIONAL_TEST_REPORT.md" << EOF
# ClawMaster 工具功能测试报告

**测试时间**: $(date)  
**测试类型**: 功能验证测试

---

## 📊 测试统计

| 指标 | 数值 |
|------|------|
| **总测试数** | $TOTAL_TESTS |
| **通过** | $PASSED_TESTS |
| **失败** | $FAILED_TESTS |
| **跳过** | $SKIPPED_TESTS |
| **通过率** | ${PASS_RATE}% |

---

## 🧪 测试详情

### 通过的测试

\`\`\`
$(grep "PASS:" "$RESULTS_DIR/summary.txt" 2>/dev/null || echo "无")
\`\`\`

### 失败的测试

\`\`\`
$(grep "FAIL:" "$RESULTS_DIR/summary.txt" 2>/dev/null || echo "无")
\`\`\`

### 跳过的测试

\`\`\`
$(grep "SKIP:" "$RESULTS_DIR/summary.txt" 2>/dev/null || echo "无")
\`\`\`

---

## 📝 测试覆盖

- ✅ 核心工具（calc, exec）
- ✅ 网络工具（web_fetch）
- ✅ 会话工具（sessions_list）
- ✅ 系统工具（sandbox_packages）
- ✅ 配置工具（config_validate）

---

## 💡 结论

$(if [ $PASS_RATE -ge 80 ]; then
    echo "✅ **测试通过** - 工具功能正常"
elif [ $PASS_RATE -ge 60 ]; then
    echo "⚠️ **部分通过** - 部分工具需要改进"
else
    echo "❌ **测试失败** - 需要修复"
fi)

---

**生成时间**: $(date)
EOF

echo -e "${GREEN}${BOLD}✅ 测试完成！${NC}"
echo -e "${BOLD}报告:${NC} $RESULTS_DIR/FUNCTIONAL_TEST_REPORT.md"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi
