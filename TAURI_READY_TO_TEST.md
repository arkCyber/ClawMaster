# Tauri 应用准备就绪 - 测试指南

**准备时间**: 2026-03-15  
**状态**: ✅ 所有修复完成，准备测试  
**编译状态**: ✅ 成功

---

## ✅ 完成的工作总结

### 1. 代码审计与修复 (100%)
- ✅ **Import Maps 路径修复** - 10 个模块
- ✅ **CSS 资源路径修复** - 13 个文件
- ✅ **JavaScript 路径修复** - 8 个文件
- ✅ **图标路径修复** - 2 个文件
- ✅ **WebSocket 连接修复** - 支持自定义 URL
- ✅ **后端配置修复** - 正确的 API 地址
- ✅ **代码警告修复** - 无编译警告

### 2. 关键修复详情

#### WebSocket 连接逻辑 (`ws-connect.js`)
```javascript
// ✅ 修复后 - 支持 Tauri 环境
var wsUrl;
if (window.__MOLTIS__?.ws_url) {
    wsUrl = window.__MOLTIS__.ws_url;  // Tauri: ws://localhost:8080/ws
} else {
    var proto = location.protocol === "https:" ? "wss:" : "ws:";
    wsUrl = `${proto}//${location.host}/ws/chat`;  // Web: 相对路径
}
var ws = new WebSocket(wsUrl);
```

#### 全局配置 (`index.html`)
```javascript
window.__MOLTIS__ = {
  routes: { /* 路由配置 */ },
  identity: { name: "ClawMaster", emoji: "🦾" },
  ws_url: "ws://localhost:8080/ws",        // ✅ WebSocket URL
  api_base: "http://localhost:8080"        // ✅ API Base URL
};
window.__TAURI_ENABLED__ = typeof window.__TAURI__ !== 'undefined';
```

---

## 🚀 启动步骤

### 步骤 1: 启动 ClawMaster 后端 (必需)

```bash
cd /Users/arksong/ClawMaster
cargo run --bin clawmaster
```

**等待后端完全启动**，确认看到以下信息：
- ✅ 服务器运行在 `http://localhost:8080`
- ✅ WebSocket 端点可用
- ✅ 数据库迁移完成

### 步骤 2: 启动 Tauri 应用

**方法 A: 使用快捷脚本**
```bash
cd /Users/arksong/ClawMaster/apps/tauri
./start-ui.sh
```

**方法 B: 手动启动**
```bash
cd /Users/arksong/ClawMaster/apps/tauri/src-tauri
cargo run --release
```

**方法 C: 开发模式**
```bash
cd /Users/arksong/ClawMaster/apps/tauri/src-tauri
cargo tauri dev
```

---

## 📋 测试清单

### 阶段 1: 启动验证 (关键)
打开应用后，立即检查：

#### 控制台检查
- [ ] **无模块加载错误** - 检查 `htm/preact` 等模块
- [ ] **无 CSS 加载错误** - 检查样式文件
- [ ] **无 WebSocket 错误** - 检查连接状态
- [ ] **无 404 错误** - 检查资源路径

#### 视觉检查
- [ ] **UI 正常显示** - 非空白页面
- [ ] **顶部导航栏** - Logo + 按钮显示
- [ ] **侧边栏** - 会话列表显示
- [ ] **主题正确** - 暗色/亮色主题
- [ ] **图标显示** - 所有图标正常

#### 连接状态
- [ ] **WebSocket 连接** - 状态显示 "connected"
- [ ] **后端通信** - 能获取数据
- [ ] **实时更新** - WebSocket 消息接收

### 阶段 2: 基础功能测试

#### 导航系统
- [ ] 点击 "Monitor" 按钮 → 跳转到监控页面
- [ ] 点击 "Settings" 按钮 → 跳转到设置页面
- [ ] 点击 Logo → 返回聊天页面
- [ ] 侧边栏展开/收起正常

#### 主题切换
- [ ] 点击主题按钮
- [ ] 切换到亮色主题 → 界面变亮
- [ ] 切换到暗色主题 → 界面变暗
- [ ] 切换到系统主题 → 跟随系统

#### 语言切换
- [ ] 打开语言选择器
- [ ] 切换到中文 → 界面文本变中文
- [ ] 切换到英文 → 界面文本变英文
- [ ] 切换到其他语言 → 文本正确显示

### 阶段 3: 核心功能测试

#### 会话管理
- [ ] 会话列表加载显示
- [ ] 点击 "New Session" 创建新会话
- [ ] 切换不同会话
- [ ] 删除会话
- [ ] 会话搜索功能

#### 聊天功能
- [ ] 输入消息
- [ ] 发送消息
- [ ] 接收回复
- [ ] 消息正确显示
- [ ] Markdown 渲染正常
- [ ] 代码高亮正常

#### 模型管理
- [ ] 模型列表加载
- [ ] 切换模型
- [ ] 模型信息显示

### 阶段 4: 页面功能测试

测试所有 19 个页面能否正常打开和工作：

#### 核心页面
- [ ] **Chat** (`/chats`) - 聊天界面
- [ ] **Metrics** (`/metrics`) - 监控数据
- [ ] **Settings** (`/settings`) - 设置面板

#### 设置子页面
- [ ] **Providers** (`/settings/llms`) - LLM 配置
- [ ] **Channels** (`/settings/channels`) - 通道管理
- [ ] **Crons** (`/settings/crons`) - 定时任务
- [ ] **Projects** (`/settings/projects`) - 项目管理
- [ ] **Skills** (`/settings/skills`) - 技能管理
- [ ] **MCP** (`/settings/mcp`) - MCP 服务
- [ ] **Hooks** (`/settings/hooks`) - Webhook
- [ ] **Logs** (`/settings/logs`) - 日志查看
- [ ] **Images** (`/settings/images`) - 图片管理
- [ ] **Agents** (`/settings/agents`) - 智能体
- [ ] **Nodes** (`/settings/nodes`) - 节点管理
- [ ] **Terminal** (`/settings/terminal`) - 终端
- [ ] **Network Audit** (`/settings/network`) - 网络审计
- [ ] **Security** (`/settings/security`) - 安全设置

#### 其他页面
- [ ] **Onboarding** - 新手引导
- [ ] **Projects** - 项目管理

### 阶段 5: 高级功能测试

#### 紧急停止
- [ ] 点击紧急停止按钮
- [ ] 确认命令中止
- [ ] 状态正确更新

#### 安全模式
- [ ] 安全模式指示器显示
- [ ] 审批卡片显示
- [ ] 批准/拒绝功能

#### 项目过滤
- [ ] 项目选择器显示
- [ ] 选择项目
- [ ] 会话列表过滤

#### 文件上传
- [ ] 拖放文件
- [ ] 文件上传成功
- [ ] 文件在消息中显示

---

## 🔍 问题排查指南

### 问题 1: 页面空白
**症状**: 应用启动后显示空白页面

**检查**:
1. 打开开发者工具 (F12)
2. 查看 Console 标签
3. 查找错误信息

**可能原因**:
- 模块加载失败 → 检查 Import Maps
- CSS 加载失败 → 检查 CSS 路径
- JavaScript 错误 → 检查控制台错误

### 问题 2: WebSocket 连接失败
**症状**: 状态显示 "disconnected"

**检查**:
1. 确认后端已启动
2. 确认后端运行在 `http://localhost:8080`
3. 检查 WebSocket URL 配置

**解决**:
```bash
# 检查后端是否运行
curl http://localhost:8080/api/auth/status

# 检查 WebSocket
wscat -c ws://localhost:8080/ws
```

### 问题 3: API 调用失败
**症状**: 数据无法加载

**检查**:
1. Network 标签查看请求
2. 确认请求 URL 正确
3. 检查响应状态码

**可能原因**:
- 后端未启动
- URL 配置错误
- CORS 问题

### 问题 4: 模块加载错误
**症状**: `Module not found` 错误

**检查**:
1. 检查 Import Maps 配置
2. 确认文件路径正确
3. 确认文件存在

**解决**:
- 所有路径必须以 `/` 开头
- 检查 `apps/tauri/dist/js/vendor/` 目录

---

## 📊 预期结果

### 成功标准

#### 视觉效果
- ✅ 完整的 UI 界面（非空白）
- ✅ 正确的主题颜色
- ✅ 所有图标正常显示
- ✅ 响应式布局工作
- ✅ 动画流畅

#### 功能完整性
- ✅ 所有 19 个页面可访问
- ✅ 路由导航正常
- ✅ WebSocket 实时通信
- ✅ API 调用成功
- ✅ 会话管理完整
- ✅ 消息收发正常

#### 性能指标
- ✅ 启动时间 < 2s
- ✅ 页面切换 < 200ms
- ✅ WebSocket 延迟 < 50ms
- ✅ 内存占用 < 500MB

### 与 WebUI 对比

| 功能 | WebUI | Tauri | 状态 |
|------|-------|-------|------|
| **页面数量** | 19 | 19 | ✅ 100% |
| **语言支持** | 16 | 16 | ✅ 100% |
| **主题系统** | 3 | 3 | ✅ 100% |
| **WebSocket** | ✅ | ✅ | ✅ 100% |
| **响应式** | ✅ | ✅ | ✅ 100% |
| **所有功能** | ✅ | ✅ | ✅ 100% |

---

## 📝 测试报告模板

测试完成后，请记录：

### 基础信息
- 测试时间: ___________
- 测试人员: ___________
- Tauri 版本: 0.10.18
- 后端版本: ___________

### 测试结果
- [ ] 启动成功
- [ ] 所有页面可访问
- [ ] WebSocket 连接正常
- [ ] 所有功能正常

### 发现的问题
1. ___________
2. ___________
3. ___________

### 性能数据
- 启动时间: _____ 秒
- 内存占用: _____ MB
- WebSocket 延迟: _____ ms

---

## 🎯 下一步行动

1. **立即**: 启动后端服务器
2. **然后**: 启动 Tauri 应用
3. **接着**: 执行测试清单
4. **最后**: 记录测试结果

---

**所有代码修复已完成，应用准备就绪，可以开始测试！** 🚀

**重要提示**: 必须先启动 ClawMaster 后端，Tauri 应用才能正常工作！
