# ClawMaster Tauri 快速启动指南

**更新时间**: 2026-03-15  
**状态**: ✅ 已修复路径问题

---

## 🚀 启动步骤

### 前提条件
1. **启动 ClawMaster 后端服务器**
   ```bash
   cd /Users/arksong/ClawMaster
   cargo run --bin clawmaster
   ```
   
   确保后端运行在 `http://localhost:8080`

2. **启动 Tauri 应用**
   ```bash
   cd /Users/arksong/ClawMaster/apps/tauri
   ./start-ui.sh
   ```

---

## ✅ 已修复的问题

### 1. Import Maps 路径
- ✅ 所有模块路径改为绝对路径 (`/js/vendor/...`)
- ✅ 修复 `htm/preact` 模块加载错误

### 2. 资源路径
- ✅ CSS 文件路径改为绝对路径 (`/css/...`)
- ✅ JS 文件路径改为绝对路径 (`/js/...`)
- ✅ 图标路径改为绝对路径 (`/icons/...`)

### 3. 后端配置
- ✅ 后端 URL 改为 `http://localhost:8080`
- ✅ WebSocket URL 配置为 `ws://localhost:8080/ws`
- ✅ 添加 Tauri 环境检测

---

## 🔧 配置说明

### window.__MOLTIS__ 配置
```javascript
window.__MOLTIS__ = {
  routes: {
    chats: "/chats",
    metrics: "/metrics",
    settings: "/settings",
    providers: "/settings/llms",
    security: "/settings/security",
    projects: "/projects"
  },
  identity: {
    name: "ClawMaster",
    emoji: "🦾"
  },
  ws_url: "ws://localhost:8080/ws",
  api_base: "http://localhost:8080"
};
```

### Tauri 后端 API
- `get_app_info()` - 获取应用信息
- `fetch_backend(path)` - GET 请求
- `post_backend(path, body)` - POST 请求
- `get_sessions()` - 获取会话列表
- `create_session(name)` - 创建新会话
- `send_message(message, session_id)` - 发送消息
- `emergency_stop()` - 紧急停止

---

## 📋 测试清单

启动后测试：
- [ ] 页面正常加载，无控制台错误
- [ ] CSS 样式正确应用
- [ ] JavaScript 模块正确加载
- [ ] WebSocket 连接成功
- [ ] 顶部导航栏显示正常
- [ ] 主题切换功能正常
- [ ] 语言切换功能正常
- [ ] 侧边栏显示正常
- [ ] 路由导航正常

---

## 🎯 下一步

1. 启动后端服务器
2. 启动 Tauri 应用
3. 测试所有功能
4. 报告任何问题

---

**重要**: 必须先启动 ClawMaster 后端服务器，Tauri 应用才能正常工作！
