#!/bin/bash
# ClawMaster 所有工具和 Skills 全面测试脚本
# 使用自然语言输入测试所有功能

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

REPORT_FILE="./comprehensive_test_report_$(date +%Y%m%d_%H%M%S).md"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}║     ClawMaster 所有工具和 Skills 全面测试                 ║${NC}"
echo -e "${CYAN}║                                                            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 初始化报告
cat > "$REPORT_FILE" << 'EOF'
# ClawMaster 所有工具和 Skills 测试报告

**执行时间**: $(date)

---

## 测试结果

EOF

echo -e "${YELLOW}注意: 本测试需要 WebUI 运行${NC}"
echo -e "${YELLOW}测试将列出所有测试场景，请在 WebUI 中依次输入${NC}"
echo ""

# 测试场景数组
declare -a TEST_SCENARIOS=(
    # news_search 工具测试
    "news_search|中文新闻查询|今天有什么中国新闻？|应调用 news_search，location=China"
    "news_search|英文新闻查询|What's the latest news in USA?|应调用 news_search，location=USA"
    "news_search|科技新闻|给我看看美国的科技新闻|应调用 news_search，category=technology"
    "news_search|体育新闻|Show me sports news from UK|应调用 news_search，category=sports"
    
    # calc 工具测试
    "calc|简单计算|计算 123 + 456|应调用 calc，返回 579"
    "calc|复杂表达式|Calculate (15 + 25) * 3|应调用 calc，返回 120"
    "calc|幂运算|What is 2 to the power of 10?|应调用 calc，返回 1024"
    "calc|除法和模运算|100 divided by 7, remainder?|应调用 calc"
    
    # web_search 工具测试
    "web_search|一般搜索|Search for Rust programming tutorials|应调用 web_search"
    "web_search|中文搜索|搜索 Rust 编程教程|应调用 web_search"
    "web_search|技术问题|How to fix async/await in Rust?|应调用 web_search"
    
    # task_list 工具测试
    "task_list|添加任务|Add a task: Review code changes|应调用 task_list"
    "task_list|列出任务|Show me my tasks|应调用 task_list"
    "task_list|完成任务|Mark task 1 as complete|应调用 task_list"
    
    # sessions 工具测试
    "sessions|列出会话|List all sessions|应调用 sessions_list"
    "sessions|会话历史|Show history of session main|应调用 sessions_history"
    
    # 地图和位置工具测试
    "map|显示地图|Show me a map of Beijing|应调用 show_map"
    "location|获取位置|Where am I?|应调用 location"
    
    # 身份问答（不应调用工具）
    "identity|中文身份|你是谁？|应直接回答，不调用工具"
    "identity|英文身份|What can you do?|应直接回答，不调用工具"
    "identity|能力查询|你能帮我做什么？|应直接回答，不调用工具"
    
    # Skills 功能测试
    "skills|创建技能|Create a new skill called weather_checker|应调用 create_skill"
    "skills|列出技能|List all available skills|应列出 skills"
)

# 统计变量
TOTAL_TESTS=${#TEST_SCENARIOS[@]}
CURRENT_TEST=0

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}测试场景总数: ${TOTAL_TESTS}${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# 显示所有测试场景
echo -e "${GREEN}测试场景列表：${NC}"
echo ""

for scenario in "${TEST_SCENARIOS[@]}"; do
    IFS='|' read -r tool_name test_name input expected <<< "$scenario"
    CURRENT_TEST=$((CURRENT_TEST + 1))
    
    echo -e "${CYAN}[${CURRENT_TEST}/${TOTAL_TESTS}] ${tool_name} - ${test_name}${NC}"
    echo -e "  ${YELLOW}输入:${NC} ${input}"
    echo -e "  ${MAGENTA}预期:${NC} ${expected}"
    echo ""
    
    # 写入报告
    cat >> "$REPORT_FILE" << EOF
### 测试 ${CURRENT_TEST}: ${tool_name} - ${test_name}

**输入**: ${input}  
**预期**: ${expected}  
**状态**: ⏳ 待手动测试

**测试步骤**:
1. 在 WebUI 中输入上述消息
2. 观察后端日志
3. 记录工具调用和结果

---

EOF
done

echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}测试指南${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""
echo -e "${YELLOW}1. 确保 WebUI 正在运行 (https://localhost:59233)${NC}"
echo -e "${YELLOW}2. 在 WebUI 中依次输入上述测试消息${NC}"
echo -e "${YELLOW}3. 观察后端日志输出${NC}"
echo -e "${YELLOW}4. 记录以下信息：${NC}"
echo "   - 是否调用了工具"
echo "   - 调用的工具名称"
echo "   - 提取的参数"
echo "   - 返回的结果"
echo "   - 任何错误信息"
echo ""
echo -e "${YELLOW}5. 分析结果并识别需要补全的代码${NC}"
echo ""

# 写入报告摘要
cat >> "$REPORT_FILE" << 'EOF'

## 测试摘要

| 工具类别 | 测试数 | 通过 | 失败 | 待测试 |
|---------|--------|------|------|--------|
| news_search | 4 | - | - | 4 |
| calc | 4 | - | - | 4 |
| web_search | 3 | - | - | 3 |
| task_list | 3 | - | - | 3 |
| sessions | 2 | - | - | 2 |
| map/location | 2 | - | - | 2 |
| identity | 3 | - | - | 3 |
| skills | 2 | - | - | 2 |
| **总计** | **23** | **-** | **-** | **23** |

---

## 需要补全的代码

（测试完成后填写）

---

**报告生成时间**: $(date)

EOF

echo -e "${GREEN}测试场景已列出${NC}"
echo -e "${GREEN}报告已生成: ${REPORT_FILE}${NC}"
echo ""
echo -e "${CYAN}按 Enter 键查看详细测试指南...${NC}"
read

# 显示详细指南
cat << 'GUIDE'

╔════════════════════════════════════════════════════════════╗
║              详细测试指南                                  ║
╚════════════════════════════════════════════════════════════╝

1. 工具调用验证
   ✓ 检查日志中是否有 "executing tool tool=<工具名>"
   ✓ 验证工具名称是否正确

2. 参数提取验证
   ✓ 检查日志中的 args={...}
   ✓ 验证参数是否正确提取

3. 结果分析
   ✓ 检查工具是否返回了结果
   ✓ 验证结果格式是否正确
   ✓ 检查是否有错误信息

4. 性能指标
   ✓ 记录迭代次数
   ✓ 记录响应时间
   ✓ 记录 Token 使用

5. 需要补全的代码
   ✓ 缺失的日志
   ✓ 错误处理
   ✓ 参数验证
   ✓ 结果格式化

6. 日志关键词
   - "executing tool" - 工具调用
   - "Successfully parsed" - 解析成功
   - "ERROR" / "error" - 错误
   - "WARN" / "warn" - 警告
   - "iteration=" - 迭代次数
   - "input_tokens=" - 输入 tokens
   - "output_tokens=" - 输出 tokens

GUIDE

echo ""
echo -e "${GREEN}开始测试！${NC}"
echo ""
