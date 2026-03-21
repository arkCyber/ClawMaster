#!/bin/bash
# ClawMaster 全面工具测试脚本
# 测试 30+ 种工具，每个工具 3 个应用场景
# 显示完整的测试过程和结果

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color
BOLD='\033[1m'

# 测试统计
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# ClawMaster CLI 命令
CLAWMASTER_CLI="cargo run --bin clawmaster --"

# 日志文件
LOG_FILE="tool_test_results_$(date +%Y%m%d_%H%M%S).log"
DETAILED_LOG="tool_test_detailed_$(date +%Y%m%d_%H%M%S).log"

# 打印标题
print_header() {
    echo ""
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BOLD}${CYAN}  $1${NC}"
    echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
}

# 打印工具测试标题
print_tool_header() {
    local tool_name=$1
    local tool_num=$2
    echo ""
    echo -e "${BOLD}${MAGENTA}┌─────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${BOLD}${MAGENTA}│  工具 #${tool_num}: ${tool_name}${NC}"
    echo -e "${BOLD}${MAGENTA}└─────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
}

# 打印场景标题
print_scenario() {
    local scenario_num=$1
    local scenario_desc=$2
    echo -e "${BOLD}${BLUE}  📋 场景 ${scenario_num}: ${scenario_desc}${NC}"
}

# 打印测试输入
print_test_input() {
    local input=$1
    echo -e "${CYAN}  ➤ 测试输入:${NC} ${input}"
    echo "TEST INPUT: $input" >> "$DETAILED_LOG"
}

# 执行测试
run_test() {
    local test_name=$1
    local test_input=$2
    local expected_pattern=$3
    
    ((TOTAL_TESTS++))
    
    echo -e "${YELLOW}  ⏳ 执行中...${NC}"
    
    # 记录到详细日志
    echo "========================================" >> "$DETAILED_LOG"
    echo "Test: $test_name" >> "$DETAILED_LOG"
    echo "Time: $(date)" >> "$DETAILED_LOG"
    echo "Input: $test_input" >> "$DETAILED_LOG"
    echo "----------------------------------------" >> "$DETAILED_LOG"
    
    # 执行测试（使用 echo 模拟 CLI 输入）
    local output
    local exit_code
    
    # 这里使用 echo 将输入传递给 clawmaster chat 命令
    output=$(echo "$test_input" | timeout 30s $CLAWMASTER_CLI chat --agent default 2>&1 || echo "TIMEOUT_OR_ERROR")
    exit_code=$?
    
    # 记录输出
    echo "$output" >> "$DETAILED_LOG"
    echo "Exit Code: $exit_code" >> "$DETAILED_LOG"
    echo "========================================" >> "$DETAILED_LOG"
    echo "" >> "$DETAILED_LOG"
    
    # 显示输出（截取前 200 字符）
    local short_output=$(echo "$output" | head -c 200)
    echo -e "${CYAN}  ✓ 输出:${NC} ${short_output}..."
    
    # 检查结果
    if echo "$output" | grep -qi "$expected_pattern" || [ "$expected_pattern" = "ANY" ]; then
        echo -e "${GREEN}  ✅ PASS${NC}: $test_name"
        ((PASSED_TESTS++))
        echo "PASS: $test_name" >> "$LOG_FILE"
        return 0
    else
        echo -e "${RED}  ❌ FAIL${NC}: $test_name"
        echo -e "${RED}     预期包含: $expected_pattern${NC}"
        ((FAILED_TESTS++))
        echo "FAIL: $test_name - Expected: $expected_pattern" >> "$LOG_FILE"
        return 1
    fi
}

# 跳过测试
skip_test() {
    local test_name=$1
    local reason=$2
    
    ((TOTAL_TESTS++))
    ((SKIPPED_TESTS++))
    
    echo -e "${YELLOW}  ⏭  SKIP${NC}: $test_name"
    echo -e "${YELLOW}     原因: $reason${NC}"
    echo "SKIP: $test_name - Reason: $reason" >> "$LOG_FILE"
}

# 开始测试
print_header "🧪 ClawMaster 全面工具测试"
echo -e "${BOLD}测试时间:${NC} $(date)"
echo -e "${BOLD}日志文件:${NC} $LOG_FILE"
echo -e "${BOLD}详细日志:${NC} $DETAILED_LOG"
echo ""

# ============================================================================
# 1. exec - 命令执行工具
# ============================================================================
print_tool_header "exec - 命令执行" "1"

print_scenario "1" "列出当前目录文件"
print_test_input "列出当前目录的所有文件"
run_test "exec_ls" "列出当前目录的所有文件" "ls"

print_scenario "2" "查看系统信息"
print_test_input "显示当前系统的操作系统信息"
run_test "exec_uname" "显示当前系统的操作系统信息" "Darwin\|Linux"

print_scenario "3" "检查磁盘使用"
print_test_input "检查磁盘空间使用情况"
run_test "exec_df" "检查磁盘空间使用情况" "Filesystem\|disk"

# ============================================================================
# 2. calc - 计算器工具
# ============================================================================
print_tool_header "calc - 计算器" "2"

print_scenario "1" "简单加法"
print_test_input "计算 123 + 456"
run_test "calc_add" "计算 123 + 456" "579"

print_scenario "2" "复杂表达式"
print_test_input "计算 (100 + 50) * 2 - 30"
run_test "calc_complex" "计算 (100 + 50) * 2 - 30" "270"

print_scenario "3" "科学计算"
print_test_input "计算 2 的 10 次方"
run_test "calc_power" "计算 2 的 10 次方" "1024"

# ============================================================================
# 3. web_search - 网页搜索工具
# ============================================================================
print_tool_header "web_search - 网页搜索" "3"

print_scenario "1" "搜索新闻"
print_test_input "搜索最新的 AI 技术新闻"
run_test "search_ai_news" "搜索最新的 AI 技术新闻" "AI\|artificial intelligence"

print_scenario "2" "搜索技术文档"
print_test_input "搜索 Rust 编程语言官方文档"
run_test "search_rust_docs" "搜索 Rust 编程语言官方文档" "rust\|programming"

print_scenario "3" "搜索开源项目"
print_test_input "搜索 GitHub 上的热门 Rust 项目"
run_test "search_github" "搜索 GitHub 上的热门 Rust 项目" "github\|rust"

# ============================================================================
# 4. web_fetch - 网页获取工具
# ============================================================================
print_tool_header "web_fetch - 网页获取" "4"

print_scenario "1" "获取网页内容"
print_test_input "获取 https://www.rust-lang.org 的内容"
run_test "fetch_rust_org" "获取 https://www.rust-lang.org 的内容" "rust\|programming"

print_scenario "2" "获取 API 数据"
print_test_input "获取 https://api.github.com/repos/rust-lang/rust 的信息"
run_test "fetch_github_api" "获取 https://api.github.com/repos/rust-lang/rust 的信息" "rust\|repository"

print_scenario "3" "获取 JSON 数据"
print_test_input "获取 https://jsonplaceholder.typicode.com/posts/1 的数据"
run_test "fetch_json" "获取 https://jsonplaceholder.typicode.com/posts/1 的数据" "userId\|title"

# ============================================================================
# 5. browser - 浏览器自动化工具
# ============================================================================
print_tool_header "browser - 浏览器自动化" "5"

print_scenario "1" "打开网页"
print_test_input "使用浏览器打开 https://www.rust-lang.org"
skip_test "browser_open" "需要浏览器环境"

print_scenario "2" "截图"
print_test_input "对当前页面进行截图"
skip_test "browser_screenshot" "需要浏览器环境"

print_scenario "3" "点击元素"
print_test_input "点击页面上的 'Get Started' 按钮"
skip_test "browser_click" "需要浏览器环境"

# ============================================================================
# 6. task_list - 任务列表工具
# ============================================================================
print_tool_header "task_list - 任务列表" "6"

print_scenario "1" "添加任务"
print_test_input "添加一个任务：完成代码审计"
run_test "task_add" "添加一个任务：完成代码审计" "task\|added"

print_scenario "2" "列出任务"
print_test_input "列出所有待办任务"
run_test "task_list" "列出所有待办任务" "task\|list"

print_scenario "3" "完成任务"
print_test_input "标记第一个任务为已完成"
run_test "task_complete" "标记第一个任务为已完成" "complete\|done"

# ============================================================================
# 7. sessions_list - 会话列表工具
# ============================================================================
print_tool_header "sessions_list - 会话列表" "7"

print_scenario "1" "列出所有会话"
print_test_input "显示所有活动的会话"
run_test "sessions_list_all" "显示所有活动的会话" "session\|list"

print_scenario "2" "查找特定会话"
print_test_input "查找包含 'test' 的会话"
run_test "sessions_find" "查找包含 'test' 的会话" "session\|test"

print_scenario "3" "统计会话数量"
print_test_input "统计当前有多少个会话"
run_test "sessions_count" "统计当前有多少个会话" "session\|count\|[0-9]"

# ============================================================================
# 8. sessions_history - 会话历史工具
# ============================================================================
print_tool_header "sessions_history - 会话历史" "8"

print_scenario "1" "查看会话历史"
print_test_input "查看当前会话的历史记录"
run_test "history_view" "查看当前会话的历史记录" "history\|message"

print_scenario "2" "搜索历史消息"
print_test_input "在历史记录中搜索关键词 'test'"
run_test "history_search" "在历史记录中搜索关键词 'test'" "test\|search"

print_scenario "3" "导出历史"
print_test_input "导出最近 10 条历史记录"
run_test "history_export" "导出最近 10 条历史记录" "history\|export"

# ============================================================================
# 9. sessions_send - 会话发送工具
# ============================================================================
print_tool_header "sessions_send - 会话发送" "9"

print_scenario "1" "发送消息到其他会话"
print_test_input "发送消息 'Hello' 到会话 'test-session'"
skip_test "sessions_send_msg" "需要目标会话存在"

print_scenario "2" "广播消息"
print_test_input "向所有会话广播 'System update'"
skip_test "sessions_broadcast" "需要多个会话"

print_scenario "3" "发送带附件的消息"
print_test_input "发送文件 test.txt 到会话 'main'"
skip_test "sessions_send_file" "需要文件和目标会话"

# ============================================================================
# 10. process - 进程管理工具
# ============================================================================
print_tool_header "process - 进程管理" "10"

print_scenario "1" "启动进程"
print_test_input "启动一个新的 bash 进程"
skip_test "process_start" "需要 tmux 环境"

print_scenario "2" "查看进程输出"
print_test_input "查看进程的输出"
skip_test "process_output" "需要运行中的进程"

print_scenario "3" "终止进程"
print_test_input "终止进程"
skip_test "process_kill" "需要运行中的进程"

# ============================================================================
# 11. cron - 定时任务工具
# ============================================================================
print_tool_header "cron - 定时任务" "11"

print_scenario "1" "创建定时任务"
print_test_input "创建一个每天早上 9 点运行的任务"
run_test "cron_create" "创建一个每天早上 9 点运行的任务" "cron\|schedule"

print_scenario "2" "列出定时任务"
print_test_input "列出所有定时任务"
run_test "cron_list" "列出所有定时任务" "cron\|list"

print_scenario "3" "删除定时任务"
print_test_input "删除 ID 为 1 的定时任务"
run_test "cron_delete" "删除 ID 为 1 的定时任务" "cron\|delete\|remove"

# ============================================================================
# 12. show_map - 地图显示工具
# ============================================================================
print_tool_header "show_map - 地图显示" "12"

print_scenario "1" "显示位置地图"
print_test_input "显示上海的地图"
run_test "map_shanghai" "显示上海的地图" "map\|shanghai\|location"

print_scenario "2" "显示坐标地图"
print_test_input "显示坐标 31.2304,121.4737 的地图"
run_test "map_coords" "显示坐标 31.2304,121.4737 的地图" "map\|coordinates"

print_scenario "3" "显示路线"
print_test_input "显示从北京到上海的路线"
run_test "map_route" "显示从北京到上海的路线" "map\|route\|beijing\|shanghai"

# ============================================================================
# 13. location - 位置获取工具
# ============================================================================
print_tool_header "location - 位置获取" "13"

print_scenario "1" "获取当前位置"
print_test_input "获取我的当前位置"
skip_test "location_current" "需要位置权限"

print_scenario "2" "获取位置详情"
print_test_input "获取当前位置的详细信息"
skip_test "location_details" "需要位置权限"

print_scenario "3" "位置历史"
print_test_input "查看位置历史记录"
skip_test "location_history" "需要位置权限"

# ============================================================================
# 14. send_image - 图片发送工具
# ============================================================================
print_tool_header "send_image - 图片发送" "14"

print_scenario "1" "发送图片"
print_test_input "发送图片 /path/to/image.png"
skip_test "send_image_file" "需要图片文件"

print_scenario "2" "发送截图"
print_test_input "发送屏幕截图"
skip_test "send_screenshot" "需要截图功能"

print_scenario "3" "发送带说明的图片"
print_test_input "发送图片并添加说明 'Test image'"
skip_test "send_image_caption" "需要图片文件"

# ============================================================================
# 15. image_tool - 图片分析工具
# ============================================================================
print_tool_header "image_tool - 图片分析" "15"

print_scenario "1" "分析图片"
print_test_input "分析图片 /path/to/image.png 的内容"
skip_test "image_analyze" "需要图片文件和 Vision API"

print_scenario "2" "识别图片中的文字"
print_test_input "识别图片中的文字"
skip_test "image_ocr" "需要图片文件和 OCR 功能"

print_scenario "3" "图片分类"
print_test_input "对图片进行分类"
skip_test "image_classify" "需要图片文件和分类模型"

# ============================================================================
# 16. pdf_tool - PDF 工具
# ============================================================================
print_tool_header "pdf_tool - PDF 处理" "16"

print_scenario "1" "读取 PDF"
print_test_input "读取 PDF 文件 /path/to/document.pdf"
skip_test "pdf_read" "需要 PDF 文件"

print_scenario "2" "提取 PDF 文本"
print_test_input "提取 PDF 的文本内容"
skip_test "pdf_extract" "需要 PDF 文件"

print_scenario "3" "PDF 转图片"
print_test_input "将 PDF 转换为图片"
skip_test "pdf_to_image" "需要 PDF 文件"

# ============================================================================
# 17. news_tool - 新闻工具
# ============================================================================
print_tool_header "news_tool - 新闻获取" "17"

print_scenario "1" "获取头条新闻"
print_test_input "获取今天的头条新闻"
run_test "news_headlines" "获取今天的头条新闻" "news\|headline"

print_scenario "2" "搜索特定新闻"
print_test_input "搜索关于 AI 的新闻"
run_test "news_search" "搜索关于 AI 的新闻" "news\|AI"

print_scenario "3" "按类别获取新闻"
print_test_input "获取科技类新闻"
run_test "news_category" "获取科技类新闻" "news\|technology\|tech"

# ============================================================================
# 18. apply_patch - 补丁应用工具
# ============================================================================
print_tool_header "apply_patch - 补丁应用" "18"

print_scenario "1" "应用代码补丁"
print_test_input "应用补丁到文件 test.rs"
skip_test "patch_apply" "需要补丁文件"

print_scenario "2" "预览补丁"
print_test_input "预览补丁内容"
skip_test "patch_preview" "需要补丁文件"

print_scenario "3" "回滚补丁"
print_test_input "回滚最近的补丁"
skip_test "patch_revert" "需要已应用的补丁"

# ============================================================================
# 19. create_skill - 技能创建工具
# ============================================================================
print_tool_header "create_skill - 技能创建" "19"

print_scenario "1" "创建新技能"
print_test_input "创建一个名为 'test-skill' 的新技能"
run_test "skill_create" "创建一个名为 'test-skill' 的新技能" "skill\|create"

print_scenario "2" "创建带描述的技能"
print_test_input "创建技能 'helper' 描述为 'Helper skill'"
run_test "skill_create_desc" "创建技能 'helper' 描述为 'Helper skill'" "skill\|helper"

print_scenario "3" "创建带代码的技能"
print_test_input "创建技能并添加代码"
skip_test "skill_create_code" "需要代码内容"

# ============================================================================
# 20. update_skill - 技能更新工具
# ============================================================================
print_tool_header "update_skill - 技能更新" "20"

print_scenario "1" "更新技能代码"
print_test_input "更新技能 'test-skill' 的代码"
skip_test "skill_update" "需要已存在的技能"

print_scenario "2" "更新技能描述"
print_test_input "更新技能描述"
skip_test "skill_update_desc" "需要已存在的技能"

print_scenario "3" "更新技能配置"
print_test_input "更新技能配置"
skip_test "skill_update_config" "需要已存在的技能"

# ============================================================================
# 21. delete_skill - 技能删除工具
# ============================================================================
print_tool_header "delete_skill - 技能删除" "21"

print_scenario "1" "删除技能"
print_test_input "删除技能 'test-skill'"
skip_test "skill_delete" "需要已存在的技能"

print_scenario "2" "批量删除技能"
print_test_input "删除所有测试技能"
skip_test "skill_delete_batch" "需要多个技能"

print_scenario "3" "强制删除技能"
print_test_input "强制删除技能（忽略依赖）"
skip_test "skill_delete_force" "需要已存在的技能"

# ============================================================================
# 22. spawn_agent - 智能体生成工具
# ============================================================================
print_tool_header "spawn_agent - 智能体生成" "22"

print_scenario "1" "生成新智能体"
print_test_input "生成一个新的智能体"
skip_test "agent_spawn" "需要智能体配置"

print_scenario "2" "生成专用智能体"
print_test_input "生成一个专门处理代码的智能体"
skip_test "agent_spawn_code" "需要智能体配置"

print_scenario "3" "生成临时智能体"
print_test_input "生成一个临时智能体"
skip_test "agent_spawn_temp" "需要智能体配置"

# ============================================================================
# 23. agents_list - 智能体列表工具
# ============================================================================
print_tool_header "agents_list - 智能体列表" "23"

print_scenario "1" "列出所有智能体"
print_test_input "列出所有可用的智能体"
run_test "agents_list_all" "列出所有可用的智能体" "agent\|list"

print_scenario "2" "查找特定智能体"
print_test_input "查找名为 'default' 的智能体"
run_test "agents_find" "查找名为 'default' 的智能体" "agent\|default"

print_scenario "3" "显示智能体详情"
print_test_input "显示智能体的详细信息"
run_test "agents_details" "显示智能体的详细信息" "agent\|detail\|info"

# ============================================================================
# 24. nodes_list - 节点列表工具
# ============================================================================
print_tool_header "nodes_list - 节点列表" "24"

print_scenario "1" "列出所有节点"
print_test_input "列出所有可用的节点"
run_test "nodes_list_all" "列出所有可用的节点" "node\|list"

print_scenario "2" "按状态过滤节点"
print_test_input "列出所有在线的节点"
run_test "nodes_online" "列出所有在线的节点" "node\|online\|active"

print_scenario "3" "按类型过滤节点"
print_test_input "列出所有计算节点"
run_test "nodes_compute" "列出所有计算节点" "node\|compute"

# ============================================================================
# 25. nodes_describe - 节点描述工具
# ============================================================================
print_tool_header "nodes_describe - 节点描述" "25"

print_scenario "1" "描述特定节点"
print_test_input "描述节点 'node-1' 的详细信息"
skip_test "nodes_describe_one" "需要节点存在"

print_scenario "2" "描述节点资源"
print_test_input "描述节点的资源使用情况"
skip_test "nodes_describe_resources" "需要节点存在"

print_scenario "3" "描述节点状态"
print_test_input "描述节点的健康状态"
skip_test "nodes_describe_health" "需要节点存在"

# ============================================================================
# 26. nodes_select - 节点选择工具
# ============================================================================
print_tool_header "nodes_select - 节点选择" "26"

print_scenario "1" "选择最佳节点"
print_test_input "选择性能最好的节点"
skip_test "nodes_select_best" "需要多个节点"

print_scenario "2" "按条件选择节点"
print_test_input "选择 CPU 使用率低于 50% 的节点"
skip_test "nodes_select_cpu" "需要节点监控数据"

print_scenario "3" "选择空闲节点"
print_test_input "选择当前空闲的节点"
skip_test "nodes_select_idle" "需要节点状态数据"

# ============================================================================
# 27. loop_detection - 循环检测工具
# ============================================================================
print_tool_header "loop_detection - 循环检测" "27"

print_scenario "1" "检测循环"
print_test_input "检测是否存在循环调用"
run_test "loop_detect" "检测是否存在循环调用" "loop\|detect"

print_scenario "2" "重置循环检测"
print_test_input "重置循环检测状态"
run_test "loop_reset" "重置循环检测状态" "loop\|reset"

print_scenario "3" "查看循环统计"
print_test_input "查看循环检测统计信息"
run_test "loop_stats" "查看循环检测统计信息" "loop\|stats\|count"

# ============================================================================
# 28. sandbox_packages - 沙箱包管理工具
# ============================================================================
print_tool_header "sandbox_packages - 沙箱包管理" "28"

print_scenario "1" "列出沙箱包"
print_test_input "列出所有可用的沙箱包"
run_test "sandbox_list" "列出所有可用的沙箱包" "sandbox\|package\|list"

print_scenario "2" "按类别列出包"
print_test_input "列出开发工具类的沙箱包"
run_test "sandbox_dev" "列出开发工具类的沙箱包" "sandbox\|dev\|tool"

print_scenario "3" "搜索沙箱包"
print_test_input "搜索包含 'python' 的沙箱包"
run_test "sandbox_search" "搜索包含 'python' 的沙箱包" "sandbox\|python"

# ============================================================================
# 29. gateway_config - 网关配置工具
# ============================================================================
print_tool_header "gateway_config - 网关配置" "29"

print_scenario "1" "查看配置"
print_test_input "查看当前网关配置"
run_test "config_view" "查看当前网关配置" "config\|gateway"

print_scenario "2" "验证配置"
print_test_input "验证配置文件是否正确"
run_test "config_validate" "验证配置文件是否正确" "config\|valid"

print_scenario "3" "重载配置"
print_test_input "重新加载配置文件"
run_test "config_reload" "重新加载配置文件" "config\|reload"

# ============================================================================
# 30. branch_session - 会话分支工具
# ============================================================================
print_tool_header "branch_session - 会话分支" "30"

print_scenario "1" "创建会话分支"
print_test_input "从当前会话创建一个新分支"
skip_test "branch_create" "需要会话上下文"

print_scenario "2" "列出分支"
print_test_input "列出所有会话分支"
skip_test "branch_list" "需要会话分支存在"

print_scenario "3" "切换分支"
print_test_input "切换到分支 'test-branch'"
skip_test "branch_switch" "需要分支存在"

# ============================================================================
# 31. approval - 审批工具
# ============================================================================
print_tool_header "approval - 审批" "31"

print_scenario "1" "请求审批"
print_test_input "请求执行敏感操作的审批"
skip_test "approval_request" "需要审批流程配置"

print_scenario "2" "查看待审批项"
print_test_input "查看所有待审批的请求"
skip_test "approval_pending" "需要待审批项"

print_scenario "3" "批准请求"
print_test_input "批准审批请求 ID 123"
skip_test "approval_approve" "需要待审批项"

# ============================================================================
# 测试总结
# ============================================================================
print_header "📊 测试总结"

echo -e "${BOLD}总测试数:${NC} $TOTAL_TESTS"
echo -e "${GREEN}${BOLD}通过:${NC} $PASSED_TESTS"
echo -e "${RED}${BOLD}失败:${NC} $FAILED_TESTS"
echo -e "${YELLOW}${BOLD}跳过:${NC} $SKIPPED_TESTS"
echo ""

# 计算通过率
if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${BOLD}通过率:${NC} ${PASS_RATE}%"
fi

echo ""
echo -e "${BOLD}详细日志:${NC} $LOG_FILE"
echo -e "${BOLD}完整输出:${NC} $DETAILED_LOG"
echo ""

# 生成 Markdown 报告
REPORT_FILE="TOOL_TEST_REPORT_$(date +%Y%m%d_%H%M%S).md"
cat > "$REPORT_FILE" << EOF
# ClawMaster 工具测试报告

**测试时间**: $(date)  
**测试工具数**: 31 个  
**测试场景数**: 93 个

---

## 📊 测试统计

| 指标 | 数值 |
|------|------|
| **总测试数** | $TOTAL_TESTS |
| **通过** | $PASSED_TESTS |
| **失败** | $FAILED_TESTS |
| **跳过** | $SKIPPED_TESTS |
| **通过率** | ${PASS_RATE}% |

---

## 📝 测试详情

详见日志文件：
- 简要日志: \`$LOG_FILE\`
- 详细日志: \`$DETAILED_LOG\`

---

## 🔧 测试的工具列表

1. **exec** - 命令执行
2. **calc** - 计算器
3. **web_search** - 网页搜索
4. **web_fetch** - 网页获取
5. **browser** - 浏览器自动化
6. **task_list** - 任务列表
7. **sessions_list** - 会话列表
8. **sessions_history** - 会话历史
9. **sessions_send** - 会话发送
10. **process** - 进程管理
11. **cron** - 定时任务
12. **show_map** - 地图显示
13. **location** - 位置获取
14. **send_image** - 图片发送
15. **image_tool** - 图片分析
16. **pdf_tool** - PDF 处理
17. **news_tool** - 新闻获取
18. **apply_patch** - 补丁应用
19. **create_skill** - 技能创建
20. **update_skill** - 技能更新
21. **delete_skill** - 技能删除
22. **spawn_agent** - 智能体生成
23. **agents_list** - 智能体列表
24. **nodes_list** - 节点列表
25. **nodes_describe** - 节点描述
26. **nodes_select** - 节点选择
27. **loop_detection** - 循环检测
28. **sandbox_packages** - 沙箱包管理
29. **gateway_config** - 网关配置
30. **branch_session** - 会话分支
31. **approval** - 审批

---

## ✅ 测试结论

$(if [ $PASS_RATE -ge 80 ]; then
    echo "✅ **测试通过** - 通过率达到 ${PASS_RATE}%"
elif [ $PASS_RATE -ge 60 ]; then
    echo "⚠️ **部分通过** - 通过率为 ${PASS_RATE}%，需要改进"
else
    echo "❌ **测试失败** - 通过率仅为 ${PASS_RATE}%，需要修复"
fi)

---

**生成时间**: $(date)
EOF

echo -e "${GREEN}${BOLD}✅ 测试完成！${NC}"
echo -e "${BOLD}Markdown 报告:${NC} $REPORT_FILE"
echo ""

# 返回退出码
if [ $FAILED_TESTS -eq 0 ]; then
    exit 0
else
    exit 1
fi
