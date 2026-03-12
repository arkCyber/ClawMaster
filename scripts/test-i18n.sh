#!/bin/bash
# Script to verify i18n implementation

set -e

echo "🧪 ClawMaster 多语言功能验证脚本"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check functions
check_file() {
    if [ -f "$1" ]; then
        echo -e "${GREEN}✅${NC} $2"
        return 0
    else
        echo -e "${RED}❌${NC} $2 - 文件不存在: $1"
        return 1
    fi
}

check_dir() {
    if [ -d "$1" ]; then
        echo -e "${GREEN}✅${NC} $2"
        return 0
    else
        echo -e "${RED}❌${NC} $2 - 目录不存在: $1"
        return 1
    fi
}

# Base directory
BASE_DIR="/Users/arksong/ClawMaster"
LOCALES_DIR="$BASE_DIR/crates/web/src/assets/js/locales"

echo "📁 检查核心文件..."
echo ""

# Check core files
check_file "$BASE_DIR/crates/web/src/assets/js/i18n.js" "i18n 核心模块"
check_file "$BASE_DIR/crates/web/src/assets/js/language-selector.js" "语言选择器组件"
check_file "$BASE_DIR/crates/web/src/assets/css/language-selector.css" "语言选择器样式"
check_file "$BASE_DIR/crates/web/src/templates/index.html" "主 HTML 模板"
check_file "$BASE_DIR/crates/web/src/assets/js/app.js" "应用入口文件"

echo ""
echo "🌐 检查语言目录..."
echo ""

# Check language directories
LANGUAGES=("en" "fr" "zh" "es" "de" "ja" "ko" "ru" "pt" "it" "ar" "hi" "tr" "nl" "pl" "vi")
MISSING_LANGS=()

for lang in "${LANGUAGES[@]}"; do
    if check_dir "$LOCALES_DIR/$lang" "语言目录: $lang"; then
        # Count files in directory
        file_count=$(ls -1 "$LOCALES_DIR/$lang" | wc -l | tr -d ' ')
        if [ "$file_count" -eq 18 ]; then
            echo "   └─ 包含 $file_count 个翻译文件 ✓"
        else
            echo -e "   └─ ${YELLOW}⚠️${NC} 包含 $file_count 个翻译文件（应为 18 个）"
        fi
    else
        MISSING_LANGS+=("$lang")
    fi
done

echo ""
echo "📊 统计信息..."
echo ""

# Count total files
total_langs=${#LANGUAGES[@]}
existing_langs=$((total_langs - ${#MISSING_LANGS[@]}))
total_files=$(find "$LOCALES_DIR" -name "*.js" | wc -l | tr -d ' ')

echo "支持的语言数: $existing_langs / $total_langs"
echo "翻译文件总数: $total_files"

echo ""
echo "🔍 检查代码集成..."
echo ""

# Check if language selector is imported in app.js
if grep -q "LanguageSelector" "$BASE_DIR/crates/web/src/assets/js/app.js"; then
    echo -e "${GREEN}✅${NC} app.js 导入了 LanguageSelector"
else
    echo -e "${RED}❌${NC} app.js 未导入 LanguageSelector"
fi

# Check if language selector is rendered
if grep -q "languageSelectorContainer" "$BASE_DIR/crates/web/src/assets/js/app.js"; then
    echo -e "${GREEN}✅${NC} app.js 渲染了语言选择器"
else
    echo -e "${RED}❌${NC} app.js 未渲染语言选择器"
fi

# Check if HTML has the container
if grep -q "languageSelectorContainer" "$BASE_DIR/crates/web/src/templates/index.html"; then
    echo -e "${GREEN}✅${NC} index.html 包含语言选择器容器"
else
    echo -e "${RED}❌${NC} index.html 未包含语言选择器容器"
fi

# Check if CSS is linked
if grep -q "language-selector.css" "$BASE_DIR/crates/web/src/templates/index.html"; then
    echo -e "${GREEN}✅${NC} index.html 引用了语言选择器样式"
else
    echo -e "${RED}❌${NC} index.html 未引用语言选择器样式"
fi

# Check if i18n.js exports the new variables
if grep -q "supportedLocales" "$BASE_DIR/crates/web/src/assets/js/i18n.js" && \
   grep -q "localeNames" "$BASE_DIR/crates/web/src/assets/js/i18n.js"; then
    echo -e "${GREEN}✅${NC} i18n.js 导出了新的语言变量"
else
    echo -e "${RED}❌${NC} i18n.js 未导出新的语言变量"
fi

echo ""
echo "📝 检查文档..."
echo ""

check_file "$BASE_DIR/I18N_IMPLEMENTATION_REPORT.md" "实现报告"
check_file "$BASE_DIR/I18N_QUICK_START.md" "快速开始指南"
check_file "$BASE_DIR/scripts/create-missing-locales.sh" "语言创建脚本"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ ${#MISSING_LANGS[@]} -eq 0 ]; then
    echo -e "${GREEN}✅ 所有检查通过！多语言功能已正确实现。${NC}"
    echo ""
    echo "🚀 下一步："
    echo "   1. 启动服务: ./target/debug/clawmaster gateway"
    echo "   2. 访问界面: https://localhost:65233"
    echo "   3. 测试语言选择器功能"
    echo ""
else
    echo -e "${YELLOW}⚠️  发现 ${#MISSING_LANGS[@]} 个缺失的语言目录${NC}"
    echo "   缺失的语言: ${MISSING_LANGS[*]}"
    echo ""
    echo "运行以下命令创建缺失的语言："
    echo "   ./scripts/create-missing-locales.sh"
    echo ""
fi

echo "📖 查看完整文档："
echo "   cat I18N_IMPLEMENTATION_REPORT.md"
echo "   cat I18N_QUICK_START.md"
echo ""
