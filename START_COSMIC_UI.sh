#!/bin/bash
# ClawMaster Cosmic UI 启动脚本

echo "🚀 ClawMaster Cosmic UI 启动"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cd "$(dirname "$0")"

# 检查二进制文件
if [ ! -f "target/release/clawmaster-cosmic" ]; then
    echo "⚠️  未找到二进制文件，正在编译..."
    cargo build --release -p clawmaster-cosmic
    if [ $? -ne 0 ]; then
        echo "❌ 编译失败"
        exit 1
    fi
    echo "✅ 编译成功"
fi

echo "📊 系统信息："
echo "   版本: ClawMaster Cosmic UI v0.1.0"
echo "   日志: /tmp/clawmaster.log"
echo ""
echo "💡 功能："
echo "   ✓ Dashboard - 系统状态监控"
echo "   ✓ Chat - AI 聊天界面（开发中）"
echo "   ✓ Settings - 配置管理（开发中）"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎨 启动 UI..."
echo ""

RUST_LOG=clawmaster_cosmic=info ./target/release/clawmaster-cosmic
