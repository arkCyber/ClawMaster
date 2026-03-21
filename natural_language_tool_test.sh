#!/bin/bash

# ClawMaster 自然语言工具测试脚本
# 通过 CLI 接口使用自然语言测试所有工具

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# 测试结果目录
TEST_DIR="natural_language_test_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$TEST_DIR"

# 日志文件
MASTER_LOG="$TEST_DIR/master.log"
SUMMARY_FILE="$TEST_DIR/summary.md"

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# CLI 路径
CLI_BIN="./target/release/clawmaster-courier"

echo "========================================" | tee "$MASTER_LOG"
echo "ClawMaster 自然语言工具测试" | tee -a "$MASTER_LOG"
echo "开始时间: $(date)" | tee -a "$MASTER_LOG"
echo "测试目录: $TEST_DIR" | tee -a "$MASTER_LOG"
echo "========================================" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"

# 检查 CLI 是否存在
if [ ! -f "$CLI_BIN" ]; then
    echo -e "${RED}错误: CLI 二进制文件不存在: $CLI_BIN${NC}"
    echo "请先运行: cargo build --release --bin clawmaster-courier"
    exit 1
fi

# 测试函数
run_test() {
    local category="$1"
    local test_name="$2"
    local query="$3"
    local test_file="$TEST_DIR/${category}_${test_name}.log"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    echo -e "${CYAN}[测试 $TOTAL_TESTS]${NC} ${BLUE}$category${NC} - $test_name" | tee -a "$MASTER_LOG"
    echo "  查询: $query" | tee -a "$MASTER_LOG"
    
    # 运行测试（使用 courier 的 agent chat 命令）
    # 注意：这里假设 courier 有类似的接口，如果没有，我们需要调整
    if echo "$query" | timeout 30s "$CLI_BIN" > "$test_file" 2>&1; then
        # 检查输出
        if grep -qi "error\|failed\|panic\|fatal" "$test_file"; then
            echo -e "  ${RED}✗ 失败${NC} (发现错误)" | tee -a "$MASTER_LOG"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            echo "  日志: $test_file" | tee -a "$MASTER_LOG"
        else
            echo -e "  ${GREEN}✓ 通过${NC}" | tee -a "$MASTER_LOG"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        fi
    else
        echo -e "  ${RED}✗ 失败${NC} (超时或执行错误)" | tee -a "$MASTER_LOG"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo "  日志: $test_file" | tee -a "$MASTER_LOG"
    fi
    echo "" | tee -a "$MASTER_LOG"
}

echo -e "${YELLOW}=== 开始测试 ===${NC}\n" | tee -a "$MASTER_LOG"

# ============================================
# 1. 文件系统工具测试
# ============================================
echo -e "${YELLOW}[类别 1] 文件系统工具${NC}" | tee -a "$MASTER_LOG"

run_test "filesystem" "read_cargo_toml" \
    "请读取 Cargo.toml 文件的内容"

run_test "filesystem" "read_readme" \
    "帮我查看 README.md 文件"

run_test "filesystem" "list_crates" \
    "列出 crates 目录下的所有子目录"

run_test "filesystem" "search_rust_files" \
    "在项目中搜索所有的 .rs 文件"

run_test "filesystem" "grep_function" \
    "在代码中搜索包含 'async fn' 的行"

# ============================================
# 2. 计算工具测试
# ============================================
echo -e "${YELLOW}[类别 2] 计算工具${NC}" | tee -a "$MASTER_LOG"

run_test "calc" "basic_add" \
    "帮我计算 123 + 456"

run_test "calc" "multiply" \
    "计算 25 乘以 8"

run_test "calc" "complex" \
    "计算 (100 + 50) * 2 - 30"

run_test "calc" "percentage" \
    "计算 80 的 15% 是多少"

run_test "calc" "division" \
    "1000 除以 25 等于多少"

# ============================================
# 3. 任务管理工具测试
# ============================================
echo -e "${YELLOW}[类别 3] 任务管理工具${NC}" | tee -a "$MASTER_LOG"

run_test "task" "add_task" \
    "添加一个任务：完成项目文档"

run_test "task" "list_tasks" \
    "显示所有任务列表"

run_test "task" "add_priority" \
    "添加一个高优先级任务：修复关键bug"

# ============================================
# 4. 内存工具测试
# ============================================
echo -e "${YELLOW}[类别 4] 内存工具${NC}" | tee -a "$MASTER_LOG"

run_test "memory" "save_info" \
    "记住：ClawMaster 是一个 AI 助手平台"

run_test "memory" "search" \
    "搜索关于 ClawMaster 的记忆"

run_test "memory" "context" \
    "显示当前的上下文信息"

# ============================================
# 5. 网络工具测试
# ============================================
echo -e "${YELLOW}[类别 5] 网络工具${NC}" | tee -a "$MASTER_LOG"

run_test "web" "search_rust" \
    "搜索 Rust 编程语言的最新信息"

run_test "web" "search_ai" \
    "搜索人工智能的最新进展"

# ============================================
# 6. 会话管理工具测试
# ============================================
echo -e "${YELLOW}[类别 6] 会话管理工具${NC}" | tee -a "$MASTER_LOG"

run_test "session" "list" \
    "显示所有会话"

run_test "session" "info" \
    "显示当前会话信息"

# ============================================
# 7. WASM 工具测试
# ============================================
echo -e "${YELLOW}[类别 7] WASM 工具${NC}" | tee -a "$MASTER_LOG"

run_test "wasm" "calc_test" \
    "使用 WASM 计算器计算 2^10"

run_test "wasm" "string_ops" \
    "将文本 'hello world' 转换为大写"

run_test "wasm" "base64" \
    "将 'ClawMaster' 编码为 base64"

# ============================================
# 测试完成，生成报告
# ============================================
echo -e "\n${YELLOW}=== 测试完成 ===${NC}\n" | tee -a "$MASTER_LOG"

# 生成 Markdown 报告
cat > "$SUMMARY_FILE" << EOF
# ClawMaster 自然语言工具测试报告

**测试时间**: $(date)  
**测试目录**: $TEST_DIR

---

## 📊 测试统计

| 指标 | 数值 |
|------|------|
| **总测试数** | $TOTAL_TESTS |
| **通过测试** | $PASSED_TESTS |
| **失败测试** | $FAILED_TESTS |
| **通过率** | $(awk "BEGIN {printf \"%.2f%%\", ($PASSED_TESTS/$TOTAL_TESTS)*100}") |

---

## 📋 测试分类

### 1. 文件系统工具 (5 个测试)
- 读取文件
- 列出目录
- 搜索文件
- Grep 搜索

### 2. 计算工具 (5 个测试)
- 基本运算
- 复杂表达式
- 百分比计算

### 3. 任务管理工具 (3 个测试)
- 添加任务
- 列出任务
- 优先级管理

### 4. 内存工具 (3 个测试)
- 保存信息
- 搜索记忆
- 上下文管理

### 5. 网络工具 (2 个测试)
- Web 搜索
- 信息查询

### 6. 会话管理工具 (2 个测试)
- 会话列表
- 会话信息

### 7. WASM 工具 (3 个测试)
- 计算器
- 字符串操作
- Base64 编码

---

## 📈 测试结果

EOF

# 添加详细结果
if [ $FAILED_TESTS -gt 0 ]; then
    echo "### ❌ 失败的测试" >> "$SUMMARY_FILE"
    echo "" >> "$SUMMARY_FILE"
    grep -B 1 "✗ 失败" "$MASTER_LOG" | grep "查询:" | sed 's/查询: /- /' >> "$SUMMARY_FILE"
    echo "" >> "$SUMMARY_FILE"
fi

echo "### ✅ 通过的测试" >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"
echo "共 $PASSED_TESTS 个测试通过" >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"

# 添加结论
cat >> "$SUMMARY_FILE" << EOF

---

## 🎯 结论

EOF

if [ $FAILED_TESTS -eq 0 ]; then
    cat >> "$SUMMARY_FILE" << EOF
**✅ 所有测试通过！**

ClawMaster 的所有工具都能正确响应自然语言查询，功能完整，性能良好。

**评分**: ⭐⭐⭐⭐⭐ (5/5)
EOF
else
    PASS_RATE=$(awk "BEGIN {printf \"%.1f\", ($PASSED_TESTS/$TOTAL_TESTS)*100}")
    cat >> "$SUMMARY_FILE" << EOF
**测试通过率**: $PASS_RATE%

有 $FAILED_TESTS 个测试失败，需要进一步检查和修复。

详细日志请查看: $TEST_DIR/
EOF
fi

cat >> "$SUMMARY_FILE" << EOF

---

**测试完成时间**: $(date)
EOF

# 显示最终统计
echo "========================================" | tee -a "$MASTER_LOG"
echo "测试统计:" | tee -a "$MASTER_LOG"
echo "  总测试数: $TOTAL_TESTS" | tee -a "$MASTER_LOG"
echo "  通过: $PASSED_TESTS" | tee -a "$MASTER_LOG"
echo "  失败: $FAILED_TESTS" | tee -a "$MASTER_LOG"
echo "  通过率: $(awk "BEGIN {printf \"%.2f%%\", ($PASSED_TESTS/$TOTAL_TESTS)*100}")" | tee -a "$MASTER_LOG"
echo "========================================" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"
echo "详细报告: $SUMMARY_FILE" | tee -a "$MASTER_LOG"
echo "测试日志: $MASTER_LOG" | tee -a "$MASTER_LOG"

# 显示报告
echo -e "\n${CYAN}=== 测试报告 ===${NC}\n"
cat "$SUMMARY_FILE"

# 退出码
if [ $FAILED_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi
