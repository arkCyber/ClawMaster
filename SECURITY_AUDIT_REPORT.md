# ClawMaster 安全措施审计报告

**审计日期**: 2026-03-13  
**审计范围**: AI 智能体高风险操作确认机制  
**审计标准**: 企业级安全最佳实践

---

## 📋 审计总结

### ✅ 已实现的安全功能
| 功能 | 状态 | 位置 |
|------|------|------|
| 命令执行审批系统 | ✅ 完整实现 | 后端 + 前端 |
| 危险命令检测 | ✅ 18 种模式 | 后端自动检测 |
| 实时用户确认 | ✅ WebSocket 推送 | 前端审批卡片 |
| 超时保护机制 | ✅ 120 秒倒计时 | 前端 + 后端 |
| 审批卡片 UI | ✅ 完整实现 | 聊天界面 |

### ⚠️ 发现的问题
| 问题 | 严重程度 | 建议 |
|------|----------|------|
| 缺少主界面熔断开关 | 🟡 中等 | 需要添加 |
| 缺少配置入口提示 | 🟡 中等 | 需要添加 |
| 审批模式不够明显 | 🟡 中等 | 需要改进 |
| 缺少安全状态指示器 | 🟡 中等 | 需要添加 |

---

## 🔍 详细审计结果

### 1. 终端对话中的审批实现 ✅

#### 后端实现
**文件**: `crates/tools/src/exec.rs:463-520`

```rust
// 审批门控逻辑
if !is_sandboxed && let Some(ref mgr) = self.approval_manager {
    let action = mgr.check_command(command).await?;
    if action == ApprovalAction::NeedsApproval {
        info!(command, "command needs approval, waiting...");
        let (req_id, rx) = mgr.create_request(command).await;

        // 广播到连接的客户端
        if let Some(ref bc) = self.broadcaster
            && let Err(e) = bc.broadcast_request(&req_id, command).await
        {
            warn!(error = %e, "failed to broadcast approval request");
        }

        let decision = mgr.wait_for_decision(rx).await;
        match decision {
            ApprovalDecision::Approved => {
                info!(command, "command approved");
            },
            ApprovalDecision::Denied => {
                return Err(Error::message(format!("command denied by user: {command}")).into());
            },
            ApprovalDecision::Timeout => {
                return Err(Error::message(format!("approval timed out for command: {command}")).into());
            },
        }
    }
}
```

**评估**: ✅ **完整实现**
- 在命令执行前自动检查
- 支持 WebSocket 实时推送
- 完整的超时处理
- 详细的日志记录

---

### 2. UI 界面审批卡片 ✅

#### 前端实现
**文件**: `crates/web/src/templates/index.html:448-458`

```html
<template id="tpl-approval-card">
  <div class="msg approval-card">
    <div class="approval-label">Command requires approval:</div>
    <code class="approval-cmd"></code>
    <div class="approval-btns">
      <button class="approval-btn approval-allow">Allow</button>
      <button class="approval-btn approval-deny">Deny</button>
    </div>
    <div class="approval-countdown"></div>
  </div>
</template>
```

**JavaScript 渲染**: `crates/web/src/assets/js/chat-ui.js:192-228`

```javascript
export function renderApprovalCard(requestId, command) {
    if (!S.chatMsgBox) return;
    clearChatEmptyState();
    var tpl = document.getElementById("tpl-approval-card");
    var frag = tpl.content.cloneNode(true);
    var card = frag.firstElementChild;
    card.id = `approval-${requestId}`;

    card.querySelector(".approval-cmd").textContent = command;

    var allowBtn = card.querySelector(".approval-allow");
    var denyBtn = card.querySelector(".approval-deny");
    allowBtn.onclick = () => {
        resolveApproval(requestId, "approved", command, card);
    };
    denyBtn.onclick = () => {
        resolveApproval(requestId, "denied", null, card);
    };

    // 120 秒倒计时
    var countdown = card.querySelector(".approval-countdown");
    var remaining = 120;
    var timer = setInterval(() => {
        remaining--;
        countdown.textContent = `${remaining}s`;
        if (remaining <= 0) {
            clearInterval(timer);
            card.classList.add("approval-expired");
            allowBtn.disabled = true;
            denyBtn.disabled = true;
            countdown.textContent = "expired";
        }
    }, 1000);
    countdown.textContent = `${remaining}s`;

    S.chatMsgBox.appendChild(card);
    S.chatMsgBox.scrollTop = S.chatMsgBox.scrollHeight;
}
```

**CSS 样式**: `crates/web/src/assets/css/chat.css:620-690`

```css
/* ── Approval Cards ── */

.approval-card {
  background: var(--surface2);
  border: 1px solid var(--warn);  /* 警告色边框 */
  border-radius: var(--radius);
  padding: 12px;
  margin: 8px 0;
  max-width: 500px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.approval-label {
  color: var(--warn);  /* 警告色文字 */
  font-size: 0.8rem;
  font-weight: 600;
}

.approval-cmd {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 8px;
  font-family: var(--font-mono);
  font-size: 0.8rem;
  word-break: break-all;
  display: block;
}

.approval-btns {
  gap: 8px;
  display: flex;
}

.approval-btn {
  border-radius: var(--radius-sm);
  cursor: pointer;
  border: none;
  padding: 6px 16px;
  font-size: 0.8rem;
  font-weight: 500;
  transition: opacity 0.15s;
}

.approval-btn:disabled {
  opacity: 0.4;
  cursor: default;
}

.approval-allow { background: var(--ok); color: #fff; }
.approval-deny { background: var(--error); color: #fff; }

.approval-countdown {
  color: var(--muted);
  text-align: right;
  font-size: 0.72rem;
}

.approval-resolved { opacity: 0.6; }
.approval-expired { opacity: 0.4; }
```

**评估**: ✅ **完整实现**
- 审批卡片在聊天界面中显示
- 醒目的警告色设计
- 清晰的 Allow/Deny 按钮
- 120 秒倒计时显示
- 超时自动禁用按钮

---

### 3. WebSocket 事件处理 ✅

**文件**: `crates/web/src/assets/js/websocket.js:1022-1024, 1304`

```javascript
// 事件处理器注册
var eventHandlers = {
    chat: handleChatEvent,
    error: handleWsError,
    "auth.credentials_changed": handleAuthCredentialsChanged,
    "exec.approval.requested": handleApprovalEvent,  // ✅ 审批请求
    "logs.entry": handleLogEntry,
    // ...
};

// 审批事件处理
function handleApprovalEvent(payload) {
    renderApprovalCard(payload.requestId, payload.command);
}
```

**评估**: ✅ **完整实现**
- WebSocket 实时推送
- 自动渲染审批卡片
- 无需用户手动刷新

---

### 4. ❌ 主界面缺少熔断开关

#### 当前状态
**问题**: 主界面（顶部栏）没有明显的紧急停止按钮

**当前顶部栏**: `crates/web/src/templates/index.html:118-158`
```html
<header class="flex items-center gap-3 px-4 py-2.5 border-b border-[var(--border)] bg-[var(--surface)] shrink-0">
  <a href="{{ routes.chats }}" id="titleLink">...</a>
  <span class="status-dot" id="statusDot"></span>
  <span class="text-xs text-[var(--muted)]" id="statusText">disconnected</span>
  <div class="flex-1"></div>
  
  <!-- 只有内存信息、设置按钮、主题切换 -->
  <span id="memoryInfo" class="text-xs text-[var(--muted)]"></span>
  <button id="settingsBtn" class="header-link-btn">...</button>
  <div id="languageSelectorContainer"></div>
  <div class="theme-toggle" id="themeToggle">...</div>
  <button id="logoutBtn" class="logout-btn" style="display:none">...</button>
</header>
```

**缺失内容**:
- ❌ 没有紧急停止/熔断按钮
- ❌ 没有安全模式指示器
- ❌ 没有审批模式状态显示

---

### 5. ❌ 设置界面缺少明显的安全配置入口

#### 当前状态
**问题**: 审批模式配置隐藏在配置文件中，UI 没有明显入口

**当前配置方式**: 
- 需要手动编辑 `~/.clawmaster/clawmaster.toml`
- 没有 UI 界面配置选项
- 用户不知道如何启用/配置审批模式

**缺失内容**:
- ❌ 没有"安全设置"页面
- ❌ 没有审批模式切换开关
- ❌ 没有安全等级选择器
- ❌ 没有白名单管理界面

---

## 🎯 改进建议

### 优先级 1: 添加主界面熔断开关 🔴

#### 建议实现
在顶部栏添加紧急停止按钮：

```html
<!-- 在 header 中添加 -->
<button id="emergencyStopBtn" class="emergency-stop-btn" 
        title="Emergency Stop - Abort all running commands">
  <span class="icon icon-stop-circle"></span>
  <span class="emergency-label">STOP</span>
</button>
```

```css
.emergency-stop-btn {
  background: var(--error);
  color: white;
  border: none;
  border-radius: var(--radius);
  padding: 6px 12px;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: all 0.2s;
}

.emergency-stop-btn:hover {
  background: #dc2626;
  transform: scale(1.05);
}

.emergency-stop-btn:active {
  transform: scale(0.95);
}
```

```javascript
// 紧急停止功能
document.getElementById("emergencyStopBtn").addEventListener("click", () => {
  if (confirm("确定要停止所有正在运行的命令吗？")) {
    sendRpc("chat.abort", { sessionKey: currentSession });
    sendRpc("chat.cancel_queued", { sessionKey: currentSession });
  }
});
```

---

### 优先级 2: 添加安全状态指示器 🟡

#### 建议实现
在顶部栏显示当前安全模式：

```html
<!-- 在 header 中添加 -->
<div id="securityModeIndicator" class="security-indicator">
  <span class="security-icon">🛡️</span>
  <span class="security-mode">Smart</span>
</div>
```

```css
.security-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--surface2);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  font-size: 0.75rem;
  color: var(--muted);
}

.security-mode {
  font-weight: 500;
}

/* 不同模式的颜色 */
.security-indicator[data-mode="always"] {
  border-color: var(--ok);
  color: var(--ok);
}

.security-indicator[data-mode="on-miss"] {
  border-color: var(--accent);
  color: var(--accent);
}

.security-indicator[data-mode="off"] {
  border-color: var(--warn);
  color: var(--warn);
}
```

---

### 优先级 3: 添加安全设置页面 🟡

#### 建议实现
创建专门的安全配置界面：

```javascript
// 在 page-settings.js 中添加安全设置部分
function SecuritySettings() {
  const [approvalMode, setApprovalMode] = useState("on-miss");
  const [securityLevel, setSecurityLevel] = useState("allowlist");
  const [allowlist, setAllowlist] = useState([]);

  return html`
    <div class="settings-section">
      <h2 class="settings-section-title">🛡️ 安全设置</h2>
      
      <!-- 审批模式 -->
      <div class="settings-group">
        <h3>审批模式</h3>
        <div class="radio-group">
          <label>
            <input type="radio" name="approvalMode" value="off"
                   checked=${approvalMode === "off"}
                   onChange=${() => setApprovalMode("off")} />
            <span>关闭</span>
            <span class="help-text">不需要审批（不推荐）</span>
          </label>
          <label>
            <input type="radio" name="approvalMode" value="on-miss"
                   checked=${approvalMode === "on-miss"}
                   onChange=${() => setApprovalMode("on-miss")} />
            <span>智能模式</span>
            <span class="help-text">安全命令自动通过，其他需要审批（推荐）</span>
          </label>
          <label>
            <input type="radio" name="approvalMode" value="always"
                   checked=${approvalMode === "always"}
                   onChange=${() => setApprovalMode("always")} />
            <span>总是审批</span>
            <span class="help-text">所有命令都需要审批（最安全）</span>
          </label>
        </div>
      </div>

      <!-- 安全等级 -->
      <div class="settings-group">
        <h3>安全等级</h3>
        <div class="radio-group">
          <label>
            <input type="radio" name="securityLevel" value="deny"
                   checked=${securityLevel === "deny"}
                   onChange=${() => setSecurityLevel("deny")} />
            <span>拒绝</span>
            <span class="help-text">禁止所有命令执行</span>
          </label>
          <label>
            <input type="radio" name="securityLevel" value="allowlist"
                   checked=${securityLevel === "allowlist"}
                   onChange=${() => setSecurityLevel("allowlist")} />
            <span>白名单</span>
            <span class="help-text">只允许白名单中的命令（推荐）</span>
          </label>
          <label>
            <input type="radio" name="securityLevel" value="full"
                   checked=${securityLevel === "full"}
                   onChange=${() => setSecurityLevel("full")} />
            <span>完全</span>
            <span class="help-text">允许所有命令（危险命令仍需审批）</span>
          </label>
        </div>
      </div>

      <!-- 白名单管理 -->
      <div class="settings-group">
        <h3>命令白名单</h3>
        <div class="allowlist-manager">
          ${allowlist.map(cmd => html`
            <div class="allowlist-item">
              <code>${cmd}</code>
              <button onClick=${() => removeFromAllowlist(cmd)}>删除</button>
            </div>
          `)}
          <button class="provider-btn" onClick=${addToAllowlist}>
            添加命令
          </button>
        </div>
      </div>

      <!-- 危险命令列表 -->
      <div class="settings-group">
        <h3>⚠️ 危险命令模式（自动检测）</h3>
        <div class="dangerous-patterns">
          <ul>
            <li>rm -r / (删除根目录)</li>
            <li>git reset --hard (强制重置)</li>
            <li>git push --force (强制推送)</li>
            <li>DROP TABLE/DATABASE (删除数据库)</li>
            <li>docker system prune (清理容器)</li>
            <li>terraform destroy (销毁基础设施)</li>
            <li>... 共 18 种模式</li>
          </ul>
          <p class="help-text">
            这些命令无论配置如何都会强制要求审批
          </p>
        </div>
      </div>
    </div>
  `;
}
```

---

### 优先级 4: 改进审批卡片可见性 🟢

#### 建议改进

1. **添加声音提示**
```javascript
function renderApprovalCard(requestId, command) {
    // ... 现有代码 ...
    
    // 播放提示音
    playNotificationSound();
    
    // 浏览器通知
    if (Notification.permission === "granted") {
        new Notification("需要审批", {
            body: `命令: ${command}`,
            icon: "/icons/icon-72.png"
        });
    }
}
```

2. **添加闪烁效果**
```css
@keyframes approval-pulse {
  0%, 100% { border-color: var(--warn); }
  50% { border-color: var(--error); }
}

.approval-card {
  animation: approval-pulse 1s ease-in-out 3;
}
```

3. **添加顶部横幅提示**
```html
<div id="approvalBanner" class="approval-banner" style="display:none">
  <span>⚠️ 有命令等待审批</span>
  <button onclick="scrollToApproval()">查看</button>
</div>
```

---

## 📊 安全功能对比表

| 功能 | 当前状态 | 建议改进 |
|------|----------|----------|
| 命令执行审批 | ✅ 完整实现 | - |
| 危险命令检测 | ✅ 18 种模式 | 可扩展更多模式 |
| 审批卡片 UI | ✅ 完整实现 | 添加声音/通知 |
| WebSocket 推送 | ✅ 实时推送 | - |
| 超时保护 | ✅ 120 秒 | 可配置时长 |
| 主界面熔断开关 | ❌ 缺失 | **需要添加** |
| 安全状态指示器 | ❌ 缺失 | **需要添加** |
| UI 配置界面 | ❌ 缺失 | **需要添加** |
| 白名单管理 | ⚠️ 仅配置文件 | **需要 UI** |
| 审批历史记录 | ❌ 缺失 | 可选添加 |
| 审批统计 | ❌ 缺失 | 可选添加 |

---

## 🔐 安全最佳实践建议

### 1. 默认配置
```toml
[tools.exec]
approval_mode = "on-miss"  # 智能模式
security_level = "allowlist"  # 白名单
allowlist = [
    "ls", "cat", "echo", "pwd",
    "git status", "git log", "git diff"
]

[tools.exec.sandbox]
mode = "docker"  # 启用沙箱
```

### 2. 用户教育
- 在首次启动时显示安全设置向导
- 在设置页面添加安全最佳实践说明
- 在审批卡片中显示命令风险等级

### 3. 审计日志
- 记录所有审批决策
- 记录被拒绝的命令
- 定期审查审批历史

---

## 📝 实施计划

### 第一阶段（紧急）
- [ ] 添加主界面紧急停止按钮
- [ ] 添加安全模式指示器
- [ ] 改进审批卡片可见性（声音/通知）

### 第二阶段（重要）
- [ ] 创建安全设置页面
- [ ] 添加审批模式 UI 配置
- [ ] 添加白名单管理界面
- [ ] 添加安全等级选择器

### 第三阶段（优化）
- [ ] 添加审批历史记录
- [ ] 添加审批统计面板
- [ ] 添加命令风险评分
- [ ] 添加安全设置导入/导出

---

## ✅ 审计结论

### 总体评分: B+ (良好)

**优点**:
- ✅ 核心审批系统完整实现
- ✅ 危险命令检测全面
- ✅ 实时推送机制可靠
- ✅ 超时保护完善
- ✅ UI 设计清晰

**需要改进**:
- ⚠️ 缺少主界面熔断开关
- ⚠️ 缺少明显的配置入口
- ⚠️ 缺少安全状态可视化
- ⚠️ 用户教育不足

### 建议
1. **立即实施**: 添加紧急停止按钮和安全指示器
2. **短期实施**: 创建安全设置页面
3. **长期优化**: 完善审计和统计功能

---

**审计人员**: Cascade AI  
**审计日期**: 2026-03-13  
**下次审计**: 建议在实施改进后 1 个月内
