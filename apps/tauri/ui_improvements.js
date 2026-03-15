// Tauri UI 改进脚本
// 添加 Toast 通知系统、加载状态、自定义对话框等

// ============================================
// 1. Toast 通知系统
// ============================================

function showToast(message, type = 'info', duration = 3000) {
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    
    const icon = {
        success: '✓',
        error: '✗',
        warning: '⚠',
        info: 'ℹ'
    }[type] || 'ℹ';
    
    toast.innerHTML = `
        <span class="toast-icon">${icon}</span>
        <span class="toast-message">${message}</span>
    `;
    
    document.body.appendChild(toast);
    
    // 触发动画
    requestAnimationFrame(() => {
        toast.classList.add('show');
    });
    
    // 自动移除
    setTimeout(() => {
        toast.classList.remove('show');
        setTimeout(() => toast.remove(), 300);
    }, duration);
}

// Toast CSS (添加到 <style> 标签)
const toastStyles = `
.toast {
    position: fixed;
    bottom: 20px;
    right: 20px;
    padding: 12px 20px;
    border-radius: 8px;
    background: var(--surface);
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
    transform: translateY(100px);
    opacity: 0;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    gap: 10px;
    z-index: 10000;
    max-width: 400px;
}

.toast.show {
    transform: translateY(0);
    opacity: 1;
}

.toast-icon {
    font-size: 18px;
    font-weight: bold;
}

.toast-message {
    flex: 1;
}

.toast-success {
    border-left: 4px solid #10b981;
}

.toast-success .toast-icon {
    color: #10b981;
}

.toast-error {
    border-left: 4px solid #ef4444;
}

.toast-error .toast-icon {
    color: #ef4444;
}

.toast-warning {
    border-left: 4px solid #f59e0b;
}

.toast-warning .toast-icon {
    color: #f59e0b;
}

.toast-info {
    border-left: 4px solid #3b82f6;
}

.toast-info .toast-icon {
    color: #3b82f6;
}
`;

// ============================================
// 2. 加载状态管理
// ============================================

function setButtonLoading(button, loading, loadingText = 'Loading...') {
    if (typeof button === 'string') {
        button = document.getElementById(button);
    }
    
    if (!button) return;
    
    if (loading) {
        button.disabled = true;
        button.classList.add('loading');
        button.dataset.originalHTML = button.innerHTML;
        button.innerHTML = `<span class="spinner"></span> ${loadingText}`;
    } else {
        button.disabled = false;
        button.classList.remove('loading');
        if (button.dataset.originalHTML) {
            button.innerHTML = button.dataset.originalHTML;
            delete button.dataset.originalHTML;
        }
    }
}

// Spinner CSS
const spinnerStyles = `
.spinner {
    display: inline-block;
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: currentColor;
    border-radius: 50%;
    animation: spin 0.6s linear infinite;
}

@keyframes spin {
    to { transform: rotate(360deg); }
}

button.loading {
    opacity: 0.7;
    cursor: not-allowed;
}
`;

// ============================================
// 3. 自定义确认对话框
// ============================================

function showConfirmDialog(title, message) {
    return new Promise((resolve) => {
        const overlay = document.createElement('div');
        overlay.className = 'dialog-overlay';
        overlay.innerHTML = `
            <div class="dialog">
                <div class="dialog-header">
                    <h3>${title}</h3>
                </div>
                <div class="dialog-body">
                    <p>${message}</p>
                </div>
                <div class="dialog-actions">
                    <button class="btn-secondary dialog-cancel">Cancel</button>
                    <button class="btn-primary dialog-confirm">Confirm</button>
                </div>
            </div>
        `;
        
        document.body.appendChild(overlay);
        
        // 触发动画
        requestAnimationFrame(() => {
            overlay.classList.add('show');
        });
        
        const close = (result) => {
            overlay.classList.remove('show');
            setTimeout(() => {
                overlay.remove();
                resolve(result);
            }, 200);
        };
        
        overlay.querySelector('.dialog-cancel').onclick = () => close(false);
        overlay.querySelector('.dialog-confirm').onclick = () => close(true);
        overlay.onclick = (e) => {
            if (e.target === overlay) close(false);
        };
        
        // ESC 键关闭
        const escHandler = (e) => {
            if (e.key === 'Escape') {
                document.removeEventListener('keydown', escHandler);
                close(false);
            }
        };
        document.addEventListener('keydown', escHandler);
    });
}

// ============================================
// 4. 自定义输入对话框
// ============================================

function showInputDialog(title, placeholder = '', defaultValue = '') {
    return new Promise((resolve) => {
        const overlay = document.createElement('div');
        overlay.className = 'dialog-overlay';
        overlay.innerHTML = `
            <div class="dialog">
                <div class="dialog-header">
                    <h3>${title}</h3>
                </div>
                <div class="dialog-body">
                    <input type="text" class="dialog-input" placeholder="${placeholder}" value="${defaultValue}">
                </div>
                <div class="dialog-actions">
                    <button class="btn-secondary dialog-cancel">Cancel</button>
                    <button class="btn-primary dialog-ok">OK</button>
                </div>
            </div>
        `;
        
        document.body.appendChild(overlay);
        
        const input = overlay.querySelector('.dialog-input');
        
        // 触发动画并聚焦
        requestAnimationFrame(() => {
            overlay.classList.add('show');
            input.focus();
            input.select();
        });
        
        const close = (result) => {
            overlay.classList.remove('show');
            setTimeout(() => {
                overlay.remove();
                resolve(result);
            }, 200);
        };
        
        overlay.querySelector('.dialog-cancel').onclick = () => close(null);
        overlay.querySelector('.dialog-ok').onclick = () => {
            const value = input.value.trim();
            close(value || null);
        };
        
        input.onkeydown = (e) => {
            if (e.key === 'Enter') {
                const value = input.value.trim();
                close(value || null);
            } else if (e.key === 'Escape') {
                close(null);
            }
        };
        
        overlay.onclick = (e) => {
            if (e.target === overlay) close(null);
        };
    });
}

// Dialog CSS
const dialogStyles = `
.dialog-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    opacity: 0;
    transition: opacity 0.2s ease;
}

.dialog-overlay.show {
    opacity: 1;
}

.dialog {
    background: var(--surface);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.3);
    min-width: 400px;
    max-width: 90vw;
    transform: scale(0.9);
    transition: transform 0.2s ease;
}

.dialog-overlay.show .dialog {
    transform: scale(1);
}

.dialog-header {
    padding: 20px 24px;
    border-bottom: 1px solid var(--border);
}

.dialog-header h3 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
}

.dialog-body {
    padding: 20px 24px;
}

.dialog-body p {
    margin: 0;
    color: var(--muted);
    line-height: 1.5;
}

.dialog-input {
    width: 100%;
    padding: 10px 12px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg);
    color: var(--text);
    font-size: 14px;
    font-family: inherit;
}

.dialog-input:focus {
    outline: none;
    border-color: var(--primary);
}

.dialog-actions {
    padding: 16px 24px;
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    border-top: 1px solid var(--border);
}

.btn-primary,
.btn-secondary {
    padding: 8px 16px;
    border-radius: 6px;
    border: none;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

.btn-primary {
    background: #3b82f6;
    color: white;
}

.btn-primary:hover {
    background: #2563eb;
}

.btn-secondary {
    background: var(--surface2);
    color: var(--text);
}

.btn-secondary:hover {
    background: var(--border);
}
`;

// ============================================
// 5. 改进的函数示例
// ============================================

// 改进的紧急停止函数
async function emergencyStopImproved() {
    const btn = document.getElementById('emergencyStopBtn');
    setButtonLoading(btn, true, 'Stopping...');
    
    try {
        await postBackend('/api/emergency-stop', {});
        showToast('Emergency stop executed successfully!', 'success');
    } catch (error) {
        showToast('Failed to execute emergency stop: ' + error.message, 'error');
        console.error('Emergency stop error:', error);
    } finally {
        setButtonLoading(btn, false);
    }
}

// 改进的新建会话函数
async function createNewSessionImproved() {
    const name = await showInputDialog('New Session', 'Enter session name', 'New Session');
    if (!name) return;
    
    const btn = document.getElementById('newSessionBtn');
    setButtonLoading(btn, true, 'Creating...');
    
    try {
        const result = await postBackend('/api/sessions', { label: name });
        await loadSessions();
        if (result.key || result.id) {
            selectSession(result.key || result.id);
        }
        showToast('Session created successfully!', 'success');
    } catch (error) {
        showToast('Failed to create session: ' + error.message, 'error');
        console.error('Failed to create session:', error);
    } finally {
        setButtonLoading(btn, false);
    }
}

// 改进的清空聊天函数
async function clearChatImproved() {
    const confirmed = await showConfirmDialog(
        'Clear Chat',
        'Are you sure you want to clear all messages in this session? This action cannot be undone.'
    );
    
    if (!confirmed) return;
    
    const btn = event.currentTarget;
    setButtonLoading(btn, true, 'Clearing...');
    
    try {
        await postBackend(`/api/sessions/${currentSessionId}/clear`, {});
        
        const container = document.getElementById('chatMessages');
        container.innerHTML = '';
        
        const welcomeCard = document.querySelector('.welcome-card');
        if (welcomeCard) welcomeCard.style.display = 'block';
        
        showToast('Chat cleared successfully!', 'success');
    } catch (error) {
        showToast('Failed to clear chat: ' + error.message, 'error');
        console.error('Failed to clear chat:', error);
    } finally {
        setButtonLoading(btn, false);
    }
}

// 改进的导出聊天函数
async function exportChatImproved() {
    const btn = event.currentTarget;
    setButtonLoading(btn, true, 'Exporting...');
    
    try {
        const result = await fetchBackend(`/api/sessions/${currentSessionId}/messages`);
        const blob = new Blob([JSON.stringify(result, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `chat-${currentSessionId}-${Date.now()}.json`;
        a.click();
        URL.revokeObjectURL(url);
        
        showToast('Chat exported successfully!', 'success');
    } catch (error) {
        showToast('Failed to export chat: ' + error.message, 'error');
        console.error('Failed to export chat:', error);
    } finally {
        setButtonLoading(btn, false);
    }
}

// ============================================
// 6. 初始化函数
// ============================================

function initUIImprovements() {
    // 添加样式
    const style = document.createElement('style');
    style.textContent = toastStyles + spinnerStyles + dialogStyles;
    document.head.appendChild(style);
    
    console.log('[UI Improvements] Toast, Dialog, and Loading systems initialized');
}

// 导出函数供外部使用
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        showToast,
        setButtonLoading,
        showConfirmDialog,
        showInputDialog,
        initUIImprovements
    };
}
