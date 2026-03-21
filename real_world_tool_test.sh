#!/bin/bash

# 真实场景工具测试脚本
# 通过 CLI 接口发送自然语言命令，测试工具调用情况

set -e

CLI="./target/release/clawmaster"
GATEWAY_URL="https://localhost:59233"
TIMEOUT=30
LOG_DIR="real_world_test_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$LOG_DIR"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║   真实场景工具测试 - CLI 自然语言输入                    ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "测试时间: $(date)"
echo "日志目录: $LOG_DIR"
echo ""

# 测试计数器
TOTAL=0
SUCCESS=0
FAILED=0

# 测试函数
test_tool() {
    local scenario="$1"
    local command="$2"
    local expected_pattern="$3"
    local tool_name="$4"
    
    TOTAL=$((TOTAL + 1))
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "[$TOTAL] 场景: $scenario"
    echo "    工具: $tool_name"
    echo "    命令: $command"
    echo ""
    
    # 运行命令并捕获输出
    local output_file="$LOG_DIR/test_${TOTAL}_${tool_name}.log"
    local response=""
    
    if timeout $TIMEOUT env CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" "$CLI" agent --message "$command" > "$output_file" 2>&1; then
        response=$(cat "$output_file")
        
        # 检查是否包含预期模式
        if echo "$response" | grep -q "$expected_pattern"; then
            echo "    ✅ 成功 - 工具被调用"
            SUCCESS=$((SUCCESS + 1))
        else
            echo "    ❌ 失败 - 未检测到预期结果"
            echo "    预期: $expected_pattern"
            FAILED=$((FAILED + 1))
        fi
        
        # 显示响应摘要
        local summary=$(echo "$response" | grep "✅ 响应:" | head -1 | cut -c 1-100)
        echo "    响应: $summary..."
    else
        echo "    ⏱️  超时 - 命令执行超过 ${TIMEOUT}s"
        FAILED=$((FAILED + 1))
    fi
    
    echo ""
    sleep 2  # 避免请求过快
}

echo "开始测试..."
echo ""

# ============================================================
# 场景 1: 数学计算
# ============================================================
echo "【场景组 1: 数学计算】"
test_tool "简单加法" "帮我算一下 25 + 75" "100" "calc"
test_tool "乘法运算" "计算 12 乘以 8" "96" "calc"
test_tool "复杂表达式" "算一下 (100 + 50) * 2" "300" "calc"

# ============================================================
# 场景 2: 信息查询
# ============================================================
echo "【场景组 2: 信息查询】"
test_tool "新闻搜索" "给我找一些最新的科技新闻" "news" "news_search"
test_tool "特定主题新闻" "搜索关于人工智能的新闻" "AI\|artificial intelligence\|人工智能" "news_search"
test_tool "网络搜索" "在网上搜索 Rust 编程教程" "Rust\|教程\|tutorial" "web_search"

# ============================================================
# 场景 3: 文件操作
# ============================================================
echo "【场景组 3: 文件操作】"
test_tool "列出文件" "显示当前目录有什么文件" "Cargo.toml\|README\|crates" "exec"
test_tool "查看文件内容" "读取 README.md 文件的前 10 行" "ClawMaster\|README" "read"
test_tool "查找文件" "找一下所有的 .rs 文件" ".rs\|Rust" "exec"

# ============================================================
# 场景 4: 记忆管理
# ============================================================
echo "【场景组 4: 记忆管理】"
test_tool "保存记忆" "记住我的名字是 arkSong" "记住\|保存\|saved" "memory_save"
test_tool "查询记忆" "我的名字是什么" "arkSong" "memory_search"
test_tool "保存偏好" "记住我喜欢用 Rust 编程" "Rust\|记住" "memory_save"

# ============================================================
# 场景 5: 系统信息
# ============================================================
echo "【场景组 5: 系统信息】"
test_tool "查看任务" "显示我的所有任务" "task\|任务" "task_list"
test_tool "会话列表" "列出所有的会话" "session\|会话" "sessions_list"
test_tool "系统时间" "现在几点了" "时间\|time\|点" "exec"

# ============================================================
# 场景 6: 混合场景
# ============================================================
echo "【场景组 6: 混合场景】"
test_tool "计算后保存" "算一下 365 * 24，然后记住这是一年的小时数" "8760\|记住" "calc,memory_save"
test_tool "搜索后总结" "搜索 Rust 新闻，告诉我有什么重要的" "Rust\|news" "news_search"
test_tool "文件统计" "统计一下当前目录有多少个文件" "文件\|file\|个" "exec"

# ============================================================
# 测试总结
# ============================================================
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                    测试结果总结                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "总测试数: $TOTAL"
echo "成功: $SUCCESS"
echo "失败: $FAILED"
echo "成功率: $(awk "BEGIN {printf \"%.1f%%\", ($SUCCESS/$TOTAL)*100}")"
echo ""
echo "详细日志: $LOG_DIR/"
echo ""

# 生成 Markdown 报告
REPORT_FILE="$LOG_DIR/REPORT.md"
cat > "$REPORT_FILE" << EOF
# 真实场景工具测试报告

**测试时间**: $(date)  
**测试环境**: ClawMaster v0.10.18 + Llama 3.1 8B  
**测试方式**: CLI 自然语言输入

---

## 测试结果

| 指标 | 数值 |
|------|------|
| 总测试数 | $TOTAL |
| 成功 | $SUCCESS |
| 失败 | $FAILED |
| **成功率** | **$(awk "BEGIN {printf \"%.1f%%\", ($SUCCESS/$TOTAL)*100}")** |

---

## 场景分类

### 1. 数学计算 (3 个测试)
- 简单加法
- 乘法运算
- 复杂表达式

### 2. 信息查询 (3 个测试)
- 新闻搜索
- 特定主题新闻
- 网络搜索

### 3. 文件操作 (3 个测试)
- 列出文件
- 查看文件内容
- 查找文件

### 4. 记忆管理 (3 个测试)
- 保存记忆
- 查询记忆
- 保存偏好

### 5. 系统信息 (3 个测试)
- 查看任务
- 会话列表
- 系统时间

### 6. 混合场景 (3 个测试)
- 计算后保存
- 搜索后总结
- 文件统计

---

## 详细日志

所有测试日志保存在: \`$LOG_DIR/\`

每个测试的详细输出可查看对应的日志文件。

---

## 结论

$(if [ $SUCCESS -ge $((TOTAL * 7 / 10)) ]; then
    echo "✅ **测试通过** - 工具调用成功率达到 70% 以上"
elif [ $SUCCESS -ge $((TOTAL / 2)) ]; then
    echo "⚠️ **部分通过** - 工具调用成功率在 50-70% 之间，需要优化"
else
    echo "❌ **测试失败** - 工具调用成功率低于 50%，需要重大改进"
fi)

**生成时间**: $(date)
EOF

echo "测试报告已生成: $REPORT_FILE"
echo ""

# 如果成功率低于 50%，显示警告
if [ $SUCCESS -lt $((TOTAL / 2)) ]; then
    echo "⚠️  警告: 工具调用成功率低于 50%"
    echo "建议: 检查 system prompt 和模型配置"
fi

exit 0
