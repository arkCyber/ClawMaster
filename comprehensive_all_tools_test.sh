#!/bin/bash
# ClawMaster 全面工具测试 - 37个工具 × 3个场景 = 111个测试

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m'

GATEWAY_URL="https://localhost:59233"
LOG_DIR="all_tools_test_$(date +%Y%m%d_%H%M%S)"
REPORT_FILE="$LOG_DIR/comprehensive_report.md"
PASSED=0
FAILED=0
TIMEOUT=0
TOTAL=0

mkdir -p "$LOG_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   ClawMaster 全面工具测试 (37工具 × 3场景 = 111测试)     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}后端服务器:${NC} $GATEWAY_URL"
echo -e "${BLUE}日志目录:${NC} $LOG_DIR"
echo -e "${BLUE}开始时间:${NC} $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# 初始化报告
cat > "$REPORT_FILE" << 'EOF'
# ClawMaster 全面工具测试报告

**测试时间**: $(date '+%Y-%m-%d %H:%M:%S')  
**后端服务器**: https://localhost:59233  
**测试范围**: 37个工具 × 3个场景 = 111个测试

---

## 测试结果

EOF

# 测试函数
test_tool() {
    local num=$1
    local tool="$2"
    local scenario="$3"
    local cmd="$4"
    
    ((TOTAL++))
    
    echo -e "${CYAN}[${num}/111] ${tool} - ${scenario}${NC}"
    echo -e "  ${YELLOW}命令:${NC} ${cmd}"
    
    local safe_name=$(echo "${tool}_${scenario}" | sed 's/[^a-zA-Z0-9_]/_/g')
    local log="$LOG_DIR/${num}_${safe_name}.log"
    local start_time=$(date +%s)
    
    # 运行测试（后台进程 + 超时控制）
    CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" ./target/release/clawmaster agent --message "$cmd" > "$log" 2>&1 &
    local pid=$!
    
    # 等待最多30秒
    local elapsed=0
    local timeout_limit=30
    while [ $elapsed -lt $timeout_limit ]; do
        if ! kill -0 $pid 2>/dev/null; then
            break
        fi
        sleep 1
        ((elapsed++))
    done
    
    # 检查结果
    if kill -0 $pid 2>/dev/null; then
        kill -9 $pid 2>/dev/null
        wait $pid 2>/dev/null
        echo -e "  ${YELLOW}⏱️  超时${NC} (30s)"
        ((TIMEOUT++))
        
        cat >> "$REPORT_FILE" << EOF
### ⏱️ 测试 ${num}: ${tool} - ${scenario}
**命令**: ${cmd}  
**状态**: 超时 (30s)  
**日志**: ${log}

---

EOF
    else
        wait $pid
        local exit_code=$?
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        
        if [ $exit_code -eq 0 ]; then
            local response=$(grep "✅ 响应:" "$log" -A 5 | tail -5 | tr '\n' ' ' | sed 's/^[[:space:]]*//' | cut -c1-200)
            if [ -z "$response" ]; then
                response=$(tail -3 "$log" | tr '\n' ' ' | cut -c1-200)
            fi
            echo -e "  ${GREEN}✅ 通过${NC} (${duration}s)"
            echo -e "  ${BLUE}响应:${NC} ${response}..."
            ((PASSED++))
            
            cat >> "$REPORT_FILE" << EOF
### ✅ 测试 ${num}: ${tool} - ${scenario}
**命令**: ${cmd}  
**状态**: 通过  
**耗时**: ${duration}s  
**响应**: ${response}...  
**日志**: ${log}

---

EOF
        else
            echo -e "  ${RED}❌ 失败${NC} (exit: ${exit_code}, ${duration}s)"
            ((FAILED++))
            
            cat >> "$REPORT_FILE" << EOF
### ❌ 测试 ${num}: ${tool} - ${scenario}
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

echo -e "${GREEN}开始测试 111 个场景...${NC}"
echo ""

# ===== 1. calc (计算工具) =====
test_tool 1 "calc" "简单加法" "计算 123 + 456"
test_tool 2 "calc" "复杂运算" "计算 (15 + 25) × 3 - 10"
test_tool 3 "calc" "科学计算" "计算 2 的 10 次方"

# ===== 2. task_list (任务管理) =====
test_tool 4 "task_list" "添加任务" "添加一个任务：完成项目文档"
test_tool 5 "task_list" "查看任务" "显示所有任务"
test_tool 6 "task_list" "完成任务" "标记第一个任务为已完成"

# ===== 3. sessions_list (会话列表) =====
test_tool 7 "sessions_list" "列出会话" "显示所有会话"
test_tool 8 "sessions_list" "查找会话" "查找名为 main 的会话"
test_tool 9 "sessions_list" "统计会话" "统计有多少个会话"

# ===== 4. sessions_create (创建会话) =====
test_tool 10 "sessions_create" "创建新会话" "创建一个名为 test_session 的新会话"
test_tool 11 "sessions_create" "创建项目会话" "为项目 myproject 创建会话"
test_tool 12 "sessions_create" "创建临时会话" "创建一个临时测试会话"

# ===== 5. sessions_history (会话历史) =====
test_tool 13 "sessions_history" "查看历史" "显示当前会话的历史记录"
test_tool 14 "sessions_history" "查看最近消息" "显示最近5条消息"
test_tool 15 "sessions_history" "搜索历史" "在历史中搜索关于 Rust 的对话"

# ===== 6. sessions_send (发送消息到会话) =====
test_tool 16 "sessions_send" "发送消息" "向 main 会话发送消息：Hello"
test_tool 17 "sessions_send" "发送通知" "通知所有会话：系统更新"
test_tool 18 "sessions_send" "发送数据" "向会话发送测试数据"

# ===== 7. sessions_delete (删除会话) =====
test_tool 19 "sessions_delete" "删除会话" "删除名为 test_session 的会话"
test_tool 20 "sessions_delete" "清理会话" "清理所有空会话"
test_tool 21 "sessions_delete" "删除旧会话" "删除超过30天的会话"

# ===== 8. memory_save (保存记忆) =====
test_tool 22 "memory_save" "保存偏好" "记住：我喜欢使用 Rust 编程"
test_tool 23 "memory_save" "保存信息" "记住：项目截止日期是 2026年4月1日"
test_tool 24 "memory_save" "保存设置" "记住：默认使用深色主题"

# ===== 9. memory_search (搜索记忆) =====
test_tool 25 "memory_search" "搜索偏好" "你记得我喜欢什么编程语言吗？"
test_tool 26 "memory_search" "搜索信息" "项目截止日期是什么时候？"
test_tool 27 "memory_search" "搜索设置" "我的主题设置是什么？"

# ===== 10. memory_get (获取记忆) =====
test_tool 28 "memory_get" "获取所有记忆" "显示所有保存的记忆"
test_tool 29 "memory_get" "获取最近记忆" "显示最近保存的记忆"
test_tool 30 "memory_get" "获取特定记忆" "获取关于项目的记忆"

# ===== 11. web_search (网页搜索) =====
test_tool 31 "web_search" "技术搜索" "搜索 Rust 异步编程教程"
test_tool 32 "web_search" "新闻搜索" "搜索最新的 AI 新闻"
test_tool 33 "web_search" "文档搜索" "搜索 Tokio 官方文档"

# ===== 12. web_fetch (网页获取) =====
test_tool 34 "web_fetch" "获取网页" "获取 https://www.rust-lang.org 的内容"
test_tool 35 "web_fetch" "获取API" "获取 https://api.github.com 的数据"
test_tool 36 "web_fetch" "获取文档" "获取 Rust 官方文档首页"

# ===== 13. news_search (新闻搜索) =====
test_tool 37 "news_search" "科技新闻" "搜索最新的科技新闻"
test_tool 38 "news_search" "AI新闻" "搜索人工智能相关新闻"
test_tool 39 "news_search" "编程新闻" "搜索 Rust 编程语言新闻"

# ===== 14. exec (命令执行) =====
test_tool 40 "exec" "列出文件" "列出当前目录的文件"
test_tool 41 "exec" "查看日期" "显示当前日期和时间"
test_tool 42 "exec" "系统信息" "显示系统信息"

# ===== 15. process (进程管理) =====
test_tool 43 "process" "启动进程" "启动一个后台进程"
test_tool 44 "process" "查看进程" "显示所有运行的进程"
test_tool 45 "process" "停止进程" "停止指定的进程"

# ===== 16. browser (浏览器控制) =====
test_tool 46 "browser" "打开网页" "用浏览器打开 https://www.rust-lang.org"
test_tool 47 "browser" "截图" "对当前页面截图"
test_tool 48 "browser" "导航" "在浏览器中导航到首页"

# ===== 17. cron (定时任务) =====
test_tool 49 "cron" "创建定时任务" "创建一个每天早上9点运行的定时任务"
test_tool 50 "cron" "列出定时任务" "显示所有定时任务"
test_tool 51 "cron" "删除定时任务" "删除指定的定时任务"

# ===== 18. spawn_agent (创建代理) =====
test_tool 52 "spawn_agent" "创建分析代理" "创建一个代理来分析代码"
test_tool 53 "spawn_agent" "创建助手代理" "创建一个助手代理"
test_tool 54 "spawn_agent" "创建专家代理" "创建一个 Rust 专家代理"

# ===== 19. agents_list (代理列表) =====
test_tool 55 "agents_list" "列出代理" "显示所有代理"
test_tool 56 "agents_list" "查找代理" "查找活跃的代理"
test_tool 57 "agents_list" "统计代理" "统计代理数量"

# ===== 20. nodes_list (节点列表) =====
test_tool 58 "nodes_list" "列出节点" "显示所有节点"
test_tool 59 "nodes_list" "查看节点状态" "显示节点的状态"
test_tool 60 "nodes_list" "统计节点" "统计节点数量"

# ===== 21. nodes_describe (描述节点) =====
test_tool 61 "nodes_describe" "描述当前节点" "描述当前节点的信息"
test_tool 62 "nodes_describe" "节点详情" "显示节点的详细信息"
test_tool 63 "nodes_describe" "节点配置" "显示节点配置"

# ===== 22. nodes_select (选择节点) =====
test_tool 64 "nodes_select" "选择节点" "选择指定的节点"
test_tool 65 "nodes_select" "切换节点" "切换到另一个节点"
test_tool 66 "nodes_select" "激活节点" "激活指定节点"

# ===== 23. create_skill (创建技能) =====
test_tool 67 "create_skill" "创建代码技能" "创建一个代码分析技能"
test_tool 68 "create_skill" "创建助手技能" "创建一个助手技能"
test_tool 69 "create_skill" "创建工具技能" "创建一个工具技能"

# ===== 24. update_skill (更新技能) =====
test_tool 70 "update_skill" "更新技能" "更新现有技能"
test_tool 71 "update_skill" "修改技能" "修改技能配置"
test_tool 72 "update_skill" "优化技能" "优化技能性能"

# ===== 25. delete_skill (删除技能) =====
test_tool 73 "delete_skill" "删除技能" "删除指定技能"
test_tool 74 "delete_skill" "清理技能" "清理未使用的技能"
test_tool 75 "delete_skill" "移除技能" "移除过时的技能"

# ===== 26. show_map (显示地图) =====
test_tool 76 "show_map" "显示地图" "显示当前位置的地图"
test_tool 77 "show_map" "显示区域" "显示指定区域的地图"
test_tool 78 "show_map" "显示路线" "显示路线地图"

# ===== 27. speak (语音合成) =====
test_tool 79 "speak" "朗读文本" "朗读：Hello World"
test_tool 80 "speak" "语音输出" "用语音说：测试成功"
test_tool 81 "speak" "TTS" "将文本转换为语音：欢迎使用"

# ===== 28. transcribe (语音转录) =====
test_tool 82 "transcribe" "转录音频" "转录音频文件"
test_tool 83 "transcribe" "语音识别" "识别语音内容"
test_tool 84 "transcribe" "STT" "将语音转换为文本"

# ===== 29. send_message (发送消息) =====
test_tool 85 "send_message" "发送文本" "发送消息：测试"
test_tool 86 "send_message" "发送通知" "发送通知消息"
test_tool 87 "send_message" "发送提醒" "发送提醒消息"

# ===== 30. send_image (发送图片) =====
test_tool 88 "send_image" "发送图片" "发送一张图片"
test_tool 89 "send_image" "分享截图" "分享截图"
test_tool 90 "send_image" "上传图片" "上传图片文件"

# ===== 31. get_user_location (获取位置) =====
test_tool 91 "get_user_location" "获取位置" "获取我的当前位置"
test_tool 92 "get_user_location" "定位" "定位我的位置"
test_tool 93 "get_user_location" "GPS" "获取GPS坐标"

# ===== 32. loop_detection (循环检测) =====
test_tool 94 "loop_detection" "检测循环" "检测是否有循环"
test_tool 95 "loop_detection" "循环分析" "分析循环情况"
test_tool 96 "loop_detection" "防止循环" "防止无限循环"

# ===== 33. branch_session (分支会话) =====
test_tool 97 "branch_session" "创建分支" "创建会话分支"
test_tool 98 "branch_session" "分支会话" "从当前会话创建分支"
test_tool 99 "branch_session" "复制会话" "复制会话到新分支"

# ===== 34. session_state (会话状态) =====
test_tool 100 "session_state" "查看状态" "查看会话状态"
test_tool 101 "session_state" "状态信息" "显示会话状态信息"
test_tool 102 "session_state" "会话详情" "显示会话详细状态"

# ===== 35. apply_patch (应用补丁) =====
test_tool 103 "apply_patch" "应用补丁" "应用代码补丁"
test_tool 104 "apply_patch" "修复代码" "应用修复补丁"
test_tool 105 "apply_patch" "更新代码" "应用更新补丁"

# ===== 36. sandbox_packages (沙盒包) =====
test_tool 106 "sandbox_packages" "列出包" "列出沙盒中的包"
test_tool 107 "sandbox_packages" "查看包" "查看可用的包"
test_tool 108 "sandbox_packages" "包信息" "显示包信息"

# ===== 37. web_search_wasm / web_fetch_wasm (WASM版本) =====
test_tool 109 "web_search_wasm" "WASM搜索" "使用WASM搜索网页"
test_tool 110 "web_fetch_wasm" "WASM获取" "使用WASM获取网页"
test_tool 111 "wasm_tools" "WASM工具" "测试WASM工具"

# 生成最终统计
cat >> "$REPORT_FILE" << EOF

## 测试统计

**总测试数**: ${TOTAL}  
**通过**: ${PASSED}  
**失败**: ${FAILED}  
**超时**: ${TIMEOUT}  
**通过率**: $((PASSED * 100 / TOTAL))%

---

## 工具覆盖

测试了 37 个工具，每个工具 3 个场景，共 111 个测试。

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
echo -e "${YELLOW}超时: ${TIMEOUT}${NC}"
if [ $TOTAL -gt 0 ]; then
    echo "通过率: $((PASSED * 100 / TOTAL))%"
fi
echo ""
echo "详细报告: ${REPORT_FILE}"
echo "日志目录: ${LOG_DIR}"
echo ""

exit 0
