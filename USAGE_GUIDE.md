# ClawMaster 完整使用指南

**版本**: 0.10.18  
**更新日期**: 2026-03-13

---

## 📖 目录

1. [快速开始](#快速开始)
2. [核心功能](#核心功能)
3. [Web UI 使用](#web-ui-使用)
4. [命令行使用](#命令行使用)
5. [API 使用](#api-使用)
6. [高级功能](#高级功能)
7. [最佳实践](#最佳实践)
8. [故障排除](#故障排除)

---

## 🚀 快速开始

### 安装

```bash
# 一键安装
curl -fsSL https://www.clawmaster.org/install.sh | sh

# 或从源码构建
git clone https://github.com/clawmaster-org/clawmaster.git
cd clawmaster
cargo build --release
```

### 首次配置

```bash
# 运行交互式设置向导
clawmaster setup
```

向导将引导您：
1. 选择 LLM 提供商（OpenAI、Anthropic、Ollama 等）
2. 配置 API 密钥
3. 选择通信通道（Web UI、Telegram、Discord 等）
4. 配置 Bot Token
5. 测试连接

### 启动服务

```bash
# 前台运行
clawmaster

# 后台运行
clawmaster --daemon

# 指定配置文件
clawmaster --config /path/to/config.toml
```

---

## 🎯 核心功能

### 1. 多提供商支持

ClawMaster 支持同时使用多个 LLM 提供商：

```bash
# 列出所有提供商
clawmaster provider list

# 添加提供商
clawmaster provider add openai --key sk-your-key
clawmaster provider add anthropic --key sk-ant-your-key

# 设置默认提供商
clawmaster provider set-default openai

# 测试提供商
clawmaster provider test openai
```

### 2. 多通道支持

支持多种通信方式：

- **Web UI** - 浏览器界面（默认启用）
- **Telegram** - 移动端和群组
- **Discord** - 社区和服务器
- **Slack** - 企业协作
- **API** - 自定义集成

```bash
# 启用通道
clawmaster channel enable telegram
clawmaster channel enable discord

# 查看通道状态
clawmaster channel list

# 重启通道
clawmaster channel restart telegram
```

### 3. P0 企业级功能

7 个 DO-178C Level A 合规的企业级功能：

```bash
# 查看系统健康
clawmaster health

# 查看系统指标
clawmaster metrics

# 创建备份
clawmaster backup create

# 查看审计日志
clawmaster audit logs --tail 100

# 查看资源配额
clawmaster quota status
```

---

## 🎨 Web UI 使用

### 访问 Web UI

```
https://localhost:13131
```

### 主要功能

#### 1. 聊天界面

- 发送消息：输入文本后按 `Ctrl+Enter`
- 新建聊天：按 `Ctrl+N`
- 清空聊天：按 `Ctrl+K`
- 搜索消息：按 `Ctrl+F`

#### 2. 工具执行可视化

实时查看 AI 使用的工具：

1. 在聊天时，工具调用会自动显示在右侧面板
2. 点击工具节点查看详细信息
3. 展开/折叠查看参数和结果
4. 查看执行时间和状态

#### 3. 设置面板

按 `Ctrl+,` 打开设置，包含 5 个类别：

**LLM 提供商**:
- 启用/禁用提供商
- 配置 API 密钥
- 选择默认模型

**通道**:
- 启用/禁用通道
- 配置 Bot Token
- 管理通道设置

**外观**:
- 选择主题（Light、Dark、Auto、High Contrast）
- 调整字体大小
- 启用紧凑模式
- 显示/隐藏时间戳

**P0 功能**:
- 配置健康检查间隔
- 设置速率限制
- 启用自动备份
- 配置审计日志

**高级**:
- 最大上下文长度
- 工具执行超时
- 调试模式
- 重置设置

#### 4. 命令面板

按 `Ctrl+P` 打开命令面板：

- 输入命令名称快速搜索
- 使用 `↑↓` 导航
- 按 `Enter` 执行
- 按 `Esc` 关闭

**常用命令**:
- "Go to Chat" - 前往聊天
- "New Chat" - 新建聊天
- "Switch Provider" - 切换提供商
- "Toggle Dark Mode" - 切换暗色模式
- "Show Shortcuts" - 显示快捷键

#### 5. 键盘快捷键

按 `Ctrl+Shift+/` 查看所有快捷键。

**导航**:
- `Ctrl+1` - 前往聊天
- `Ctrl+2` - 前往智能体
- `Ctrl+3` - 前往提供商
- `Ctrl+4` - 前往设置

**聊天**:
- `Ctrl+N` - 新建聊天
- `Ctrl+K` - 清空聊天
- `Ctrl+Enter` - 发送消息
- `Shift+Enter` - 换行

**UI**:
- `Ctrl+,` - 打开设置
- `Ctrl+P` - 命令面板
- `Ctrl+Shift+D` - 切换暗色模式
- `Ctrl+/` - 切换侧边栏

---

## 💻 命令行使用

### 基本命令

```bash
# 启动服务
clawmaster

# 查看版本
clawmaster --version

# 查看帮助
clawmaster --help

# 运行设置向导
clawmaster setup
```

### 提供商管理

```bash
# 列出提供商
clawmaster provider list

# 添加提供商
clawmaster provider add openai --key sk-xxx
clawmaster provider add anthropic --key sk-ant-xxx
clawmaster provider add ollama --url http://localhost:11434

# 测试提供商
clawmaster provider test openai
clawmaster provider test-all

# 设置默认提供商
clawmaster provider set-default openai

# 删除提供商
clawmaster provider remove openrouter

# 查看使用统计
clawmaster provider stats
clawmaster provider cost --period month
```

### 通道管理

```bash
# 列出通道
clawmaster channel list

# 启用通道
clawmaster channel enable telegram
clawmaster channel enable discord

# 添加通道配置
clawmaster channel add telegram --token YOUR_TOKEN
clawmaster channel add discord --token YOUR_TOKEN

# 启动/停止通道
clawmaster channel start telegram
clawmaster channel stop telegram
clawmaster channel restart telegram

# 查看通道日志
clawmaster channel logs telegram --tail 100
clawmaster channel logs telegram --follow

# 查看通道统计
clawmaster channel stats
```

### 聊天交互

```bash
# 发送单条消息
clawmaster chat "你好，请介绍一下你自己"

# 指定模型
clawmaster chat --model gpt-4 "写一段代码"

# 指定会话
clawmaster chat --session my-session "继续之前的对话"

# 流式输出
clawmaster chat --stream "讲一个长故事"
```

### 系统管理

```bash
# 查看系统状态
clawmaster status

# 查看健康状态
clawmaster health
clawmaster health --component database
clawmaster health --component providers

# 查看系统指标
clawmaster metrics

# 查看日志
clawmaster logs --tail 100
clawmaster logs --follow
clawmaster logs --level error
```

### 备份和恢复

```bash
# 创建备份
clawmaster backup create
clawmaster backup create --output /path/to/backup.tar.gz

# 列出备份
clawmaster backup list

# 恢复备份
clawmaster backup restore backup-2026-03-13.tar.gz

# 自动备份
clawmaster backup auto-enable --interval 24h
```

### 审计日志

```bash
# 查看审计日志
clawmaster audit logs --tail 100

# 按类型过滤
clawmaster audit logs --type api_call
clawmaster audit logs --type config_change

# 按用户过滤
clawmaster audit logs --user admin

# 导出审计日志
clawmaster audit export --output audit-2026-03.json
```

---

## 🔌 API 使用

### REST API

#### 发送消息

```bash
curl -X POST https://localhost:13131/api/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "message": "你好",
    "session_id": "user-123",
    "model": "gpt-4"
  }'
```

#### 流式响应

```bash
curl -X POST https://localhost:13131/api/chat/stream \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "message": "写一个长故事",
    "session_id": "user-123"
  }'
```

#### 查看健康状态

```bash
curl https://localhost:13131/api/p0/health
```

#### 查看系统指标

```bash
curl https://localhost:13131/api/p0/metrics
```

### WebSocket API

```javascript
const ws = new WebSocket('wss://localhost:13131/ws');

// 认证
ws.onopen = () => {
  ws.send(JSON.stringify({
    type: 'auth',
    token: 'YOUR_API_KEY'
  }));
};

// 发送消息
ws.send(JSON.stringify({
  type: 'chat',
  message: '你好',
  session_id: 'user-123'
}));

// 接收响应
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log(data);
};
```

### API 密钥管理

```bash
# 创建 API 密钥
clawmaster api-key create --name "My App" --expires 90d

# 列出 API 密钥
clawmaster api-key list

# 撤销 API 密钥
clawmaster api-key revoke KEY_ID
```

---

## 🎓 高级功能

### 1. 使用工具

ClawMaster 支持多种工具，AI 可以自动调用：

**内置工具**:
- `bash` - 执行 shell 命令
- `file_read` - 读取文件
- `file_write` - 写入文件
- `web_search` - 网页搜索
- `web_fetch` - 获取网页内容

**示例对话**:
```
用户: 帮我搜索 Rust 异步编程的最佳实践
AI: [使用 web_search 工具]
    找到了以下资源...

用户: 把搜索结果保存到文件
AI: [使用 file_write 工具]
    已保存到 rust-async-best-practices.md
```

### 2. MCP 服务器

Model Context Protocol 允许扩展工具能力：

```bash
# 添加 MCP 服务器
clawmaster mcp add filesystem --path /path/to/project

# 列出 MCP 服务器
clawmaster mcp list

# 测试 MCP 服务器
clawmaster mcp test filesystem

# 删除 MCP 服务器
clawmaster mcp remove filesystem
```

### 3. 技能系统

技能是预定义的提示词模板：

**创建技能** `~/.config/clawmaster/skills/code-review.toml`:

```toml
name = "Code Review"
description = "Perform comprehensive code review"

[parameters]
language = { type = "string", required = true }
file_path = { type = "string", required = true }

[prompts]
system = """
You are an expert code reviewer.
Analyze for security, performance, and best practices.
"""

user = """
Review the following {{language}} code:
File: {{file_path}}
"""
```

**使用技能**:
```bash
clawmaster skill run code-review \
  --language rust \
  --file-path src/main.rs
```

### 4. 记忆系统

ClawMaster 维护长期记忆：

```bash
# 查看记忆
clawmaster memory list

# 搜索记忆
clawmaster memory search "Rust async"

# 添加记忆
clawmaster memory add "重要信息" --tags rust,async

# 删除记忆
clawmaster memory delete MEMORY_ID
```

### 5. 调度任务

使用 cron 表达式调度任务：

```bash
# 添加定时任务
clawmaster cron add "0 9 * * *" "每日总结" \
  --prompt "总结昨天的对话"

# 列出任务
clawmaster cron list

# 删除任务
clawmaster cron remove JOB_ID
```

---

## 💡 最佳实践

### 1. 安全配置

**保护 API 密钥**:
```bash
# 使用环境变量
export OPENAI_API_KEY=sk-xxx

# 不要在配置文件中硬编码
# ❌ api_key = "sk-xxx"
# ✅ api_key_env = "OPENAI_API_KEY"
```

**启用身份验证**:
```bash
clawmaster auth set-password
clawmaster auth enable-passkey
```

**配置 HTTPS**:
```toml
[server]
tls_enabled = true
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
```

### 2. 性能优化

**选择合适的模型**:
- 简单任务：GPT-3.5-turbo（快速且便宜）
- 复杂任务：GPT-4（高质量）
- 本地开发：Ollama（免费）

**配置缓存**:
```toml
[cache]
enabled = true
ttl_seconds = 3600
max_size_mb = 1024
```

**启用流式响应**:
```bash
clawmaster chat --stream "长文本生成"
```

### 3. 监控和日志

**启用详细日志**:
```toml
[logging]
level = "debug"
file = "/var/log/clawmaster/app.log"
```

**配置指标收集**:
```toml
[metrics]
enabled = true
prometheus_port = 9090
```

**设置告警**:
```toml
[alerts]
error_threshold = 10
notify_channel = "telegram"
notify_user_id = 123456789
```

### 4. 备份策略

**自动备份**:
```bash
clawmaster backup auto-enable --interval 24h
```

**备份到远程**:
```toml
[backup]
remote_enabled = true
remote_type = "s3"
remote_bucket = "clawmaster-backups"
```

---

## 🐛 故障排除

### 常见问题

#### 1. 服务无法启动

**检查端口占用**:
```bash
lsof -i :13131
```

**更改端口**:
```toml
[server]
port = 13132
```

#### 2. API 调用失败

**检查 API 密钥**:
```bash
clawmaster provider test openai --verbose
```

**查看详细日志**:
```bash
clawmaster logs --level debug --tail 100
```

#### 3. 数据库错误

**重建数据库**:
```bash
clawmaster db reset
```

**迁移数据库**:
```bash
clawmaster db migrate
```

#### 4. 内存占用过高

**调整资源配额**:
```toml
[p0features.resource_quota]
max_memory_mb = 512
max_sessions = 100
```

**清理缓存**:
```bash
clawmaster cache clear
```

#### 5. Telegram Bot 无响应

**检查 Token**:
```bash
curl https://api.telegram.org/bot<TOKEN>/getMe
```

**重启通道**:
```bash
clawmaster channel restart telegram
```

### 获取帮助

- 📖 [完整文档](https://docs.clawmaster.org)
- 💬 [Discord 社区](https://discord.gg/clawmaster)
- 🐛 [GitHub Issues](https://github.com/clawmaster-org/clawmaster/issues)
- 📧 [邮件支持](mailto:support@clawmaster.org)

---

## 📚 相关资源

### 教程
- [快速开始](docs/tutorials/01-quick-start.md)
- [配置提供商](docs/tutorials/02-configure-providers.md)
- [设置通道](docs/tutorials/03-setup-channels.md)

### 文档
- [P0 功能文档](P0_COMPLETION_SUMMARY.md)
- [Web UI 改进](WEB_UI_IMPROVEMENTS.md)
- [项目总结](COMPLETE_PROJECT_SUMMARY_2026-03-13.md)

---

**版本**: 0.10.18  
**更新日期**: 2026-03-13  
**状态**: ✅ 完整且最新
