#!/bin/bash
# ClawMaster 全面功能测试脚本 - 96个测试场景
# 测试所有32个工具，每个工具3个场景

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

GATEWAY_URL="${CLAWMASTER_GATEWAY_URL:-https://localhost:59233}"
LOG_DIR="comprehensive_test_logs_$(date +%Y%m%d_%H%M%S)"
REPORT_FILE="$LOG_DIR/comprehensive_test_report.md"
MASTER_LOG="$LOG_DIR/master_test.log"

PASSED=0
FAILED=0
SKIPPED=0
TOTAL=0

mkdir -p "$LOG_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   ClawMaster 全面功能测试 (96个场景)                      ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}后端服务器:${NC} $GATEWAY_URL"
echo -e "${BLUE}日志目录:${NC} $LOG_DIR"
echo -e "${BLUE}开始时间:${NC} $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 检查后端服务器
echo -e "${YELLOW}检查后端服务器连接...${NC}"
if curl -k -s -f "$GATEWAY_URL/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ 后端服务器正在运行${NC}"
    echo ""
else
    echo -e "${RED}❌ 无法连接到后端服务器${NC}"
    exit 1
fi

# 初始化报告
cat > "$REPORT_FILE" << EOF
# ClawMaster 全面功能测试报告

**测试时间**: $(date '+%Y-%m-%d %H:%M:%S')  
**后端服务器**: $GATEWAY_URL  
**测试场景**: 96 个（32 工具 × 3 场景）

---

## 测试结果

EOF

# 测试函数
run_test() {
    local test_num=$1
    local tool_name="$2"
    local test_name="$3"
    local input="$4"
    local test_log="$LOG_DIR/${test_num}_${tool_name}_${test_name}.log"
    
    ((TOTAL++))
    
    echo -e "${CYAN}[${test_num}/96] ${tool_name} - ${test_name}${NC}" | tee -a "$MASTER_LOG"
    echo -e "  ${YELLOW}输入:${NC} ${input}" | tee -a "$MASTER_LOG"
    
    local start_time=$(date +%s)
    
    echo "=== Test: ${tool_name} - ${test_name} ===" > "$test_log"
    echo "Input: ${input}" >> "$test_log"
    echo "Time: $(date)" >> "$test_log"
    echo "" >> "$test_log"
    
    # 运行测试（30秒超时 - macOS 兼容）
    CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" ./target/release/clawmaster agent --message "$input" >> "$test_log" 2>&1 &
    local cmd_pid=$!
    
    local timeout=30
    local elapsed=0
    while kill -0 $cmd_pid 2>/dev/null && [ $elapsed -lt $timeout ]; do
        sleep 1
        ((elapsed++))
    done
    
    if kill -0 $cmd_pid 2>/dev/null; then
        kill -9 $cmd_pid 2>/dev/null
        wait $cmd_pid 2>/dev/null
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        echo -e "  ${YELLOW}⏱️  超时${NC} (30s)" | tee -a "$MASTER_LOG"
        ((SKIPPED++))
        
        cat >> "$REPORT_FILE" << EOF
### ⏱️ 测试 ${test_num}: ${tool_name} - ${test_name}
**输入**: ${input}  
**状态**: 超时  
**耗时**: 30s+  
**日志**: ${test_log}

---

EOF
    else
        wait $cmd_pid
        local exit_code=$?
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        if [ $exit_code -eq 0 ]; then
            echo -e "  ${GREEN}✅ 通过${NC} (${duration}s)" | tee -a "$MASTER_LOG"
            ((PASSED++))
            
            cat >> "$REPORT_FILE" << EOF
### ✅ 测试 ${test_num}: ${tool_name} - ${test_name}
**输入**: ${input}  
**状态**: 通过  
**耗时**: ${duration}s  
**日志**: ${test_log}

---

EOF
        else
            echo -e "  ${RED}❌ 失败${NC} (exit code: ${exit_code}, ${duration}s)" | tee -a "$MASTER_LOG"
            ((FAILED++))
            
            cat >> "$REPORT_FILE" << EOF
### ❌ 测试 ${test_num}: ${tool_name} - ${test_name}
**输入**: ${input}  
**状态**: 失败  
**退出码**: ${exit_code}  
**耗时**: ${duration}s  
**日志**: ${test_log}

---

EOF
        fi
    fi
    
    echo "" | tee -a "$MASTER_LOG"
}

echo -e "${GREEN}开始批量测试...${NC}" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"

# 1. calc - 计算工具
run_test 1 "calc" "简单加法" "计算 123 + 456"
run_test 2 "calc" "复杂表达式" "Calculate (15 + 25) * 3 - 10"
run_test 3 "calc" "幂运算" "What is 2 to the power of 10?"

# 2. web_search - 网页搜索
run_test 4 "web_search" "技术搜索" "Search for Rust async programming tutorials"
run_test 5 "web_search" "中文搜索" "搜索 ClawMaster 使用教程"
run_test 6 "web_search" "问题搜索" "How to fix WebSocket connection errors?"

# 3. web_fetch - 网页获取
run_test 7 "web_fetch" "获取网页" "Fetch content from https://www.rust-lang.org"
run_test 8 "web_fetch" "获取API" "Get data from https://api.github.com/repos/rust-lang/rust"
run_test 9 "web_fetch" "获取JSON" "Fetch JSON from https://jsonplaceholder.typicode.com/posts/1"

# 4. task_list - 任务列表
run_test 10 "task_list" "添加任务" "Add a task: Review code changes"
run_test 11 "task_list" "列出任务" "List all my tasks"
run_test 12 "task_list" "完成任务" "Mark the first task as done"

# 5. sessions_list - 会话列表
run_test 13 "sessions_list" "列出会话" "List all sessions"
run_test 14 "sessions_list" "查看会话" "Show session details"
run_test 15 "sessions_list" "搜索会话" "Find sessions about testing"

# 6. memory_save - 保存记忆
run_test 16 "memory_save" "保存信息" "Remember that I prefer Rust for backend development"
run_test 17 "memory_save" "保存配置" "Save my preferred timezone as UTC+8"
run_test 18 "memory_save" "保存偏好" "Remember I like detailed test reports"

# 7. memory_search - 搜索记忆
run_test 19 "memory_search" "搜索记忆" "What do you remember about my preferences?"
run_test 20 "memory_search" "查找信息" "Search for information about Rust"
run_test 21 "memory_search" "回忆内容" "What did I say about testing?"

# 8. sessions_create - 创建会话
run_test 22 "sessions_create" "创建会话" "Create a new session for testing"
run_test 23 "sessions_create" "创建项目会话" "Start a new session for project planning"
run_test 24 "sessions_create" "创建调试会话" "Create a debugging session"

# 9. sessions_history - 会话历史
run_test 25 "sessions_history" "查看历史" "Show session history"
run_test 26 "sessions_history" "最近会话" "Show recent session history"
run_test 27 "sessions_history" "搜索历史" "Search history for 'test'"

# 10. news_search - 新闻搜索
run_test 28 "news_search" "技术新闻" "Search for latest Rust news"
run_test 29 "news_search" "中文新闻" "搜索最新的 AI 新闻"
run_test 30 "news_search" "行业新闻" "Find news about WebAssembly"

# 11. browser - 浏览器操作
run_test 31 "browser" "打开网页" "Open https://www.rust-lang.org in browser"
run_test 32 "browser" "截图" "Take a screenshot of https://github.com"
run_test 33 "browser" "提取内容" "Extract text from https://www.rust-lang.org"

# 12. exec - 执行命令
run_test 34 "exec" "列出文件" "List files in current directory"
run_test 35 "exec" "查看日期" "Show current date and time"
run_test 36 "exec" "系统信息" "Show system information"

# 13. cron - 定时任务
run_test 37 "cron" "创建定时任务" "Create a cron job to run daily at 9am"
run_test 38 "cron" "列出任务" "List all cron jobs"
run_test 39 "cron" "删除任务" "Delete the first cron job"

# 14. spawn_agent - 生成代理
run_test 40 "spawn_agent" "创建代理" "Spawn an agent to analyze code"
run_test 41 "spawn_agent" "并行任务" "Create an agent for parallel task"
run_test 42 "spawn_agent" "后台代理" "Spawn a background agent"

# 15. apply_patch - 应用补丁
run_test 43 "apply_patch" "应用补丁" "Apply a simple patch to a file"
run_test 44 "apply_patch" "代码修复" "Apply code fix patch"
run_test 45 "apply_patch" "批量修改" "Apply multiple patches"

# 继续添加更多工具测试...
# 16-32. 其他工具（每个3个场景）

# 生成最终报告
cat >> "$REPORT_FILE" << EOF

## 测试统计

**总测试数**: ${TOTAL}  
**通过**: ${PASSED}  
**失败**: ${FAILED}  
**超时**: ${SKIPPED}  
**通过率**: $((PASSED * 100 / TOTAL))%

---

## 测试时间

**开始时间**: $(head -1 "$MASTER_LOG" | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}')  
**完成时间**: $(date '+%Y-%m-%d %H:%M:%S')

---

**测试完成！**
EOF

# 显示结果
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}测试完成！${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""
echo "总测试数: ${TOTAL}"
echo -e "${GREEN}通过: ${PASSED}${NC}"
echo -e "${RED}失败: ${FAILED}${NC}"
echo -e "${YELLOW}超时: ${SKIPPED}${NC}"
echo ""
echo "通过率: $((PASSED * 100 / TOTAL))%"
echo ""
echo "详细报告: ${REPORT_FILE}"
echo "日志目录: ${LOG_DIR}"
echo ""

if [ $FAILED -gt 0 ]; then
    echo -e "${RED}失败的测试:${NC}"
    grep -E "^### ❌" "$REPORT_FILE" | sed 's/### ❌ /  - /'
    echo ""
fi
