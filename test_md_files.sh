#!/bin/bash
# 测试 .md 文件功能的脚本

set -e

echo "🧪 测试 ClawMaster .md 文件功能"
echo "================================"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 测试计数
TESTS_PASSED=0
TESTS_FAILED=0

# 测试函数
test_file_exists() {
    local file=$1
    local description=$2
    
    if [ -f "$file" ]; then
        echo -e "${GREEN}✅ PASS${NC}: $description"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}: $description"
        echo "   文件不存在: $file"
        ((TESTS_FAILED++))
        return 1
    fi
}

test_file_not_empty() {
    local file=$1
    local description=$2
    
    if [ -s "$file" ]; then
        echo -e "${GREEN}✅ PASS${NC}: $description"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}: $description"
        echo "   文件为空: $file"
        ((TESTS_FAILED++))
        return 1
    fi
}

test_file_contains() {
    local file=$1
    local pattern=$2
    local description=$3
    
    if grep -q "$pattern" "$file"; then
        echo -e "${GREEN}✅ PASS${NC}: $description"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}❌ FAIL${NC}: $description"
        echo "   未找到模式: $pattern"
        echo "   文件: $file"
        ((TESTS_FAILED++))
        return 1
    fi
}

# 1. 测试文件存在性
echo "📁 测试 1: 检查 .md 文件是否存在"
echo "-----------------------------------"
test_file_exists "$HOME/.clawmaster/AGENTS.md" "AGENTS.md 存在"
test_file_exists "$HOME/.clawmaster/HEARTBEAT.md" "HEARTBEAT.md 存在"
test_file_exists "$HOME/.clawmaster/USER.md" "USER.md 存在"
test_file_exists "$HOME/.clawmaster/SOUL.md" "SOUL.md 存在"
test_file_exists "$HOME/.clawmaster/TOOLS.md" "TOOLS.md 存在"
test_file_exists "$HOME/.clawmaster/MEMORY.md" "MEMORY.md 存在"
echo ""

# 2. 测试文件内容
echo "📝 测试 2: 检查文件内容是否完整"
echo "-----------------------------------"
test_file_not_empty "$HOME/.clawmaster/AGENTS.md" "AGENTS.md 不为空"
test_file_not_empty "$HOME/.clawmaster/HEARTBEAT.md" "HEARTBEAT.md 不为空"
test_file_not_empty "$HOME/.clawmaster/USER.md" "USER.md 不为空"
test_file_not_empty "$HOME/.clawmaster/SOUL.md" "SOUL.md 不为空"
test_file_not_empty "$HOME/.clawmaster/TOOLS.md" "TOOLS.md 不为空"
echo ""

# 3. 测试 AGENTS.md 关键内容
echo "🔍 测试 3: 验证 AGENTS.md 关键内容"
echo "-----------------------------------"
test_file_contains "$HOME/.clawmaster/AGENTS.md" "Core Principles" "包含核心原则"
test_file_contains "$HOME/.clawmaster/AGENTS.md" "Code Quality Standards" "包含代码质量标准"
test_file_contains "$HOME/.clawmaster/AGENTS.md" "DO-178C Level A" "包含航空航天标准"
test_file_contains "$HOME/.clawmaster/AGENTS.md" "Security & Privacy" "包含安全规则"
test_file_contains "$HOME/.clawmaster/AGENTS.md" "unwrap()" "包含 unwrap 规则"
echo ""

# 4. 测试 HEARTBEAT.md 关键内容
echo "💓 测试 4: 验证 HEARTBEAT.md 关键内容"
echo "-----------------------------------"
test_file_contains "$HOME/.clawmaster/HEARTBEAT.md" "Priority 1: Critical" "包含优先级 1"
test_file_contains "$HOME/.clawmaster/HEARTBEAT.md" "Priority 2: Important" "包含优先级 2"
test_file_contains "$HOME/.clawmaster/HEARTBEAT.md" "Priority 3: Informational" "包含优先级 3"
test_file_contains "$HOME/.clawmaster/HEARTBEAT.md" "System Health" "包含系统健康检查"
test_file_contains "$HOME/.clawmaster/HEARTBEAT.md" "Security" "包含安全检查"
echo ""

# 5. 测试 USER.md 关键内容
echo "👤 测试 5: 验证 USER.md 关键内容"
echo "-----------------------------------"
test_file_contains "$HOME/.clawmaster/USER.md" "arkSong" "包含用户名"
test_file_contains "$HOME/.clawmaster/USER.md" "Asia/Shanghai" "包含时区"
test_file_contains "$HOME/.clawmaster/USER.md" "Communication Preferences" "包含通信偏好"
test_file_contains "$HOME/.clawmaster/USER.md" "Work Preferences" "包含工作偏好"
test_file_contains "$HOME/.clawmaster/USER.md" "Technical Preferences" "包含技术偏好"
echo ""

# 6. 测试 SOUL.md 关键内容
echo "🌟 测试 6: 验证 SOUL.md 关键内容"
echo "-----------------------------------"
test_file_contains "$HOME/.clawmaster/SOUL.md" "genuinely helpful" "包含核心价值观"
test_file_contains "$HOME/.clawmaster/SOUL.md" "Have opinions" "包含个性化"
test_file_contains "$HOME/.clawmaster/SOUL.md" "Boundaries" "包含边界规则"
echo ""

# 7. 测试 TOOLS.md 关键内容
echo "🔧 测试 7: 验证 TOOLS.md 关键内容"
echo "-----------------------------------"
test_file_contains "$HOME/.clawmaster/TOOLS.md" "Do not narrate" "包含默认行为"
test_file_contains "$HOME/.clawmaster/TOOLS.md" "When to Narrate" "包含叙述规则"
test_file_contains "$HOME/.clawmaster/TOOLS.md" "Examples" "包含示例"
echo ""

# 8. 测试代码中的支持
echo "💻 测试 8: 验证代码支持"
echo "-----------------------------------"
if grep -q "AGENTS.md" crates/agents/src/prompt.rs; then
    echo -e "${GREEN}✅ PASS${NC}: prompt.rs 支持 AGENTS.md"
    ((TESTS_PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}: prompt.rs 不支持 AGENTS.md"
    ((TESTS_FAILED++))
fi

if grep -q "WORKSPACE_FILE_MAX_CHARS" crates/agents/src/prompt.rs; then
    echo -e "${GREEN}✅ PASS${NC}: 有文件大小限制配置"
    ((TESTS_PASSED++))
else
    echo -e "${RED}❌ FAIL${NC}: 缺少文件大小限制配置"
    ((TESTS_FAILED++))
fi

if grep -q "USER.md" crates/agents/src/prompt.rs; then
    echo -e "${GREEN}✅ PASS${NC}: prompt.rs 支持 USER.md"
    ((TESTS_PASSED++))
else
    echo -e "${YELLOW}⚠️  WARN${NC}: prompt.rs 可能不支持 USER.md（通过 UserProfile）"
fi
echo ""

# 9. 测试文件大小
echo "📏 测试 9: 检查文件大小"
echo "-----------------------------------"
for file in AGENTS.md HEARTBEAT.md USER.md SOUL.md TOOLS.md; do
    filepath="$HOME/.clawmaster/$file"
    if [ -f "$filepath" ]; then
        size=$(wc -c < "$filepath")
        if [ $size -gt 100 ]; then
            echo -e "${GREEN}✅ PASS${NC}: $file 大小合理 (${size} bytes)"
            ((TESTS_PASSED++))
        else
            echo -e "${YELLOW}⚠️  WARN${NC}: $file 可能太小 (${size} bytes)"
        fi
    fi
done
echo ""

# 10. 测试文件版本信息
echo "📌 测试 10: 检查版本信息"
echo "-----------------------------------"
for file in AGENTS.md HEARTBEAT.md USER.md; do
    filepath="$HOME/.clawmaster/$file"
    if [ -f "$filepath" ]; then
        if grep -q "Version: 2.0" "$filepath"; then
            echo -e "${GREEN}✅ PASS${NC}: $file 包含版本信息"
            ((TESTS_PASSED++))
        else
            echo -e "${YELLOW}⚠️  WARN${NC}: $file 缺少版本信息"
        fi
    fi
done
echo ""

# 总结
echo "================================"
echo "📊 测试总结"
echo "================================"
echo -e "通过: ${GREEN}${TESTS_PASSED}${NC}"
echo -e "失败: ${RED}${TESTS_FAILED}${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 所有测试通过！${NC}"
    exit 0
else
    echo -e "${RED}❌ 有 ${TESTS_FAILED} 个测试失败${NC}"
    exit 1
fi
