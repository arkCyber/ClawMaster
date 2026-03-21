#!/bin/bash
# ClawMaster 自然语言全面功能测试 - 改进版

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

GATEWAY_URL="https://localhost:59233"
LOG_DIR="final_test_$(date +%Y%m%d_%H%M%S)"
REPORT_FILE="$LOG_DIR/test_report.md"
PASSED=0
FAILED=0
TIMEOUT=0
TOTAL=0

mkdir -p "$LOG_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   ClawMaster 自然语言全面功能测试                         ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}后端服务器:${NC} $GATEWAY_URL"
echo -e "${CYAN}日志目录:${NC} $LOG_DIR"
echo -e "${CYAN}开始时间:${NC} $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 初始化报告
cat > "$REPORT_FILE" << EOF
# ClawMaster 自然语言全面功能测试报告

**测试时间**: $(date '+%Y-%m-%d %H:%M:%S')  
**后端服务器**: $GATEWAY_URL  
**测试方式**: 自然语言命令 → 自然语言结果

---

## 测试结果

EOF

# 测试函数
test_command() {
    local num=$1
    local tool="$2"
    local desc="$3"
    local cmd="$4"
    
    ((TOTAL++))
    
    echo -e "${CYAN}[${num}/${TOTAL}] ${tool} - ${desc}${NC}"
    echo -e "  ${YELLOW}命令:${NC} ${cmd}"
    
    local log="$LOG_DIR/${num}_${tool}_${desc}.log"
    local start_time=$(date +%s)
    
    # 运行测试（后台进程 + 超时控制）
    CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" ./target/release/clawmaster agent --message "$cmd" > "$log" 2>&1 &
    local pid=$!
    
    # 等待最多30秒
    local elapsed=0
    local timeout_limit=30
    while [ $elapsed -lt $timeout_limit ]; do
        if ! kill -0 $pid 2>/dev/null; then
            # 进程已结束
            break
        fi
        sleep 1
        ((elapsed++))
    done
    
    # 检查结果
    if kill -0 $pid 2>/dev/null; then
        # 进程还在运行，超时
        kill -9 $pid 2>/dev/null
        wait $pid 2>/dev/null
        echo -e "  ${YELLOW}⏱️  超时${NC} (30s)"
        ((TIMEOUT++))
        
        cat >> "$REPORT_FILE" << EOF
### ⏱️ 测试 ${num}: ${tool} - ${desc}
**命令**: ${cmd}  
**状态**: 超时 (30s)  
**日志**: ${log}

---

EOF
    else
        # 进程已结束
        wait $pid
        local exit_code=$?
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        if [ $exit_code -eq 0 ]; then
            # 提取响应
            local response=$(grep -v "^2026-" "$log" | grep -v "^🔗" | grep -v "^📤" | grep "^✅" | sed 's/^✅ 响应://' | xargs)
            if [ -z "$response" ]; then
                response=$(tail -1 "$log")
            fi
            echo -e "  ${GREEN}✅ 通过${NC} (${duration}s)"
            echo -e "  ${CYAN}响应:${NC} ${response}"
            ((PASSED++))
            
            cat >> "$REPORT_FILE" << EOF
### ✅ 测试 ${num}: ${tool} - ${desc}
**命令**: ${cmd}  
**状态**: 通过  
**耗时**: ${duration}s  
**响应**: ${response}  
**日志**: ${log}

---

EOF
        else
            echo -e "  ${RED}❌ 失败${NC} (exit: ${exit_code}, ${duration}s)"
            ((FAILED++))
            
            cat >> "$REPORT_FILE" << EOF
### ❌ 测试 ${num}: ${tool} - ${desc}
**命令**: ${cmd}  
**状态**: 失败  
**退出码**: ${exit_code}  
**耗时**: ${duration}s  
**日志**: ${log}

---

EOF
        fi
    fi
    
    echo ""
}

echo -e "${GREEN}开始测试...${NC}"
echo ""

# 运行测试
test_command 1 "calc" "简单加法" "帮我计算 123 加 456 等于多少"
test_command 2 "calc" "复杂运算" "请计算 (15 + 25) 乘以 3 再减去 10"
test_command 3 "calc" "幂运算" "2 的 10 次方是多少？"
test_command 4 "task_list" "添加任务" "帮我添加一个任务：测试 ClawMaster 的所有功能"
test_command 5 "task_list" "查看任务" "显示我的所有任务列表"
test_command 6 "sessions_list" "列出会话" "显示所有的会话"
test_command 7 "memory_save" "保存记忆" "请记住：我喜欢使用 Rust 进行后端开发"
test_command 8 "memory_search" "搜索记忆" "你记得我喜欢什么编程语言吗？"
test_command 9 "web_search" "技术搜索" "搜索 Rust 异步编程教程"
test_command 10 "exec" "列出文件" "列出当前目录的文件"

# 生成最终统计
cat >> "$REPORT_FILE" << EOF

## 测试统计

**总测试数**: ${TOTAL}  
**通过**: ${PASSED}  
**失败**: ${FAILED}  
**超时**: ${TIMEOUT}  
**通过率**: $((PASSED * 100 / TOTAL))%

---

## 测试时间

**开始时间**: $(date '+%Y-%m-%d %H:%M:%S')  
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
echo -e "${YELLOW}超时: ${TIMEOUT}${NC}"
echo "通过率: $((PASSED * 100 / TOTAL))%"
echo ""
echo "详细报告: ${REPORT_FILE}"
echo "日志目录: ${LOG_DIR}"
echo ""

exit 0
