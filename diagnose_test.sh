#!/bin/bash
# 诊断测试 - 检查 clawmaster 是否能正常运行

set -e

echo "=========================================="
echo "ClawMaster 诊断测试"
echo "=========================================="
echo ""

# 测试 1: 检查二进制文件
echo "[1/4] 检查二进制文件..."
if [ -f "target/release/clawmaster" ]; then
    echo "✅ 二进制文件存在"
    ls -lh target/release/clawmaster
else
    echo "❌ 二进制文件不存在"
    exit 1
fi
echo ""

# 测试 2: 检查版本
echo "[2/4] 检查版本..."
./target/release/clawmaster --version || echo "版本命令失败"
echo ""

# 测试 3: 检查帮助
echo "[3/4] 检查帮助..."
./target/release/clawmaster --help | head -20
echo ""

# 测试 4: 尝试运行 agent 命令
echo "[4/4] 测试 agent 命令..."
echo "输入: 计算 2 + 2"
timeout 10s ./target/release/clawmaster agent --message "计算 2 + 2" 2>&1 | head -50 || {
    exit_code=$?
    echo ""
    echo "退出码: $exit_code"
    if [ $exit_code -eq 124 ]; then
        echo "⏱️  超时（10秒）"
    elif [ $exit_code -eq 127 ]; then
        echo "❌ 命令未找到"
    else
        echo "❌ 命令失败"
    fi
}

echo ""
echo "=========================================="
echo "诊断完成"
echo "=========================================="
