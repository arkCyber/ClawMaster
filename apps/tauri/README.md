# ClawMaster Tauri Desktop Application

ClawMaster 的第三个 UI 界面 - 使用 Tauri 构建的原生桌面应用。

## 🎯 特性

- ✅ **原生 macOS 应用** - 真正的 .app 文件
- ✅ **小体积** - 约 5-10MB
- ✅ **复用 WebUI** - 使用现有的 Web 界面
- ✅ **系统集成** - 原生菜单、通知、托盘
- ✅ **跨平台** - macOS, Windows, Linux

## 🏗️ 架构

```
Tauri App
├─ Rust 后端 (src-tauri/)
│  ├─ main.rs - Tauri 应用入口
│  ├─ Cargo.toml - Rust 依赖
│  └─ tauri.conf.json - Tauri 配置
└─ 前端 (复用 WebUI)
   └─ 加载 https://localhost:59233
```

## 🚀 开发

### 前提条件

```bash
# 安装 Tauri CLI
cargo install tauri-cli --locked

# 确保 ClawMaster 后端运行
cargo run --bin clawmaster
```

### 开发模式

```bash
cd apps/tauri
cargo tauri dev
```

### 构建应用

```bash
cd apps/tauri
cargo tauri build
```

构建产物：
- macOS: `src-tauri/target/release/bundle/macos/ClawMaster.app`
- Windows: `src-tauri/target/release/bundle/msi/ClawMaster.msi`
- Linux: `src-tauri/target/release/bundle/deb/clawmaster.deb`

## 📦 安装

### macOS

```bash
# 构建
cd apps/tauri
cargo tauri build

# 安装
open src-tauri/target/release/bundle/macos/ClawMaster.app
```

### Windows

```bash
# 构建
cd apps/tauri
cargo tauri build

# 安装
.\src-tauri\target\release\bundle\msi\ClawMaster.msi
```

## 🎨 与其他 UI 的对比

| 特性 | WebUI | Cosmic UI | Tauri UI |
|------|-------|-----------|----------|
| 平台 | 全平台 | Linux | macOS/Win/Linux |
| 安装 | 浏览器 | 需编译 | .app/.msi/.deb |
| 体积 | 0 | ~15MB | ~5-10MB |
| 远程访问 | ✅ | ❌ | ❌ |
| 原生集成 | 有限 | ✅ | ✅ |
| 开发时间 | 已完成 | 已完成 | 新增 |

## 🔧 配置

### tauri.conf.json

主要配置项：
- `devUrl`: 开发模式 URL (https://localhost:59233)
- `identifier`: 应用标识符 (ai.clawmaster.app)
- `windows`: 窗口配置

### Cargo.toml

依赖：
- `tauri`: Tauri 框架
- `clawmaster-cosmic-client`: 复用客户端库

## 📝 注意事项

1. **需要后端运行**: Tauri 应用加载 WebUI，需要 ClawMaster 后端在 59233 端口运行
2. **HTTPS 证书**: 首次运行需要信任自签名证书
3. **macOS 权限**: 可能需要在系统偏好设置中允许应用运行

## 🎯 使用场景

### 适合使用 Tauri UI
- ✅ macOS/Windows 桌面用户
- ✅ 需要原生应用体验
- ✅ 需要系统托盘集成
- ✅ 需要离线安装包

### 适合使用 WebUI
- ✅ 远程访问
- ✅ 跨平台一致性
- ✅ 无需安装
- ✅ 快速开发迭代

### 适合使用 Cosmic UI
- ✅ Linux 桌面用户
- ✅ 需要完全原生体验
- ✅ 系统深度集成

## 🚀 快速开始

```bash
# 1. 启动 ClawMaster 后端
cargo run --bin clawmaster

# 2. 在新终端启动 Tauri 应用
cd apps/tauri
cargo tauri dev
```

## 📚 相关文档

- [Tauri 官方文档](https://tauri.app/)
- [ClawMaster WebUI](../../crates/web/)
- [ClawMaster Cosmic UI](../cosmic/)

---

**版本**: 0.10.18  
**维护者**: ClawMaster Engineering Team  
**状态**: 开发中
