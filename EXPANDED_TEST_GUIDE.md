# ClawMaster 扩大测试范围指南

**创建时间**: 2026-03-19 22:58  
**测试范围**: 32 个工具（除 news_search），96 个场景  
**测试方式**: CLI 界面实时查看

---

## 📊 测试概览

### 测试范围扩大

**原计划**: 测试 news_search（已完成）  
**新计划**: 测试所有其他 32 个工具  
**测试场景**: 96 个（每个工具 3 个场景）

### 工具列表（32个）

1. calc - 计算器
2. web_search - 网页搜索
3. web_fetch - 网页获取
4. browser - 浏览器控制
5. exec - 命令执行
6. process - 进程管理
7. task_list - 任务列表
8. sessions_list - 会话列表
9. sessions_history - 会话历史
10. sessions_send - 会话发送
11. sessions_create - 创建会话
12. sessions_delete - 删除会话
13. spawn_agent - 生成代理
14. show_map - 显示地图
15. get_user_location - 获取位置
16. send_image - 发送图片
17. image - 图片生成
18. sandbox_packages - 沙箱包管理
19. nodes_list - 节点列表
20. nodes_describe - 节点描述
21. nodes_select - 节点选择
22. loop_detection - 循环检测
23. create_skill - 创建技能
24. update_skill - 更新技能
25. delete_skill - 删除技能
26. cron - 定时任务
27. apply_patch - 应用补丁
28. branch_session - 分支会话
29. session_state - 会话状态
30. gateway - 网关配置
31. agents_list - 代理列表
32. pdf - PDF 处理

---

## 🚀 测试方法

### 方法 1: 完整批量测试（推荐）

**脚本**: `test_all_tools_cli.sh`  
**测试数**: 96 个场景  
**预计时间**: 1-2 小时

```bash
./test_all_tools_cli.sh
```

**特点**:
- ✅ 自动化执行所有测试
- ✅ 实时显示测试进度
- ✅ 每个测试独立日志
- ✅ 自动生成测试报告
- ✅ 超时保护（30秒/测试）
- ✅ 彩色输出便于查看

**输出**:
- 主日志: `test_logs_*/master_test.log`
- 详细日志: `test_logs_*/*.log`
- 测试报告: `test_logs_*/test_report.md`

---

### 方法 2: 快速示例测试

**脚本**: `quick_test_sample.sh`  
**测试数**: 9 个场景（3个工具）  
**预计时间**: 5-10 分钟

```bash
./quick_test_sample.sh
```

**测试工具**:
- calc（3个场景）
- task_list（3个场景）
- sessions_list（3个场景）

**特点**:
- ✅ 快速验证系统
- ✅ 实时查看输出
- ✅ 适合初步测试

---

### 方法 3: 手动单个测试

**命令格式**:
```bash
cargo run --release --bin clawmaster -- agent --message "你的测试消息"
```

**示例**:
```bash
# 测试 calc
cargo run --release --bin clawmaster -- agent --message "计算 123 + 456"

# 测试 task_list
cargo run --release --bin clawmaster -- agent --message "Show me my tasks"

# 测试 sessions_list
cargo run --release --bin clawmaster -- agent --message "List all sessions"
```

**特点**:
- ✅ 完全控制
- ✅ 实时查看详细输出
- ✅ 适合调试特定工具

---

## 📋 测试场景详情

### calc 工具（3个场景）

```bash
# 场景 1: 简单算术
cargo run --release --bin clawmaster -- agent --message "计算 123 + 456"

# 场景 2: 复杂表达式
cargo run --release --bin clawmaster -- agent --message "Calculate (15 + 25) * 3"

# 场景 3: 幂运算
cargo run --release --bin clawmaster -- agent --message "What is 2 to the power of 10?"
```

### task_list 工具（3个场景）

```bash
# 场景 1: 添加任务
cargo run --release --bin clawmaster -- agent --message "Add a task: Review code changes"

# 场景 2: 列出任务
cargo run --release --bin clawmaster -- agent --message "Show me my tasks"

# 场景 3: 完成任务
cargo run --release --bin clawmaster -- agent --message "Mark task 1 as complete"
```

### sessions_list 工具（3个场景）

```bash
# 场景 1: 列出所有会话
cargo run --release --bin clawmaster -- agent --message "List all sessions"

# 场景 2: 活跃会话
cargo run --release --bin clawmaster -- agent --message "Show me active sessions"

# 场景 3: 查找会话
cargo run --release --bin clawmaster -- agent --message "Find session named 'main'"
```

### web_search 工具（3个场景）

```bash
# 场景 1: 技术搜索
cargo run --release --bin clawmaster -- agent --message "Search for Rust programming tutorials"

# 场景 2: 中文搜索
cargo run --release --bin clawmaster -- agent --message "搜索 Rust 编程教程"

# 场景 3: 问题搜索
cargo run --release --bin clawmaster -- agent --message "How to fix async/await in Rust?"
```

... 以及其他 28 个工具的场景

---

## 📊 测试输出说明

### CLI 输出格式

```
[1/96] 测试: calc - 简单算术
  输入: 计算 123 + 456
  ✅ 通过 (2s)

[2/96] 测试: calc - 复杂表达式
  输入: Calculate (15 + 25) * 3
  ✅ 通过 (3s)

[3/96] 测试: calc - 幂运算
  输入: What is 2 to the power of 10?
  ❌ 失败 (exit code: 1, 5s)
```

### 状态标识

- ✅ **通过**: 测试成功完成
- ❌ **失败**: 测试执行失败
- ⏱️ **超时**: 测试超过 30 秒

### 日志文件结构

```
test_logs_20260319_225800/
├── master_test.log           # 主日志（所有测试摘要）
├── 1_calc_简单算术.log        # 测试 1 详细日志
├── 2_calc_复杂表达式.log      # 测试 2 详细日志
├── 3_calc_幂运算.log          # 测试 3 详细日志
├── ...
└── test_report.md            # 测试报告
```

---

## 🔍 观察要点

### 1. 工具调用识别

**查看日志中的**:
```
INFO clawmaster_agents::runner: executing tool tool=calc
```

**验证**:
- ✅ 工具名称正确
- ✅ 工具被成功调用

### 2. 参数提取

**查看日志中的**:
```
INFO clawmaster_agents::runner: executing tool tool=calc args={"expression":"123 + 456"}
```

**验证**:
- ✅ 参数名称正确
- ✅ 参数值正确
- ✅ 参数类型正确

### 3. 迭代次数

**查看日志中的**:
```
INFO clawmaster_agents::runner: streaming LLM response complete iteration=5
```

**验证**:
- ✅ 迭代次数 < 5（理想）
- ⚠️ 迭代次数 5-10（可接受）
- ❌ 迭代次数 > 10（需优化）

### 4. Token 使用

**查看日志中的**:
```
input_tokens=2345 output_tokens=67
```

**验证**:
- ✅ 输入 tokens 合理
- ✅ 输出 tokens 简洁

### 5. 错误信息

**查看日志中的**:
```
ERROR clawmaster_tools::calc: Failed to evaluate expression
WARN clawmaster_agents::runner: High iteration count: 15/20
```

**记录**:
- ❌ 所有 ERROR 级别日志
- ⚠️ 所有 WARN 级别日志

---

## 📈 数据收集

### 需要收集的数据

1. **成功率**
   - 通过的测试数
   - 失败的测试数
   - 超时的测试数

2. **性能数据**
   - 平均响应时间
   - 平均迭代次数
   - 平均 token 使用

3. **错误模式**
   - 常见错误类型
   - 失败的工具
   - 失败的场景

4. **工具调用数据**
   - 工具调用成功率
   - 参数提取准确率
   - 工具执行时间

---

## 🐛 问题识别清单

### 高优先级问题

- [ ] 工具无法被调用
- [ ] 参数提取错误
- [ ] 工具执行失败
- [ ] 系统崩溃

### 中优先级问题

- [ ] 迭代次数过多（> 10）
- [ ] 响应时间过长（> 30s）
- [ ] Token 使用过多
- [ ] 警告日志频繁

### 低优先级问题

- [ ] 结果格式不理想
- [ ] 日志信息不够详细
- [ ] 用户体验可优化

---

## 💡 测试后分析流程

### 1. 收集测试报告

```bash
# 查看测试摘要
cat test_logs_*/test_report.md

# 查看主日志
cat test_logs_*/master_test.log

# 查看失败的测试
grep "❌" test_logs_*/master_test.log
```

### 2. 分析失败原因

```bash
# 查看特定测试的详细日志
cat test_logs_*/3_calc_幂运算.log

# 搜索错误信息
grep -i "error" test_logs_*/*.log
grep -i "warn" test_logs_*/*.log
```

### 3. 统计性能数据

```bash
# 统计迭代次数
grep "iteration=" test_logs_*/*.log

# 统计 token 使用
grep "input_tokens=" test_logs_*/*.log

# 统计工具调用
grep "executing tool" test_logs_*/*.log
```

### 4. 识别代码问题

根据测试结果识别需要补全的代码：
- 工具实现问题
- 参数解析问题
- 错误处理问题
- 性能优化问题

### 5. 补全代码

针对发现的问题进行代码补全和优化

### 6. 生成最终报告

汇总所有测试结果和代码改进

---

## 🎯 预期结果

### 成功标准

**工具调用**:
- ✅ 工具调用成功率 > 95%
- ✅ 参数提取准确率 > 95%

**性能**:
- ✅ 平均迭代次数 < 5
- ✅ 平均响应时间 < 10s
- ✅ 超时率 < 5%

**代码质量**:
- ✅ 无系统崩溃
- ✅ 错误处理完善
- ✅ 日志信息完整

---

## 📝 测试执行步骤

### 步骤 1: 准备环境

```bash
cd /Users/arksong/ClawMaster

# 确保脚本可执行
chmod +x test_all_tools_cli.sh quick_test_sample.sh

# 检查编译
cargo build --release --bin clawmaster
```

### 步骤 2: 选择测试方式

**选项 A - 完整测试**:
```bash
./test_all_tools_cli.sh
```

**选项 B - 快速测试**:
```bash
./quick_test_sample.sh
```

**选项 C - 手动测试**:
```bash
cargo run --release --bin clawmaster -- agent --message "你的测试"
```

### 步骤 3: 监控测试过程

- 观察终端输出
- 查看测试进度
- 记录异常情况

### 步骤 4: 收集测试结果

```bash
# 查看测试报告
cat test_logs_*/test_report.md

# 查看主日志
less test_logs_*/master_test.log
```

### 步骤 5: 分析测试数据

- 统计成功率
- 分析失败原因
- 识别性能问题
- 发现代码缺陷

### 步骤 6: 补全代码

- 修复发现的问题
- 优化性能瓶颈
- 增强错误处理
- 改进用户体验

### 步骤 7: 生成最终报告

- 汇总测试结果
- 记录代码改进
- 提供优化建议

---

## 🎉 开始测试

**推荐方式**: 先运行快速测试，验证系统正常后再运行完整测试

```bash
# 1. 快速测试（5-10分钟）
./quick_test_sample.sh

# 2. 如果快速测试通过，运行完整测试（1-2小时）
./test_all_tools_cli.sh
```

**测试过程中你将看到**:
- ✅ 实时测试进度
- ✅ 彩色状态输出
- ✅ 详细日志信息
- ✅ 自动生成报告

---

**文档状态**: ✅ 完整  
**测试准备**: ✅ 就绪  
**开始测试**: 运行上述脚本即可
