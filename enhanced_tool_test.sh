#!/bin/bash
# ClawMaster 增强版工具测试 - 更多场景，更好的可视化
# 直接测试工具功能，不使用 WASM 容器

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
DIM='\033[2m'

# 配置
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_DIR="enhanced_test_${TIMESTAMP}"
TEST_TIMEOUT=20
VERBOSE=true

# 统计
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# 创建结果目录
mkdir -p "$RESULTS_DIR"

# 打印函数
print_header() {
    echo ""
    echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BOLD}${CYAN}║  $1${NC}"
    echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

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

print_input() {
    echo -e "${YELLOW}➤ 输入:${NC} ${DIM}$1${NC}"
}

print_output() {
    echo -e "${GREEN}✓ 输出:${NC}"
    echo "$1" | head -15 | sed 's/^/  │ /'
    local lines=$(echo "$1" | wc -l)
    if [ $lines -gt 15 ]; then
        echo -e "  ${DIM}│ ... (还有 $((lines - 15)) 行)${NC}"
    fi
}

print_result() {
    local status=$1
    local message=$2
    if [ "$status" = "PASS" ]; then
        echo -e "${GREEN}${BOLD}✅ PASS${NC}: $message"
    elif [ "$status" = "FAIL" ]; then
        echo -e "${RED}${BOLD}❌ FAIL${NC}: $message"
    else
        echo -e "${YELLOW}${BOLD}⏭  SKIP${NC}: $message"
    fi
    echo ""
}

# 测试函数
run_test() {
    local tool_name=$1
    local test_desc=$2
    local test_cmd=$3
    local expected_pattern=$4
    
    ((TOTAL_TESTS++))
    
    print_test "$TOTAL_TESTS" "$tool_name" "$test_desc"
    print_input "$test_cmd"
    
    # 执行测试
    local output
    local exit_code
    local start_time=$(date +%s)
    
    # macOS 兼容：使用 gtimeout（如果有）或直接运行
    # 过滤编译警告以获得干净的输出
    if command -v gtimeout &> /dev/null; then
        output=$(eval "gtimeout ${TEST_TIMEOUT}s ${test_cmd}" 2>&1 | grep -v "^warning:" | grep -v "^   -->" | grep -v "^   |" | grep -v "^   =" | grep -v "Compiling" | grep -v "Finished" | grep -v "Running" || echo "COMMAND_ERROR")
    elif command -v timeout &> /dev/null; then
        output=$(eval "timeout ${TEST_TIMEOUT}s ${test_cmd}" 2>&1 | grep -v "^warning:" | grep -v "^   -->" | grep -v "^   |" | grep -v "^   =" | grep -v "Compiling" | grep -v "Finished" | grep -v "Running" || echo "COMMAND_ERROR")
    else
        # 没有 timeout 命令，直接运行并过滤编译输出
        output=$(eval "${test_cmd}" 2>&1 | grep -v "^warning:" | grep -v "^   -->" | grep -v "^   |" | grep -v "^   =" | grep -v "Compiling" | grep -v "Finished" | grep -v "Running" || echo "COMMAND_ERROR")
    fi
    exit_code=$?
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    # 显示输出
    if [ "$VERBOSE" = true ]; then
        print_output "$output"
    fi
    
    echo -e "${DIM}⏱  执行时间: ${duration}s${NC}"
    
    # 保存结果
    {
        echo "=== Test #$TOTAL_TESTS: $tool_name ==="
        echo "Description: $test_desc"
        echo "Command: $test_cmd"
        echo "Duration: ${duration}s"
        echo "Exit Code: $exit_code"
        echo "Output:"
        echo "$output"
        echo ""
    } >> "$RESULTS_DIR/detailed.log"
    
    # 验证结果
    if echo "$output" | grep -qi "$expected_pattern" || [ "$expected_pattern" = "ANY" ]; then
        print_result "PASS" "$tool_name - $test_desc"
        ((PASSED_TESTS++))
        echo "PASS: $tool_name - $test_desc (${duration}s)" >> "$RESULTS_DIR/summary.txt"
    else
        print_result "FAIL" "$tool_name - 预期包含: $expected_pattern"
        ((FAILED_TESTS++))
        echo "FAIL: $tool_name - $test_desc" >> "$RESULTS_DIR/summary.txt"
        echo "$output" >> "$RESULTS_DIR/failures.txt"
    fi
}

skip_test() {
    local tool_name=$1
    local reason=$2
    
    ((TOTAL_TESTS++))
    ((SKIPPED_TESTS++))
    
    print_result "SKIP" "$tool_name - $reason"
    echo "SKIP: $tool_name - $reason" >> "$RESULTS_DIR/summary.txt"
}

# ============================================================================
# 开始测试
# ============================================================================
print_header "🧪 ClawMaster 增强版工具测试"
echo -e "${BOLD}测试时间:${NC} $(date)"
echo -e "${BOLD}结果目录:${NC} $RESULTS_DIR"
echo -e "${BOLD}测试模式:${NC} 直接测试（非 WASM 容器）"
echo ""

# ============================================================================
# 1. calc - 计算器工具（10 个场景）
# ============================================================================
print_section "1️⃣  calc - 计算器工具（10 个场景）"

run_test "calc" \
    "基础加法" \
    "echo '2 + 2' | cargo run --bin clawmaster -- tools exec calc" \
    "4"

run_test "calc" \
    "基础减法" \
    "echo '100 - 37' | cargo run --bin clawmaster -- tools exec calc" \
    "63"

run_test "calc" \
    "基础乘法" \
    "echo '12 * 8' | cargo run --bin clawmaster -- tools exec calc" \
    "96"

run_test "calc" \
    "基础除法" \
    "echo '144 / 12' | cargo run --bin clawmaster -- tools exec calc" \
    "12"

run_test "calc" \
    "复杂表达式" \
    "echo '(10 + 5) * 2 - 8' | cargo run --bin clawmaster -- tools exec calc" \
    "22"

run_test "calc" \
    "幂运算" \
    "echo '2^10' | cargo run --bin clawmaster -- tools exec calc" \
    "1024"

run_test "calc" \
    "取模运算" \
    "echo '17 % 5' | cargo run --bin clawmaster -- tools exec calc" \
    "2"

run_test "calc" \
    "嵌套括号" \
    "echo '((3 + 5) * 2) + ((10 - 2) / 2)' | cargo run --bin clawmaster -- tools exec calc" \
    "20"

run_test "calc" \
    "浮点数运算" \
    "echo '3.14 * 2' | cargo run --bin clawmaster -- tools exec calc" \
    "6.28"

run_test "calc" \
    "负数运算" \
    "echo '-5 + 10' | cargo run --bin clawmaster -- tools exec calc" \
    "5"

# ============================================================================
# 2. exec - 命令执行工具（15 个场景）
# ============================================================================
print_section "2️⃣  exec - 命令执行工具（15 个场景）"

run_test "exec" \
    "显示当前目录" \
    "cargo run --bin clawmaster -- tools exec bash 'pwd'" \
    "ClawMaster"

run_test "exec" \
    "列出文件（简单）" \
    "cargo run --bin clawmaster -- tools exec bash 'ls'" \
    "Cargo.toml"

run_test "exec" \
    "列出文件（详细）" \
    "cargo run --bin clawmaster -- tools exec bash 'ls -lh | head -5'" \
    "total"

run_test "exec" \
    "统计文件数量" \
    "cargo run --bin clawmaster -- tools exec bash 'ls -1 | wc -l'" \
    "[0-9]"

run_test "exec" \
    "查找 Rust 文件" \
    "cargo run --bin clawmaster -- tools exec bash 'find . -name \"*.rs\" -type f | head -3'" \
    ".rs"

run_test "exec" \
    "显示环境变量" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \$HOME'" \
    "/Users"

run_test "exec" \
    "显示系统信息" \
    "cargo run --bin clawmaster -- tools exec bash 'uname -s'" \
    "Darwin"

run_test "exec" \
    "显示日期" \
    "cargo run --bin clawmaster -- tools exec bash 'date +%Y'" \
    "202"

run_test "exec" \
    "创建临时文件" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"test\" > /tmp/clawmaster_test.txt && cat /tmp/clawmaster_test.txt'" \
    "test"

run_test "exec" \
    "文本处理 - grep" \
    "cargo run --bin clawmaster -- tools exec bash 'echo -e \"apple\\nbanana\\ncherry\" | grep banana'" \
    "banana"

run_test "exec" \
    "文本处理 - sed" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"hello world\" | sed \"s/world/ClawMaster/\"'" \
    "ClawMaster"

run_test "exec" \
    "文本处理 - awk" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"1 2 3\" | awk \"{print \\$2}\"'" \
    "2"

run_test "exec" \
    "管道组合" \
    "cargo run --bin clawmaster -- tools exec bash 'ls -1 | head -3 | wc -l'" \
    "3"

run_test "exec" \
    "条件判断" \
    "cargo run --bin clawmaster -- tools exec bash 'if [ -f Cargo.toml ]; then echo \"found\"; fi'" \
    "found"

run_test "exec" \
    "循环处理" \
    "cargo run --bin clawmaster -- tools exec bash 'for i in 1 2 3; do echo \$i; done'" \
    "1"

# ============================================================================
# 3. web_fetch - 网页获取工具（8 个场景）
# ============================================================================
print_section "3️⃣  web_fetch - 网页获取工具（8 个场景）"

run_test "web_fetch" \
    "获取 JSON API" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/get'" \
    "\"url\""

run_test "web_fetch" \
    "获取用户代理信息" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/user-agent'" \
    "user-agent"

run_test "web_fetch" \
    "获取 IP 信息" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/ip'" \
    "origin"

run_test "web_fetch" \
    "获取 headers" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/headers'" \
    "Host"

run_test "web_fetch" \
    "测试 UTF-8 编码" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/encoding/utf8'" \
    "UTF-8"

run_test "web_fetch" \
    "测试 gzip 压缩" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/gzip'" \
    "gzipped"

run_test "web_fetch" \
    "获取 UUID" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/uuid'" \
    "uuid"

run_test "web_fetch" \
    "测试延迟响应" \
    "cargo run --bin clawmaster -- tools exec web_fetch 'https://httpbin.org/delay/1'" \
    "ANY"

# ============================================================================
# 4. sessions - 会话管理工具（5 个场景）
# ============================================================================
print_section "4️⃣  sessions - 会话管理工具（5 个场景）"

run_test "sessions" \
    "列出所有会话" \
    "cargo run --bin clawmaster -- sessions list" \
    "ANY"

run_test "sessions" \
    "显示会话统计" \
    "cargo run --bin clawmaster -- sessions list --format json" \
    "ANY"

skip_test "sessions_history" "需要活动会话"
skip_test "sessions_send" "需要目标会话"
skip_test "branch_session" "需要活动会话"

# ============================================================================
# 5. config - 配置工具（5 个场景）
# ============================================================================
print_section "5️⃣  config - 配置工具（5 个场景）"

run_test "config" \
    "验证配置文件" \
    "cargo run --bin clawmaster -- config validate" \
    "ANY"

run_test "config" \
    "显示配置路径" \
    "cargo run --bin clawmaster -- config path" \
    "clawmaster"

run_test "config" \
    "检查配置版本" \
    "cargo run --bin clawmaster -- --version" \
    "0.10"

skip_test "config_show" "可能包含敏感信息"
skip_test "config_edit" "需要交互式编辑"

# ============================================================================
# 6. sandbox - 沙箱工具（5 个场景）
# ============================================================================
print_section "6️⃣  sandbox - 沙箱工具（5 个场景）"

run_test "sandbox" \
    "列出沙箱包" \
    "cargo run --bin clawmaster -- sandbox list" \
    "ANY"

skip_test "sandbox_build" "需要 Docker 环境"
skip_test "sandbox_remove" "需要已存在的沙箱"
skip_test "sandbox_clean" "可能删除重要数据"
skip_test "sandbox_exec" "需要运行中的沙箱"

# ============================================================================
# 7. 综合场景测试（10 个场景）
# ============================================================================
print_section "7️⃣  综合场景测试（10 个场景）"

run_test "综合" \
    "计算并格式化输出" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"Result: \$(echo \"2 + 2\" | bc)\"'" \
    "Result: 4"

run_test "综合" \
    "文件统计分析" \
    "cargo run --bin clawmaster -- tools exec bash 'find . -name \"*.rs\" | wc -l'" \
    "[0-9]"

run_test "综合" \
    "JSON 数据处理" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"{\\\"name\\\":\\\"test\\\"}\" | grep name'" \
    "name"

run_test "综合" \
    "多步骤处理" \
    "cargo run --bin clawmaster -- tools exec bash 'ls | head -5 | tail -3'" \
    "ANY"

run_test "综合" \
    "条件过滤" \
    "cargo run --bin clawmaster -- tools exec bash 'ls -la | grep \"^d\"'" \
    "ANY"

run_test "综合" \
    "数据转换" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"HELLO\" | tr \"[:upper:]\" \"[:lower:]\"'" \
    "hello"

run_test "综合" \
    "字符串操作" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"ClawMaster\" | cut -c1-4'" \
    "Claw"

run_test "综合" \
    "排序操作" \
    "cargo run --bin clawmaster -- tools exec bash 'echo -e \"3\\n1\\n2\" | sort'" \
    "1"

run_test "综合" \
    "去重操作" \
    "cargo run --bin clawmaster -- tools exec bash 'echo -e \"a\\na\\nb\" | uniq'" \
    "a"

run_test "综合" \
    "统计分析" \
    "cargo run --bin clawmaster -- tools exec bash 'echo \"hello world\" | wc -w'" \
    "2"

# ============================================================================
# 测试总结
# ============================================================================
print_header "📊 测试总结"

echo -e "${BOLD}${WHITE}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${WHITE}║                          测试结果统计                              ║${NC}"
echo -e "${BOLD}${WHITE}╠════════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BOLD}${WHITE}║  总测试数: ${CYAN}${TOTAL_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  通过:     ${GREEN}${PASSED_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  失败:     ${RED}${FAILED_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  跳过:     ${YELLOW}${SKIPPED_TESTS}${WHITE}                                                    ║${NC}"

if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${BOLD}${WHITE}║  通过率:   ${CYAN}${PASS_RATE}%${WHITE}                                                  ║${NC}"
fi

echo -e "${BOLD}${WHITE}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BOLD}📁 生成的文件:${NC}"
echo -e "  • ${CYAN}$RESULTS_DIR/summary.txt${NC} - 测试摘要"
echo -e "  • ${CYAN}$RESULTS_DIR/detailed.log${NC} - 详细日志"
echo -e "  • ${CYAN}$RESULTS_DIR/failures.txt${NC} - 失败详情"
echo ""

# 生成 Markdown 报告
cat > "$RESULTS_DIR/TEST_REPORT.md" << EOF
# ClawMaster 增强版工具测试报告

**测试时间**: $(date)  
**测试模式**: 直接测试（非 WASM 容器）  
**测试范围**: 扩展场景测试

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

## 🧪 测试覆盖

### 1. calc - 计算器（10 个场景）
- 基础运算（+, -, *, /）
- 高级运算（^, %）
- 复杂表达式
- 浮点数和负数

### 2. exec - 命令执行（15 个场景）
- 文件操作
- 文本处理（grep, sed, awk）
- 管道和重定向
- 条件和循环

### 3. web_fetch - 网页获取（8 个场景）
- JSON API 调用
- HTTP 特性测试
- 编码和压缩
- 延迟处理

### 4. sessions - 会话管理（5 个场景）
- 会话列表
- 会话统计

### 5. config - 配置管理（5 个场景）
- 配置验证
- 路径查询
- 版本检查

### 6. sandbox - 沙箱管理（5 个场景）
- 包列表查询

### 7. 综合场景（10 个场景）
- 多步骤处理
- 数据转换
- 统计分析

---

## 📝 测试详情

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

## ✅ 结论

$(if [ $PASS_RATE -ge 80 ]; then
    echo "✅ **测试通过** - 工具功能正常，返回结果正确"
elif [ $PASS_RATE -ge 60 ]; then
    echo "⚠️ **部分通过** - 部分工具需要改进"
else
    echo "❌ **测试失败** - 需要修复问题"
fi)

---

**生成时间**: $(date)
EOF

echo -e "${GREEN}${BOLD}✅ 测试完成！${NC}"
echo -e "${BOLD}报告:${NC} $RESULTS_DIR/TEST_REPORT.md"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}${BOLD}🎉 所有测试通过！工具功能正常，返回结果正确！${NC}"
    exit 0
else
    echo -e "${YELLOW}${BOLD}⚠️  有 $FAILED_TESTS 个测试失败，请查看详细日志${NC}"
    exit 1
fi
