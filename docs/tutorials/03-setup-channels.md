# 设置通信通道

本教程将指导您如何配置 ClawMaster 的各种通信通道，让用户可以通过多种方式与 AI 交互。

---

## 📋 支持的通道

| 通道 | 类型 | 用途 | 需要 Token |
|------|------|------|------------|
| **Web UI** | 浏览器 | 主要界面 | ❌ |
| **Telegram** | 消息应用 | 移动端、群组 | ✅ |
| **Discord** | 社区平台 | 服务器、频道 | ✅ |
| **Slack** | 企业协作 | 工作空间 | ✅ |
| **API** | HTTP/WebSocket | 自定义集成 | ❌ |

---

## 🌐 Web UI（默认启用）

Web UI 是 ClawMaster 的主要界面，始终启用。

### 访问 Web UI

```
https://localhost:13131
```

### 首次登录

1. 访问 Web UI
2. 设置管理员密码
3. （可选）启用 Passkey 认证

### 功能特性

- ✅ 实时聊天
- ✅ 工具执行可视化
- ✅ 设置管理
- ✅ 命令面板（Ctrl+P）
- ✅ 键盘快捷键
- ✅ 多主题支持
- ✅ PWA 支持（可安装）

### 自定义配置

```toml
[channels.web]
enabled = true
host = "127.0.0.1"
port = 13131
tls_enabled = true
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
```

---

## 📱 Telegram

### 创建 Telegram Bot

1. 在 Telegram 中搜索 [@BotFather](https://t.me/botfather)
2. 发送 `/newbot` 命令
3. 按提示设置 Bot 名称和用户名
4. 复制 Bot Token（格式：`123456:ABC-DEF...`）

### 配置 ClawMaster

**方法 1: Web UI**
1. 打开设置 → Channels
2. 启用 Telegram
3. 粘贴 Bot Token
4. 保存

**方法 2: 命令行**
```bash
clawmaster channel add telegram --token YOUR_BOT_TOKEN
clawmaster channel enable telegram
```

**方法 3: 配置文件**
```bash
# ~/.config/clawmaster/.env
TELEGRAM_BOT_TOKEN=123456:ABC-DEF...
```

```toml
# ~/.config/clawmaster/clawmaster.toml
[channels.telegram]
enabled = true
token_env = "TELEGRAM_BOT_TOKEN"
```

### 启动 Bot

```bash
clawmaster channel start telegram
```

### 使用 Bot

1. 在 Telegram 中搜索您的 Bot
2. 点击 "Start"
3. 发送消息开始对话

### 高级功能

#### 群组支持

1. 将 Bot 添加到群组
2. 授予管理员权限（可选）
3. 使用 `/start` 激活
4. 使用 `@YourBot` 提及 Bot

#### 命令支持

```
/start - 开始对话
/help - 显示帮助
/reset - 重置会话
/settings - 查看设置
/model - 切换模型
```

#### 权限控制

```toml
[channels.telegram.access_control]
allowed_users = [123456789, 987654321]
allowed_groups = [-1001234567890]
admin_users = [123456789]
```

#### Webhook 模式（推荐用于生产）

```toml
[channels.telegram]
mode = "webhook"
webhook_url = "https://your-domain.com/telegram/webhook"
webhook_secret = "your-secret-token"
```

---

## 💬 Discord

### 创建 Discord Bot

1. 访问 [Discord Developer Portal](https://discord.com/developers/applications)
2. 点击 "New Application"
3. 进入 "Bot" 标签
4. 点击 "Add Bot"
5. 复制 Bot Token
6. 启用 "Message Content Intent"

### 邀请 Bot 到服务器

1. 进入 "OAuth2" → "URL Generator"
2. 选择 Scopes: `bot`, `applications.commands`
3. 选择 Permissions:
   - Send Messages
   - Read Message History
   - Use Slash Commands
4. 复制生成的 URL
5. 在浏览器中打开并选择服务器

### 配置 ClawMaster

**环境变量**:
```bash
export DISCORD_BOT_TOKEN=your-discord-token
```

**配置文件**:
```toml
[channels.discord]
enabled = true
token_env = "DISCORD_BOT_TOKEN"
```

### 启动 Bot

```bash
clawmaster channel start discord
```

### 使用 Bot

**直接消息**:
```
@YourBot 你好
```

**斜杠命令**:
```
/chat 帮我写一段代码
/model gpt-4
/reset
```

### 高级功能

#### 频道配置

```toml
[channels.discord.channels]
allowed_channels = [123456789, 987654321]
admin_channels = [123456789]
```

#### 角色权限

```toml
[channels.discord.roles]
allowed_roles = ["Member", "Premium"]
admin_roles = ["Admin", "Moderator"]
```

#### 自定义前缀

```toml
[channels.discord]
command_prefix = "!"
mention_required = false
```

---

## 💼 Slack

### 创建 Slack App

1. 访问 [Slack API](https://api.slack.com/apps)
2. 点击 "Create New App"
3. 选择 "From scratch"
4. 输入 App 名称和工作空间
5. 进入 "OAuth & Permissions"
6. 添加 Bot Token Scopes:
   - `chat:write`
   - `channels:history`
   - `groups:history`
   - `im:history`
   - `mpim:history`
7. 安装 App 到工作空间
8. 复制 Bot User OAuth Token

### 配置 ClawMaster

**环境变量**:
```bash
export SLACK_BOT_TOKEN=xoxb-your-token
```

**配置文件**:
```toml
[channels.slack]
enabled = true
token_env = "SLACK_BOT_TOKEN"
```

### 启动 Bot

```bash
clawmaster channel start slack
```

### 使用 Bot

**直接消息**:
1. 在 Slack 中找到您的 Bot
2. 发送消息

**频道中**:
```
@YourBot 帮我总结这个讨论
```

**斜杠命令**:
```
/clawmaster chat 你好
/clawmaster model gpt-4
```

### 高级功能

#### 事件订阅

```toml
[channels.slack.events]
url = "https://your-domain.com/slack/events"
verification_token = "your-verification-token"
```

#### 交互式组件

```toml
[channels.slack.interactive]
enabled = true
url = "https://your-domain.com/slack/interactive"
```

---

## 🔌 API 访问

### REST API

**发送消息**:
```bash
curl -X POST https://localhost:13131/api/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "message": "你好",
    "session_id": "user-123"
  }'
```

**流式响应**:
```bash
curl -X POST https://localhost:13131/api/chat/stream \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "message": "写一个长故事",
    "session_id": "user-123"
  }'
```

### WebSocket

```javascript
const ws = new WebSocket('wss://localhost:13131/ws');

ws.onopen = () => {
  ws.send(JSON.stringify({
    type: 'auth',
    token: 'YOUR_API_KEY'
  }));
  
  ws.send(JSON.stringify({
    type: 'chat',
    message: '你好',
    session_id: 'user-123'
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log(data);
};
```

### 生成 API 密钥

```bash
clawmaster api-key create --name "My App" --expires 90d
```

---

## 🎯 多通道管理

### 列出所有通道

```bash
clawmaster channel list
```

输出示例:
```
Channel    Status    Users    Messages/day
────────────────────────────────────────
web        ✓ Active  5        120
telegram   ✓ Active  12       450
discord    ✓ Active  8        200
slack      ✗ Disabled 0       0
```

### 启用/禁用通道

```bash
# 启用
clawmaster channel enable discord

# 禁用
clawmaster channel disable slack
```

### 重启通道

```bash
clawmaster channel restart telegram
```

### 查看通道日志

```bash
clawmaster channel logs telegram --tail 100
```

---

## 🔒 安全配置

### 访问控制

```toml
[channels.access_control]
# 全局白名单
allowed_users = ["user1", "user2"]

# 黑名单
blocked_users = ["spammer1"]

# 速率限制
rate_limit_per_user = 60  # 每分钟
rate_limit_per_channel = 1000
```

### 内容过滤

```toml
[channels.content_filter]
enabled = true
max_message_length = 4000
block_urls = false
block_mentions = false
profanity_filter = true
```

### 审计日志

```toml
[channels.audit]
enabled = true
log_all_messages = true
log_user_info = true
retention_days = 90
```

---

## 📊 监控和统计

### 查看通道统计

```bash
clawmaster channel stats
```

### 实时监控

```bash
clawmaster channel monitor --channel telegram
```

### 导出数据

```bash
clawmaster channel export telegram --format json --output telegram-data.json
```

---

## 🐛 故障排除

### Telegram Bot 无响应

**检查 Token**:
```bash
curl https://api.telegram.org/bot<YOUR_TOKEN>/getMe
```

**检查 Webhook**:
```bash
clawmaster channel test telegram
```

### Discord Bot 离线

**检查 Intents**:
确保在 Discord Developer Portal 中启用了必要的 Intents。

**检查权限**:
```bash
clawmaster channel permissions discord
```

### Slack 事件未接收

**检查事件订阅 URL**:
```bash
clawmaster channel verify-webhook slack
```

**重新订阅事件**:
```bash
clawmaster channel resubscribe slack
```

---

## 💡 最佳实践

### 1. 使用多通道

为不同用户群体提供不同通道：
- **Web UI** - 技术用户、管理员
- **Telegram** - 移动用户、个人使用
- **Discord** - 社区、开发者
- **Slack** - 企业团队

### 2. 配置通知

```toml
[channels.notifications]
error_channel = "telegram"
admin_user_id = 123456789
alert_on_errors = true
daily_summary = true
```

### 3. 负载均衡

```toml
[channels.load_balancing]
max_concurrent_per_channel = 100
queue_size = 1000
timeout_seconds = 30
```

### 4. 备份配置

```bash
clawmaster channel export-config --output channels-backup.toml
```

---

## 📚 下一步

- [使用工具和技能](04-using-tools.md)
- [记忆系统](05-memory-system.md)
- [生产部署](06-production-deployment.md)

---

**版本**: 0.10.18  
**更新日期**: 2026-03-13  
**难度**: 中级  
**预计时间**: 15-20 分钟
