#!/bin/bash
# 切换到 Llama 3.2 1B 模型

echo "🔄 切换到 Llama 3.2 1B 模型"
echo ""

# 停止当前服务
echo "1. 停止当前 ClawMaster..."
pkill -9 -f clawmaster
sleep 2
echo "   ✅ 已停止"
echo ""

# 备份配置
echo "2. 备份配置文件..."
cp ~/.clawmaster/clawmaster.toml ~/.clawmaster/clawmaster.toml.backup
echo "   ✅ 已备份到 ~/.clawmaster/clawmaster.toml.backup"
echo ""

# 修改配置
echo "3. 修改配置..."
cat > /tmp/llama32_config.txt << 'EOF'
[providers.local-llm]
enabled = true
model_id = "llama3.2-1b-instruct-q4_k_m"
gpu_layers = 0
temperature = 0.7
context_size = 8192
EOF

# 使用 Python 或手动提示用户
echo "   请手动编辑配置文件："
echo "   vim ~/.clawmaster/clawmaster.toml"
echo ""
echo "   找到 [providers.local-llm] 部分，修改为："
echo ""
cat /tmp/llama32_config.txt
echo ""
echo "   或者运行："
echo "   cat /tmp/llama32_config.txt"
echo ""

read -p "配置修改完成后按 Enter 继续..."

# 重启服务
echo ""
echo "4. 重启 ClawMaster..."
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_llama32.log 2>&1 &
echo "   ✅ 已启动"
echo ""

# 等待启动
echo "5. 等待服务启动..."
sleep 10

# 检查状态
if pgrep -f clawmaster > /dev/null; then
    echo "   ✅ ClawMaster 运行中"
    echo ""
    echo "6. 检查日志..."
    tail -20 /tmp/clawmaster_llama32.log | grep -E "(listening|model|loaded)"
    echo ""
    echo "🎉 切换完成！"
    echo ""
    echo "现在请访问: https://localhost:59233"
    echo "输入测试: 美国新闻"
else
    echo "   ❌ 启动失败"
    echo ""
    echo "查看日志:"
    echo "   tail -50 /tmp/clawmaster_llama32.log"
fi
