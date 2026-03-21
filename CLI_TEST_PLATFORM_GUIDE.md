# ClawMaster CLI 测试平台使用指南

**创建时间**: 2026-03-19  
**版本**: 1.0

---

## 📋 概述

ClawMaster CLI 测试平台提供了完善的命令行测试工具，让你可以在不启动 WebUI 的情况下进行全面的功能测试。

### 核心特性

1. ✅ **交互式测试** - 实时对话，即时反馈
2. ✅ **自动化测试** - 批量执行，生成报告
3. ✅ **性能测试** - 响应时间、Token 统计
4. ✅ **日志分析** - 自动提取关键信息

---

## 🚀 快速开始

### 1. 交互式测试

最简单的测试方式，适合快速验证功能：

```bash
cd cli_test_platform
./interactive_test.sh
```

**功能**:
- 实时输入测试消息
- 查看即时响应
- 统计成功/失败率
- 保存完整日志

**使用示例**:
```
> 今天有什么中国新闻？
[查询 #1] 今天有什么中国新闻？

✓ 查询成功
响应时间: 3245ms

> 计算 123 + 456
[查询 #2] 计算 123 + 456

✓ 查询成功
响应时间: 1523ms

> stats
========================================
测试统计
========================================
总查询数: 2
成功: 2
失败: 0
平均响应时间: 2384ms
```

---

### 2. 自动化测试

执行预定义的测试场景，生成详细报告：

```bash
cd cli_test_platform
./auto_test.sh
```

**测试场景**:
1. 中文新闻查询
2. 英文新闻查询
3. 科技新闻查询
4. 身份问答（中文）
5. 身份问答（英文）
6. 简单计算
7. 复杂表达式
8. 城市新闻查询
9. 模糊新闻查询
10. 体育新闻查询

**输出**:
- 实时测试进度
- 通过/失败统计
- Markdown 格式报告
- 详细日志文件

---

### 3. 性能测试

测试系统性能指标：

```bash
cd cli_test_platform
./performance_test.sh
```

**测试指标**:
- 响应时间（ms）
- 迭代次数
- 输入 Tokens
- 输出 Tokens
- 工具调用次数

**报告示例**:
```markdown
### 新闻查询

**输入**: 今天有什么中国新闻？

| 指标 | 数值 |
|------|------|
| 响应时间 | 3245ms |
| 迭代次数 | 3 |
| 输入 Tokens | 6127 |
| 输出 Tokens | 19 |
| 工具调用 | 1 |
```

---

### 4. 日志分析

分析测试日志，提取关键信息：

```bash
cd cli_test_platform
./log_analyzer.sh
```

**分析内容**:
- 工具调用统计
- 错误/警告统计
- 平均迭代次数
- 常见错误列表

---

## 📁 目录结构

```
cli_test_platform/
├── README.md                 # 平台说明
├── interactive_test.sh       # 交互式测试
├── auto_test.sh             # 自动化测试
├── performance_test.sh      # 性能测试
├── log_analyzer.sh          # 日志分析
└── test_logs/               # 日志目录（自动创建）
    ├── query_1.log
    ├── query_2.log
    └── ...
```

---

## 🛠️ 详细使用说明

### 交互式测试命令

在交互模式中可用的命令：

| 命令 | 说明 |
|------|------|
| `help` | 显示帮助信息 |
| `stats` | 显示测试统计 |
| `clear` | 清屏 |
| `quit` 或 `exit` | 退出测试 |

### 自定义测试场景

编辑 `auto_test.sh`，添加新的测试场景：

```bash
run_test "测试名称" \
    "测试消息" \
    "预期行为"
```

**示例**:
```bash
run_test "德国新闻查询" \
    "What's happening in Germany?" \
    "调用 news_search 工具，location=Germany"
```

---

## 📊 测试报告

### 自动化测试报告

生成的报告包含：

1. **测试场景详情**
   - 输入消息
   - 预期行为
   - 实际结果
   - 响应时间
   - 工具调用次数

2. **测试摘要**
   - 总测试数
   - 通过/失败数
   - 通过率

### 性能测试报告

包含每个测试的性能指标：
- 响应时间
- Token 使用情况
- 迭代次数
- 工具调用统计

### 日志分析报告

包含：
- 工具调用统计
- 错误/警告统计
- 性能指标
- 常见错误列表

---

## 🔧 高级用法

### 1. 修改日志级别

在脚本中修改 `--log-level` 参数：

```bash
$CLAWMASTER agent --message "$message" --log-level debug
```

可选级别：`trace`, `debug`, `info`, `warn`, `error`

### 2. 自定义超时时间

添加超时控制：

```bash
timeout 30s $CLAWMASTER agent --message "$message"
```

### 3. 并发测试

使用 GNU Parallel 进行并发测试：

```bash
cat test_messages.txt | parallel -j 4 './run_single_test.sh {}'
```

---

## 📝 测试最佳实践

### 1. 测试前准备

- ✅ 确保 ClawMaster 已编译
- ✅ 检查配置文件
- ✅ 确认模型已加载

### 2. 测试执行

- ✅ 从简单测试开始
- ✅ 逐步增加复杂度
- ✅ 记录异常情况

### 3. 结果分析

- ✅ 查看测试报告
- ✅ 分析失败原因
- ✅ 检查性能指标

### 4. 问题排查

- ✅ 查看详细日志
- ✅ 使用日志分析工具
- ✅ 对比成功/失败案例

---

## 🎯 常见测试场景

### 新闻查询测试

```bash
# 中文
今天有什么中国新闻？
上海有什么新闻？

# 英文
What's the latest news in USA?
Show me technology news from Germany

# 特定类别
给我看看美国的科技新闻
Show me sports news from UK
```

### 身份问答测试

```bash
# 中文
你是谁？
你能做什么？

# 英文
What can you do?
Who are you?
```

### 计算功能测试

```bash
# 简单计算
计算 123 + 456
Calculate 100 * 50

# 复杂表达式
Calculate (15 + 25) * 3
计算 (100 - 50) / 2
```

---

## 🐛 故障排除

### 问题 1: 命令未找到

**错误**: `clawmaster: command not found`

**解决**:
```bash
# 检查路径
ls -la ../target/debug/clawmaster

# 重新编译
cd .. && cargo build
```

### 问题 2: 权限错误

**错误**: `Permission denied`

**解决**:
```bash
chmod +x cli_test_platform/*.sh
```

### 问题 3: 日志目录错误

**错误**: `Cannot create directory`

**解决**:
```bash
mkdir -p cli_test_platform/test_logs
```

---

## 📈 性能优化建议

### 1. 减少迭代次数

- 优化 prompt
- 使用更强的模型
- 调整 agent_max_iterations

### 2. 提高响应速度

- 使用本地模型
- 优化工具执行
- 减少日志级别

### 3. 降低 Token 使用

- 简化 system prompt
- 限制历史消息数
- 优化工具结果格式

---

## 🎉 总结

ClawMaster CLI 测试平台提供了完善的测试工具，让你可以：

1. ✅ 快速验证功能
2. ✅ 自动化批量测试
3. ✅ 分析性能指标
4. ✅ 追踪问题根源

**推荐工作流程**:

1. 使用交互式测试快速验证
2. 运行自动化测试套件
3. 执行性能测试
4. 分析日志找出问题
5. 修复后重新测试

---

**文档版本**: 1.0  
**最后更新**: 2026-03-19  
**维护者**: ClawMaster Team
