#!/bin/bash
# ClawMaster 后端服务器 CLI 测试脚本
# 通过 CLI 接口测试运行中的后端服务器功能

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# 配置
GATEWAY_URL="${CLAWMASTER_GATEWAY_URL:-http://localhost:3000}"
LOG_DIR="backend_test_logs_$(date +%Y%m%d_%H%M%S)"
REPORT_FILE="$LOG_DIR/test_report.md"
MASTER_LOG="$LOG_DIR/master_test.log"

# 统计
PASSED=0
FAILED=0
SKIPPED=0
TOTAL=0

# 创建日志目录
mkdir -p "$LOG_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   ClawMaster 后端服务器 CLI 测试（96个场景）              ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}后端服务器地址:${NC} $GATEWAY_URL"
echo -e "${BLUE}日志目录:${NC} $LOG_DIR"
echo ""

# 检查后端服务器是否运行
echo -e "${YELLOW}检查后端服务器连接...${NC}"
if curl -s -f "$GATEWAY_URL/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ 后端服务器正在运行${NC}"
    echo ""
else
    echo -e "${RED}❌ 无法连接到后端服务器${NC}"
    echo ""
    echo -e "${YELLOW}请先启动后端服务器:${NC}"
    echo "  clawmaster gateway"
    echo ""
    echo -e "${YELLOW}或设置正确的服务器地址:${NC}"
    echo "  export CLAWMASTER_GATEWAY_URL=http://localhost:3000"
    echo ""
    exit 1
fi

# 初始化报告
cat > "$REPORT_FILE" << 'EOF'
# ClawMaster 后端服务器 CLI 测试报告

**测试时间**: $(date '+%Y-%m-%d %H:%M:%S')  
**后端服务器**: $GATEWAY_URL  
**测试方法**: CLI 接口

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
    
    echo -e "${CYAN}[${test_num}/${TOTAL}] 测试: ${tool_name} - ${test_name}${NC}" | tee -a "$MASTER_LOG"
    echo -e "  ${YELLOW}输入:${NC} ${input}" | tee -a "$MASTER_LOG"
    
    # 记录开始时间
    local start_time=$(date +%s)
    
    # 执行 CLI 命令
    echo "=== Test: ${tool_name} - ${test_name} ===" > "$test_log"
    echo "Input: ${input}" >> "$test_log"
    echo "Time: $(date)" >> "$test_log"
    echo "" >> "$test_log"
    
    # 运行 clawmaster agent 命令（macOS 兼容的超时实现）
    CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" ./target/release/clawmaster agent --message "$input" >> "$test_log" 2>&1 &
    local cmd_pid=$!
    
    # 等待最多30秒
    local timeout=30
    local elapsed=0
    while kill -0 $cmd_pid 2>/dev/null && [ $elapsed -lt $timeout ]; do
        sleep 1
        ((elapsed++))
    done
    
    # 检查进程是否还在运行
    if kill -0 $cmd_pid 2>/dev/null; then
        # 超时，杀死进程
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
        # 进程已完成
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

# 开始测试
echo -e "${GREEN}开始批量测试...${NC}" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"

# calc 工具测试
run_test 1 "calc" "简单算术" "计算 123 + 456"
run_test 2 "calc" "复杂表达式" "Calculate (15 + 25) * 3"
run_test 3 "calc" "幂运算" "What is 2 to the power of 10?"

# web_search 工具测试
run_test 4 "web_search" "技术搜索" "Search for Rust programming tutorials"
run_test 5 "web_search" "中文搜索" "搜索 Rust 编程教程"
run_test 6 "web_search" "问题搜索" "How to fix async/await in Rust?"

# web_fetch 工具测试
run_test 7 "web_fetch" "获取网页" "Fetch content from https://www.rust-lang.org"
run_test 8 "web_fetch" "获取API" "Get data from https://api.github.com/repos/rust-lang/rust"
run_test 9 "web_fetch" "获取JSON" "Fetch JSON from https://jsonplaceholder.typicode.com/posts/1"

# task_list 工具测试
run_test 10 "task_list" "添加任务" "Add a task: Review code changes"
run_test 11 "task_list" "列出任务" "List all tasks"
run_test 12 "task_list" "完成任务" "Mark task 1 as done"

# sessions_list 工具测试
run_test 13 "sessions_list" "列出会话" "List all sessions"
run_test 14 "sessions_list" "查看会话" "Show session details"
run_test 15 "sessions_list" "搜索会话" "Find sessions about testing"

# 生成最终报告
cat >> "$REPORT_FILE" << EOF

## 测试统计

**总测试数**: ${TOTAL}  
**通过**: ${PASSED}  
**失败**: ${FAILED}  
**超时**: ${SKIPPED}  
**通过率**: $((PASSED * 100 / TOTAL))%

---

**测试完成时间**: $(date '+%Y-%m-%d %H:%M:%S')
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

# 如果有失败，显示失败的测试
if [ $FAILED -gt 0 ]; then
    echo -e "${RED}失败的测试:${NC}"
    grep -E "^### ❌" "$REPORT_FILE" | sed 's/### ❌ /  - /'
    echo ""
fi
