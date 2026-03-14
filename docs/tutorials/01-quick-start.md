# ClawMaster 快速开始指南

欢迎使用 ClawMaster！本指南将帮助您在 5 分钟内完成安装和首次配置。

---

## 📋 前置要求

- macOS 10.15+ / Linux / Windows 10+
- 至少 500MB 可用磁盘空间
- 互联网连接（用于下载和 LLM API 调用）

---

## 🚀 安装

### 方法 1: 一键安装（推荐）

**macOS / Linux**:
```bash
curl -fsSL https://www.clawmaster.org/install.sh | sh
```

**Windows (PowerShell)**:
```powershell
iwr https://www.clawmaster.org/install.ps1 -useb | iex
```

### 方法 2: 从源码构建

```bash
# 克隆仓库
git clone https://github.com/clawmaster-org/clawmaster.git
cd clawmaster

# 构建
cargo build --release

# 安装
cargo install --path crates/cli
```

### 方法 3: 下载预编译二进制

访问 [GitHub Releases](https://github.com/clawmaster-org/clawmaster/releases) 下载适合您系统的版本。

---

## ⚙️ 首次配置

### 使用交互式设置向导（推荐）

安装完成后，运行设置向导：

```bash
clawmaster setup
```

向导将引导您完成：
1. ✅ 选择 LLM 提供商
2. ✅ 配置 API 密钥
3. ✅ 选择通信通道
4. ✅ 配置 Bot Token
5. ✅ 测试连接
6. ✅ 保存配置

**预计时间**: 2-3 分钟

### 手动配置（高级用户）

创建配置文件 `~/.config/clawmaster/clawmaster.toml`:

```toml
# ClawMaster 配置文件

[providers]
openai = true
anthropic = false

[channels]
web = true
telegram = false
discord = false

[server]
host = "127.0.0.1"
port = 13131
```

创建环境变量文件 `~/.config/clawmaster/.env`:

```bash
# LLM API 密钥
OPENAI_API_KEY=sk-your-api-key-here

# 通道 Token（可选）
# TELEGRAM_BOT_TOKEN=123456:ABC-DEF...
# DISCORD_BOT_TOKEN=your-discord-token
```

---

## 🎯 启动 ClawMaster

### 启动服务

```bash
clawmaster
```

您将看到类似输出：

```
🦾 ClawMaster v0.10.18
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✓ Configuration loaded
✓ Database initialized
✓ Providers initialized (1 active)
✓ Channels initialized (1 active)
✓ P0 features enabled (7 active)

🌐 Web UI: https://localhost:13131
📊 Health Check: https://localhost:13131/api/p0/health
📈 Metrics: https://localhost:13131/api/p0/metrics

Press Ctrl+C to stop
```

### 后台运行

```bash
# 使用 systemd (Linux)
sudo systemctl start clawmaster

# 使用 launchd (macOS)
launchctl load ~/Library/LaunchAgents/org.clawmaster.plist

# 使用 Docker
docker run -d -p 13131:13131 clawmaster/clawmaster:latest
```

---

## 💬 发送第一条消息

### 使用 Web UI

1. 打开浏览器访问 `https://localhost:13131`
2. 首次访问会提示设置密码
3. 登录后进入聊天界面
4. 在输入框输入消息，例如：

```
你好！请介绍一下你自己。
```

5. 按 `Ctrl+Enter` 或点击发送按钮

### 使用 API

```bash
curl -X POST https://localhost:13131/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "你好！请介绍一下你自己。",
    "session_id": "test-session"
  }'
```

### 使用 Telegram（如果已配置）

1. 在 Telegram 中搜索您的 Bot
2. 点击 "Start" 开始对话
3. 发送消息：`你好！`

---

## 🎨 Web UI 功能介绍

### 主要功能

1. **聊天界面** - 与 AI 对话
2. **工具可视化** - 实时查看工具执行
3. **设置面板** - 配置所有选项
4. **命令面板** - 快速访问功能（按 `Ctrl+P`）
5. **键盘快捷键** - 高效操作（按 `Ctrl+Shift+/` 查看）

### 快捷键速查

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+P` | 打开命令面板 |
| `Ctrl+N` | 新建聊天 |
| `Ctrl+K` | 清空聊天 |
| `Ctrl+Enter` | 发送消息 |
| `Ctrl+,` | 打开设置 |
| `Ctrl+Shift+D` | 切换暗色模式 |
| `Ctrl+Shift+/` | 显示所有快捷键 |

---

## 🔧 常见任务

### 切换 LLM 提供商

**Web UI**:
1. 按 `Ctrl+,` 打开设置
2. 点击 "LLM Providers" 标签
3. 启用/禁用提供商
4. 输入 API 密钥
5. 点击 "Save Changes"

**命令行**:
```bash
clawmaster provider add openai --key sk-your-key
clawmaster provider list
clawmaster provider set-default openai
```

### 添加新通道

**Web UI**:
1. 打开设置 → "Channels" 标签
2. 启用通道（如 Telegram）
3. 输入 Bot Token
4. 保存配置

**命令行**:
```bash
clawmaster channel add telegram --token YOUR_BOT_TOKEN
clawmaster channel enable telegram
```

### 查看系统状态

**Web UI**:
- 访问 `https://localhost:13131/api/p0/health`

**命令行**:
```bash
clawmaster status
clawmaster health
```

### 备份数据

```bash
# 创建备份
clawmaster backup create

# 列出备份
clawmaster backup list

# 恢复备份
clawmaster backup restore backup-2026-03-13.tar.gz
```

---

## 🛠️ 高级功能

### 使用工具

ClawMaster 支持多种工具，包括：

- **bash** - 执行 shell 命令
- **file_read** - 读取文件
- **file_write** - 写入文件
- **web_search** - 网页搜索
- **web_fetch** - 获取网页内容

**示例对话**:
```
用户: 帮我搜索一下 Rust 异步编程的最佳实践
AI: [使用 web_search 工具搜索]
    [返回搜索结果和总结]

用户: 把搜索结果保存到文件
AI: [使用 file_write 工具保存]
    已保存到 rust-async-best-practices.md
```

### 使用 MCP 服务器

```bash
# 添加 MCP 服务器
clawmaster mcp add filesystem --path /path/to/project

# 列出 MCP 服务器
clawmaster mcp list

# 测试 MCP 服务器
clawmaster mcp test filesystem
```

### 创建技能

技能是预定义的提示词模板，可以快速执行常见任务。

**创建技能文件** `~/.config/clawmaster/skills/code-review.toml`:

```toml
name = "Code Review"
description = "Perform comprehensive code review"

[parameters]
language = { type = "string", required = true }
file_path = { type = "string", required = true }

[prompts]
system = """
You are an expert code reviewer. Analyze the code for:
- Security vulnerabilities
- Performance issues
- Code style violations
- Best practices
"""

user = """
Review the following {{language}} code:
File: {{file_path}}
"""
```

**使用技能**:
```bash
clawmaster skill run code-review --language rust --file-path src/main.rs
```

---

## 📊 监控和日志

### 查看日志

```bash
# 实时日志
clawmaster logs --follow

# 查看最近 100 行
clawmaster logs --tail 100

# 按级别过滤
clawmaster logs --level error
```

### 查看指标

**Web UI**:
- 访问 `https://localhost:13131/api/p0/metrics`

**命令行**:
```bash
clawmaster metrics
```

### 健康检查

```bash
# 完整健康检查
clawmaster health

# 仅检查特定组件
clawmaster health --component database
clawmaster health --component providers
```

---

## 🔒 安全最佳实践

### 1. 保护 API 密钥

- ✅ 使用环境变量存储 API 密钥
- ✅ 不要将 `.env` 文件提交到版本控制
- ✅ 定期轮换 API 密钥
- ❌ 不要在配置文件中硬编码密钥

### 2. 启用身份验证

```bash
# 设置密码
clawmaster auth set-password

# 启用 Passkey（WebAuthn）
clawmaster auth enable-passkey
```

### 3. 配置 HTTPS

```toml
[server]
tls_enabled = true
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"
```

### 4. 启用审计日志

```toml
[p0features]
audit_log = true
```

---

## 🐛 故障排除

### 问题：无法启动服务

**检查端口占用**:
```bash
lsof -i :13131
```

**更改端口**:
```toml
[server]
port = 13132
```

### 问题：API 调用失败

**检查 API 密钥**:
```bash
clawmaster provider test openai
```

**查看详细日志**:
```bash
clawmaster logs --level debug
```

### 问题：数据库错误

**重建数据库**:
```bash
clawmaster db reset
```

**迁移数据库**:
```bash
clawmaster db migrate
```

### 问题：内存占用过高

**调整配额**:
```toml
[p0features.resource_quota]
max_memory_mb = 512
max_sessions = 100
```

---

## 📚 下一步

恭喜！您已经完成了 ClawMaster 的快速开始。

**继续学习**:
- [配置 LLM 提供商](02-configure-providers.md)
- [设置通信通道](03-setup-channels.md)
- [使用工具和技能](04-using-tools.md)
- [记忆系统](05-memory-system.md)
- [生产部署](06-production-deployment.md)

**获取帮助**:
- 📖 [完整文档](https://docs.clawmaster.org)
- 💬 [Discord 社区](https://discord.gg/clawmaster)
- 🐛 [GitHub Issues](https://github.com/clawmaster-org/clawmaster/issues)
- 📧 [邮件支持](mailto:support@clawmaster.org)

---

## 🎉 总结

您现在已经：
- ✅ 安装了 ClawMaster
- ✅ 完成了首次配置
- ✅ 发送了第一条消息
- ✅ 了解了基本功能
- ✅ 知道如何获取帮助

**开始使用 ClawMaster，享受 AI 助手的强大功能！** 🚀

---

**版本**: 0.10.18  
**更新日期**: 2026-03-13  
**难度**: 初级  
**预计时间**: 5-10 分钟
