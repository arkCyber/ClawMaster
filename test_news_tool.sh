#!/bin/bash
# 新闻工具自动测试脚本

echo "=== 新闻工具自动测试 ==="
echo ""

# 1. 检查 TOOLS.md 文件
echo "1. 检查 TOOLS.md 文件..."
if [ -f ~/.clawmaster/TOOLS.md ]; then
    echo "✅ ~/.clawmaster/TOOLS.md 存在"
    echo "文件大小: $(wc -c < ~/.clawmaster/TOOLS.md) bytes"
    echo "包含 'news' 关键词: $(grep -c "news" ~/.clawmaster/TOOLS.md || echo 0) 次"
else
    echo "❌ ~/.clawmaster/TOOLS.md 不存在"
fi
echo ""

if [ -f ~/.clawmaster/agents/main/TOOLS.md ]; then
    echo "✅ ~/.clawmaster/agents/main/TOOLS.md 存在"
    echo "文件大小: $(wc -c < ~/.clawmaster/agents/main/TOOLS.md) bytes"
else
    echo "❌ ~/.clawmaster/agents/main/TOOLS.md 不存在"
fi
echo ""

# 2. 检查工具注册
echo "2. 检查工具注册..."
if pgrep -f clawmaster > /dev/null; then
    echo "✅ ClawMaster 进程运行中"
else
    echo "❌ ClawMaster 进程未运行"
    exit 1
fi
echo ""

# 3. 测试 API 连接
echo "3. 测试 API 连接..."
if curl -s -k https://localhost:59233/api/gon > /dev/null 2>&1; then
    echo "✅ API 可访问"
else
    echo "❌ API 不可访问"
    exit 1
fi
echo ""

# 4. 检查模型列表
echo "4. 检查模型列表..."
MODEL_COUNT=$(curl -s -k https://localhost:59233/api/gon 2>/dev/null | grep -o '"models":\[' | wc -l)
if [ "$MODEL_COUNT" -gt 0 ]; then
    echo "✅ 模型列表可访问"
else
    echo "⚠️  模型列表可能为空"
fi
echo ""

# 5. 模拟新闻查询（通过 WebSocket 或 HTTP）
echo "5. 创建测试会话并发送新闻查询..."
SESSION_KEY="test-news-$(date +%s)"

# 发送消息
RESPONSE=$(curl -s -k -X POST https://localhost:59233/api/chat.send \
    -H "Content-Type: application/json" \
    -d "{
        \"_session_key\": \"$SESSION_KEY\",
        \"content\": \"美国新闻\",
        \"stream\": false
    }" 2>/dev/null)

echo "响应: $RESPONSE"
echo ""

# 6. 检查日志中的工具调用
echo "6. 检查最近的工具调用日志..."
if [ -f /tmp/clawmaster.log ]; then
    echo "最近 10 条包含 'news' 的日志:"
    tail -100 /tmp/clawmaster.log | grep -i "news" | tail -10
    echo ""
    echo "最近 5 条包含 'tool' 的日志:"
    tail -100 /tmp/clawmaster.log | grep -i "tool" | tail -5
else
    echo "⚠️  日志文件不存在"
fi
echo ""

# 7. 检查 system prompt 是否包含 TOOLS.md
echo "7. 检查 system prompt..."
PROMPT_TEST=$(cat > /tmp/test_prompt.json << 'EOF'
{
    "_session_key": "test-prompt"
}
EOF
)

# 注意：这个 API 可能需要认证
echo "尝试获取 system prompt..."
curl -s -k -X POST https://localhost:59233/api/chat.system_prompt \
    -H "Content-Type: application/json" \
    -d @/tmp/test_prompt.json 2>/dev/null | \
    grep -o "TOOLS.md" | head -1 && echo "✅ system prompt 包含 TOOLS.md" || echo "❌ system prompt 不包含 TOOLS.md"

rm -f /tmp/test_prompt.json
echo ""

echo "=== 测试完成 ==="
