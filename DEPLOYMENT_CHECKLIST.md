# ClawMaster 生产部署检查清单

**版本**: 0.10.18  
**更新日期**: 2026-03-13

---

## 📋 部署前检查

### ✅ 系统要求

- [ ] 操作系统：Linux (Ubuntu 20.04+) / macOS 10.15+ / Windows 10+
- [ ] CPU：2+ 核心
- [ ] 内存：4GB+ RAM
- [ ] 磁盘：10GB+ 可用空间
- [ ] 网络：稳定的互联网连接

### ✅ 依赖检查

```bash
# 检查 Rust 版本
rustc --version  # 需要 1.70+

# 检查数据库
sqlite3 --version

# 检查 OpenSSL
openssl version
```

---

## 🔧 配置检查

### ✅ 基础配置

**配置文件位置**: `~/.config/clawmaster/clawmaster.toml`

```toml
# 必需配置
[server]
host = "0.0.0.0"  # 生产环境
port = 13131
tls_enabled = true  # 生产环境必须启用
tls_cert = "/path/to/cert.pem"
tls_key = "/path/to/key.pem"

[providers]
default = "openai"

[providers.openai]
enabled = true
api_key_env = "OPENAI_API_KEY"  # 使用环境变量

[channels]
web = true
```

### ✅ 环境变量

**文件位置**: `~/.config/clawmaster/.env`

```bash
# LLM API 密钥
OPENAI_API_KEY=sk-xxx
ANTHROPIC_API_KEY=sk-ant-xxx

# 通道 Token
TELEGRAM_BOT_TOKEN=xxx
DISCORD_BOT_TOKEN=xxx

# 数据库
DATABASE_URL=sqlite:///path/to/clawmaster.db

# 安全
JWT_SECRET=your-secret-key
```

**检查**:
- [ ] 所有必需的环境变量已设置
- [ ] API 密钥有效且未过期
- [ ] 密钥文件权限正确 (600)

### ✅ P0 功能配置

```toml
[p0features]
# 健康检查
health_check = true
health_check_interval = 30  # 秒

# 资源配额
rate_limiting = true
rate_limit_per_minute = 60
max_memory_mb = 2048
max_sessions = 1000

# 审计日志
audit_log = true
audit_retention_days = 90

# 自动备份
auto_backup = true
backup_interval = 24  # 小时
backup_retention_days = 30
```

**检查**:
- [ ] P0 功能已启用
- [ ] 配额设置合理
- [ ] 备份策略已配置

---

## 🔒 安全检查

### ✅ 身份验证

```bash
# 设置管理员密码
clawmaster auth set-password

# 启用 Passkey（推荐）
clawmaster auth enable-passkey

# 生成 API 密钥
clawmaster api-key create --name "Production" --expires 365d
```

**检查**:
- [ ] 管理员密码已设置（强密码）
- [ ] Passkey 已启用
- [ ] API 密钥已生成并安全存储

### ✅ TLS/SSL

```bash
# 生成自签名证书（开发）
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# 生产环境使用 Let's Encrypt
certbot certonly --standalone -d your-domain.com
```

**检查**:
- [ ] TLS 证书有效
- [ ] 证书未过期
- [ ] 证书路径正确配置

### ✅ 防火墙

```bash
# 允许必要端口
sudo ufw allow 13131/tcp  # ClawMaster
sudo ufw allow 443/tcp    # HTTPS
sudo ufw enable
```

**检查**:
- [ ] 防火墙已配置
- [ ] 仅开放必要端口
- [ ] SSH 端口已保护

### ✅ 访问控制

```toml
[security]
# IP 白名单
allowed_ips = ["192.168.1.0/24", "10.0.0.0/8"]

# 速率限制
rate_limit_enabled = true
rate_limit_per_ip = 100  # 每分钟

# CORS
cors_enabled = true
cors_origins = ["https://your-domain.com"]
```

**检查**:
- [ ] IP 白名单已配置（如需要）
- [ ] 速率限制已启用
- [ ] CORS 策略正确

---

## 📊 监控检查

### ✅ 健康检查端点

```bash
# 测试健康检查
curl https://localhost:13131/api/p0/health

# 测试就绪探针
curl https://localhost:13131/api/p0/ready

# 测试存活探针
curl https://localhost:13131/api/p0/live
```

**检查**:
- [ ] 健康检查端点响应正常
- [ ] 所有组件状态为 healthy
- [ ] 响应时间 <100ms

### ✅ 日志配置

```toml
[logging]
level = "info"  # 生产环境使用 info
file = "/var/log/clawmaster/app.log"
max_size_mb = 100
max_backups = 10
compress = true
```

**检查**:
- [ ] 日志目录存在且可写
- [ ] 日志轮转已配置
- [ ] 日志级别适当

### ✅ 指标收集

```toml
[metrics]
enabled = true
prometheus_port = 9090
```

**检查**:
- [ ] Prometheus 端点可访问
- [ ] 指标正常收集
- [ ] Grafana 仪表板已配置（可选）

---

## 💾 数据管理

### ✅ 数据库

```bash
# 初始化数据库
clawmaster db init

# 运行迁移
clawmaster db migrate

# 检查数据库状态
clawmaster db status
```

**检查**:
- [ ] 数据库已初始化
- [ ] 所有迁移已应用
- [ ] 数据库权限正确

### ✅ 备份策略

```bash
# 启用自动备份
clawmaster backup auto-enable --interval 24h

# 测试备份
clawmaster backup create --output /tmp/test-backup.tar.gz

# 测试恢复
clawmaster backup restore /tmp/test-backup.tar.gz --dry-run
```

**检查**:
- [ ] 自动备份已启用
- [ ] 备份目录有足够空间
- [ ] 备份恢复测试成功
- [ ] 远程备份已配置（推荐）

---

## 🚀 部署方式

### 选项 1: Systemd 服务（Linux）

**创建服务文件** `/etc/systemd/system/clawmaster.service`:

```ini
[Unit]
Description=ClawMaster AI Gateway
After=network.target

[Service]
Type=simple
User=clawmaster
Group=clawmaster
WorkingDirectory=/opt/clawmaster
EnvironmentFile=/opt/clawmaster/.env
ExecStart=/usr/local/bin/clawmaster
Restart=always
RestartSec=10

# 安全加固
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/clawmaster /var/log/clawmaster

[Install]
WantedBy=multi-user.target
```

**部署步骤**:
```bash
# 创建用户
sudo useradd -r -s /bin/false clawmaster

# 创建目录
sudo mkdir -p /opt/clawmaster /var/lib/clawmaster /var/log/clawmaster
sudo chown clawmaster:clawmaster /opt/clawmaster /var/lib/clawmaster /var/log/clawmaster

# 复制二进制
sudo cp target/release/clawmaster /usr/local/bin/
sudo chmod +x /usr/local/bin/clawmaster

# 启动服务
sudo systemctl daemon-reload
sudo systemctl enable clawmaster
sudo systemctl start clawmaster

# 检查状态
sudo systemctl status clawmaster
```

**检查**:
- [ ] 服务已启动
- [ ] 服务自动重启正常
- [ ] 日志无错误

### 选项 2: Docker 部署

**Dockerfile**:
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/clawmaster /usr/local/bin/
COPY --from=builder /app/crates/web/src/assets /opt/clawmaster/assets

RUN useradd -r -s /bin/false clawmaster
USER clawmaster

EXPOSE 13131
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f https://localhost:13131/api/p0/health || exit 1

CMD ["clawmaster"]
```

**docker-compose.yml**:
```yaml
version: '3.8'

services:
  clawmaster:
    build: .
    ports:
      - "13131:13131"
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    volumes:
      - ./data:/var/lib/clawmaster
      - ./logs:/var/log/clawmaster
      - ./config:/etc/clawmaster
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "https://localhost:13131/api/p0/health"]
      interval: 30s
      timeout: 3s
      retries: 3
```

**部署步骤**:
```bash
# 构建镜像
docker-compose build

# 启动服务
docker-compose up -d

# 查看日志
docker-compose logs -f

# 检查状态
docker-compose ps
```

**检查**:
- [ ] 容器正常运行
- [ ] 健康检查通过
- [ ] 数据卷正确挂载

### 选项 3: Kubernetes 部署

**deployment.yaml**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: clawmaster
spec:
  replicas: 3
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
        image: clawmaster/clawmaster:0.10.18
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
        resources:
          requests:
            memory: "512Mi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "2000m"
```

**检查**:
- [ ] Pod 正常运行
- [ ] 探针检查通过
- [ ] 资源限制合理

---

## 🧪 测试检查

### ✅ 功能测试

```bash
# 测试 Web UI
curl https://localhost:13131

# 测试聊天 API
curl -X POST https://localhost:13131/api/chat \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{"message": "测试消息", "session_id": "test"}'

# 测试提供商
clawmaster provider test-all

# 测试通道
clawmaster channel test telegram
clawmaster channel test discord
```

**检查**:
- [ ] Web UI 可访问
- [ ] API 响应正常
- [ ] 所有提供商测试通过
- [ ] 所有通道测试通过

### ✅ 性能测试

```bash
# 负载测试
ab -n 1000 -c 10 https://localhost:13131/api/p0/health

# 压力测试
wrk -t4 -c100 -d30s https://localhost:13131/api/p0/health
```

**检查**:
- [ ] 响应时间 <100ms (P50)
- [ ] 吞吐量 >100 req/s
- [ ] 无错误响应

### ✅ 安全测试

```bash
# SSL 测试
testssl.sh https://localhost:13131

# 安全扫描
nmap -sV localhost

# 依赖扫描
cargo audit
```

**检查**:
- [ ] SSL 配置安全
- [ ] 无已知漏洞
- [ ] 依赖项无安全问题

---

## 📝 文档检查

### ✅ 运维文档

- [ ] 部署文档已更新
- [ ] 配置文档已完善
- [ ] 故障排除指南已准备
- [ ] 联系方式已记录

### ✅ 用户文档

- [ ] 快速开始指南
- [ ] API 文档
- [ ] 使用指南
- [ ] FAQ

---

## 🎯 上线检查清单

### 部署前 (T-1 天)

- [ ] 所有代码已合并到 main 分支
- [ ] 所有测试通过
- [ ] 配置文件已审核
- [ ] 备份策略已测试
- [ ] 回滚计划已准备

### 部署中 (T-0)

- [ ] 通知用户维护窗口
- [ ] 创建当前状态备份
- [ ] 部署新版本
- [ ] 运行数据库迁移
- [ ] 验证健康检查
- [ ] 执行冒烟测试

### 部署后 (T+1 小时)

- [ ] 监控系统指标
- [ ] 检查错误日志
- [ ] 验证核心功能
- [ ] 用户反馈收集
- [ ] 性能基准对比

### 部署后 (T+24 小时)

- [ ] 全面功能测试
- [ ] 性能分析
- [ ] 安全审计
- [ ] 用户满意度调查
- [ ] 文档更新

---

## 🚨 应急响应

### 回滚计划

```bash
# 停止服务
sudo systemctl stop clawmaster

# 恢复备份
clawmaster backup restore /path/to/last-good-backup.tar.gz

# 重启服务
sudo systemctl start clawmaster

# 验证
clawmaster health
```

### 紧急联系人

- **技术负责人**: [姓名] - [电话] - [邮箱]
- **运维负责人**: [姓名] - [电话] - [邮箱]
- **安全负责人**: [姓名] - [电话] - [邮箱]

---

## ✅ 最终确认

部署前，确认所有项目已完成：

- [ ] 系统要求满足
- [ ] 配置正确完整
- [ ] 安全措施到位
- [ ] 监控已配置
- [ ] 备份策略有效
- [ ] 测试全部通过
- [ ] 文档已更新
- [ ] 应急计划就绪

**签字确认**:

- 技术负责人: _____________ 日期: _______
- 运维负责人: _____________ 日期: _______
- 安全负责人: _____________ 日期: _______

---

**版本**: 0.10.18  
**更新日期**: 2026-03-13  
**状态**: ✅ 生产就绪
