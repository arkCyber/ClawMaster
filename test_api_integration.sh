#!/bin/bash
# API 集成测试 - 实际测试 WebUI 接口

set -e

API_URL="https://localhost:59233"
SESSION_ID="test-$(date +%s)"

echo "=== ClawMaster API 集成测试 ==="
echo "API URL: $API_URL"
echo "Session ID: $SESSION_ID"
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数器
TOTAL=0
PASSED=0
FAILED=0

# 测试函数
test_api() {
    local test_name="$1"
    local message="$2"
    local check_type="$3"
    local expected="$4"
    
    TOTAL=$((TOTAL + 1))
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "测试 #$TOTAL: $test_name"
    echo "输入: $message"
    echo ""
    
    # 发送请求
    response=$(curl -k -s -X POST "$API_URL/api/chat/send" \
        -H "Content-Type: application/json" \
        -d "{
            \"message\": \"$message\",
            \"session_id\": \"$SESSION_ID\",
            \"model\": \"local-llm::mistral-7b-q5_k_m\",
            \"stream\": false
        }" 2>&1)
    
    # 检查响应
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ 请求失败${NC}"
        echo "错误: $response"
        FAILED=$((FAILED + 1))
        echo ""
        return 1
    fi
    
    # 提取响应文本
    response_text=$(echo "$response" | jq -r '.text // .error // "无响应"' 2>/dev/null)
    
    if [ -z "$response_text" ] || [ "$response_text" = "null" ]; then
        echo -e "${RED}❌ 无有效响应${NC}"
        echo "原始响应: $response"
        FAILED=$((FAILED + 1))
        echo ""
        return 1
    fi
    
    echo "响应预览:"
    echo "$response_text" | head -20
    echo ""
    
    # 执行检查
    case "$check_type" in
        "contains")
            if echo "$response_text" | grep -qi "$expected"; then
                echo -e "${GREEN}✅ 测试通过${NC}: 找到预期内容 '$expected'"
                PASSED=$((PASSED + 1))
            else
                echo -e "${RED}❌ 测试失败${NC}: 未找到预期内容 '$expected'"
                FAILED=$((FAILED + 1))
            fi
            ;;
        "not_contains")
            if ! echo "$response_text" | grep -qi "$expected"; then
                echo -e "${GREEN}✅ 测试通过${NC}: 未找到不应出现的内容 '$expected'"
                PASSED=$((PASSED + 1))
            else
                echo -e "${RED}❌ 测试失败${NC}: 找到了不应出现的内容 '$expected'"
                FAILED=$((FAILED + 1))
            fi
            ;;
        *)
            echo -e "${YELLOW}⚠️  未知检查类型: $check_type${NC}"
            ;;
    esac
    
    echo ""
}

# 开始测试
echo "开始执行测试套件..."
echo ""

# 测试1: 中文新闻请求（应该调用工具并返回时间戳）
test_api "中文新闻请求" "美国新闻？" "contains" "时间"

# 测试2: 验证不会出现 "I will call"
test_api "验证无解释性文字" "美国新闻？" "not_contains" "I will call"

# 测试3: 身份问答（不应触发工具）
test_api "身份问答" "你是谁？" "contains" "arkSong"

# 测试4: 验证中文回答
test_api "中文语言匹配" "你好" "contains" "你好"

# 测试5: 科技新闻
test_api "科技新闻请求" "给我搜索科技新闻" "contains" "时间"

# 打印总结
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "测试总结"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "总测试数: $TOTAL"
echo -e "${GREEN}通过: $PASSED${NC}"
echo -e "${RED}失败: $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 所有测试通过！${NC}"
    exit 0
else
    echo -e "${RED}⚠️  有 $FAILED 个测试失败${NC}"
    exit 1
fi
