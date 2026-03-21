#!/bin/bash
# 实时监控测试进度

LOG_DIR=$(ls -td comprehensive_test_logs_* 2>/dev/null | head -1)

if [ -z "$LOG_DIR" ]; then
    echo "没有找到测试日志目录"
    exit 1
fi

echo "监控测试进度: $LOG_DIR"
echo ""

while true; do
    clear
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║   ClawMaster 测试进度监控                                 ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    
    # 统计测试结果
    PASSED=$(grep -c "✅ 通过" "$LOG_DIR/master_test.log" 2>/dev/null || echo 0)
    FAILED=$(grep -c "❌ 失败" "$LOG_DIR/master_test.log" 2>/dev/null || echo 0)
    TIMEOUT=$(grep -c "⏱️  超时" "$LOG_DIR/master_test.log" 2>/dev/null || echo 0)
    TOTAL=$((PASSED + FAILED + TIMEOUT))
    
    echo "测试进度: $TOTAL / 45"
    echo "通过: $PASSED"
    echo "失败: $FAILED"
    echo "超时: $TIMEOUT"
    echo ""
    
    # 显示最近的测试
    echo "最近测试:"
    tail -10 "$LOG_DIR/master_test.log" 2>/dev/null || echo "等待测试开始..."
    
    # 检查是否完成
    if [ $TOTAL -ge 45 ]; then
        echo ""
        echo "测试完成！"
        break
    fi
    
    sleep 2
done
