#!/bin/bash
# 彻底解决 Cargo 锁问题的测试脚本

set -e

echo "🔧 Step 1: 终止所有 Cargo 进程..."
killall -9 cargo rust-analyzer 2>/dev/null || true
sleep 2

echo "🧹 Step 2: 清理所有锁文件..."
find ~/.cargo -name "*.lock" -type f -delete 2>/dev/null || true
rm -rf ~/.cargo/.package-cache* 2>/dev/null || true

echo "✅ Step 3: 验证没有 Cargo 进程运行..."
if ps aux | grep -E "cargo|rust-analyzer" | grep -v grep; then
    echo "❌ 仍有 Cargo 进程运行，再次终止..."
    killall -9 cargo rust-analyzer 2>/dev/null || true
    sleep 2
fi

echo "🧪 Step 4: 运行单元测试 (单线程模式)..."
cd /Users/arksong/ClawMaster
cargo test --package clawmaster-bundled-skills --lib -- --test-threads=1 --nocapture

echo "✅ 测试完成！"
