#!/bin/bash
# 分析后端日志脚本
# 提取关键信息：工具调用、参数、错误等

echo "========================================"
echo "ClawMaster 日志分析"
echo "========================================"
echo ""

echo "【1】工具调用统计"
echo "---"
echo "news_search 调用次数:"
grep -c "executing tool tool=news_search" /dev/stdin 2>/dev/null || echo "0"
echo ""

echo "【2】参数提取分析"
echo "---"
grep "Location extracted by LLM" /dev/stdin 2>/dev/null || echo "无 location 提取"
echo ""

echo "【3】查询生成"
echo "---"
grep "Searching news:" /dev/stdin 2>/dev/null || echo "无查询生成"
echo ""

echo "【4】数据源选择"
echo "---"
grep "Selected.*feeds" /dev/stdin 2>/dev/null || echo "无数据源选择"
echo ""

echo "【5】错误信息"
echo "---"
grep -i "error\|failed\|warn" /dev/stdin 2>/dev/null || echo "无错误"
echo ""

echo "【6】性能指标"
echo "---"
grep "iteration.*input_tokens.*output_tokens" /dev/stdin 2>/dev/null || echo "无性能数据"
echo ""
