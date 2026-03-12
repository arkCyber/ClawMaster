#!/bin/bash
set -e

echo "🧪 ClawMaster 完整测试套件"
echo "======================================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数器
PASSED=0
FAILED=0

# 测试函数
run_test() {
    local name="$1"
    local command="$2"
    
    echo -e "${YELLOW}▶ 运行: $name${NC}"
    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ 通过: $name${NC}"
        ((PASSED++))
    else
        echo -e "${RED}❌ 失败: $name${NC}"
        ((FAILED++))
    fi
    echo ""
}

echo "📦 1. 前端资源构建测试"
echo "--------------------------------------"
run_test "Tailwind CSS 构建" "cd crates/web/ui && ./build.sh"

echo "🦀 2. Rust 单元测试"
echo "--------------------------------------"
run_test "配置模块测试" "cargo test -p clawmaster-config --lib --quiet"
run_test "Onboarding 模块测试" "cargo test -p clawmaster-onboarding --lib --quiet"
run_test "会话模块测试" "cargo test -p clawmaster-sessions --lib --quiet"
run_test "聊天模块测试" "cargo test -p clawmaster-chat --lib --quiet"
run_test "项目模块测试" "cargo test -p clawmaster-projects --lib --quiet"

echo "🔍 3. 代码质量检查"
echo "--------------------------------------"
run_test "Rust 格式检查" "cargo +nightly-2025-11-30 fmt --all -- --check"
run_test "JavaScript 格式检查" "cd crates/web/ui && npx biome check ../src/assets/js/*.js"

echo "🏗️ 4. 编译测试"
echo "--------------------------------------"
run_test "Debug 构建" "cargo build --quiet"

echo ""
echo "======================================"
echo "📊 测试结果汇总"
echo "======================================"
echo -e "${GREEN}✅ 通过: $PASSED${NC}"
echo -e "${RED}❌ 失败: $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 所有测试通过！${NC}"
    exit 0
else
    echo -e "${RED}⚠️  有 $FAILED 个测试失败${NC}"
    exit 1
fi
