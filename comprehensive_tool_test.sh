#!/bin/bash

# ClawMaster 全面工具测试脚本
# 使用自然语言通过 CLI 接口测试所有工具

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 测试结果目录
TEST_DIR="comprehensive_test_results_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$TEST_DIR"

# 日志文件
MASTER_LOG="$TEST_DIR/master_test.log"
SUMMARY_FILE="$TEST_DIR/test_summary.md"

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo "========================================" | tee -a "$MASTER_LOG"
echo "ClawMaster 全面工具测试" | tee -a "$MASTER_LOG"
echo "开始时间: $(date)" | tee -a "$MASTER_LOG"
echo "测试目录: $TEST_DIR" | tee -a "$MASTER_LOG"
echo "========================================" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"

# 测试函数
run_test() {
    local test_name="$1"
    local test_query="$2"
    local test_file="$TEST_DIR/${test_name}.log"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    echo -e "${BLUE}[测试 $TOTAL_TESTS]${NC} $test_name" | tee -a "$MASTER_LOG"
    echo "查询: $test_query" | tee -a "$MASTER_LOG"
    
    # 运行测试
    if timeout 30s cargo run --release --bin clawmaster -- agent chat "$test_query" > "$test_file" 2>&1; then
        # 检查输出是否包含错误
        if grep -qi "error\|failed\|panic" "$test_file"; then
            echo -e "${RED}✗ 失败${NC}" | tee -a "$MASTER_LOG"
            FAILED_TESTS=$((FAILED_TESTS + 1))
        else
            echo -e "${GREEN}✓ 通过${NC}" | tee -a "$MASTER_LOG"
            PASSED_TESTS=$((PASSED_TESTS + 1))
        fi
    else
        echo -e "${RED}✗ 超时或崩溃${NC}" | tee -a "$MASTER_LOG"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
    
    echo "" | tee -a "$MASTER_LOG"
}

# ============================================
# 1. 文件系统工具测试
# ============================================
echo -e "${YELLOW}=== 文件系统工具测试 ===${NC}" | tee -a "$MASTER_LOG"

# read_file 测试
run_test "01_read_file_cargo_toml" "读取 Cargo.toml 文件的内容"
run_test "02_read_file_readme" "查看 README.md 文件"
run_test "03_read_file_specific_lines" "读取 Cargo.toml 的前 20 行"

# write_file 测试
run_test "04_write_file_create" "创建一个名为 test_output.txt 的文件，内容是 'Hello ClawMaster'"
run_test "05_write_file_append" "在 test_output.txt 文件末尾追加一行 'Test completed'"
run_test "06_write_file_json" "创建 test_data.json 文件，包含 {\"name\": \"test\", \"value\": 123}"

# list_directory 测试
run_test "07_list_dir_root" "列出当前目录的所有文件和文件夹"
run_test "08_list_dir_crates" "列出 crates 目录下的所有子目录"
run_test "09_list_dir_recursive" "递归列出 crates/tools/src 目录的内容"

# grep_tool 测试
run_test "10_grep_search_function" "在 crates/tools/src 目录中搜索包含 'pub fn' 的行"
run_test "11_grep_search_error" "在所有 Rust 文件中搜索 'Error' 关键字"
run_test "12_grep_search_todo" "查找代码中的 TODO 注释"

# search_files 测试
run_test "13_search_files_rust" "查找所有 .rs 文件"
run_test "14_search_files_toml" "查找所有 Cargo.toml 文件"
run_test "15_search_files_pattern" "查找名称包含 'test' 的文件"

# ============================================
# 2. 计算工具测试
# ============================================
echo -e "${YELLOW}=== 计算工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "16_calc_basic_add" "计算 123 + 456"
run_test "17_calc_basic_multiply" "计算 25 * 48"
run_test "18_calc_complex" "计算 (100 + 50) * 2 - 30"
run_test "19_calc_percentage" "计算 250 的 15%"
run_test "20_calc_power" "计算 2 的 10 次方"

# ============================================
# 3. 任务管理工具测试
# ============================================
echo -e "${YELLOW}=== 任务管理工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "21_task_add_simple" "添加一个任务：完成代码审查"
run_test "22_task_add_priority" "添加高优先级任务：修复安全漏洞"
run_test "23_task_list_all" "列出所有任务"
run_test "24_task_complete" "完成第一个任务"
run_test "25_task_list_pending" "列出未完成的任务"

# ============================================
# 4. 会话管理工具测试
# ============================================
echo -e "${YELLOW}=== 会话管理工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "26_session_list" "列出所有会话"
run_test "27_session_create" "创建一个新会话：项目讨论"
run_test "28_session_history" "查看当前会话的历史记录"
run_test "29_session_switch" "切换到最近的会话"
run_test "30_session_info" "显示当前会话的详细信息"

# ============================================
# 5. 内存工具测试
# ============================================
echo -e "${YELLOW}=== 内存工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "31_memory_save" "记住这个信息：项目使用 Rust 语言开发"
run_test "32_memory_search" "搜索关于 Rust 的记忆"
run_test "33_memory_query" "查询项目使用什么语言"
run_test "34_memory_list" "列出所有保存的记忆"
run_test "35_memory_context" "获取当前上下文的相关记忆"

# ============================================
# 6. 网络工具测试
# ============================================
echo -e "${YELLOW}=== 网络工具测试 ===${NC}" | tee -a "$MASTER_LOG"

# web_search 测试
run_test "36_web_search_rust" "搜索 Rust 编程语言的最新特性"
run_test "37_web_search_ai" "搜索 AI 代理的最佳实践"
run_test "38_web_search_specific" "搜索 DO-178C 标准的详细信息"

# web_fetch 测试
run_test "39_web_fetch_github" "获取 https://github.com 的内容"
run_test "40_web_fetch_api" "获取 https://api.github.com 的 API 信息"
run_test "41_web_fetch_json" "获取 https://jsonplaceholder.typicode.com/posts/1 的 JSON 数据"

# news 测试
run_test "42_news_tech" "搜索最新的科技新闻"
run_test "43_news_ai" "搜索关于人工智能的新闻"
run_test "44_news_rust" "搜索 Rust 编程语言的新闻"

# ============================================
# 7. 技能工具测试
# ============================================
echo -e "${YELLOW}=== 技能工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "45_skill_list" "列出所有可用的技能"
run_test "46_skill_search" "搜索与编程相关的技能"
run_test "47_skill_info" "显示某个技能的详细信息"
run_test "48_skill_install" "安装一个新技能"
run_test "49_skill_remove" "移除未使用的技能"

# ============================================
# 8. 定时任务测试
# ============================================
echo -e "${YELLOW}=== 定时任务测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "50_cron_create" "创建一个每天上午 9 点运行的定时任务"
run_test "51_cron_list" "列出所有定时任务"
run_test "52_cron_info" "显示定时任务的详细信息"
run_test "53_cron_disable" "禁用某个定时任务"
run_test "54_cron_delete" "删除某个定时任务"

# ============================================
# 9. 补丁工具测试
# ============================================
echo -e "${YELLOW}=== 补丁工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "55_patch_create" "创建一个代码补丁"
run_test "56_patch_apply" "应用代码补丁到文件"
run_test "57_patch_verify" "验证补丁是否正确应用"

# ============================================
# 10. 浏览器工具测试
# ============================================
echo -e "${YELLOW}=== 浏览器工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "58_browser_open" "打开浏览器访问 https://rust-lang.org"
run_test "59_browser_screenshot" "对当前页面截图"
run_test "60_browser_extract" "提取页面的主要内容"

# ============================================
# 11. 图像工具测试
# ============================================
echo -e "${YELLOW}=== 图像工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "61_image_generate" "生成一张图片：蓝天白云"
run_test "62_image_analyze" "分析图片内容"
run_test "63_image_optimize" "优化图片大小"

# ============================================
# 12. PDF 工具测试
# ============================================
echo -e "${YELLOW}=== PDF 工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "64_pdf_read" "读取 PDF 文件内容"
run_test "65_pdf_extract" "提取 PDF 中的文本"
run_test "66_pdf_analyze" "分析 PDF 文档结构"

# ============================================
# 13. 位置工具测试
# ============================================
echo -e "${YELLOW}=== 位置工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "67_location_current" "获取当前位置"
run_test "68_location_search" "搜索北京的位置信息"
run_test "69_location_distance" "计算两地之间的距离"

# ============================================
# 14. 地图工具测试
# ============================================
echo -e "${YELLOW}=== 地图工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "70_map_search" "在地图上搜索餐厅"
run_test "71_map_route" "规划从 A 到 B 的路线"
run_test "72_map_nearby" "查找附近的加油站"

# ============================================
# 15. 代理工具测试
# ============================================
echo -e "${YELLOW}=== 代理工具测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "73_agent_list" "列出所有可用的代理"
run_test "74_agent_spawn" "创建一个新的代理"
run_test "75_agent_communicate" "与另一个代理通信"

# ============================================
# 16. 分支会话测试
# ============================================
echo -e "${YELLOW}=== 分支会话测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "76_branch_create" "创建一个分支会话"
run_test "77_branch_switch" "切换到分支会话"
run_test "78_branch_merge" "合并分支会话"

# ============================================
# 17. 网关配置测试
# ============================================
echo -e "${YELLOW}=== 网关配置测试 ===${NC}" | tee -a "$MASTER_LOG"

run_test "79_gateway_status" "查看网关状态"
run_test "80_gateway_config" "显示网关配置"
run_test "81_gateway_health" "检查网关健康状态"

# ============================================
# 生成测试报告
# ============================================
echo "" | tee -a "$MASTER_LOG"
echo "========================================" | tee -a "$MASTER_LOG"
echo "测试完成" | tee -a "$MASTER_LOG"
echo "结束时间: $(date)" | tee -a "$MASTER_LOG"
echo "========================================" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"
echo "测试统计:" | tee -a "$MASTER_LOG"
echo "  总测试数: $TOTAL_TESTS" | tee -a "$MASTER_LOG"
echo "  通过: $PASSED_TESTS" | tee -a "$MASTER_LOG"
echo "  失败: $FAILED_TESTS" | tee -a "$MASTER_LOG"
echo "  成功率: $(awk "BEGIN {printf \"%.2f\", ($PASSED_TESTS/$TOTAL_TESTS)*100}")%" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"

# 生成 Markdown 报告
cat > "$SUMMARY_FILE" << EOF
# ClawMaster 全面工具测试报告

**测试时间**: $(date)  
**测试目录**: $TEST_DIR

---

## 📊 测试统计

| 指标 | 数量 | 百分比 |
|------|------|--------|
| **总测试数** | $TOTAL_TESTS | 100% |
| **通过测试** | $PASSED_TESTS | $(awk "BEGIN {printf \"%.2f\", ($PASSED_TESTS/$TOTAL_TESTS)*100}")% |
| **失败测试** | $FAILED_TESTS | $(awk "BEGIN {printf \"%.2f\", ($FAILED_TESTS/$TOTAL_TESTS)*100}")% |

---

## 📋 测试分类

### 1. 文件系统工具 (15 个测试)
- read_file: 3 个场景
- write_file: 3 个场景
- list_directory: 3 个场景
- grep_tool: 3 个场景
- search_files: 3 个场景

### 2. 计算工具 (5 个测试)
- 基础运算
- 复杂表达式
- 百分比计算
- 幂运算

### 3. 任务管理工具 (5 个测试)
- 添加任务
- 列出任务
- 完成任务
- 查询任务

### 4. 会话管理工具 (5 个测试)
- 列出会话
- 创建会话
- 查看历史
- 切换会话
- 会话信息

### 5. 内存工具 (5 个测试)
- 保存记忆
- 搜索记忆
- 查询记忆
- 列出记忆
- 上下文记忆

### 6. 网络工具 (9 个测试)
- web_search: 3 个场景
- web_fetch: 3 个场景
- news: 3 个场景

### 7. 技能工具 (5 个测试)
- 列出技能
- 搜索技能
- 技能信息
- 安装技能
- 移除技能

### 8. 定时任务 (5 个测试)
- 创建定时任务
- 列出定时任务
- 任务信息
- 禁用任务
- 删除任务

### 9. 补丁工具 (3 个测试)
- 创建补丁
- 应用补丁
- 验证补丁

### 10. 浏览器工具 (3 个测试)
- 打开浏览器
- 截图
- 提取内容

### 11. 图像工具 (3 个测试)
- 生成图片
- 分析图片
- 优化图片

### 12. PDF 工具 (3 个测试)
- 读取 PDF
- 提取文本
- 分析结构

### 13. 位置工具 (3 个测试)
- 获取位置
- 搜索位置
- 计算距离

### 14. 地图工具 (3 个测试)
- 搜索地点
- 规划路线
- 查找附近

### 15. 代理工具 (3 个测试)
- 列出代理
- 创建代理
- 代理通信

### 16. 分支会话 (3 个测试)
- 创建分支
- 切换分支
- 合并分支

### 17. 网关配置 (3 个测试)
- 查看状态
- 显示配置
- 健康检查

---

## 📁 详细日志

所有测试的详细日志保存在: \`$TEST_DIR/\`

- 主日志: \`master_test.log\`
- 各测试日志: \`01_*.log\` 到 \`81_*.log\`

---

## 🎯 下一步行动

1. 分析失败的测试
2. 识别常见问题模式
3. 提出改进建议
4. 实施修复方案
5. 重新测试验证

EOF

echo -e "${GREEN}测试报告已生成: $SUMMARY_FILE${NC}"
echo -e "${BLUE}查看详细日志: $MASTER_LOG${NC}"

# 显示失败的测试
if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${RED}失败的测试:${NC}"
    grep -B 1 "✗" "$MASTER_LOG" | grep "测试" | tee -a "$SUMMARY_FILE"
fi

echo ""
echo "测试完成！"
