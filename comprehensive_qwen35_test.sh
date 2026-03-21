#!/bin/bash
# Qwen 3.5 9B 综合工具调用测试
# 测试更多工具和真实应用场景

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m'

GATEWAY_URL="https://localhost:50699"
LOG_DIR="qwen35_comprehensive_test_$(date +%Y%m%d_%H%M%S)"
PASSED=0
FAILED=0
TOTAL=0

mkdir -p "$LOG_DIR"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   Qwen 3.5 9B 综合工具调用测试                           ║${NC}"
echo -e "${CYAN}║   测试更多工具和真实应用场景                             ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}测试时间: $(date)${NC}"
echo -e "${BLUE}日志目录: $LOG_DIR${NC}"
echo ""

# 测试函数
test_scenario() {
    local num=$1
    local category="$2"
    local tool="$3"
    local cmd="$4"
    local description="$5"
    
    ((TOTAL++))
    
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${YELLOW}[${num}/${TOTAL}] ${category} - ${tool}${NC}"
    echo -e "  ${BLUE}场景:${NC} ${description}"
    echo -e "  ${BLUE}命令:${NC} ${cmd}"
    echo ""
    
    local log="$LOG_DIR/${num}_${category}_${tool}.log"
    
    # 运行测试
    if timeout 45 env CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" \
        ./target/release/clawmaster agent --message "$cmd" > "$log" 2>&1; then
        
        local response=$(cat "$log")
        
        # 检查是否有实际响应
        if echo "$response" | grep -q "✅ 响应:"; then
            echo -e "  ${GREEN}✅ 成功${NC}"
            ((PASSED++))
        else
            echo -e "  ${RED}❌ 失败 - 无响应${NC}"
            ((FAILED++))
        fi
    else
        echo -e "  ${RED}❌ 失败 - 超时或错误${NC}"
        ((FAILED++))
    fi
    
    echo ""
}

echo -e "${CYAN}开始综合测试...${NC}"
echo ""

# ==================== 数学和计算 ====================
echo -e "${BLUE}▶ 类别 1: 数学和计算${NC}"
echo ""

test_scenario 1 "数学" "calc" \
    "计算 1234 + 5678" \
    "基础加法运算"

test_scenario 2 "数学" "calc" \
    "计算 999 * 888" \
    "乘法运算"

test_scenario 3 "数学" "calc" \
    "计算 (123 + 456) * 2 - 100" \
    "复杂表达式"

test_scenario 4 "数学" "calc" \
    "计算圆周率乘以10的平方" \
    "数学常数和幂运算"

# ==================== 时间和日期 ====================
echo -e "${BLUE}▶ 类别 2: 时间和日期${NC}"
echo ""

test_scenario 5 "时间" "time" \
    "现在几点了" \
    "当前时间查询"

test_scenario 6 "时间" "time" \
    "今天是星期几" \
    "日期查询"

test_scenario 7 "时间" "time" \
    "告诉我现在的完整日期和时间" \
    "完整时间信息"

# ==================== 文件操作 ====================
echo -e "${BLUE}▶ 类别 3: 文件操作${NC}"
echo ""

test_scenario 8 "文件" "read_file" \
    "读取 README.md 文件的前5行" \
    "文件读取"

test_scenario 9 "文件" "glob" \
    "查找所有的 Rust 源代码文件" \
    "文件查找（.rs）"

test_scenario 10 "文件" "glob" \
    "列出所有的 Markdown 文档" \
    "文件查找（.md）"

test_scenario 11 "文件" "list_dir" \
    "列出 crates 目录下的所有子目录" \
    "目录列表"

test_scenario 12 "文件" "grep" \
    "在 Cargo.toml 中搜索 clawmaster" \
    "文本搜索"

# ==================== Web 搜索 ====================
echo -e "${BLUE}▶ 类别 4: Web 搜索${NC}"
echo ""

test_scenario 13 "搜索" "search_web" \
    "搜索 Rust 编程语言的最新特性" \
    "技术搜索"

test_scenario 14 "搜索" "search_news" \
    "搜索最新的人工智能新闻" \
    "新闻搜索"

test_scenario 15 "搜索" "search_web" \
    "搜索 ClawMaster 项目相关信息" \
    "项目搜索"

test_scenario 16 "搜索" "search_news" \
    "搜索最新的科技新闻" \
    "科技新闻"

# ==================== 内存和知识 ====================
echo -e "${BLUE}▶ 类别 5: 内存和知识${NC}"
echo ""

test_scenario 17 "内存" "memory_save" \
    "记住：我最喜欢的编程语言是 Rust" \
    "保存个人偏好"

test_scenario 18 "内存" "memory_search" \
    "我最喜欢什么编程语言" \
    "检索记忆"

test_scenario 19 "内存" "memory_save" \
    "记住：Qwen 3.5 9B 是一个优秀的中文模型" \
    "保存知识"

# ==================== 任务管理 ====================
echo -e "${BLUE}▶ 类别 6: 任务管理${NC}"
echo ""

test_scenario 20 "任务" "task_list" \
    "列出所有待办任务" \
    "任务列表查询"

test_scenario 21 "任务" "sessions_list" \
    "显示所有聊天会话" \
    "会话列表"

# ==================== 复杂场景 ====================
echo -e "${BLUE}▶ 类别 7: 复杂应用场景${NC}"
echo ""

test_scenario 22 "综合" "multi_tool" \
    "搜索 Rust 最新版本，然后告诉我现在的时间" \
    "多工具组合"

test_scenario 23 "综合" "multi_tool" \
    "计算 100 + 200，然后搜索这个数字的含义" \
    "计算+搜索"

test_scenario 24 "综合" "multi_tool" \
    "读取 Cargo.toml 文件，找出项目名称" \
    "文件读取+信息提取"

test_scenario 25 "综合" "multi_tool" \
    "查找所有 .rs 文件，统计有多少个" \
    "文件查找+统计"

# ==================== 中文理解测试 ====================
echo -e "${BLUE}▶ 类别 8: 中文理解能力${NC}"
echo ""

test_scenario 26 "中文" "understanding" \
    "帮我算一下一千二百三十四加五千六百七十八等于多少" \
    "中文数字理解"

test_scenario 27 "中文" "understanding" \
    "现在是什么时候？用中文告诉我" \
    "中文时间表达"

test_scenario 28 "中文" "understanding" \
    "找一找项目里有哪些 Rust 代码文件" \
    "中文指令理解"

test_scenario 29 "中文" "understanding" \
    "搜一下最近有什么关于机器学习的新闻" \
    "口语化指令"

test_scenario 30 "中文" "understanding" \
    "帮我看看 README 文件里写了什么" \
    "自然语言指令"

# ==================== 测试总结 ====================
echo ""
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   测试完成                                                ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}总测试数: ${TOTAL}${NC}"
echo -e "${GREEN}成功: ${PASSED}${NC}"
echo -e "${RED}失败: ${FAILED}${NC}"

if [ $TOTAL -gt 0 ]; then
    SUCCESS_RATE=$(echo "scale=2; $PASSED * 100 / $TOTAL" | bc)
    echo -e "${YELLOW}成功率: ${SUCCESS_RATE}%${NC}"
fi

echo ""
echo -e "${BLUE}详细日志: $LOG_DIR${NC}"
echo ""

# 生成测试报告
REPORT="$LOG_DIR/TEST_REPORT.md"
cat > "$REPORT" << EOF
# Qwen 3.5 9B 综合测试报告

**测试时间**: $(date)  
**测试场景**: 30 个  
**日志目录**: $LOG_DIR

---

## 📊 测试结果

- **总测试数**: $TOTAL
- **成功**: $PASSED
- **失败**: $FAILED
- **成功率**: ${SUCCESS_RATE}%

---

## 🧪 测试类别

### 1. 数学和计算 (4 个测试)
- 基础运算
- 复杂表达式
- 数学常数

### 2. 时间和日期 (3 个测试)
- 当前时间
- 日期查询
- 完整时间信息

### 3. 文件操作 (5 个测试)
- 文件读取
- 文件查找
- 目录列表
- 文本搜索

### 4. Web 搜索 (4 个测试)
- 技术搜索
- 新闻搜索
- 项目搜索

### 5. 内存和知识 (3 个测试)
- 保存记忆
- 检索记忆
- 知识管理

### 6. 任务管理 (2 个测试)
- 任务列表
- 会话管理

### 7. 复杂场景 (4 个测试)
- 多工具组合
- 信息提取
- 统计分析

### 8. 中文理解 (5 个测试)
- 中文数字
- 口语化指令
- 自然语言

---

## 📝 详细日志

查看 $LOG_DIR 目录下的各个 .log 文件

---

**生成时间**: $(date)
EOF

echo -e "${GREEN}✅ 测试报告已生成: $REPORT${NC}"
echo ""
