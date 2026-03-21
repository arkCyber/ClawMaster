#!/bin/bash
# ClawMaster 所有 33 个工具完整测试脚本
# 每个工具 3 个场景，共 99 个测试

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

REPORT_FILE="./all_tools_test_report_$(date +%Y%m%d_%H%M%S).md"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}║     ClawMaster 所有工具完整测试（99个场景）                ║${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}工具总数: 33${NC}"
echo -e "${BLUE}测试场景: 99 (每个工具 3 个场景)${NC}"
echo ""

# 初始化报告
cat > "$REPORT_FILE" << 'EOF'
# ClawMaster 所有工具测试报告

**执行时间**: $(date)  
**工具总数**: 33  
**测试场景**: 99

---

## 测试结果

EOF

# 测试场景数组（99个）
declare -a TEST_SCENARIOS=(
    # 1. news_search (3)
    "news_search|中文新闻|今天有什么中国新闻？|调用 news_search，location=China"
    "news_search|英文新闻|What's the latest news in USA?|调用 news_search，location=USA"
    "news_search|科技新闻|给我看看美国的科技新闻|调用 news_search，category=technology"
    
    # 2. calc (3)
    "calc|简单算术|计算 123 + 456|调用 calc，返回 579"
    "calc|复杂表达式|Calculate (15 + 25) * 3|调用 calc，返回 120"
    "calc|幂运算|What is 2 to the power of 10?|调用 calc，返回 1024"
    
    # 3. web_search (3)
    "web_search|技术搜索|Search for Rust programming tutorials|调用 web_search"
    "web_search|中文搜索|搜索 Rust 编程教程|调用 web_search"
    "web_search|问题搜索|How to fix async/await in Rust?|调用 web_search"
    
    # 4. web_fetch (3)
    "web_fetch|获取网页|Fetch content from https://www.rust-lang.org|调用 web_fetch"
    "web_fetch|获取API|Get data from https://api.github.com/repos/rust-lang/rust|调用 web_fetch"
    "web_fetch|获取JSON|Fetch JSON from https://jsonplaceholder.typicode.com/posts/1|调用 web_fetch"
    
    # 5. browser (3)
    "browser|打开网页|Open https://www.rust-lang.org in browser|调用 browser"
    "browser|截图|Take a screenshot of https://github.com|调用 browser"
    "browser|提取内容|Extract text from https://www.rust-lang.org|调用 browser"
    
    # 6. exec (3)
    "exec|执行命令|Run 'ls -la' command|调用 exec"
    "exec|系统信息|What's my system information?|调用 exec"
    "exec|查看文件|Show me the contents of README.md|调用 exec"
    
    # 7. process (3)
    "process|启动进程|Start a new process for monitoring|调用 process"
    "process|列出进程|List all running processes|调用 process"
    "process|停止进程|Stop process with ID 1234|调用 process"
    
    # 8. task_list (3)
    "task_list|添加任务|Add a task: Review code changes|调用 task_list"
    "task_list|列出任务|Show me my tasks|调用 task_list"
    "task_list|完成任务|Mark task 1 as complete|调用 task_list"
    
    # 9. sessions_list (3)
    "sessions_list|列出会话|List all sessions|调用 sessions_list"
    "sessions_list|活跃会话|Show me active sessions|调用 sessions_list"
    "sessions_list|查找会话|Find session named 'main'|调用 sessions_list"
    
    # 10. sessions_history (3)
    "sessions_history|查看历史|Show history of session main|调用 sessions_history"
    "sessions_history|最近消息|Show me the last 10 messages in session main|调用 sessions_history"
    "sessions_history|搜索历史|Search for 'news' in session main history|调用 sessions_history"
    
    # 11. sessions_send (3)
    "sessions_send|发送消息|Send 'Hello' to session test|调用 sessions_send"
    "sessions_send|发送通知|Notify session main about the update|调用 sessions_send"
    "sessions_send|广播消息|Broadcast 'System maintenance' to all sessions|调用 sessions_send"
    
    # 12. sessions_create (3)
    "sessions_create|创建会话|Create a new session named 'test'|调用 sessions_create"
    "sessions_create|临时会话|Create a temporary session for debugging|调用 sessions_create"
    "sessions_create|配置会话|Create session 'dev' with debug mode enabled|调用 sessions_create"
    
    # 13. sessions_delete (3)
    "sessions_delete|删除会话|Delete session named 'test'|调用 sessions_delete"
    "sessions_delete|清理会话|Delete all inactive sessions|调用 sessions_delete"
    "sessions_delete|删除临时|Remove temporary sessions|调用 sessions_delete"
    
    # 14. spawn_agent (3)
    "spawn_agent|生成代理|Spawn a new agent for monitoring|调用 spawn_agent"
    "spawn_agent|专用代理|Create an agent specialized in news gathering|调用 spawn_agent"
    "spawn_agent|临时代理|Spawn a temporary agent for this task|调用 spawn_agent"
    
    # 15. show_map (3)
    "show_map|城市地图|Show me a map of Beijing|调用 show_map"
    "show_map|地区地图|Display a map of Silicon Valley|调用 show_map"
    "show_map|路线地图|Show route from Beijing to Shanghai|调用 show_map"
    
    # 16. get_user_location (3)
    "get_user_location|当前位置|Where am I?|调用 get_user_location"
    "get_user_location|位置详情|What's my current location with details?|调用 get_user_location"
    "get_user_location|GPS坐标|Give me my GPS coordinates|调用 get_user_location"
    
    # 17. send_image (3)
    "send_image|发送图片|Send image.png to the chat|调用 send_image"
    "send_image|发送截图|Send a screenshot of the current screen|调用 send_image"
    "send_image|发送图表|Send the chart as an image|调用 send_image"
    
    # 18. image (3)
    "image|生成图片|Generate an image of a sunset|调用 image"
    "image|创建图表|Create a bar chart showing sales data|调用 image"
    "image|生成图标|Generate an icon for the app|调用 image"
    
    # 19. sandbox_packages (3)
    "sandbox_packages|列出包|List all sandbox packages|调用 sandbox_packages"
    "sandbox_packages|搜索包|Find packages related to Python|调用 sandbox_packages"
    "sandbox_packages|包详情|Show details of package 'rust'|调用 sandbox_packages"
    
    # 20. nodes_list (3)
    "nodes_list|列出节点|List all nodes|调用 nodes_list"
    "nodes_list|活跃节点|Show active nodes|调用 nodes_list"
    "nodes_list|按类型列出|List compute nodes|调用 nodes_list"
    
    # 21. nodes_describe (3)
    "nodes_describe|描述节点|Describe node 'node1'|调用 nodes_describe"
    "nodes_describe|节点状态|What's the status of node 'node1'?|调用 nodes_describe"
    "nodes_describe|节点配置|Show configuration of node 'node1'|调用 nodes_describe"
    
    # 22. nodes_select (3)
    "nodes_select|选择节点|Select node 'node1'|调用 nodes_select"
    "nodes_select|最佳节点|Select the best node for computation|调用 nodes_select"
    "nodes_select|空闲节点|Select an idle node|调用 nodes_select"
    
    # 23. loop_detection (3)
    "loop_detection|检测循环|Check for loops in the current process|调用 loop_detection"
    "loop_detection|分析循环|Analyze loop patterns|调用 loop_detection"
    "loop_detection|报告循环|Report any detected loops|调用 loop_detection"
    
    # 24. create_skill (3)
    "create_skill|创建技能|Create a new skill called 'weather_checker'|调用 create_skill"
    "create_skill|数据技能|Create a skill for data analysis|调用 create_skill"
    "create_skill|自动化技能|Create an automation skill|调用 create_skill"
    
    # 25. update_skill (3)
    "update_skill|更新技能|Update skill 'weather_checker'|调用 update_skill"
    "update_skill|修改配置|Modify configuration of skill 'data_analyzer'|调用 update_skill"
    "update_skill|升级技能|Upgrade skill to latest version|调用 update_skill"
    
    # 26. delete_skill (3)
    "delete_skill|删除技能|Delete skill 'old_skill'|调用 delete_skill"
    "delete_skill|移除未使用|Remove unused skills|调用 delete_skill"
    "delete_skill|清理技能|Clean up old skills|调用 delete_skill"
    
    # 27. cron (3)
    "cron|创建定时|Schedule a task to run every day at 9am|调用 cron"
    "cron|列出定时|Show all scheduled tasks|调用 cron"
    "cron|删除定时|Remove the daily backup task|调用 cron"
    
    # 28. apply_patch (3)
    "apply_patch|应用补丁|Apply patch file 'fix.patch'|调用 apply_patch"
    "apply_patch|代码更新|Apply the code update patch|调用 apply_patch"
    "apply_patch|安全补丁|Apply security patch|调用 apply_patch"
    
    # 29. branch_session (3)
    "branch_session|创建分支|Branch current session|调用 branch_session"
    "branch_session|实验分支|Create an experimental branch of this session|调用 branch_session"
    "branch_session|新上下文|Branch session to new context|调用 branch_session"
    
    # 30. session_state (3)
    "session_state|查看状态|Show session state|调用 session_state"
    "session_state|检查健康|Check session health|调用 session_state"
    "session_state|会话信息|Get current session information|调用 session_state"
    
    # 31. gateway (3)
    "gateway|查看配置|Show gateway configuration|调用 gateway"
    "gateway|更新设置|Update gateway settings|调用 gateway"
    "gateway|检查状态|Check gateway status|调用 gateway"
    
    # 32. agents_list (3)
    "agents_list|列出代理|List all agents|调用 agents_list"
    "agents_list|活跃代理|Show active agents|调用 agents_list"
    "agents_list|查找代理|Find agent named 'monitor'|调用 agents_list"
    
    # 33. pdf (3)
    "pdf|读取PDF|Read content from document.pdf|调用 pdf"
    "pdf|提取文本|Extract text from report.pdf|调用 pdf"
    "pdf|分析PDF|Analyze the PDF document|调用 pdf"
)

TOTAL_TESTS=${#TEST_SCENARIOS[@]}

echo -e "${GREEN}测试场景总数: ${TOTAL_TESTS}${NC}"
echo ""
echo -e "${YELLOW}请在 WebUI 中依次输入以下测试消息...${NC}"
echo ""

# 显示所有测试
for i in "${!TEST_SCENARIOS[@]}"; do
    IFS='|' read -r tool_name test_name input expected <<< "${TEST_SCENARIOS[$i]}"
    
    echo -e "${CYAN}[$(($i+1))/${TOTAL_TESTS}] ${tool_name} - ${test_name}${NC}"
    echo -e "  ${GREEN}输入:${NC} ${input}"
    echo -e "  ${MAGENTA}预期:${NC} ${expected}"
    echo ""
    
    # 写入报告
    cat >> "$REPORT_FILE" << EOF
### 测试 $(($i+1)): ${tool_name} - ${test_name}

**输入**: ${input}  
**预期**: ${expected}  
**状态**: ⏳ 待测试

---

EOF
done

# 写入摘要
cat >> "$REPORT_FILE" << EOF

## 测试摘要

**总测试数**: ${TOTAL_TESTS}  
**工具数**: 33  
**每工具场景数**: 3

---

**报告生成时间**: $(date)

EOF

echo -e "${GREEN}测试场景已列出${NC}"
echo -e "${GREEN}报告已生成: ${REPORT_FILE}${NC}"
echo ""
