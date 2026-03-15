#!/bin/bash
# ClawMaster Tauri 开发模式启动脚本

set -e

echo "🚀 启动 ClawMaster Tauri 开发模式"
echo ""

# 检查后端是否运行
if ! curl -s https://localhost:59233/api/health > /dev/null 2>&1; then
    echo "⚠️  ClawMaster 后端未运行"
    echo "请在另一个终端运行: cargo run --bin clawmaster"
    exit 1
fi

echo "✅ 后端已运行"
echo ""

# 检查 Tauri CLI
if ! command -v cargo-tauri &> /dev/null; then
    echo "⚠️  Tauri CLI 未安装"
    echo "正在安装..."
    cargo install tauri-cli --locked
fi

echo "✅ Tauri CLI 已安装"
echo ""

# 启动 Tauri 开发模式
echo "🎨 启动 Tauri 应用..."
cd "$(dirname "$0")"
cargo tauri dev
