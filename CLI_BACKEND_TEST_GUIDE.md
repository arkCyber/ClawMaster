# ClawMaster CLI 后端测试指南

**更新时间**: 2026-03-20 07:50

---

## ✅ 完成的工作

### 1. CLI 客户端实现 ✅

**新增文件**: `crates/cli/src/agent_client.rs`

**功能**:
- WebSocket 连接到后端服务器
- 发送 agent 消息请求
- 接收流式响应
- 错误处理和重试

**协议**: ClawMaster Protocol v4

### 2. CLI 命令更新 ✅

**修改文件**: `crates/cli/src/main.rs`

**更新内容**:
- `agent` 命令现在连接到运行中的后端服务器
- 支持环境变量 `CLAWMASTER_GATEWAY_URL`
- 友好的错误提示

### 3. 测试脚本创建 ✅

**新增文件**: `test_backend_cli.sh`

**功能**:
- 自动测试 15 个场景（可扩展到 96 个）
- 后端服务器健康检查
- 详细日志记录
- 自动生成测试报告

---

## 🚀 快速开始

### 步骤 1: 启动后端服务器

在一个终端窗口中启动后端服务器：

```bash
clawmaster gateway
```

**确认服务器运行**:
- 看到 "Gateway listening on..." 消息
- 可以访问 http://localhost:3000/health

### 步骤 2: 运行 CLI 测试

在另一个终端窗口中运行测试：

```bash
# 赋予执行权限
chmod +x test_backend_cli.sh

# 运行测试
./test_backend_cli.sh
```

### 步骤 3: 查看测试结果

测试完成后，查看报告：

```bash
# 查看测试报告
cat backend_test_logs_*/test_report.md

# 查看主日志
cat backend_test_logs_*/master_test.log

# 查看特定测试的详细日志
cat backend_test_logs_*/1_calc_简单算术.log
```

---

## 🔧 配置选项

### 环境变量

```bash
# 设置后端服务器地址（默认: http://localhost:3000）
export CLAWMASTER_GATEWAY_URL=http://localhost:3000

# 运行测试
./test_backend_cli.sh
```

### 手动测试单个命令

```bash
# 测试 calc 工具
./target/release/clawmaster agent --message "计算 123 + 456"

# 测试 web_search 工具
./target/release/clawmaster agent --message "搜索 Rust 教程"

# 测试 task_list 工具
./target/release/clawmaster agent --message "添加任务: 测试代码"
```

---

## 📊 测试覆盖

### 当前测试场景（15 个）

1. **calc** (3 个场景)
   - 简单算术
   - 复杂表达式
   - 幂运算

2. **web_search** (3 个场景)
   - 技术搜索
   - 中文搜索
   - 问题搜索

3. **web_fetch** (3 个场景)
   - 获取网页
   - 获取 API
   - 获取 JSON

4. **task_list** (3 个场景)
   - 添加任务
   - 列出任务
   - 完成任务

5. **sessions_list** (3 个场景)
   - 列出会话
   - 查看会话
   - 搜索会话

### 扩展到 96 个场景

编辑 `test_backend_cli.sh`，添加更多工具的测试：

```bash
# browser 工具测试
run_test 16 "browser" "打开网页" "Open https://www.rust-lang.org in browser"
run_test 17 "browser" "截图" "Take a screenshot of https://github.com"
run_test 18 "browser" "提取内容" "Extract text from https://www.rust-lang.org"

# 继续添加其他工具...
```

---

## 🔍 故障排除

### 问题 1: 无法连接到后端服务器

**错误信息**:
```
❌ 无法连接到后端服务器
```

**解决方法**:
1. 确保后端服务器正在运行
2. 检查服务器地址是否正确
3. 检查防火墙设置

### 问题 2: WebSocket 连接失败

**错误信息**:
```
Connection closed before hello
```

**解决方法**:
1. 检查后端服务器日志
2. 确认 WebSocket 端点可访问
3. 检查网络代理设置

### 问题 3: 测试超时

**原因**:
- LLM 推理时间过长
- 网络请求缓慢
- 工具执行时间过长

**解决方法**:
- 增加超时时间（编辑脚本中的 `timeout=30`）
- 优化 LLM 配置
- 检查网络连接

---

## 📈 测试结果分析

### 查看通过率

```bash
# 查看测试统计
grep "通过率" backend_test_logs_*/test_report.md
```

### 查看失败的测试

```bash
# 列出所有失败的测试
grep "^### ❌" backend_test_logs_*/test_report.md
```

### 查看详细日志

```bash
# 查看特定测试的完整输出
cat backend_test_logs_*/1_calc_简单算术.log
```

---

## 💡 最佳实践

### 1. 测试前准备

- ✅ 确保后端服务器已启动
- ✅ 确认 LLM 提供商已配置
- ✅ 检查网络连接
- ✅ 清理旧的测试日志

### 2. 测试执行

- ✅ 使用独立的终端窗口
- ✅ 监控后端服务器日志
- ✅ 记录异常情况
- ✅ 保存测试结果

### 3. 结果分析

- ✅ 查看通过率
- ✅ 分析失败原因
- ✅ 检查性能指标
- ✅ 生成改进建议

---

## 🎯 下一步

### 扩展测试覆盖

1. 添加更多工具测试（目标: 96 个场景）
2. 添加边界条件测试
3. 添加错误处理测试
4. 添加性能测试

### 自动化改进

1. 集成到 CI/CD 流程
2. 添加测试报告可视化
3. 添加性能基准测试
4. 添加回归测试

### 文档完善

1. 添加更多示例
2. 添加视频教程
3. 添加常见问题解答
4. 添加贡献指南

---

## 📝 技术细节

### CLI 客户端实现

**WebSocket 协议流程**:
1. 连接到 `/ws` 端点
2. 发送 `connect` 请求（v4 协议）
3. 接收 `hello` 响应
4. 发送 `chat.send` 请求
5. 接收流式响应事件
6. 关闭连接

**消息格式**:
```json
{
  "jsonrpc": "2.0",
  "id": "uuid",
  "method": "chat.send",
  "params": {
    "message": "用户消息",
    "session_key": "cli_test_session"
  }
}
```

### 测试脚本架构

**核心函数**:
- `run_test()` - 执行单个测试
- 超时控制 - 使用后台进程 + kill 信号
- 日志记录 - 每个测试独立日志文件
- 报告生成 - Markdown 格式

---

**文档更新时间**: 2026-03-20 07:50  
**CLI 客户端版本**: 0.10.18  
**协议版本**: v4  
**测试状态**: ✅ 就绪
