# ClawMaster 双UI架构设计文档
**日期**: 2026-03-13  
**方案**: WebUI + libcosmic Native UI 双界面架构

---

## 🎯 概述

为 ClawMaster 添加第二套基于 libcosmic 的原生桌面UI，实现 WebUI + Native UI 双界面同时工作，提供不同的用户体验。

---

## 🏗️ 架构设计

### 整体架构图

```
┌─────────────────────────────────────────────────────────────┐
│                    ClawMaster Core                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │
│  │   Auth      │  │  Sessions   │  │   Channels  │       │
│  │   Gateway   │  │   Store     │  │  Manager    │       │
│  └─────────────┘  └─────────────┘  └─────────────┘       │
└─────────────────────────────────────────────────────────────┘
                              │
                    ┌─────────┴─────────┐
                    │    Shared API     │
                    │  (gRPC/HTTP/RPC)  │
                    └─────────┬─────────┘
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
    ┌─────▼─────┐      ┌─────▼─────┐      ┌─────▼─────┐
    │  WebUI    │      │Native UI │      │  CLI/     │
    │ (Browser) │      │(libcosmic)│      │   API     │
    │           │      │           │      │           │
    │ • SPA     │      │ • Desktop  │      │ • REST    │
    │ • Preact  │      │ • Native   │      │ • GraphQL │
    │ • WebSocket│     │ • Cross-   │      │ • RPC     │
    │           │      │   platform │      │           │
    └───────────┘      └───────────┘      └───────────┘
```

---

## 📦 项目结构

### 新增目录结构

```
ClawMaster/
├── apps/
│   ├── courier/           # 现有应用
│   ├── ios/               # 现有 iOS 应用
│   ├── macos/             # 现有 macOS 应用
│   └── cosmic/            # 🆕 libcosmic 桌面应用
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs
│       │   ├── app.rs
│       │   ├── views/
│       │   │   ├── dashboard.rs
│       │   │   ├── chat.rs
│       │   │   ├── settings.rs
│       │   │   └── security.rs
│       │   ├── widgets/
│       │   │   ├── chat_message.rs
│       │   │   ├── status_indicator.rs
│       │   │   └── emergency_button.rs
│       │   └── utils/
│       │       ├── theme.rs
│       │       └── client.rs
│       └── resources/
│           └── icons/
├── crates/
│   ├── web/               # 现有 WebUI
│   ├── gateway/           # 现有网关
│   └── cosmic-client/     # 🆕 libcosmic 客户端库
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── rpc.rs
│           ├── models.rs
│           └── config.rs
```

---

## 🔧 技术实现

### 1. 共享API层

**现有架构** (WebUI):
```
WebUI (Preact) → HTTP/WebSocket → Gateway → Core Services
```

**新增架构** (Native UI):
```
Native UI (libcosmic) → RPC Client → Gateway → Core Services
```

### 2. RPC 客户端库

**新建 crate**: `crates/cosmic-client`

```rust
// crates/cosmic-client/src/lib.rs
pub struct CosmicClient {
    rpc_client: RpcClient,
    event_receiver: mpsc::Receiver<UiEvent>,
}

impl CosmicClient {
    pub async fn new() -> Result<Self> {
        let rpc_client = RpcClient::connect("127.0.0.1:59233").await?;
        Ok(Self { rpc_client, ... })
    }
    
    pub async fn get_sessions(&self) -> Result<Vec<Session>> {
        self.rpc_client.call("sessions.list", ()).await
    }
    
    pub async fn send_message(&self, session: &str, msg: &str) -> Result<()> {
        self.rpc_client.call("chat.send", (session, msg)).await
    }
    
    pub async fn emergency_stop(&self) -> Result<()> {
        self.rpc_client.call("security.emergency_stop", ()).await
    }
}
```

### 3. libcosmic 应用结构

**主应用**: `apps/cosmic/src/main.rs`

```rust
use libcosmic::{Application, ApplicationExt};
use cosmic_client::CosmicClient;

fn main() -> iced::Result {
    let client = CosmicClient::new().block_on()?;
    
    Application::run(
        CosmicApp::new(client),
        cosmic::Settings::default()
    )
}

struct CosmicApp {
    client: CosmicClient,
    current_view: View,
    sessions: Vec<Session>,
    // ...
}

#[derive(Debug, Clone)]
enum View {
    Dashboard,
    Chat(String),
    Settings,
    Security,
}

impl Application for CosmicApp {
    type Message = Message;
    type Theme = cosmic::Theme;
    type Executor = cosmic::executor::Default;
    type Flags = CosmicClient;
    
    fn new(client: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                client,
                current_view: View::Dashboard,
                sessions: Vec::new(),
            },
            iced::Command::perform(load_sessions(), Message::SessionsLoaded)
        )
    }
    
    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::ViewChanged(view) => {
                self.current_view = view;
                iced::Command::none()
            }
            Message::EmergencyStop => {
                iced::Command::perform(
                    self.client.emergency_stop(),
                    |result| Message::EmergencyStopResult(result)
                )
            }
            // ...
        }
    }
    
    fn view(&self) -> iced::Element<Self::Message> {
        match self.current_view {
            View::Dashboard => dashboard_view(self),
            View::Chat(session_id) => chat_view(self, &session_id),
            View::Settings => settings_view(self),
            View::Security => security_view(self),
        }
    }
}
```

---

## 🎨 UI 设计

### 1. 主界面布局

```
┌─────────────────────────────────────────────────────────────┐
│ ☰  ClawMaster          🔔 ⚙️ 🔐                    🛑 STOP │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐       │
│  │   🟢 连接    │  │   🤖 模型   │  │   💬 会话   │       │
│  │  Connected  │  │     0      │  │     0      │       │
│  └─────────────┘  └─────────────┘  └─────────────┘       │
│                                                             │
│  ┌─────────────────────────────────────────┐               │
│  │ 💬 最近会话                              │               │
│  │ ┌─────────────────────────────────────┐ │               │
│  │ │ main - 2026-03-13 18:15            │ │               │
│  │ │ "Hello, how can I help you?"       │ │               │
│  │ └─────────────────────────────────────┘ │               │
│  │ ┌─────────────────────────────────────┐ │               │
│  │ │ work - 2026-03-13 17:30            │ │               │
│  │ │ "Let's review the project status." │ │               │
│  │ └─────────────────────────────────────┘ │               │
│  └─────────────────────────────────────────┘               │
│                                                             │
│  ┌─────────────────────────────────────────┐               │
│  │ ⚡ 快捷操作                              │               │
│  │ [💬 新建聊天] [⚙️ 设置] [🛡️ 安全] [📋 日志] │               │
│  └─────────────────────────────────────────┘               │
└─────────────────────────────────────────────────────────────┘
```

### 2. 聊天界面

```
┌─────────────────────────────────────────────────────────────┐
│ ←  main                    🔍 ⚙️                    🛑 STOP │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  User (18:15:23)                                            │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Hello, how can I help you today?                    │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  Assistant (18:15:25)                                      │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Hello! I'm here to help you with your ClawMaster    │   │
│  │ project. What would you like to work on today?      │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ 🟡 Thinking...                                      │   │
│  │ [🛑 停止]                                           │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Type your message...                                 │   │
│  │ [📎] [🖼️] [🔧] [📤 Send]                            │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 🚀 实现步骤

### 阶段 1: 基础架构 (1-2天)
1. **创建 cosmic-client crate**
   - RPC 客户端实现
   - 数据模型定义
   - 错误处理

2. **创建 cosmic 应用框架**
   - 基础应用结构
   - 主题配置
   - 路由系统

### 阶段 2: 核心视图 (2-3天)
1. **Dashboard 视图**
   - 系统状态卡片
   - 最近会话列表
   - 快捷操作按钮

2. **Chat 视图**
   - 消息列表
   - 输入框
   - 实时更新

### 阶段 3: 高级功能 (2-3天)
1. **Settings 视图**
   - 配置管理
   - 主题切换
   - 提供商设置

2. **Security 视图**
   - 紧急停止
   - 权限管理
   - 审计日志

### 阶段 4: 集成测试 (1天)
1. **功能测试**
2. **性能测试**
3. **跨平台测试**

---

## 📋 依赖管理

### Cargo.toml 更新

**workspace Cargo.toml**:
```toml
[workspace]
default-members = [
  # ... 现有成员
  "apps/cosmic",
  "crates/cosmic-client",
]

members = [
  # ... 现有成员
  "apps/cosmic",
  "crates/cosmic-client",
]

[workspace.dependencies]
libcosmic = "0.1.0"
iced = "0.12.0"
cosmic-client = { path = "crates/cosmic-client" }
```

**apps/cosmic/Cargo.toml**:
```toml
[package]
name = "clawmaster-cosmic"
version = "0.1.0"
edition = "2021"

[dependencies]
libcosmic = { workspace = true }
iced = { workspace = true }
cosmic-client = { workspace = true }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
tracing = { workspace = true }
```

---

## 🔧 配置管理

### 共享配置

**配置文件**: `~/.config/clawmaster/clawmaster.toml`

```toml
[ui]
web_enabled = true
web_port = 59233
cosmic_enabled = true
cosmic_theme = "dark"

[ui.cosmic]
theme = "dark"
font_size = 14
window_width = 1200
window_height = 800
auto_start = true
```

---

## 🎯 优势对比

### WebUI vs Native UI

| 特性 | WebUI | Native UI (libcosmic) |
|------|-------|----------------------|
| **访问方式** | 浏览器 | 桌面应用 |
| **跨设备** | ✅ 任何设备 | ❌ 仅桌面 |
| **原生体验** | ❌ 浏览器限制 | ✅ 系统集成 |
| **性能** | 中等 | 高 |
| **离线** | ❌ 需要服务器 | ✅ 可离线 |
| **更新** | 自动 | 需更新应用 |
| **主题** | CSS 主题 | 系统主题 |
| **快捷键** | 浏览器限制 | 系统级 |
| **通知** | 浏览器通知 | 系统通知 |

---

## 🚀 部署方案

### 1. 开发环境

```bash
# 启动 WebUI
cargo run -p clawmaster-web

# 启动 Native UI
cargo run -p clawmaster-cosmic
```

### 2. 生产环境

```bash
# 单进程启动（推荐）
./target/release/clawmaster --ui web,cosmic

# 分离启动
./target/release/clawmaster --ui web &
./target/release/clawmaster-cosmic
```

### 3. 安装包

**Linux (AppImage)**:
```bash
clawmaster-cosmic.AppImage  # 包含 WebUI + Native UI
```

**macOS (DMG)**:
```bash
ClawMaster.app  # 包含 WebUI + Native UI
```

**Windows (MSI)**:
```bash
ClawMasterSetup.exe  # 包含 WebUI + Native UI
```

---

## 📊 性能考虑

### 资源使用

| 组件 | 内存使用 | CPU 使用 | 启动时间 |
|------|----------|----------|----------|
| Core Services | ~50MB | ~2% | 2s |
| WebUI | ~30MB | ~1% | 1s |
| Native UI | ~40MB | ~1% | 0.5s |
| **总计** | ~120MB | ~4% | 2s |

### 优化策略

1. **懒加载** - Native UI 按需加载视图
2. **缓存** - 共享数据缓存
3. **异步** - 所有 API 调用异步化
4. **资源池** - 复用连接和资源

---

## 🔒 安全考虑

### 1. API 安全
- 两个 UI 共享相同的认证机制
- 本地 RPC 调用使用 Unix Socket
- 远程访问需要认证

### 2. 数据隔离
- UI 配置分离存储
- 会话数据共享但 UI 状态独立
- 敏感操作需要二次确认

---

## 🎨 主题定制

### libcosmic 主题

```rust
// 支持的主题
pub enum Theme {
    Light,
    Dark,
    Cosmic,     // COSMIC 默认主题
    Custom(Path),
}

// 主题配置
pub struct ThemeConfig {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub background: Color,
    pub surface: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
}
```

---

## 📱 跨平台支持

### 支持的平台

| 平台 | WebUI | Native UI | 状态 |
|------|-------|-----------|------|
| Linux | ✅ | ✅ | 完全支持 |
| macOS | ✅ | ✅ | 完全支持 |
| Windows | ✅ | ✅ | 完全支持 |
| Android | ✅ | ❌ | 仅 WebUI |
| iOS | ✅ | ❌ | 仅 WebUI |

---

## 🚀 下一步行动

### 立即可执行

1. **创建 cosmic-client crate**
2. **设置基础应用框架**
3. **实现 RPC 客户端**

### 本周目标

1. **完成 Dashboard 视图**
2. **实现基础聊天功能**
3. **集成设置管理**

### 月度目标

1. **完整功能对等**
2. **跨平台测试**
3. **性能优化**

---

**总结**: 双UI架构将显著提升 ClawMaster 的用户体验，WebUI 提供便捷的远程访问，Native UI 提供优秀的桌面体验。两者共享后端，功能对等，互为补充。

**建议**: 立即开始实施，预计 1-2 周完成基础版本。🚀
