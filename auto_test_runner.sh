#!/bin/bash
# 自动化测试运行器 - 自动测试、收集结果、分析问题

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
RESULTS_DIR="test_results_${TIMESTAMP}"
GATEWAY_PORT=3000
TEST_TIMEOUT=10

echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  🤖 ClawMaster 自动化测试系统${NC}"
echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BOLD}测试时间:${NC} $(date)"
echo -e "${BOLD}结果目录:${NC} $RESULTS_DIR"
echo ""

# 创建结果目录
mkdir -p "$RESULTS_DIR"

# ============================================================================
# 步骤 1: 检查并启动 Gateway
# ============================================================================
echo -e "${BOLD}${BLUE}[1/7] 检查 Gateway 服务...${NC}"

if pgrep -f "clawmaster.*gateway" > /dev/null; then
    echo -e "${GREEN}✓ Gateway 已运行${NC}"
    GATEWAY_PID=$(pgrep -f "clawmaster.*gateway")
    echo "  PID: $GATEWAY_PID"
else
    echo -e "${YELLOW}⚠ Gateway 未运行，正在启动...${NC}"
    
    # 启动 gateway（后台运行）
    cargo run --bin clawmaster -- gateway > "$RESULTS_DIR/gateway.log" 2>&1 &
    GATEWAY_PID=$!
    echo "  PID: $GATEWAY_PID"
    
    # 等待 gateway 启动
    echo -e "${YELLOW}  等待 Gateway 启动...${NC}"
    sleep 5
    
    # 检查是否成功启动
    if curl -s http://localhost:${GATEWAY_PORT}/health > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Gateway 启动成功${NC}"
    else
        echo -e "${RED}✗ Gateway 启动失败${NC}"
        echo "  查看日志: $RESULTS_DIR/gateway.log"
        exit 1
    fi
fi

echo ""

# ============================================================================
# 步骤 2: 运行简化的工具测试
# ============================================================================
echo -e "${BOLD}${BLUE}[2/7] 运行工具测试...${NC}"

# 测试计数
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 测试函数
test_tool() {
    local tool_name=$1
    local test_input=$2
    local expected_pattern=$3
    
    ((TOTAL_TESTS++))
    
    echo -n "  测试 ${tool_name}... "
    
    # 使用 curl 发送测试请求到 gateway
    local response
    response=$(curl -s -X POST http://localhost:${GATEWAY_PORT}/api/chat \
        -H "Content-Type: application/json" \
        -d "{\"message\": \"$test_input\", \"agent\": \"default\"}" \
        --max-time $TEST_TIMEOUT 2>&1 || echo "ERROR")
    
    # 保存响应
    echo "$response" > "$RESULTS_DIR/${tool_name}_response.txt"
    
    # 检查结果
    if echo "$response" | grep -qi "$expected_pattern" || [ "$expected_pattern" = "ANY" ]; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED_TESTS++))
        echo "PASS: $tool_name" >> "$RESULTS_DIR/summary.txt"
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED_TESTS++))
        echo "FAIL: $tool_name - Expected: $expected_pattern" >> "$RESULTS_DIR/summary.txt"
        echo "Response: $response" >> "$RESULTS_DIR/failures.txt"
    fi
}

# 运行核心工具测试
echo -e "${CYAN}核心工具测试:${NC}"

test_tool "calc_add" "计算 2 + 2" "4"
test_tool "calc_multiply" "计算 10 * 5" "50"
test_tool "exec_pwd" "显示当前目录" "/"
test_tool "task_list" "列出所有任务" "task"
test_tool "sessions_list" "显示所有会话" "session"

echo ""

# ============================================================================
# 步骤 3: 收集测试结果
# ============================================================================
echo -e "${BOLD}${BLUE}[3/7] 收集测试结果...${NC}"

# 统计信息
echo -e "${CYAN}测试统计:${NC}"
echo "  总测试数: $TOTAL_TESTS"
echo "  通过: $PASSED_TESTS"
echo "  失败: $FAILED_TESTS"

if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo "  通过率: ${PASS_RATE}%"
fi

echo ""

# ============================================================================
# 步骤 4: 分析失败的测试
# ============================================================================
echo -e "${BOLD}${BLUE}[4/7] 分析失败的测试...${NC}"

if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${YELLOW}发现 $FAILED_TESTS 个失败的测试${NC}"
    echo ""
    echo -e "${CYAN}失败详情:${NC}"
    
    if [ -f "$RESULTS_DIR/failures.txt" ]; then
        cat "$RESULTS_DIR/failures.txt" | head -20
    fi
    
    # 分析失败原因
    echo "" > "$RESULTS_DIR/analysis.txt"
    echo "失败原因分析:" >> "$RESULTS_DIR/analysis.txt"
    echo "=============" >> "$RESULTS_DIR/analysis.txt"
    
    if grep -q "ERROR" "$RESULTS_DIR/failures.txt" 2>/dev/null; then
        echo "- 网络或连接错误" >> "$RESULTS_DIR/analysis.txt"
    fi
    
    if grep -q "timeout" "$RESULTS_DIR/failures.txt" 2>/dev/null; then
        echo "- 响应超时" >> "$RESULTS_DIR/analysis.txt"
    fi
    
    if grep -q "not found" "$RESULTS_DIR/failures.txt" 2>/dev/null; then
        echo "- 工具未找到或未注册" >> "$RESULTS_DIR/analysis.txt"
    fi
    
    cat "$RESULTS_DIR/analysis.txt"
else
    echo -e "${GREEN}✓ 所有测试通过！${NC}"
fi

echo ""

# ============================================================================
# 步骤 5: 检查需要补全的代码
# ============================================================================
echo -e "${BOLD}${BLUE}[5/7] 检查需要补全的代码...${NC}"

# 检查 TODO 和 FIXME
echo -e "${CYAN}扫描 TODO/FIXME:${NC}"
TODO_COUNT=$(grep -r "TODO\|FIXME" crates/tools/src --include="*.rs" | wc -l | tr -d ' ')
echo "  发现 $TODO_COUNT 个待办项"

if [ $TODO_COUNT -gt 0 ]; then
    grep -r "TODO\|FIXME" crates/tools/src --include="*.rs" | head -10 > "$RESULTS_DIR/todos.txt"
    echo "  详见: $RESULTS_DIR/todos.txt"
fi

# 检查未实现的函数
echo -e "${CYAN}扫描未实现的函数:${NC}"
UNIMPLEMENTED_COUNT=$(grep -r "unimplemented!\|todo!" crates/tools/src --include="*.rs" | wc -l | tr -d ' ')
echo "  发现 $UNIMPLEMENTED_COUNT 个未实现的函数"

if [ $UNIMPLEMENTED_COUNT -gt 0 ]; then
    grep -r "unimplemented!\|todo!" crates/tools/src --include="*.rs" > "$RESULTS_DIR/unimplemented.txt"
    echo "  详见: $RESULTS_DIR/unimplemented.txt"
fi

echo ""

# ============================================================================
# 步骤 6: 生成改进建议
# ============================================================================
echo -e "${BOLD}${BLUE}[6/7] 生成改进建议...${NC}"

cat > "$RESULTS_DIR/recommendations.md" << EOF
# 测试结果和改进建议

**测试时间**: $(date)  
**通过率**: ${PASS_RATE}%

## 测试统计

- 总测试数: $TOTAL_TESTS
- 通过: $PASSED_TESTS
- 失败: $FAILED_TESTS

## 失败的测试

$(if [ $FAILED_TESTS -gt 0 ]; then
    cat "$RESULTS_DIR/failures.txt" 2>/dev/null || echo "无详细信息"
else
    echo "无失败测试"
fi)

## 代码质量

- TODO/FIXME: $TODO_COUNT 个
- 未实现函数: $UNIMPLEMENTED_COUNT 个

## 改进建议

$(if [ $FAILED_TESTS -gt 0 ]; then
    echo "1. 修复失败的测试"
    echo "2. 检查工具注册和配置"
    echo "3. 验证 API 端点"
fi)

$(if [ $TODO_COUNT -gt 0 ]; then
    echo "4. 完成 TODO 项"
fi)

$(if [ $UNIMPLEMENTED_COUNT -gt 0 ]; then
    echo "5. 实现未完成的函数"
fi)

## 下一步行动

1. 查看详细日志: \`$RESULTS_DIR/\`
2. 修复失败的测试
3. 补全未实现的代码
4. 重新运行测试验证

EOF

echo -e "${GREEN}✓ 建议已生成: $RESULTS_DIR/recommendations.md${NC}"
echo ""

# ============================================================================
# 步骤 7: 生成最终报告
# ============================================================================
echo -e "${BOLD}${BLUE}[7/7] 生成最终报告...${NC}"

cat > "$RESULTS_DIR/FINAL_REPORT.md" << EOF
# ClawMaster 自动化测试报告

**测试时间**: $(date)  
**结果目录**: $RESULTS_DIR

---

## 📊 执行摘要

| 指标 | 数值 |
|------|------|
| **总测试数** | $TOTAL_TESTS |
| **通过** | $PASSED_TESTS |
| **失败** | $FAILED_TESTS |
| **通过率** | ${PASS_RATE}% |

---

## 🔍 测试详情

### 通过的测试

$(grep "PASS:" "$RESULTS_DIR/summary.txt" 2>/dev/null || echo "无")

### 失败的测试

$(grep "FAIL:" "$RESULTS_DIR/summary.txt" 2>/dev/null || echo "无")

---

## 📝 代码质量分析

- **TODO/FIXME**: $TODO_COUNT 个待办项
- **未实现函数**: $UNIMPLEMENTED_COUNT 个

---

## 💡 改进建议

查看详细建议: \`$RESULTS_DIR/recommendations.md\`

---

## 📂 生成的文件

- \`summary.txt\` - 测试摘要
- \`failures.txt\` - 失败详情
- \`analysis.txt\` - 失败分析
- \`todos.txt\` - TODO 列表
- \`unimplemented.txt\` - 未实现函数
- \`recommendations.md\` - 改进建议
- \`gateway.log\` - Gateway 日志
- \`*_response.txt\` - 各工具响应

---

**生成时间**: $(date)
EOF

echo -e "${GREEN}✓ 最终报告已生成: $RESULTS_DIR/FINAL_REPORT.md${NC}"
echo ""

# ============================================================================
# 总结
# ============================================================================
echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BOLD}${CYAN}  ✅ 自动化测试完成${NC}"
echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BOLD}结果目录:${NC} $RESULTS_DIR"
echo -e "${BOLD}最终报告:${NC} $RESULTS_DIR/FINAL_REPORT.md"
echo -e "${BOLD}改进建议:${NC} $RESULTS_DIR/recommendations.md"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}${BOLD}🎉 所有测试通过！${NC}"
    exit 0
else
    echo -e "${YELLOW}${BOLD}⚠️  有 $FAILED_TESTS 个测试失败，需要修复${NC}"
    exit 1
fi
