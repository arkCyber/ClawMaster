# Tauri UI 问题诊断与修复报告

**诊断时间**: 2026-03-15  
**状态**: 🔧 修复中  
**问题**: UI 功能不完整，模块加载失败

---

## 🔍 发现的问题

### 1. 模块加载错误
```
TypeError: Module name, 'htm/preact' does not resolve to a valid URL.
```

**原因**: Import Maps 中的路径使用相对路径，在 Tauri 环境中无法正确解析。

**修复**: 将所有模块路径改为绝对路径（以 `/` 开头）

### 2. WebSocket 连接失败
```
WebSocket connection to 'ws://127.0.0.1:1430/_tauri_cli' failed: WebSocket is closed due to suspension.
```

**原因**: Tauri 应用没有配置后端 WebSocket 服务器，前端尝试连接不存在的 WebSocket。

**修复方案**:
1. 配置 Tauri 后端启动 ClawMaster WebSocket 服务器
2. 或修改前端 WebSocket 连接逻辑，连接到正确的后端地址

---

## ✅ 已完成的修复

### 1. Import Maps 路径修复
**修改前**:
```javascript
{
  "imports": {
    "preact": "js/vendor/preact.mjs",
    "htm/preact": "js/vendor/htm-preact.mjs",
    // ...
  }
}
```

**修改后**:
```javascript
{
  "imports": {
    "preact": "/js/vendor/preact.mjs",
    "htm/preact": "/js/vendor/htm-preact.mjs",
    // ...
  }
}
```

### 2. CSS 资源路径修复
**修改前**:
```html
<link rel="stylesheet" href="css/base.css">
<link rel="stylesheet" href="css/layout.css">
```

**修改后**:
```html
<link rel="stylesheet" href="/css/base.css">
<link rel="stylesheet" href="/css/layout.css">
```

### 3. JavaScript 资源路径修复
**修改前**:
```html
<link rel="modulepreload" href="js/app.js">
<script type="module" src="js/app.js"></script>
```

**修改后**:
```html
<link rel="modulepreload" href="/js/app.js">
<script type="module" src="/js/app.js"></script>
```

### 4. 图标路径修复
**修改前**:
```html
<link rel="icon" href="icons/icon-96.png">
```

**修改后**:
```html
<link rel="icon" href="/icons/icon-96.png">
```

---

## 🔧 待修复的问题

### 1. WebSocket 连接配置
**问题**: 前端尝试连接 `ws://127.0.0.1:1430/_tauri_cli`，但该服务不存在。

**解决方案 A**: 配置 Tauri 后端启动 ClawMaster WebSocket 服务器
- 修改 `src-tauri/src/lib.rs`
- 添加 WebSocket 服务器启动逻辑
- 配置正确的端口和路由

**解决方案 B**: 修改前端 WebSocket 连接逻辑
- 检测 Tauri 环境
- 使用 Tauri IPC 替代 WebSocket
- 或连接到外部 ClawMaster 服务器

### 2. 后端 API 集成
**问题**: WebUI 依赖后端 API (`/api/*`)，Tauri 应用需要提供这些 API。

**解决方案**:
- 在 Tauri 后端实现必要的 API 端点
- 或配置代理到外部 ClawMaster 服务器

---

## 📋 修复计划

### 阶段 1: 资源路径修复 ✅
- [x] 修复 Import Maps 路径
- [x] 修复 CSS 路径
- [x] 修复 JS 路径
- [x] 修复图标路径

### 阶段 2: WebSocket 配置 🔧
- [ ] 分析 WebSocket 连接需求
- [ ] 配置 Tauri 后端 WebSocket 服务器
- [ ] 或实现 Tauri IPC 桥接
- [ ] 测试 WebSocket 连接

### 阶段 3: API 集成 ⏳
- [ ] 识别必需的 API 端点
- [ ] 实现 Tauri 命令处理器
- [ ] 配置 API 路由
- [ ] 测试 API 功能

### 阶段 4: 功能测试 ⏳
- [ ] 测试页面加载
- [ ] 测试路由导航
- [ ] 测试多语言切换
- [ ] 测试主题切换
- [ ] 测试所有 19 个页面

---

## 🎯 下一步行动

1. **立即**: 重启 Tauri 应用，验证路径修复效果
2. **接下来**: 配置 WebSocket 连接
3. **然后**: 实现必要的 API 端点
4. **最后**: 全面测试所有功能

---

## 📊 预期结果

修复完成后，Tauri 应用应该能够：
- ✅ 正确加载所有 JavaScript 模块
- ✅ 正确加载所有 CSS 样式
- ✅ 显示完整的 UI 界面
- ✅ 建立 WebSocket 连接
- ✅ 调用后端 API
- ✅ 所有 19 个页面正常工作
- ✅ 多语言切换正常
- ✅ 主题切换正常

---

**当前状态**: 路径修复完成，准备配置 WebSocket 和后端集成
