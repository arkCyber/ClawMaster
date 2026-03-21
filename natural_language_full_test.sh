#!/bin/bash
# ClawMaster 自然语言全面功能测试
# 测试所有工具，使用自然语言命令，返回自然语言结果

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m'

GATEWAY_URL="https://localhost:59233"
LOG_DIR="natural_language_test_$(date +%Y%m%d_%H%M%S)"
REPORT_FILE="$LOG_DIR/test_report.md"
PASSED=0
FAILED=0
TOTAL=0

mkdir -p "$LOG_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   ClawMaster 自然语言全面功能测试                         ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}后端服务器:${NC} $GATEWAY_URL"
echo -e "${BLUE}日志目录:${NC} $LOG_DIR"
echo -e "${BLUE}开始时间:${NC} $(date '+%Y-%m-%d %H:%M:%S')"
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
    
    echo -e "${CYAN}[${num}] ${tool} - ${desc}${NC}"
    echo -e "  ${YELLOW}命令:${NC} ${cmd}"
    
    local log="$LOG_DIR/${num}_${tool}_${desc}.log"
    local start_time=$(date +%s)
    
    # 运行测试（后台进程 + 超时控制）
    CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" ./target/release/clawmaster agent --message "$cmd" > "$log" 2>&1 &
    local pid=$!
    
    # 等待最多30秒
    local elapsed=0
    while kill -0 $pid 2>/dev/null && [ $elapsed -lt 30 ]; do
        sleep 1
        ((elapsed++))
    done
    
    # 检查结果
    if kill -0 $pid 2>/dev/null; then
        kill -9 $pid 2>/dev/null
        wait $pid 2>/dev/null
        echo -e "  ${YELLOW}⏱️  超时${NC} (30s)"
        ((FAILED++))
        
        cat >> "$REPORT_FILE" << EOF
### ⏱️ 测试 ${num}: ${tool} - ${desc}
**命令**: ${cmd}  
**状态**: 超时  
**日志**: ${log}

---

EOF
    else
        wait $pid
        local exit_code=$?
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        if [ $exit_code -eq 0 ]; then
            # 提取响应
            local response=$(grep -v "^2026-" "$log" | grep -v "^🔗" | grep -v "^📤" | tail -1)
            echo -e "  ${GREEN}✅ 通过${NC} (${duration}s)"
            echo -e "  ${BLUE}响应:${NC} ${response}"
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

echo -e "${GREEN}开始测试所有工具...${NC}"
echo ""

# ===== 1. 计算工具 (calc) =====
test_command 1 "calc" "简单加法" "帮我计算 123 加 456 等于多少"
test_command 2 "calc" "复杂运算" "请计算 (15 + 25) 乘以 3 再减去 10"
test_command 3 "calc" "幂运算" "2 的 10 次方是多少？"

# ===== 2. 任务管理 (task_list) =====
test_command 4 "task_list" "添加任务" "帮我添加一个任务：测试 ClawMaster 的所有功能"
test_command 5 "task_list" "查看任务" "显示我的所有任务列表"
test_command 6 "task_list" "完成任务" "把第一个任务标记为已完成"

# ===== 3. 会话管理 (sessions) =====
test_command 7 "sessions_list" "列出会话" "显示所有的会话"
test_command 8 "sessions_create" "创建会话" "创建一个新的会话用于测试"
test_command 9 "sessions_history" "会话历史" "显示会话的历史记录"

# ===== 4. 记忆系统 (memory) =====
test_command 10 "memory_save" "保存记忆" "请记住：我喜欢使用 Rust 进行后端开发"
test_command 11 "memory_search" "搜索记忆" "你记得我喜欢什么编程语言吗？"
test_command 12 "memory_get" "获取记忆" "搜索关于 Rust 的记忆"

# ===== 5. 网页搜索 (web_search) =====
test_command 13 "web_search" "技术搜索" "搜索 Rust 异步编程教程"
test_command 14 "web_search" "中文搜索" "搜索 ClawMaster 使用文档"
test_command 15 "web_search" "问题搜索" "搜索如何修复 WebSocket 连接错误"

# ===== 6. 网页获取 (web_fetch) =====
test_command 16 "web_fetch" "获取网页" "获取 https://www.rust-lang.org 的内容"
test_command 17 "web_fetch" "获取API" "获取 https://api.github.com/repos/rust-lang/rust 的数据"

# ===== 7. 新闻搜索 (news_search) =====
test_command 18 "news_search" "技术新闻" "搜索最新的 Rust 相关新闻"
test_command 19 "news_search" "AI新闻" "搜索最新的人工智能新闻"

# ===== 8. 命令执行 (exec) =====
test_command 20 "exec" "列出文件" "列出当前目录的文件"
test_command 21 "exec" "查看日期" "显示当前的日期和时间"
test_command 22 "exec" "系统信息" "显示系统信息"

# ===== 9. 定时任务 (cron) =====
test_command 23 "cron" "创建定时任务" "创建一个每天早上9点运行的定时任务"
test_command 24 "cron" "列出定时任务" "显示所有的定时任务"

# ===== 10. 代理生成 (spawn_agent) =====
test_command 25 "spawn_agent" "创建代理" "创建一个代理来分析代码"

# ===== 11. 节点操作 (nodes) =====
test_command 26 "nodes_list" "列出节点" "显示所有节点"
test_command 27 "nodes_describe" "描述节点" "描述当前节点"

# ===== 12. 技能管理 (skills) =====
test_command 28 "create_skill" "创建技能" "创建一个新的技能"
test_command 29 "update_skill" "更新技能" "更新现有的技能"

# ===== 13. 地图显示 (show_map) =====
test_command 30 "show_map" "显示地图" "显示当前的地图"

# ===== 14. 语音合成 (speak) =====
test_command 31 "speak" "语音输出" "朗读：Hello World"

# ===== 15. 转录 (transcribe) =====
test_command 32 "transcribe" "音频转录" "转录音频文件"

# ===== 16. 浏览器 (browser) =====
test_command 33 "browser" "打开网页" "用浏览器打开 https://www.rust-lang.org"

# ===== 17. 发送消息 (send_message) =====
test_command 34 "send_message" "发送消息" "发送一条消息"

# ===== 18. 发送图片 (send_image) =====
test_command 35 "send_image" "发送图片" "发送一张图片"

# ===== 19. 代理列表 (agents_list) =====
test_command 36 "agents_list" "列出代理" "显示所有的代理"

# ===== 20. 循环检测 (loop_detection) =====
test_command 37 "loop_detection" "检测循环" "检测是否有循环"

# 生成最终统计
cat >> "$REPORT_FILE" << EOF

## 测试统计

**总测试数**: ${TOTAL}  
**通过**: ${PASSED}  
**失败**: ${FAILED}  
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
echo "通过率: $((PASSED * 100 / TOTAL))%"
echo ""
echo "详细报告: ${REPORT_FILE}"
echo "日志目录: ${LOG_DIR}"
echo ""

if [ $FAILED -gt 0 ]; then
    echo -e "${RED}失败的测试:${NC}"
    grep -E "^### ❌" "$REPORT_FILE" | sed 's/### ❌ /  - /' || true
    echo ""
fi

exit 0
