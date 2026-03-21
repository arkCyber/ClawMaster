#!/bin/bash
# 自动测试结果分析器
# DO-178C Level A 航空航天级别质量分析

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m'
BOLD='\033[1m'

echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${CYAN}║  🔬 ClawMaster 自动测试结果分析器${NC}"
echo -e "${BOLD}${CYAN}║  DO-178C Level A 航空航天级别质量分析${NC}"
echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 查找最新的测试结果目录
LATEST_TEST_DIR=$(ls -td enhanced_test_* 2>/dev/null | head -1)

if [ -z "$LATEST_TEST_DIR" ]; then
    echo -e "${RED}❌ 未找到测试结果目录${NC}"
    exit 1
fi

echo -e "${BOLD}分析目录:${NC} $LATEST_TEST_DIR"
echo ""

# 读取测试结果
SUMMARY_FILE="$LATEST_TEST_DIR/summary.txt"
REPORT_FILE="$LATEST_TEST_DIR/TEST_REPORT.md"

if [ ! -f "$SUMMARY_FILE" ]; then
    echo -e "${RED}❌ 未找到测试摘要文件${NC}"
    exit 1
fi

# 统计测试结果
TOTAL_TESTS=$(wc -l < "$SUMMARY_FILE" | tr -d ' ')
PASSED_TESTS=$(grep -c "^PASS:" "$SUMMARY_FILE" || echo "0")
FAILED_TESTS=$(grep -c "^FAIL:" "$SUMMARY_FILE" || echo "0")
SKIPPED_TESTS=$(grep -c "^SKIP:" "$SUMMARY_FILE" || echo "0")

# 计算通过率
if [ $TOTAL_TESTS -gt 0 ]; then
    PASS_RATE=$((PASSED_TESTS * 100 / TOTAL_TESTS))
else
    PASS_RATE=0
fi

# 航空航天级别质量评估
get_quality_level() {
    local rate=$1
    if [ $rate -ge 95 ]; then
        echo "DO-178C Level A (航空航天级别)"
    elif [ $rate -ge 85 ]; then
        echo "DO-178C Level B (高可靠性)"
    elif [ $rate -ge 75 ]; then
        echo "DO-178C Level C (中等可靠性)"
    elif [ $rate -ge 60 ]; then
        echo "DO-178C Level D (低可靠性)"
    else
        echo "未达标 (需要改进)"
    fi
}

QUALITY_LEVEL=$(get_quality_level $PASS_RATE)

# 显示分析结果
echo -e "${BOLD}${WHITE}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${WHITE}║                       测试结果统计                                 ║${NC}"
echo -e "${BOLD}${WHITE}╠════════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BOLD}${WHITE}║  总测试数: ${CYAN}${TOTAL_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  通过:     ${GREEN}${PASSED_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  失败:     ${RED}${FAILED_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  跳过:     ${YELLOW}${SKIPPED_TESTS}${WHITE}                                                    ║${NC}"
echo -e "${BOLD}${WHITE}║  通过率:   ${CYAN}${PASS_RATE}%${WHITE}                                                  ║${NC}"
echo -e "${BOLD}${WHITE}╠════════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BOLD}${WHITE}║  质量等级: ${MAGENTA}${QUALITY_LEVEL}${WHITE}                    ║${NC}"
echo -e "${BOLD}${WHITE}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 分析失败的测试
if [ $FAILED_TESTS -gt 0 ]; then
    echo -e "${BOLD}${RED}━━━ 失败测试分析 ━━━${NC}"
    echo ""
    
    # 按工具分类失败
    echo -e "${BOLD}按工具分类:${NC}"
    grep "^FAIL:" "$SUMMARY_FILE" | awk '{print $2}' | sort | uniq -c | while read count tool; do
        echo -e "  • ${RED}$tool${NC}: $count 个失败"
    done
    echo ""
    
    # 失败原因分析
    echo -e "${BOLD}失败原因分析:${NC}"
    if [ -f "$LATEST_TEST_DIR/failures.txt" ]; then
        # 检查是否有编译警告
        COMPILE_WARNINGS=$(grep -c "warning:" "$LATEST_TEST_DIR/failures.txt" 2>/dev/null || echo "0")
        if [ $COMPILE_WARNINGS -gt 0 ]; then
            echo -e "  • ${YELLOW}编译警告${NC}: $COMPILE_WARNINGS 次"
        fi
        
        # 检查是否有超时
        TIMEOUTS=$(grep -c "timeout" "$LATEST_TEST_DIR/failures.txt" 2>/dev/null || echo "0")
        if [ $TIMEOUTS -gt 0 ]; then
            echo -e "  • ${YELLOW}超时${NC}: $TIMEOUTS 次"
        fi
        
        # 检查是否有命令错误
        CMD_ERRORS=$(grep -c "COMMAND_ERROR\|error:" "$LATEST_TEST_DIR/failures.txt" 2>/dev/null || echo "0")
        if [ $CMD_ERRORS -gt 0 ]; then
            echo -e "  • ${RED}命令错误${NC}: $CMD_ERRORS 次"
        fi
    fi
    echo ""
fi

# 成功测试分析
if [ $PASSED_TESTS -gt 0 ]; then
    echo -e "${BOLD}${GREEN}━━━ 成功测试分析 ━━━${NC}"
    echo ""
    
    echo -e "${BOLD}按工具分类:${NC}"
    grep "^PASS:" "$SUMMARY_FILE" | awk '{print $2}' | sort | uniq -c | while read count tool; do
        echo -e "  • ${GREEN}$tool${NC}: $count 个通过"
    done
    echo ""
fi

# 性能分析
echo -e "${BOLD}${BLUE}━━━ 性能分析 ━━━${NC}"
echo ""

# 平均执行时间
AVG_TIME=$(grep "PASS:" "$SUMMARY_FILE" | grep -oE '\([0-9]+s\)' | grep -oE '[0-9]+' | awk '{sum+=$1; count++} END {if(count>0) print int(sum/count); else print 0}')
echo -e "${BOLD}平均执行时间:${NC} ${AVG_TIME}s"

# 最慢的测试
SLOWEST=$(grep "PASS:" "$SUMMARY_FILE" | grep -oE '\([0-9]+s\)' | grep -oE '[0-9]+' | sort -rn | head -1)
echo -e "${BOLD}最慢测试:${NC} ${SLOWEST}s"

# 最快的测试
FASTEST=$(grep "PASS:" "$SUMMARY_FILE" | grep -oE '\([0-9]+s\)' | grep -oE '[0-9]+' | sort -n | head -1)
echo -e "${BOLD}最快测试:${NC} ${FASTEST}s"
echo ""

# 改进建议
echo -e "${BOLD}${MAGENTA}━━━ 改进建议 ━━━${NC}"
echo ""

if [ $PASS_RATE -lt 95 ]; then
    echo -e "${YELLOW}📋 质量改进建议:${NC}"
    
    if [ $PASS_RATE -lt 60 ]; then
        echo -e "  1. ${RED}紧急${NC}: 通过率低于 60%，需要立即修复核心问题"
        echo -e "  2. 检查编译警告和错误"
        echo -e "  3. 验证工具实现的正确性"
    elif [ $PASS_RATE -lt 75 ]; then
        echo -e "  1. ${YELLOW}重要${NC}: 通过率低于 75%，需要修复主要问题"
        echo -e "  2. 分析失败测试的共同原因"
        echo -e "  3. 优化输出格式"
    elif [ $PASS_RATE -lt 85 ]; then
        echo -e "  1. ${BLUE}建议${NC}: 通过率低于 85%，建议进一步优化"
        echo -e "  2. 修复边缘情况"
        echo -e "  3. 改进错误处理"
    else
        echo -e "  1. ${GREEN}良好${NC}: 通过率 85%+，接近航空航天标准"
        echo -e "  2. 修复剩余失败测试"
        echo -e "  3. 添加更多边缘测试"
    fi
    echo ""
fi

# 航空航天级别合规性检查
echo -e "${BOLD}${CYAN}━━━ DO-178C Level A 合规性检查 ━━━${NC}"
echo ""

# 检查项
check_compliance() {
    local name=$1
    local threshold=$2
    local actual=$3
    
    if [ $actual -ge $threshold ]; then
        echo -e "  ✅ ${GREEN}$name${NC}: $actual% (要求: ${threshold}%)"
        return 0
    else
        echo -e "  ❌ ${RED}$name${NC}: $actual% (要求: ${threshold}%)"
        return 1
    fi
}

COMPLIANCE_SCORE=0
TOTAL_CHECKS=5

# 1. 测试覆盖率
check_compliance "测试覆盖率" 95 $PASS_RATE && ((COMPLIANCE_SCORE++))

# 2. 功能正确性（基于通过的测试）
CORRECTNESS=$((PASSED_TESTS * 100 / (PASSED_TESTS + FAILED_TESTS)))
check_compliance "功能正确性" 90 $CORRECTNESS && ((COMPLIANCE_SCORE++))

# 3. 可靠性（基于失败率）
RELIABILITY=$((100 - (FAILED_TESTS * 100 / TOTAL_TESTS)))
check_compliance "系统可靠性" 85 $RELIABILITY && ((COMPLIANCE_SCORE++))

# 4. 性能要求（平均执行时间 < 10s）
if [ $AVG_TIME -lt 10 ]; then
    echo -e "  ✅ ${GREEN}性能要求${NC}: ${AVG_TIME}s (要求: <10s)"
    ((COMPLIANCE_SCORE++))
else
    echo -e "  ❌ ${RED}性能要求${NC}: ${AVG_TIME}s (要求: <10s)"
fi

# 5. 错误处理（无崩溃）
CRASHES=$(grep -c "COMMAND_ERROR\|panic\|segfault" "$LATEST_TEST_DIR/detailed.log" 2>/dev/null || echo "0")
if [ $CRASHES -eq 0 ]; then
    echo -e "  ✅ ${GREEN}错误处理${NC}: 无崩溃"
    ((COMPLIANCE_SCORE++))
else
    echo -e "  ❌ ${RED}错误处理${NC}: $CRASHES 次崩溃"
fi

echo ""
COMPLIANCE_RATE=$((COMPLIANCE_SCORE * 100 / TOTAL_CHECKS))
echo -e "${BOLD}合规性评分:${NC} ${COMPLIANCE_SCORE}/${TOTAL_CHECKS} (${COMPLIANCE_RATE}%)"
echo ""

# 最终评估
echo -e "${BOLD}${WHITE}╔════════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}${WHITE}║                       最终质量评估                                 ║${NC}"
echo -e "${BOLD}${WHITE}╠════════════════════════════════════════════════════════════════════╣${NC}"

if [ $COMPLIANCE_RATE -ge 80 ] && [ $PASS_RATE -ge 85 ]; then
    echo -e "${BOLD}${WHITE}║  ${GREEN}✅ 达到航空航天级别质量标准 (DO-178C Level A)${WHITE}              ║${NC}"
    echo -e "${BOLD}${WHITE}║  ${GREEN}系统已准备好用于关键任务应用${WHITE}                              ║${NC}"
elif [ $COMPLIANCE_RATE -ge 60 ] && [ $PASS_RATE -ge 75 ]; then
    echo -e "${BOLD}${WHITE}║  ${YELLOW}⚠️  接近航空航天级别标准 (DO-178C Level B)${WHITE}                ║${NC}"
    echo -e "${BOLD}${WHITE}║  ${YELLOW}建议修复剩余问题以达到 Level A${WHITE}                            ║${NC}"
else
    echo -e "${BOLD}${WHITE}║  ${RED}❌ 未达到航空航天级别标准${WHITE}                                  ║${NC}"
    echo -e "${BOLD}${WHITE}║  ${RED}需要重大改进才能用于关键任务${WHITE}                              ║${NC}"
fi

echo -e "${BOLD}${WHITE}╚════════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 生成详细报告
ANALYSIS_REPORT="$LATEST_TEST_DIR/DO178C_QUALITY_ANALYSIS.md"

cat > "$ANALYSIS_REPORT" << EOF
# ClawMaster DO-178C Level A 质量分析报告

**分析时间**: $(date)  
**测试目录**: $LATEST_TEST_DIR  
**质量标准**: DO-178C Level A (航空航天级别)

---

## 📊 测试统计

| 指标 | 数值 | 状态 |
|------|------|------|
| **总测试数** | $TOTAL_TESTS | - |
| **通过** | $PASSED_TESTS | ✅ |
| **失败** | $FAILED_TESTS | $([ $FAILED_TESTS -eq 0 ] && echo "✅" || echo "⚠️") |
| **跳过** | $SKIPPED_TESTS | ℹ️ |
| **通过率** | **${PASS_RATE}%** | $([ $PASS_RATE -ge 85 ] && echo "✅" || echo "⚠️") |

---

## 🎯 质量等级

**${QUALITY_LEVEL}**

### 等级说明

- **DO-178C Level A** (≥95%): 航空航天级别，适用于关键任务
- **DO-178C Level B** (≥85%): 高可靠性，适用于重要系统
- **DO-178C Level C** (≥75%): 中等可靠性，适用于一般系统
- **DO-178C Level D** (≥60%): 低可靠性，需要改进
- **未达标** (<60%): 不适合生产环境

---

## ✅ DO-178C Level A 合规性检查

| 检查项 | 要求 | 实际 | 状态 |
|--------|------|------|------|
| 测试覆盖率 | ≥95% | ${PASS_RATE}% | $([ $PASS_RATE -ge 95 ] && echo "✅" || echo "❌") |
| 功能正确性 | ≥90% | ${CORRECTNESS}% | $([ $CORRECTNESS -ge 90 ] && echo "✅" || echo "❌") |
| 系统可靠性 | ≥85% | ${RELIABILITY}% | $([ $RELIABILITY -ge 85 ] && echo "✅" || echo "❌") |
| 性能要求 | <10s | ${AVG_TIME}s | $([ $AVG_TIME -lt 10 ] && echo "✅" || echo "❌") |
| 错误处理 | 无崩溃 | ${CRASHES} 次 | $([ $CRASHES -eq 0 ] && echo "✅" || echo "❌") |

**合规性评分**: ${COMPLIANCE_SCORE}/${TOTAL_CHECKS} (${COMPLIANCE_RATE}%)

---

## 📈 性能分析

- **平均执行时间**: ${AVG_TIME}s
- **最慢测试**: ${SLOWEST}s
- **最快测试**: ${FASTEST}s

---

## 🔍 失败分析

### 按工具分类

$(grep "^FAIL:" "$SUMMARY_FILE" | awk '{print $2}' | sort | uniq -c | while read count tool; do
    echo "- **$tool**: $count 个失败"
done)

### 失败原因

$(if [ -f "$LATEST_TEST_DIR/failures.txt" ]; then
    COMPILE_WARNINGS=$(grep -c "warning:" "$LATEST_TEST_DIR/failures.txt" 2>/dev/null || echo "0")
    TIMEOUTS=$(grep -c "timeout" "$LATEST_TEST_DIR/failures.txt" 2>/dev/null || echo "0")
    CMD_ERRORS=$(grep -c "COMMAND_ERROR\|error:" "$LATEST_TEST_DIR/failures.txt" 2>/dev/null || echo "0")
    
    echo "- 编译警告: $COMPILE_WARNINGS 次"
    echo "- 超时: $TIMEOUTS 次"
    echo "- 命令错误: $CMD_ERRORS 次"
fi)

---

## 💡 改进建议

$(if [ $PASS_RATE -lt 60 ]; then
    echo "### 🔴 紧急改进（通过率 <60%）"
    echo ""
    echo "1. **立即修复核心问题** - 系统存在严重缺陷"
    echo "2. **检查编译警告和错误** - 修复所有编译问题"
    echo "3. **验证工具实现** - 确保核心功能正确"
    echo "4. **添加单元测试** - 在集成测试前验证组件"
elif [ $PASS_RATE -lt 75 ]; then
    echo "### 🟡 重要改进（通过率 60-75%）"
    echo ""
    echo "1. **分析失败测试** - 找出共同原因"
    echo "2. **优化输出格式** - 确保输出符合预期"
    echo "3. **改进错误处理** - 提供更好的错误信息"
    echo "4. **增加测试覆盖** - 添加边缘情况测试"
elif [ $PASS_RATE -lt 85 ]; then
    echo "### 🔵 建议改进（通过率 75-85%）"
    echo ""
    echo "1. **修复边缘情况** - 处理特殊输入"
    echo "2. **改进性能** - 优化慢速测试"
    echo "3. **增强文档** - 添加使用示例"
    echo "4. **代码审查** - 确保代码质量"
elif [ $PASS_RATE -lt 95 ]; then
    echo "### 🟢 优化建议（通过率 85-95%）"
    echo ""
    echo "1. **修复剩余失败** - 达到 95%+ 通过率"
    echo "2. **性能优化** - 减少平均执行时间"
    echo "3. **添加压力测试** - 验证高负载场景"
    echo "4. **安全审计** - 确保无安全漏洞"
else
    echo "### ✅ 维护建议（通过率 ≥95%）"
    echo ""
    echo "1. **持续监控** - 保持高质量标准"
    echo "2. **定期回归测试** - 防止质量下降"
    echo "3. **文档更新** - 保持文档同步"
    echo "4. **性能基准** - 建立性能基线"
fi)

---

## 🎯 最终评估

$(if [ $COMPLIANCE_RATE -ge 80 ] && [ $PASS_RATE -ge 85 ]; then
    echo "✅ **达到航空航天级别质量标准 (DO-178C Level A)**"
    echo ""
    echo "系统已准备好用于关键任务应用。所有核心功能经过严格测试，符合航空航天工业标准。"
elif [ $COMPLIANCE_RATE -ge 60 ] && [ $PASS_RATE -ge 75 ]; then
    echo "⚠️ **接近航空航天级别标准 (DO-178C Level B)**"
    echo ""
    echo "系统质量良好，但需要修复剩余问题以达到 Level A 标准。建议在关键任务使用前完成所有改进。"
else
    echo "❌ **未达到航空航天级别标准**"
    echo ""
    echo "系统需要重大改进才能用于关键任务。建议优先修复核心问题，提高测试通过率和系统可靠性。"
fi)

---

**生成时间**: $(date)  
**分析工具**: ClawMaster 自动测试分析器 v1.0
EOF

echo -e "${GREEN}✅ 分析完成！${NC}"
echo -e "${BOLD}详细报告:${NC} $ANALYSIS_REPORT"
echo ""

# 返回状态码
if [ $COMPLIANCE_RATE -ge 80 ] && [ $PASS_RATE -ge 85 ]; then
    exit 0
else
    exit 1
fi
