#!/bin/bash
# ClawMaster 所有工具 CLI 批量测试脚本
# 测试 32 个工具（除 news_search 外），共 96 个场景

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

# 日志文件
LOG_DIR="./test_logs_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$LOG_DIR"
MASTER_LOG="$LOG_DIR/master_test.log"
REPORT_FILE="$LOG_DIR/test_report.md"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}║     ClawMaster 所有工具 CLI 批量测试                       ║${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}工具总数: 32（除 news_search）${NC}"
echo -e "${BLUE}测试场景: 96（每个工具 3 个场景）${NC}"
echo -e "${BLUE}日志目录: ${LOG_DIR}${NC}"
echo ""

# 初始化报告
cat > "$REPORT_FILE" << EOF
# ClawMaster 所有工具 CLI 测试报告

**执行时间**: $(date)  
**工具总数**: 32  
**测试场景**: 96  
**日志目录**: ${LOG_DIR}

---

## 测试结果

EOF

# 测试场景数组（96个，除去 news_search 的 3 个）
declare -a TEST_SCENARIOS=(
    # 2. calc (3)
    "calc|简单算术|计算 123 + 456"
    "calc|复杂表达式|Calculate (15 + 25) * 3"
    "calc|幂运算|What is 2 to the power of 10?"
    
    # 3. web_search (3)
    "web_search|技术搜索|Search for Rust programming tutorials"
    "web_search|中文搜索|搜索 Rust 编程教程"
    "web_search|问题搜索|How to fix async/await in Rust?"
    
    # 4. web_fetch (3)
    "web_fetch|获取网页|Fetch content from https://www.rust-lang.org"
    "web_fetch|获取API|Get data from https://api.github.com/repos/rust-lang/rust"
    "web_fetch|获取JSON|Fetch JSON from https://jsonplaceholder.typicode.com/posts/1"
    
    # 5. browser (3)
    "browser|打开网页|Open https://www.rust-lang.org in browser"
    "browser|截图|Take a screenshot of https://github.com"
    "browser|提取内容|Extract text from https://www.rust-lang.org"
    
    # 6. exec (3)
    "exec|执行命令|Run 'echo Hello World' command"
    "exec|系统信息|What's my system information?"
    "exec|查看文件|Show me the contents of README.md"
    
    # 7. process (3)
    "process|启动进程|Start a new process for monitoring"
    "process|列出进程|List all running processes"
    "process|停止进程|Stop process with ID 1234"
    
    # 8. task_list (3)
    "task_list|添加任务|Add a task: Review code changes"
    "task_list|列出任务|Show me my tasks"
    "task_list|完成任务|Mark task 1 as complete"
    
    # 9. sessions_list (3)
    "sessions_list|列出会话|List all sessions"
    "sessions_list|活跃会话|Show me active sessions"
    "sessions_list|查找会话|Find session named 'main'"
    
    # 10. sessions_history (3)
    "sessions_history|查看历史|Show history of session main"
    "sessions_history|最近消息|Show me the last 10 messages in session main"
    "sessions_history|搜索历史|Search for 'news' in session main history"
    
    # 11. sessions_send (3)
    "sessions_send|发送消息|Send 'Hello' to session test"
    "sessions_send|发送通知|Notify session main about the update"
    "sessions_send|广播消息|Broadcast 'System maintenance' to all sessions"
    
    # 12. sessions_create (3)
    "sessions_create|创建会话|Create a new session named 'test'"
    "sessions_create|临时会话|Create a temporary session for debugging"
    "sessions_create|配置会话|Create session 'dev' with debug mode enabled"
    
    # 13. sessions_delete (3)
    "sessions_delete|删除会话|Delete session named 'test'"
    "sessions_delete|清理会话|Delete all inactive sessions"
    "sessions_delete|删除临时|Remove temporary sessions"
    
    # 14. spawn_agent (3)
    "spawn_agent|生成代理|Spawn a new agent for monitoring"
    "spawn_agent|专用代理|Create an agent specialized in news gathering"
    "spawn_agent|临时代理|Spawn a temporary agent for this task"
    
    # 15. show_map (3)
    "show_map|城市地图|Show me a map of Beijing"
    "show_map|地区地图|Display a map of Silicon Valley"
    "show_map|路线地图|Show route from Beijing to Shanghai"
    
    # 16. get_user_location (3)
    "get_user_location|当前位置|Where am I?"
    "get_user_location|位置详情|What's my current location with details?"
    "get_user_location|GPS坐标|Give me my GPS coordinates"
    
    # 17. send_image (3)
    "send_image|发送图片|Send image.png to the chat"
    "send_image|发送截图|Send a screenshot of the current screen"
    "send_image|发送图表|Send the chart as an image"
    
    # 18. image (3)
    "image|生成图片|Generate an image of a sunset"
    "image|创建图表|Create a bar chart showing sales data"
    "image|生成图标|Generate an icon for the app"
    
    # 19. sandbox_packages (3)
    "sandbox_packages|列出包|List all sandbox packages"
    "sandbox_packages|搜索包|Find packages related to Python"
    "sandbox_packages|包详情|Show details of package 'rust'"
    
    # 20. nodes_list (3)
    "nodes_list|列出节点|List all nodes"
    "nodes_list|活跃节点|Show active nodes"
    "nodes_list|按类型列出|List compute nodes"
    
    # 21. nodes_describe (3)
    "nodes_describe|描述节点|Describe node 'node1'"
    "nodes_describe|节点状态|What's the status of node 'node1'?"
    "nodes_describe|节点配置|Show configuration of node 'node1'"
    
    # 22. nodes_select (3)
    "nodes_select|选择节点|Select node 'node1'"
    "nodes_select|最佳节点|Select the best node for computation"
    "nodes_select|空闲节点|Select an idle node"
    
    # 23. loop_detection (3)
    "loop_detection|检测循环|Check for loops in the current process"
    "loop_detection|分析循环|Analyze loop patterns"
    "loop_detection|报告循环|Report any detected loops"
    
    # 24. create_skill (3)
    "create_skill|创建技能|Create a new skill called 'weather_checker'"
    "create_skill|数据技能|Create a skill for data analysis"
    "create_skill|自动化技能|Create an automation skill"
    
    # 25. update_skill (3)
    "update_skill|更新技能|Update skill 'weather_checker'"
    "update_skill|修改配置|Modify configuration of skill 'data_analyzer'"
    "update_skill|升级技能|Upgrade skill to latest version"
    
    # 26. delete_skill (3)
    "delete_skill|删除技能|Delete skill 'old_skill'"
    "delete_skill|移除未使用|Remove unused skills"
    "delete_skill|清理技能|Clean up old skills"
    
    # 27. cron (3)
    "cron|创建定时|Schedule a task to run every day at 9am"
    "cron|列出定时|Show all scheduled tasks"
    "cron|删除定时|Remove the daily backup task"
    
    # 28. apply_patch (3)
    "apply_patch|应用补丁|Apply patch file 'fix.patch'"
    "apply_patch|代码更新|Apply the code update patch"
    "apply_patch|安全补丁|Apply security patch"
    
    # 29. branch_session (3)
    "branch_session|创建分支|Branch current session"
    "branch_session|实验分支|Create an experimental branch of this session"
    "branch_session|新上下文|Branch session to new context"
    
    # 30. session_state (3)
    "session_state|查看状态|Show session state"
    "session_state|检查健康|Check session health"
    "session_state|会话信息|Get current session information"
    
    # 31. gateway (3)
    "gateway|查看配置|Show gateway configuration"
    "gateway|更新设置|Update gateway settings"
    "gateway|检查状态|Check gateway status"
    
    # 32. agents_list (3)
    "agents_list|列出代理|List all agents"
    "agents_list|活跃代理|Show active agents"
    "agents_list|查找代理|Find agent named 'monitor'"
    
    # 33. pdf (3)
    "pdf|读取PDF|Read content from document.pdf"
    "pdf|提取文本|Extract text from report.pdf"
    "pdf|分析PDF|Analyze the PDF document"
)

TOTAL_TESTS=${#TEST_SCENARIOS[@]}
PASSED=0
FAILED=0
SKIPPED=0

echo -e "${GREEN}开始批量测试...${NC}" | tee -a "$MASTER_LOG"
echo ""

# 测试函数
run_test() {
    local test_num=$1
    local tool_name=$2
    local test_name=$3
    local input=$4
    
    local test_log="$LOG_DIR/${test_num}_${tool_name}_${test_name}.log"
    
    echo -e "${CYAN}[${test_num}/${TOTAL_TESTS}] 测试: ${tool_name} - ${test_name}${NC}" | tee -a "$MASTER_LOG"
    echo -e "  ${YELLOW}输入:${NC} ${input}" | tee -a "$MASTER_LOG"
    
    # 记录开始时间
    local start_time=$(date +%s)
    
    # 执行 CLI 命令
    echo "=== Test: ${tool_name} - ${test_name} ===" > "$test_log"
    echo "Input: ${input}" >> "$test_log"
    echo "Time: $(date)" >> "$test_log"
    echo "" >> "$test_log"
    
    # 运行 clawmaster agent 命令（macOS 兼容的超时实现）
    cargo run --release --bin clawmaster -- agent --message "$input" >> "$test_log" 2>&1 &
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
    
    # 短暂延迟避免过载
    sleep 1
}

# 执行所有测试
for i in "${!TEST_SCENARIOS[@]}"; do
    IFS='|' read -r tool_name test_name input <<< "${TEST_SCENARIOS[$i]}"
    run_test $((i+1)) "$tool_name" "$test_name" "$input"
done

# 生成摘要
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}" | tee -a "$MASTER_LOG"
echo -e "${GREEN}测试完成！${NC}" | tee -a "$MASTER_LOG"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"
echo -e "${BLUE}总测试数: ${TOTAL_TESTS}${NC}" | tee -a "$MASTER_LOG"
echo -e "${GREEN}通过: ${PASSED}${NC}" | tee -a "$MASTER_LOG"
echo -e "${RED}失败: ${FAILED}${NC}" | tee -a "$MASTER_LOG"
echo -e "${YELLOW}超时: ${SKIPPED}${NC}" | tee -a "$MASTER_LOG"
echo "" | tee -a "$MASTER_LOG"

# 计算通过率
if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED * 100 / TOTAL_TESTS))
    echo -e "${BLUE}通过率: ${PASS_RATE}%${NC}" | tee -a "$MASTER_LOG"
fi

# 写入报告摘要
cat >> "$REPORT_FILE" << EOF

## 测试摘要

**总测试数**: ${TOTAL_TESTS}  
**通过**: ${PASSED} ✅  
**失败**: ${FAILED} ❌  
**超时**: ${SKIPPED} ⏱️  
**通过率**: ${PASS_RATE}%

---

## 日志文件

- **主日志**: ${MASTER_LOG}
- **详细日志**: ${LOG_DIR}/*.log

---

**报告生成时间**: $(date)

EOF

echo -e "${GREEN}报告已生成: ${REPORT_FILE}${NC}" | tee -a "$MASTER_LOG"
echo -e "${GREEN}日志目录: ${LOG_DIR}${NC}" | tee -a "$MASTER_LOG"
echo ""
