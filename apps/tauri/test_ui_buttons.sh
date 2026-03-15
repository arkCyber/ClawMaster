#!/bin/bash
# Tauri UI 按钮功能自动化测试脚本

set -e

echo "=========================================="
echo "Tauri UI 按钮功能测试"
echo "=========================================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数器
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# 测试函数
test_api() {
    local name=$1
    local url=$2
    local method=${3:-GET}
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -n "测试 $name ... "
    
    if [ "$method" = "GET" ]; then
        response=$(curl -k -s -w "\n%{http_code}" "$url" 2>/dev/null)
    else
        response=$(curl -k -s -w "\n%{http_code}" -X POST "$url" 2>/dev/null)
    fi
    
    http_code=$(echo "$response" | tail -n1)
    
    if [ "$http_code" = "200" ] || [ "$http_code" = "302" ]; then
        echo -e "${GREEN}✓ PASS${NC} (HTTP $http_code)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC} (HTTP $http_code)"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

echo "1. 后端服务检查"
echo "----------------------------------------"

# 检查后端服务是否运行
if lsof -i :59233 > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} 后端服务运行中 (端口 59233)"
else
    echo -e "${RED}✗${NC} 后端服务未运行"
    echo "请先启动后端服务: clawmaster gateway"
    exit 1
fi

echo ""
echo "2. API 端点测试"
echo "----------------------------------------"

# 测试所有 API 端点
test_api "连接状态 (GON)" "https://localhost:59233/api/gon"
test_api "会话列表" "https://localhost:59233/api/sessions"
test_api "模型列表" "https://localhost:59233/api/models"
test_api "提供商列表" "https://localhost:59233/api/providers"
test_api "Dashboard 页面" "https://localhost:59233/dashboard"
test_api "Settings 页面" "https://localhost:59233/settings"

echo ""
echo "3. 功能端点测试"
echo "----------------------------------------"

# 测试紧急停止
test_api "紧急停止" "https://localhost:59233/api/emergency-stop" "POST"

echo ""
echo "4. 前端资源检查"
echo "----------------------------------------"

# 检查 HTML 文件
if [ -f "/Users/arksong/ClawMaster/apps/tauri/dist/index.html" ]; then
    echo -e "${GREEN}✓${NC} index.html 存在"
    PASSED_TESTS=$((PASSED_TESTS + 1))
else
    echo -e "${RED}✗${NC} index.html 不存在"
    FAILED_TESTS=$((FAILED_TESTS + 1))
fi
TOTAL_TESTS=$((TOTAL_TESTS + 1))

echo ""
echo "5. JavaScript 代码检查"
echo "----------------------------------------"

# 检查关键函数是否存在
html_file="/Users/arksong/ClawMaster/apps/tauri/dist/index.html"

check_function() {
    local func_name=$1
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -n "检查函数 $func_name ... "
    
    if grep -q "function $func_name" "$html_file" || grep -q "async function $func_name" "$html_file"; then
        echo -e "${GREEN}✓ PASS${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

check_function "checkConnection"
check_function "loadSessions"
check_function "loadModels"
check_function "sendMessage"
check_function "emergencyStop"
check_function "createNewSession"
check_function "clearChat"
check_function "exportChat"
check_function "openDashboard"
check_function "openSettings"
check_function "setLanguage"
check_function "setTheme"

echo ""
echo "6. 事件绑定检查"
echo "----------------------------------------"

check_event_binding() {
    local element_id=$1
    local event_type=$2
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo -n "检查 $element_id 的 $event_type 事件 ... "
    
    if grep -q "getElementById('$element_id').addEventListener('$event_type'" "$html_file"; then
        echo -e "${GREEN}✓ PASS${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
        return 0
    else
        echo -e "${RED}✗ FAIL${NC}"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return 1
    fi
}

check_event_binding "emergencyStopBtn" "click"
check_event_binding "dashboardBtn" "click"
check_event_binding "settingsBtn" "click"
check_event_binding "languageBtn" "click"
check_event_binding "newSessionBtn" "click"
check_event_binding "sendBtn" "click"
check_event_binding "modelSelector" "click"
check_event_binding "searchInput" "input"

echo ""
echo "=========================================="
echo "测试结果汇总"
echo "=========================================="
echo "总测试数: $TOTAL_TESTS"
echo -e "通过: ${GREEN}$PASSED_TESTS${NC}"
echo -e "失败: ${RED}$FAILED_TESTS${NC}"

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "\n${GREEN}✓ 所有测试通过！${NC}"
    exit 0
else
    echo -e "\n${RED}✗ 有 $FAILED_TESTS 个测试失败${NC}"
    exit 1
fi
