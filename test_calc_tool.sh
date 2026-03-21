#!/bin/bash
# 测试 calc 工具是否能被调用

echo "=== 测试 calc 工具 ==="
echo ""
echo "这个测试将帮助我们确定问题是："
echo "1. 所有工具都不能调用（通用问题）"
echo "2. 只有 news_search 不能调用（特定问题）"
echo ""

# 清空旧日志
> /tmp/calc_test.log

echo "发送查询: 请使用 calc 工具计算 2+2"
echo ""

# 等待几秒让系统准备好
sleep 3

# 通过 WebUI 手动测试，或者查看日志
echo "请在 WebUI 中输入: 请使用 calc 工具计算 2+2"
echo ""
echo "然后运行以下命令查看结果:"
echo "  tail -100 /tmp/clawmaster_debug.log | grep -E '(calc|tool_call|tool_use)'"
echo ""
echo "如果看到 calc 被调用 → 问题在 news_search"
echo "如果 calc 也不被调用 → 问题在工具调用机制"
