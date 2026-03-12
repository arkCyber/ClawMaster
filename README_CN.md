<div align="center">

# 🦾 ClawMaster

**一个你可以信赖的 Rust 原生 AI 智能体**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE.md)
[![Rust](https://img.shields.io/badge/Rust-1.91%2B-orange.svg)](https://www.rust-lang.org)
[![GitHub](https://img.shields.io/github/stars/arksong/ClawMaster?style=social)](https://github.com/arksong/ClawMaster)

</div>

---

## 📖 项目简介

ClawMaster 是一个**本地优先的 AI 智能网关**，使用 Rust 编写的单一二进制文件，可在你自己的硬件上运行。它在你和多个 LLM 提供商之间架起桥梁，所有数据都保存在本地，无需云端中继。

### 为什么选择 ClawMaster？

- **🔒 安全第一** — 你的 API 密钥永远不会离开你的机器，每个命令都在隔离的沙箱容器中执行
- **💻 完全自主** — 可在 Mac Mini、树莓派或任何你拥有的服务器上运行，无需依赖第三方云服务
- **🎯 功能完整** — 内置语音输入输出、长期记忆、任务调度、多平台通信（Telegram、Discord、Teams）、浏览器自动化、MCP 服务器支持
- **🔍 代码可审计** — 核心代码约 19.6 万行，分布在 46 个模块化 crate 中，拥有 3100+ 测试用例，零 `unsafe` 代码*
- **⚡ 高性能** — 使用 Rust 编写，内存安全且高效，单一二进制文件，无需 Node.js 运行时

## 🚀 快速开始

### 安装方式

#### 方式 1: 一键安装脚本（推荐）

```bash
# macOS / Linux
curl -fsSL https://raw.githubusercontent.com/arksong/ClawMaster/main/install.sh | sh
```

#### 方式 2: 使用 Homebrew

```bash
brew tap arksong/clawmaster
brew install clawmaster
```

#### 方式 3: Docker 容器

```bash
docker pull ghcr.io/arksong/clawmaster:latest
```

#### 方式 4: 从源码构建

```bash
# 克隆仓库
git clone https://github.com/arksong/ClawMaster.git
cd ClawMaster

# 构建 WASM 沙箱工具（可选）
cargo build --target wasm32-wasip2 \
  -p clawmaster-wasm-calc \
  -p clawmaster-wasm-web-fetch \
  -p clawmaster-wasm-web-search \
  --release

# 预编译 WASM 组件
cargo run -p clawmaster-wasm-precompile --release

# 构建主程序
cargo build --release

# 运行
./target/release/clawmaster
```

### 首次运行

启动 ClawMaster 后，终端会显示一个设置代码。在浏览器中打开 `https://localhost:13131`，输入设置代码来创建你的账户（支持密码或 Passkey 认证）。

### Docker 部署

```bash
docker run -d \
  --name clawmaster \
  -p 13131:13131 \
  -v clawmaster-config:/root/.config/clawmaster \
  -v clawmaster-data:/root/.clawmaster \
  -v /var/run/docker.sock:/var/run/docker.sock \
  ghcr.io/arksong/clawmaster:latest
```

访问 `https://localhost:13131` 完成初始设置。

## 🏗️ 系统架构

```
┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│   Web UI    │  │  Telegram   │  │  Discord    │
│   网页界面   │  │   电报机器人  │  │  Discord机器人│
└──────┬──────┘  └──────┬──────┘  └──────┬──────┘
       │                │                │
       └────────┬───────┴────────┬───────┘
                │   WebSocket    │
                │   实时通信协议   │
                ▼                ▼
        ┌─────────────────────────────────┐
        │       Gateway Server            │
        │       网关服务器                 │
        │   (Axum · HTTP · WS · Auth)     │
        ├─────────────────────────────────┤
        │        Chat Service             │
        │        聊天服务                  │
        │  ┌───────────┐ ┌─────────────┐  │
        │  │   Agent   │ │    Tool     │  │
        │  │  智能体    │◄┤   工具注册表 │  │
        │  │   Runner  │ │   Registry  │  │
        │  └─────┬─────┘ └─────────────┘  │
        │        │                        │
        │  ┌─────▼─────────────────────┐  │
        │  │    Provider Registry      │  │
        │  │    提供商注册表             │  │
        │  │  (OpenAI · Copilot · 本地) │  │
        │  └───────────────────────────┘  │
        ├─────────────────────────────────┤
        │  Sessions  │ Memory  │  Hooks   │
        │  会话管理   │  记忆系统 │  钩子系统│
        │  (JSONL)   │ (SQLite)│ (events) │
        └─────────────────────────────────┘
                       │
               ┌───────▼───────┐
               │    Sandbox    │
               │    沙箱环境    │
               │ Docker/Apple  │
               │  Container    │
               └───────────────┘
```

## 📦 核心模块

ClawMaster 采用模块化设计，由 46 个独立的 Rust crate 组成：

### 核心 Crates（必需）

| Crate | 代码行数 | 功能描述 |
|-------|---------|---------|
| `clawmaster` (cli) | 4.0K | 命令行入口点，CLI 命令处理 |
| `clawmaster-agents` | 9.6K | 智能体循环、流式处理、提示词组装 |
| `clawmaster-providers` | 17.6K | LLM 提供商实现（OpenAI、Copilot 等）|
| `clawmaster-gateway` | 36.1K | HTTP/WebSocket 服务器、RPC、身份验证 |
| `clawmaster-chat` | 11.5K | 聊天引擎、智能体编排 |
| `clawmaster-tools` | 21.9K | 工具执行、沙箱管理 |
| `clawmaster-config` | 7.0K | 配置管理、验证 |
| `clawmaster-sessions` | 3.8K | 会话持久化 |

### 可选模块（按需启用）

| 类别 | Crates | 总代码行数 |
|------|--------|-----------|
| Web UI | `clawmaster-web` | 4.5K |
| GraphQL API | `clawmaster-graphql` | 4.8K |
| 语音功能 | `clawmaster-voice` | 6.0K |
| 记忆系统 | `clawmaster-memory`, `clawmaster-qmd` | 5.9K |
| 通信渠道 | `clawmaster-telegram`, `clawmaster-whatsapp`, `clawmaster-discord`, `clawmaster-msteams` | 14.9K |
| 浏览器自动化 | `clawmaster-browser` | 5.1K |
| 任务调度 | `clawmaster-cron`, `clawmaster-caldav` | 5.2K |
| 扩展性 | `clawmaster-mcp`, `clawmaster-skills` | 9.1K |

## 🔐 安全特性

ClawMaster 将安全性放在首位：

- **零 `unsafe` 代码*** — 工作区范围内禁用 unsafe 代码；仅在 `local-embeddings` 特性标志后的可选 FFI 中使用
- **沙箱执行** — 所有工具命令在 Docker 或 Apple Container 中隔离执行，每个会话独立隔离
- **密钥保护** — 使用 `secrecy::Secret` 类型，内存中的密钥在销毁时自动清零，工具输出中自动隐藏
- **身份验证** — 支持密码 + Passkey (WebAuthn) 双因素认证，具有速率限制和按 IP 节流
- **SSRF 防护** — DNS 解析后检查，自动阻止回环地址、私有地址、链路本地地址
- **源验证** — 拒绝跨源 WebSocket 连接升级请求
- **钩子门控** — `BeforeToolCall` 钩子可以检查并阻止任何工具调用

## 🎯 主要功能

### AI 网关
- 多提供商 LLM 支持（OpenAI Codex、GitHub Copilot、本地模型）
- 流式响应处理
- 智能体循环，支持子智能体委托
- 并行工具执行

### 通信方式
- 现代化 Web UI 界面
- Telegram 机器人集成
- Microsoft Teams 集成
- Discord 机器人集成
- RESTful API 访问
- 语音输入输出（8 种 TTS + 7 种 STT 提供商）
- 移动端 PWA，支持推送通知

### 记忆与上下文
- 每个智能体独立的记忆工作区
- 基于嵌入向量的长期记忆
- 混合向量 + 全文搜索
- 会话持久化，自动压缩
- 项目上下文管理

### 可扩展性
- MCP 服务器支持（stdio + HTTP/SSE）
- 技能系统
- 15 种生命周期钩子事件
- 熔断器保护
- 危险命令防护

### 安全与加密
- 静态加密保险库（XChaCha20-Poly1305 + Argon2id）
- 密码 + Passkey + API 密钥多重认证
- 沙箱隔离
- SSRF/CSWSH 防护

### 运维功能
- Cron 任务调度
- OpenTelemetry 追踪
- Prometheus 指标监控
- 云部署支持（Fly.io、DigitalOcean）
- Tailscale 集成

## 🛠️ 开发指南

### 环境要求

- Rust 1.91+
- Node.js（用于构建 Tailwind CSS）
- Docker（可选，用于沙箱功能）

### 构建命令

```bash
# 安装 just 命令运行器
cargo install just

# 构建 CSS
just build-css

# 构建发布版本
just build-release

# 构建包含 WASM 工具的完整版本
just build-release-with-wasm

# 运行测试
cargo test

# 运行开发服务器
cargo run --release --bin clawmaster
```

### 项目结构

```
ClawMaster/
├── crates/           # 核心 Rust crates
│   ├── cli/         # 命令行入口
│   ├── agents/      # 智能体实现
│   ├── gateway/     # HTTP/WS 服务器
│   ├── chat/        # 聊天引擎
│   └── ...          # 其他模块
├── apps/            # 原生应用
│   ├── macos/       # macOS 应用
│   └── ios/         # iOS 应用
├── docs/            # 文档
└── website/         # 项目网站
```

## 📚 文档

- [AGENTS.md](AGENTS.md) - 智能体系统指南
- [CLAUDE.md](CLAUDE.md) - Claude AI 集成指南
- [CONTRIBUTING.md](CONTRIBUTING.md) - 贡献指南
- [SECURITY.md](SECURITY.md) - 安全架构文档
- [CHANGELOG.md](CHANGELOG.md) - 版本更新日志

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

在提交 PR 之前，请确保：
1. 代码通过 `cargo test` 测试
2. 代码通过 `cargo clippy` 检查
3. 代码格式符合 `cargo fmt` 规范
4. 添加了相应的测试用例
5. 更新了相关文档

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE.md](LICENSE.md)

## 👤 作者

**arksong**
- Email: [arksong2018@gmail.com](mailto:arksong2018@gmail.com)
- GitHub: [@arksong](https://github.com/arksong)

## 🙏 致谢

感谢所有为 ClawMaster 项目做出贡献的开发者！

---

<div align="center">

**用 ❤️ 打造 by arksong**

如果这个项目对你有帮助，请给个 ⭐️ 支持一下！

</div>
