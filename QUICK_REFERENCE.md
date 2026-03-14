# ClawMaster 快速参考指南

**版本**: 0.10.18  
**更新日期**: 2026-03-13

---

## 🚀 快速开始

### 安装

```bash
# 一键安装
curl -fsSL https://www.clawmaster.org/install.sh | sh

# 或使用 Homebrew
brew tap arksong/clawmaster
brew install clawmaster

# 或从源码构建
git clone https://github.com/arksong/ClawMaster.git
cd ClawMaster
cargo build --release
```

### 首次配置

```bash
# 运行交互式设置向导
clawmaster setup

# 或手动配置
vim ~/.config/clawmaster/clawmaster.toml
```

### 启动服务

```bash
# 启动
clawmaster

# 或指定配置文件
clawmaster --config /path/to/config.toml

# 后台运行
clawmaster &
```

---

## 📋 常用命令

### 基础命令

```bash
clawmaster --help              # 查看帮助
clawmaster --version           # 查看版本
clawmaster health              # 健康检查
clawmaster metrics             # 查看指标
clawmaster status              # 查看状态
```

### 配置管理

```bash
clawmaster setup               # 交互式设置
clawmaster config validate     # 验证配置
clawmaster config show         # 显示配置
clawmaster config reset        # 重置配置
```

### 提供商管理

```bash
clawmaster provider list       # 列出提供商
clawmaster provider add openai --key YOUR_KEY
clawmaster provider remove openai
clawmaster provider test openai
```

### 通道管理

```bash
clawmaster channel list        # 列出通道
clawmaster channel enable telegram
clawmaster channel disable telegram
clawmaster channel test telegram
```

### 备份和恢复

```bash
clawmaster backup create       # 创建备份
clawmaster backup list         # 列出备份
clawmaster backup restore <file>
clawmaster backup clean --older-than 30d
```

### 日志和调试

```bash
clawmaster logs                # 查看日志
clawmaster logs --tail 100     # 最后 100 行
clawmaster logs --follow       # 实时日志
clawmaster logs --level debug  # 调试级别
```

---

## 🌐 Web UI

### 访问地址

```
https://localhost:13131
```

### 键盘快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+P` | 打开命令面板 |
| `Ctrl+N` | 新建聊天 |
| `Ctrl+,` | 打开设置 |
| `Ctrl+K` | 搜索 |
| `Ctrl+Shift+D` | 切换暗色模式 |
| `Ctrl+Shift+/` | 显示所有快捷键 |
| `Ctrl+Enter` | 发送消息 |
| `Esc` | 关闭对话框 |

### 命令面板命令

在 Web UI 中按 `Ctrl+P` 打开命令面板，然后输入：

```
> new chat              # 新建聊天
> settings              # 打开设置
> providers             # 管理提供商
> channels              # 管理通道
> theme dark            # 切换到暗色主题
> theme light           # 切换到亮色主题
> help                  # 显示帮助
> shortcuts             # 显示快捷键
```

---

## 🔧 配置文件

### 位置

```
~/.config/clawmaster/clawmaster.toml
```

### 基础配置

```toml
[server]
host = "127.0.0.1"
port = 13131
tls_enabled = true

[providers.openai]
api_key = "sk-..."
model = "gpt-4"

[channels.telegram]
enabled = true
bot_token = "..."

[channels.web]
enabled = true
```

### 环境变量

```bash
# 提供商 API 密钥
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="sk-ant-..."

# 通道配置
export TELEGRAM_BOT_TOKEN="..."
export DISCORD_BOT_TOKEN="..."

# 服务器配置
export CLAWMASTER_HOST="0.0.0.0"
export CLAWMASTER_PORT="13131"
```

---

## 📊 API 端点

### P0 功能 API

```bash
# 健康检查
curl https://localhost:13131/api/p0/health

# 系统指标
curl https://localhost:13131/api/p0/metrics

# 就绪探针 (Kubernetes)
curl https://localhost:13131/api/p0/ready

# 存活探针 (Kubernetes)
curl https://localhost:13131/api/p0/live
```

### 聊天 API

```bash
# 发送消息
curl -X POST https://localhost:13131/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello"}'

# 流式响应
curl -X POST https://localhost:13131/api/chat/stream \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello"}'
```

### WebSocket

```javascript
const ws = new WebSocket('wss://localhost:13131/ws');
ws.onmessage = (event) => {
  console.log('Received:', event.data);
};
ws.send(JSON.stringify({ message: 'Hello' }));
```

---

## 🗂️ 文件位置

### 配置文件

```
~/.config/clawmaster/
├── clawmaster.toml          # 主配置
├── credentials.json         # 凭证
├── mcp-servers.json         # MCP 服务器
├── provider_keys.json       # 提供商密钥
└── AGENTS.md                # 长期记忆
```

### 数据文件

```
~/.clawmaster/
├── sessions/                # 会话数据
├── memory/                  # 记忆数据
├── backups/                 # 备份文件
├── logs/                    # 日志文件
└── cache/                   # 缓存文件
```

---

## 🔍 故障排除

### 常见问题

**端口被占用**:
```bash
# 查看占用端口的进程
lsof -i :13131

# 或更改配置中的端口
vim ~/.config/clawmaster/clawmaster.toml
```

**配置文件未找到**:
```bash
# 运行设置向导创建配置
clawmaster setup
```

**API 密钥错误**:
```bash
# 重新配置提供商
clawmaster provider add openai --key YOUR_NEW_KEY
```

**数据库错误**:
```bash
# 检查数据库状态
clawmaster db status

# 运行迁移
clawmaster db migrate

# 恢复备份
clawmaster backup restore <backup-file>
```

### 日志级别

```bash
# 设置日志级别
export RUST_LOG=debug
clawmaster

# 或在配置文件中
[logging]
level = "debug"
```

---

## 📚 文档索引

### 快速开始

- [01-quick-start.md](docs/tutorials/01-quick-start.md) - 5-10 分钟
- [02-configure-providers.md](docs/tutorials/02-configure-providers.md) - 10-15 分钟
- [03-setup-channels.md](docs/tutorials/03-setup-channels.md) - 15-20 分钟

### 核心文档

- [USAGE_GUIDE.md](USAGE_GUIDE.md) - 完整使用指南
- [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md) - 部署检查清单
- [PROJECT_INDEX.md](PROJECT_INDEX.md) - 项目索引

### 技术文档

- [P0_COMPLETION_SUMMARY.md](P0_COMPLETION_SUMMARY.md) - P0 功能总结
- [WEB_UI_IMPROVEMENTS.md](WEB_UI_IMPROVEMENTS.md) - Web UI 文档
- [TEST_REPORT_2026-03-13.md](TEST_REPORT_2026-03-13.md) - 测试报告

---

## 🛠️ 开发命令

### 构建

```bash
# 开发构建
cargo build

# Release 构建
cargo build --release

# 特定 crate
cargo build -p clawmaster-gateway
```

### 测试

```bash
# 所有测试
cargo test

# 特定 crate
cargo test -p clawmaster-agents-memory

# 详细输出
cargo test -- --nocapture

# 特定测试
cargo test test_search
```

### 代码质量

```bash
# 格式化
cargo +nightly-2025-11-30 fmt

# Clippy 检查
cargo clippy --workspace --all-features

# 验证脚本
./scripts/verify-project.sh
```

---

## 🐳 Docker 部署

### 基础部署

```bash
docker run -d \
  --name clawmaster \
  -p 13131:13131 \
  -v clawmaster-config:/root/.config/clawmaster \
  -v clawmaster-data:/root/.clawmaster \
  ghcr.io/arksong/clawmaster:latest
```

### Docker Compose

```yaml
version: '3.8'
services:
  clawmaster:
    image: ghcr.io/arksong/clawmaster:latest
    ports:
      - "13131:13131"
    volumes:
      - clawmaster-config:/root/.config/clawmaster
      - clawmaster-data:/root/.clawmaster
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - RUST_LOG=info
    restart: unless-stopped

volumes:
  clawmaster-config:
  clawmaster-data:
```

---

## ☸️ Kubernetes 部署

### 基础部署

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: clawmaster
spec:
  replicas: 1
  selector:
    matchLabels:
      app: clawmaster
  template:
    metadata:
      labels:
        app: clawmaster
    spec:
      containers:
      - name: clawmaster
        image: ghcr.io/arksong/clawmaster:latest
        ports:
        - containerPort: 13131
        env:
        - name: OPENAI_API_KEY
          valueFrom:
            secretKeyRef:
              name: clawmaster-secrets
              key: openai-api-key
        livenessProbe:
          httpGet:
            path: /api/p0/live
            port: 13131
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /api/p0/ready
            port: 13131
          initialDelaySeconds: 5
          periodSeconds: 5
```

---

## 📊 监控

### Prometheus 指标

```bash
# 访问指标端点
curl https://localhost:13131/metrics
```

### 健康检查

```bash
# 健康状态
curl https://localhost:13131/api/p0/health

# 返回示例
{
  "status": "healthy",
  "timestamp": "2026-03-13T12:00:00Z",
  "components": {
    "database": "healthy",
    "providers": "healthy",
    "channels": "healthy"
  }
}
```

---

## 🔐 安全最佳实践

### API 密钥管理

```bash
# 使用环境变量
export OPENAI_API_KEY="sk-..."

# 或使用密钥管理工具
clawmaster provider add openai --key-from-file ~/.secrets/openai.key

# 或使用系统密钥链
clawmaster provider add openai --use-keychain
```

### TLS 配置

```toml
[server]
tls_enabled = true
cert_path = "/path/to/cert.pem"
key_path = "/path/to/key.pem"
```

### 防火墙规则

```bash
# 只允许本地访问
[server]
host = "127.0.0.1"

# 或使用防火墙
sudo ufw allow from 192.168.1.0/24 to any port 13131
```

---

## 💡 提示和技巧

### 性能优化

```toml
# 增加工作线程
[runtime]
worker_threads = 8

# 启用缓存
[cache]
enabled = true
max_size = "1GB"
```

### 批量操作

```bash
# 批量添加提供商
cat providers.txt | while read provider key; do
  clawmaster provider add $provider --key $key
done
```

### 自动备份

```bash
# 添加到 crontab
0 2 * * * clawmaster backup create --compress
```

---

## 📞 获取帮助

### 文档

- 📖 [完整文档](https://docs.clawmaster.org)
- 🚀 [快速开始](docs/tutorials/01-quick-start.md)
- 📚 [使用指南](USAGE_GUIDE.md)

### 社区

- 💬 Discord: https://discord.gg/clawmaster
- 🐛 GitHub Issues: https://github.com/arksong/ClawMaster/issues
- 📧 Email: support@clawmaster.org

### 紧急支持

```bash
# 生成诊断报告
clawmaster diagnose > diagnostic.txt

# 然后发送到支持邮箱
```

---

**最后更新**: 2026-03-13  
**版本**: 0.10.18  
**状态**: ✅ 生产就绪
