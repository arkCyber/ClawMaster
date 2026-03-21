# ClawMaster 全面工具测试指南

**创建日期**: 2026-03-21  
**测试工具数**: 31 个  
**测试场景数**: 93 个（每个工具 3 个场景）

---

## 📋 概述

这是一个全面的 ClawMaster 工具测试脚本，通过自然语言 CLI 接口测试所有 30+ 种工具。每个工具包含 3 个实际应用场景，测试过程完全可视化，显示发送的测试文本和返回的结果。

---

## 🚀 快速开始

### 1. 运行完整测试

```bash
# 运行所有工具的测试
./test_all_tools.sh
```

### 2. 查看测试过程

测试脚本会实时显示：
- ✅ 当前测试的工具名称
- 📋 测试场景描述
- ➤ 发送的测试输入文本
- ⏳ 执行状态
- ✓ 输出结果（前 200 字符）
- ✅/❌ 测试通过/失败状态

### 3. 查看测试结果

测试完成后会生成三个文件：
- `tool_test_results_YYYYMMDD_HHMMSS.log` - 简要结果日志
- `tool_test_detailed_YYYYMMDD_HHMMSS.log` - 详细输出日志
- `TOOL_TEST_REPORT_YYYYMMDD_HHMMSS.md` - Markdown 格式报告

---

## 🔧 测试的工具列表

### 1. **exec** - 命令执行工具
**场景 1**: 列出当前目录文件
```
测试输入: "列出当前目录的所有文件"
预期: 执行 ls 命令
```

**场景 2**: 查看系统信息
```
测试输入: "显示当前系统的操作系统信息"
预期: 执行 uname 命令
```

**场景 3**: 检查磁盘使用
```
测试输入: "检查磁盘空间使用情况"
预期: 执行 df 命令
```

---

### 2. **calc** - 计算器工具
**场景 1**: 简单加法
```
测试输入: "计算 123 + 456"
预期结果: 579
```

**场景 2**: 复杂表达式
```
测试输入: "计算 (100 + 50) * 2 - 30"
预期结果: 270
```

**场景 3**: 科学计算
```
测试输入: "计算 2 的 10 次方"
预期结果: 1024
```

---

### 3. **web_search** - 网页搜索工具
**场景 1**: 搜索新闻
```
测试输入: "搜索最新的 AI 技术新闻"
预期: 返回 AI 相关新闻
```

**场景 2**: 搜索技术文档
```
测试输入: "搜索 Rust 编程语言官方文档"
预期: 返回 Rust 文档链接
```

**场景 3**: 搜索开源项目
```
测试输入: "搜索 GitHub 上的热门 Rust 项目"
预期: 返回 GitHub Rust 项目
```

---

### 4. **web_fetch** - 网页获取工具
**场景 1**: 获取网页内容
```
测试输入: "获取 https://www.rust-lang.org 的内容"
预期: 返回网页 HTML 或文本
```

**场景 2**: 获取 API 数据
```
测试输入: "获取 https://api.github.com/repos/rust-lang/rust 的信息"
预期: 返回 JSON 数据
```

**场景 3**: 获取 JSON 数据
```
测试输入: "获取 https://jsonplaceholder.typicode.com/posts/1 的数据"
预期: 返回 JSON 对象
```

---

### 5. **browser** - 浏览器自动化工具
**场景 1**: 打开网页
```
测试输入: "使用浏览器打开 https://www.rust-lang.org"
状态: SKIP（需要浏览器环境）
```

**场景 2**: 截图
```
测试输入: "对当前页面进行截图"
状态: SKIP（需要浏览器环境）
```

**场景 3**: 点击元素
```
测试输入: "点击页面上的 'Get Started' 按钮"
状态: SKIP（需要浏览器环境）
```

---

### 6. **task_list** - 任务列表工具
**场景 1**: 添加任务
```
测试输入: "添加一个任务：完成代码审计"
预期: 任务添加成功
```

**场景 2**: 列出任务
```
测试输入: "列出所有待办任务"
预期: 显示任务列表
```

**场景 3**: 完成任务
```
测试输入: "标记第一个任务为已完成"
预期: 任务状态更新
```

---

### 7. **sessions_list** - 会话列表工具
**场景 1**: 列出所有会话
```
测试输入: "显示所有活动的会话"
预期: 显示会话列表
```

**场景 2**: 查找特定会话
```
测试输入: "查找包含 'test' 的会话"
预期: 返回匹配的会话
```

**场景 3**: 统计会话数量
```
测试输入: "统计当前有多少个会话"
预期: 返回会话数量
```

---

### 8. **sessions_history** - 会话历史工具
**场景 1**: 查看会话历史
```
测试输入: "查看当前会话的历史记录"
预期: 显示历史消息
```

**场景 2**: 搜索历史消息
```
测试输入: "在历史记录中搜索关键词 'test'"
预期: 返回匹配的消息
```

**场景 3**: 导出历史
```
测试输入: "导出最近 10 条历史记录"
预期: 导出历史数据
```

---

### 9-31. 其他工具

完整的 31 个工具测试包括：
- sessions_send - 会话发送
- process - 进程管理
- cron - 定时任务
- show_map - 地图显示
- location - 位置获取
- send_image - 图片发送
- image_tool - 图片分析
- pdf_tool - PDF 处理
- news_tool - 新闻获取
- apply_patch - 补丁应用
- create_skill - 技能创建
- update_skill - 技能更新
- delete_skill - 技能删除
- spawn_agent - 智能体生成
- agents_list - 智能体列表
- nodes_list - 节点列表
- nodes_describe - 节点描述
- nodes_select - 节点选择
- loop_detection - 循环检测
- sandbox_packages - 沙箱包管理
- gateway_config - 网关配置
- branch_session - 会话分支
- approval - 审批

---

## 📊 测试输出示例

### 控制台输出示例

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🧪 ClawMaster 全面工具测试
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

测试时间: 2026-03-21 10:00:00
日志文件: tool_test_results_20260321_100000.log
详细日志: tool_test_detailed_20260321_100000.log

┌─────────────────────────────────────────────────────────────────┐
│  工具 #1: exec - 命令执行
└─────────────────────────────────────────────────────────────────┘

  📋 场景 1: 列出当前目录文件
  ➤ 测试输入: 列出当前目录的所有文件
  ⏳ 执行中...
  ✓ 输出: Cargo.toml
README.md
src/
target/
...
  ✅ PASS: exec_ls

  📋 场景 2: 查看系统信息
  ➤ 测试输入: 显示当前系统的操作系统信息
  ⏳ 执行中...
  ✓ 输出: Darwin 23.0.0 Darwin Kernel Version...
  ✅ PASS: exec_uname

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  📊 测试总结
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

总测试数: 93
通过: 45
失败: 5
跳过: 43

通过率: 48%

详细日志: tool_test_results_20260321_100000.log
完整输出: tool_test_detailed_20260321_100000.log

✅ 测试完成！
Markdown 报告: TOOL_TEST_REPORT_20260321_100000.md
```

---

## 📝 日志文件格式

### 简要日志 (tool_test_results_*.log)

```
PASS: exec_ls
PASS: exec_uname
PASS: exec_df
PASS: calc_add
FAIL: calc_complex - Expected: 270
SKIP: browser_open - Reason: 需要浏览器环境
...
```

### 详细日志 (tool_test_detailed_*.log)

```
========================================
Test: exec_ls
Time: 2026-03-21 10:00:01
Input: 列出当前目录的所有文件
----------------------------------------
Cargo.toml
README.md
src/
target/
...
Exit Code: 0
========================================

========================================
Test: calc_add
Time: 2026-03-21 10:00:05
Input: 计算 123 + 456
----------------------------------------
579
Exit Code: 0
========================================
```

---

## 🎯 使用场景

### 场景 1: 完整回归测试

```bash
# 在发布前运行完整测试
./test_all_tools.sh

# 检查通过率
cat TOOL_TEST_REPORT_*.md
```

### 场景 2: 调试特定工具

```bash
# 编辑脚本，只运行特定工具的测试
# 注释掉其他工具的测试部分

# 运行修改后的脚本
./test_all_tools.sh
```

### 场景 3: 持续集成

```bash
# 在 CI 管道中运行
./test_all_tools.sh || exit 1

# 上传测试报告
upload_artifact TOOL_TEST_REPORT_*.md
```

---

## ⚙️ 配置选项

### 修改超时时间

在脚本中找到：
```bash
output=$(echo "$test_input" | timeout 30s $CLAWMASTER_CLI chat --agent default 2>&1 || echo "TIMEOUT_OR_ERROR")
```

修改 `30s` 为你需要的超时时间。

### 修改输出长度

在脚本中找到：
```bash
local short_output=$(echo "$output" | head -c 200)
```

修改 `200` 为你需要的字符数。

### 修改 CLI 命令

在脚本开头修改：
```bash
CLAWMASTER_CLI="cargo run --bin clawmaster --"
```

可以改为：
```bash
CLAWMASTER_CLI="clawmaster"  # 如果已安装
```

---

## 🔍 故障排除

### 问题 1: 所有测试都失败

**原因**: ClawMaster 未启动或 CLI 命令不正确

**解决**:
```bash
# 确保 ClawMaster 可以运行
cargo build --bin clawmaster

# 或使用已安装的版本
which clawmaster
```

### 问题 2: 测试超时

**原因**: LLM 响应慢或网络问题

**解决**:
- 增加超时时间（修改脚本中的 `timeout 30s`）
- 检查网络连接
- 检查 LLM API 配置

### 问题 3: 某些工具总是 SKIP

**原因**: 这些工具需要特定的环境或配置

**解决**:
- 查看 SKIP 原因
- 配置所需的环境（如浏览器、沙箱等）
- 或接受这些工具被跳过

---

## 📈 改进建议

### 1. 添加更多场景

为每个工具添加更多测试场景：
```bash
print_scenario "4" "新场景描述"
print_test_input "新测试输入"
run_test "test_name" "新测试输入" "预期模式"
```

### 2. 添加性能测试

记录每个测试的执行时间：
```bash
start_time=$(date +%s)
run_test "test_name" "input" "pattern"
end_time=$(date +%s)
duration=$((end_time - start_time))
echo "Duration: ${duration}s" >> "$LOG_FILE"
```

### 3. 添加并发测试

测试工具的并发处理能力：
```bash
# 并发运行多个测试
for i in {1..10}; do
    run_test "concurrent_test_$i" "input" "pattern" &
done
wait
```

---

## ✅ 最佳实践

1. **定期运行**: 在每次代码变更后运行测试
2. **保存报告**: 保留历史测试报告用于对比
3. **分析趋势**: 跟踪通过率的变化趋势
4. **修复失败**: 优先修复失败的测试
5. **更新场景**: 根据实际使用情况更新测试场景

---

## 📚 相关文档

- [ClawMaster README](README.md)
- [工具文档](docs/tools/)
- [测试策略](docs/testing/)
- [CI/CD 集成](docs/ci-cd/)

---

**创建时间**: 2026-03-21  
**维护者**: arkSong  
**版本**: 1.0
