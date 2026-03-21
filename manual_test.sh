#!/bin/bash
# 手动测试新闻功能 - 获取实际响应

echo "=== 手动新闻功能测试 ==="
echo ""

SESSION_KEY="manual-test-$(date +%s)"

echo "发送查询: 美国新闻"
echo "会话 ID: $SESSION_KEY"
echo ""

# 通过 WebSocket 或 RPC 发送（这里使用简单的方法）
# 注意：chat.send 可能需要特殊处理

# 方法 1: 直接查看最新的日志
echo "等待 5 秒后查看日志..."
sleep 5

echo ""
echo "=== 最新的 agent run 日志 ==="
tail -100 /tmp/clawmaster.log | grep -A 5 "agent run complete" | tail -20

echo ""
echo "=== 工具调用统计 ==="
tail -100 /tmp/clawmaster.log | grep "tool_calls_count" | tail -5

echo ""
echo "=== news_search 相关日志 ==="
tail -200 /tmp/clawmaster.log | grep -i "news" | tail -10
