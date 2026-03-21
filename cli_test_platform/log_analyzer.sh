#!/bin/bash
# ClawMaster CLI 日志分析工具
# 分析测试日志，提取关键信息

set -e

LOG_DIR="./test_logs"
ANALYSIS_REPORT="./log_analysis_$(date +%Y%m%d_%H%M%S).md"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}ClawMaster CLI 日志分析${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

if [ ! -d "$LOG_DIR" ]; then
    echo -e "${RED}错误: 日志目录不存在: ${LOG_DIR}${NC}"
    exit 1
fi

# 初始化报告
cat > "$ANALYSIS_REPORT" << 'EOF'
# ClawMaster CLI 日志分析报告

**分析时间**: $(date)

---

## 分析结果

EOF

# 统计工具调用
echo -e "${YELLOW}分析工具调用...${NC}"
TOOL_CALLS=$(grep -r "executing tool" "$LOG_DIR" | wc -l || echo "0")
NEWS_SEARCH_CALLS=$(grep -r "executing tool tool=news_search" "$LOG_DIR" | wc -l || echo "0")
CALC_CALLS=$(grep -r "executing tool tool=calc" "$LOG_DIR" | wc -l || echo "0")

echo -e "${GREEN}总工具调用: ${TOOL_CALLS}${NC}"
echo -e "${GREEN}news_search: ${NEWS_SEARCH_CALLS}${NC}"
echo -e "${GREEN}calc: ${CALC_CALLS}${NC}"
echo ""

# 统计错误
echo -e "${YELLOW}分析错误信息...${NC}"
ERRORS=$(grep -r "ERROR\|error" "$LOG_DIR" | wc -l || echo "0")
WARNINGS=$(grep -r "WARN\|warn" "$LOG_DIR" | wc -l || echo "0")

echo -e "${RED}错误数: ${ERRORS}${NC}"
echo -e "${YELLOW}警告数: ${WARNINGS}${NC}"
echo ""

# 统计迭代
echo -e "${YELLOW}分析迭代次数...${NC}"
AVG_ITERATIONS=$(grep -r "iteration=" "$LOG_DIR" | awk -F'iteration=' '{print $2}' | awk '{print $1}' | awk '{sum+=$1; count++} END {if(count>0) print int(sum/count); else print 0}')

echo -e "${GREEN}平均迭代次数: ${AVG_ITERATIONS}${NC}"
echo ""

# 提取常见错误
echo -e "${YELLOW}提取常见错误...${NC}"
COMMON_ERRORS=$(grep -r "ERROR\|error" "$LOG_DIR" | cut -d':' -f3- | sort | uniq -c | sort -rn | head -5)

if [ -n "$COMMON_ERRORS" ]; then
    echo -e "${RED}前 5 个常见错误:${NC}"
    echo "$COMMON_ERRORS"
else
    echo -e "${GREEN}未发现错误${NC}"
fi
echo ""

# 写入报告
cat >> "$ANALYSIS_REPORT" << EOF

### 工具调用统计

| 工具 | 调用次数 |
|------|---------|
| news_search | ${NEWS_SEARCH_CALLS} |
| calc | ${CALC_CALLS} |
| **总计** | **${TOOL_CALLS}** |

### 错误统计

| 类型 | 数量 |
|------|------|
| 错误 | ${ERRORS} |
| 警告 | ${WARNINGS} |

### 性能指标

| 指标 | 数值 |
|------|------|
| 平均迭代次数 | ${AVG_ITERATIONS} |

### 常见错误

\`\`\`
${COMMON_ERRORS}
\`\`\`

---

**分析完成时间**: $(date)

EOF

echo -e "${GREEN}日志分析完成${NC}"
echo -e "${GREEN}报告已生成: ${ANALYSIS_REPORT}${NC}"
echo ""
