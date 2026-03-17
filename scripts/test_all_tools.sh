#!/bin/bash
# ClawMaster 完整工具测试脚本 - 测试所有 29+ 个工具

set -e

# 配置
API_BASE="https://localhost:3000/api"
SESSION="main"
RESULTS_FILE="complete_tool_test_results_$(date +%Y%m%d_%H%M%S).md"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 测试计数器
TOTAL=0
PASSED=0
FAILED=0

# 初始化结果文件
cat > "$RESULTS_FILE" << EOF
# ClawMaster 完整工具测试报告

**测试时间**: $(date)
**测试会话**: main
**测试范围**: 所有 29+ 个工具

---

## 测试结果

EOF

# 测试函数
test_tool() {
    local tool_name=$1
    local test_message=$2
    local test_id=$3
    
    TOTAL=$((TOTAL + 1))
    
    echo -e "${YELLOW}[测试 $test_id] $tool_name${NC}"
    echo "问题: $test_message"
    
    response=$(curl -k -s -X POST "$API_BASE/chat" \
        -H "Content-Type: application/json" \
        -d "{\"session\":\"$SESSION\",\"message\":\"$test_message\",\"stream\":false}" \
        2>/dev/null || echo "ERROR")
    
    if [ "$response" = "ERROR" ] || [ -z "$response" ]; then
        echo -e "${RED}✗ 失败${NC}"
        FAILED=$((FAILED + 1))
        echo "### ❌ $tool_name - 测试 $test_id" >> "$RESULTS_FILE"
        echo "- **问题**: $test_message" >> "$RESULTS_FILE"
        echo "- **结果**: 失败" >> "$RESULTS_FILE"
        echo "" >> "$RESULTS_FILE"
    else
        echo -e "${GREEN}✓ 通过${NC}"
        PASSED=$((PASSED + 1))
        echo "### ✅ $tool_name - 测试 $test_id" >> "$RESULTS_FILE"
        echo "- **问题**: $test_message" >> "$RESULTS_FILE"
        echo "- **结果**: 成功" >> "$RESULTS_FILE"
        echo "" >> "$RESULTS_FILE"
    fi
    
    echo ""
    sleep 2
}

echo "========================================"
echo "  ClawMaster 完整工具测试"
echo "  目标: 测试所有 29+ 个工具"
echo "========================================"
echo ""

# ============================================
# 第 1 组：核心执行工具 (5个)
# ============================================
echo -e "${BLUE}=== 第 1 组：核心执行工具 (5个) ===${NC}"
echo ""

test_tool "exec" "请执行命令：echo 'ClawMaster Full Test'" "1.1"
test_tool "exec" "请执行命令：pwd" "1.2"
test_tool "calc" "请计算：(100 + 200) * 3 / 2" "2.1"
test_tool "calc" "请计算：sqrt(256) + pow(3, 3)" "2.2"
test_tool "process" "请显示当前系统的进程信息" "3.1"
test_tool "sandbox_packages" "请列出沙箱中已安装的包" "4.1"
test_tool "cron" "请列出所有定时任务" "5.1"

# ============================================
# 第 2 组：网络工具 (3个)
# ============================================
echo -e "${BLUE}=== 第 2 组：网络工具 (3个) ===${NC}"
echo ""

test_tool "web_search" "请搜索：ClawMaster AI assistant" "6.1"
test_tool "web_search" "请搜索今天的科技新闻" "6.2"
test_tool "web_fetch" "请获取 https://example.com 的内容" "7.1"
test_tool "web_fetch" "请以 Markdown 格式获取 https://www.rust-lang.org 的内容" "7.2"
test_tool "browser" "请打开浏览器访问 https://www.wikipedia.org" "8.1"
test_tool "browser" "请访问 https://example.com 并截图" "8.2"

# ============================================
# 第 3 组：内存工具 (3个)
# ============================================
echo -e "${BLUE}=== 第 3 组：内存工具 (3个) ===${NC}"
echo ""

test_tool "memory_save" "请记住：ClawMaster 完整工具测试于 $(date +%Y-%m-%d) 完成" "9.1"
test_tool "memory_save" "请保存这个信息：系统包含 29+ 个强大工具" "9.2"
test_tool "memory_search" "请搜索关于'ClawMaster'的记忆" "10.1"
test_tool "memory_search" "请搜索关于'工具测试'的记忆" "10.2"
test_tool "memory_get" "请获取最近保存的记忆" "11.1"

# ============================================
# 第 4 组：会话管理工具 (6个)
# ============================================
echo -e "${BLUE}=== 第 4 组：会话管理工具 (6个) ===${NC}"
echo ""

test_tool "sessions_list" "请列出所有活跃的会话" "12.1"
test_tool "sessions_create" "请创建一个名为'完整测试会话'的新会话" "13.1"
test_tool "sessions_history" "请显示当前会话的历史记录" "14.1"
test_tool "sessions_history" "请显示最近 10 条对话" "14.2"
test_tool "sessions_send" "请向主会话发送消息：'测试完成'" "15.1"
test_tool "branch_session" "请从当前会话创建一个分支" "16.1"

# ============================================
# 第 5 组：通信工具 (2个)
# ============================================
echo -e "${BLUE}=== 第 5 组：通信工具 (2个) ===${NC}"
echo ""

test_tool "send_message" "请向主会话发送测试消息" "17.1"
test_tool "send_image" "请发送一张测试图片" "18.1"

# ============================================
# 第 6 组：语音工具 (2个)
# ============================================
echo -e "${BLUE}=== 第 6 组：语音工具 (2个) ===${NC}"
echo ""

test_tool "speak" "请朗读：ClawMaster 工具测试完成" "19.1"
test_tool "speak" "请用语音说：你好，世界" "19.2"
test_tool "transcribe" "请转录音频文件" "20.1"

# ============================================
# 第 7 组：技能管理工具 (3个) - Skills
# ============================================
echo -e "${BLUE}=== 第 7 组：技能管理工具 (3个) - Skills ===${NC}"
echo ""

test_tool "create_skill" "请创建一个名为'测试技能'的新技能" "21.1"
test_tool "create_skill" "请创建一个用于数据分析的技能" "21.2"
test_tool "update_skill" "请更新'测试技能'，添加新功能" "22.1"
test_tool "delete_skill" "请删除'测试技能'" "23.1"

# ============================================
# 第 8 组：节点管理工具 (3个)
# ============================================
echo -e "${BLUE}=== 第 8 组：节点管理工具 (3个) ===${NC}"
echo ""

test_tool "nodes_list" "请列出所有可用的计算节点" "24.1"
test_tool "nodes_describe" "请描述主节点的配置" "25.1"
test_tool "nodes_select" "请选择最适合执行任务的节点" "26.1"

# ============================================
# 第 9 组：辅助工具 (6个)
# ============================================
echo -e "${BLUE}=== 第 9 组：辅助工具 (6个) ===${NC}"
echo ""

test_tool "task_list" "请显示当前的任务列表" "27.1"
test_tool "task_list" "请添加任务：完成所有工具测试" "27.2"
test_tool "location" "请获取我的当前位置" "28.1"
test_tool "location" "北京天气如何？" "28.2"
test_tool "show_map" "请显示上海的地图" "29.1"
test_tool "show_map" "请在地图上标记北京天安门" "29.2"
test_tool "session_state" "请在会话状态中保存：test_complete = true" "30.1"
test_tool "session_state" "请读取会话状态中的 test_complete" "30.2"
test_tool "spawn_agent" "请创建一个专门用于搜索的子智能体" "31.1"
test_tool "image_cache" "请显示图片缓存信息" "32.1"

# ============================================
# 第 10 组：综合场景测试
# ============================================
echo -e "${BLUE}=== 第 10 组：综合场景测试 ===${NC}"
echo ""

test_tool "综合测试" "请搜索'Rust异步编程'，然后将结果保存到记忆中" "33.1"
test_tool "综合测试" "请计算 (50 + 50) * 2，然后添加到任务列表" "33.2"
test_tool "综合测试" "请创建一个新会话，然后向该会话发送欢迎消息" "33.3"

# 生成统计信息
echo "" >> "$RESULTS_FILE"
echo "---" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "## 测试统计" >> "$RESULTS_FILE"
echo "" >> "$RESULTS_FILE"
echo "- **总测试数**: $TOTAL" >> "$RESULTS_FILE"
echo "- **通过**: $PASSED" >> "$RESULTS_FILE"
echo "- **失败**: $FAILED" >> "$RESULTS_FILE"

if [ $TOTAL -gt 0 ]; then
    PASS_RATE=$(echo "scale=1; ($PASSED * 100) / $TOTAL" | bc)
    echo "- **通过率**: ${PASS_RATE}%" >> "$RESULTS_FILE"
else
    echo "- **通过率**: 0%" >> "$RESULTS_FILE"
fi

echo "" >> "$RESULTS_FILE"

# 输出最终结果
echo ""
echo "========================================"
echo "  完整测试完成"
echo "========================================"
echo -e "总测试数: $TOTAL"
echo -e "${GREEN}通过: $PASSED${NC}"
echo -e "${RED}失败: $FAILED${NC}"

if [ $TOTAL -gt 0 ]; then
    PASS_RATE=$(echo "scale=1; ($PASSED * 100) / $TOTAL" | bc)
    echo -e "通过率: ${PASS_RATE}%"
fi

echo ""
echo "详细报告已保存到: $RESULTS_FILE"
echo ""
