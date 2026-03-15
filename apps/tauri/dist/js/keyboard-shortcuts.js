/**
 * Keyboard Shortcuts System
 * Provides customizable keyboard shortcuts for common actions
 */

/**
 * Default keyboard shortcuts
 */
const DEFAULT_SHORTCUTS = {
    // Navigation
    'ctrl+1': { action: 'navigate', target: 'chat', description: 'Go to Chat' },
    'ctrl+2': { action: 'navigate', target: 'agents', description: 'Go to Agents' },
    'ctrl+3': { action: 'navigate', target: 'providers', description: 'Go to Providers' },
    'ctrl+4': { action: 'navigate', target: 'settings', description: 'Go to Settings' },
    
    // Chat actions
    'ctrl+n': { action: 'newChat', description: 'New Chat' },
    'ctrl+k': { action: 'clearChat', description: 'Clear Chat' },
    'ctrl+/': { action: 'toggleSidebar', description: 'Toggle Sidebar' },
    'ctrl+b': { action: 'toggleBold', description: 'Toggle Bold' },
    'ctrl+i': { action: 'toggleItalic', description: 'Toggle Italic' },
    
    // Search and commands
    'ctrl+f': { action: 'search', description: 'Search Messages' },
    'ctrl+p': { action: 'commandPalette', description: 'Command Palette' },
    'ctrl+shift+p': { action: 'providerSwitch', description: 'Switch Provider' },
    
    // Tool execution
    'ctrl+enter': { action: 'sendMessage', description: 'Send Message' },
    'shift+enter': { action: 'newLine', description: 'New Line' },
    'esc': { action: 'cancel', description: 'Cancel/Close' },
    
    // UI controls
    'ctrl+,': { action: 'openSettings', description: 'Open Settings' },
    'ctrl+shift+d': { action: 'toggleDarkMode', description: 'Toggle Dark Mode' },
    'ctrl+shift+t': { action: 'toggleToolViz', description: 'Toggle Tool Visualization' },
    
    // Help
    'ctrl+shift+/': { action: 'showShortcuts', description: 'Show Shortcuts' },
    '?': { action: 'showHelp', description: 'Show Help' }
};

/**
 * Keyboard shortcuts manager
 */
class KeyboardShortcutsManager {
    constructor() {
        this.shortcuts = { ...DEFAULT_SHORTCUTS };
        this.handlers = new Map();
        this.enabled = true;
        this.modalOpen = false;
        
        this.init();
    }

    init() {
        // Load custom shortcuts from localStorage
        const saved = localStorage.getItem('keyboard_shortcuts');
        if (saved) {
            try {
                this.shortcuts = { ...DEFAULT_SHORTCUTS, ...JSON.parse(saved) };
            } catch (e) {
                console.error('Failed to load shortcuts:', e);
            }
        }

        // Register global keydown listener
        document.addEventListener('keydown', this.handleKeyDown.bind(this));
    }

    handleKeyDown(event) {
        if (!this.enabled) return;
        
        // Don't intercept shortcuts when typing in inputs (except ctrl+enter)
        const isInput = event.target.tagName === 'INPUT' || 
                       event.target.tagName === 'TEXTAREA' ||
                       event.target.isContentEditable;
        
        const key = this.getKeyCombo(event);
        const shortcut = this.shortcuts[key];
        
        if (!shortcut) return;
        
        // Allow ctrl+enter in inputs
        if (isInput && key !== 'ctrl+enter' && key !== 'shift+enter' && key !== 'esc') {
            return;
        }

        // Execute the shortcut
        event.preventDefault();
        this.execute(shortcut.action, shortcut.target);
    }

    getKeyCombo(event) {
        const parts = [];
        
        if (event.ctrlKey || event.metaKey) parts.push('ctrl');
        if (event.shiftKey) parts.push('shift');
        if (event.altKey) parts.push('alt');
        
        const key = event.key.toLowerCase();
        if (key !== 'control' && key !== 'shift' && key !== 'alt' && key !== 'meta') {
            parts.push(key);
        }
        
        return parts.join('+');
    }

    execute(action, target) {
        // Emit custom event for the action
        window.dispatchEvent(new CustomEvent('shortcut:execute', {
            detail: { action, target }
        }));

        // Execute registered handlers
        const handler = this.handlers.get(action);
        if (handler) {
            handler(target);
        }
    }

    register(action, handler) {
        this.handlers.set(action, handler);
    }

    unregister(action) {
        this.handlers.delete(action);
    }

    setEnabled(enabled) {
        this.enabled = enabled;
    }

    customize(key, shortcut) {
        this.shortcuts[key] = shortcut;
        this.save();
    }

    reset() {
        this.shortcuts = { ...DEFAULT_SHORTCUTS };
        localStorage.removeItem('keyboard_shortcuts');
    }

    save() {
        try {
            localStorage.setItem('keyboard_shortcuts', JSON.stringify(this.shortcuts));
        } catch (e) {
            console.error('Failed to save shortcuts:', e);
        }
    }

    getAll() {
        return { ...this.shortcuts };
    }

    showHelp() {
        this.modalOpen = true;
        this.renderHelpModal();
    }

    renderHelpModal() {
        const modal = document.createElement('div');
        modal.className = 'shortcuts-modal';
        modal.innerHTML = `
            <div class="shortcuts-modal-overlay"></div>
            <div class="shortcuts-modal-content">
                <div class="shortcuts-modal-header">
                    <h2>Keyboard Shortcuts</h2>
                    <button class="close-btn" onclick="window.keyboardShortcuts.closeHelp()">×</button>
                </div>
                <div class="shortcuts-modal-body">
                    ${this.renderShortcutsList()}
                </div>
                <div class="shortcuts-modal-footer">
                    <button class="btn-secondary" onclick="window.keyboardShortcuts.reset()">
                        Reset to Defaults
                    </button>
                    <button class="btn-primary" onclick="window.keyboardShortcuts.closeHelp()">
                        Close
                    </button>
                </div>
            </div>
        `;

        document.body.appendChild(modal);

        // Close on overlay click
        modal.querySelector('.shortcuts-modal-overlay').addEventListener('click', () => {
            this.closeHelp();
        });

        // Close on Escape
        const escHandler = (e) => {
            if (e.key === 'Escape') {
                this.closeHelp();
                document.removeEventListener('keydown', escHandler);
            }
        };
        document.addEventListener('keydown', escHandler);
    }

    renderShortcutsList() {
        const categories = {
            'Navigation': ['navigate'],
            'Chat Actions': ['newChat', 'clearChat', 'toggleSidebar', 'sendMessage', 'newLine'],
            'Search & Commands': ['search', 'commandPalette', 'providerSwitch'],
            'UI Controls': ['openSettings', 'toggleDarkMode', 'toggleToolViz', 'toggleBold', 'toggleItalic'],
            'Help': ['showShortcuts', 'showHelp', 'cancel']
        };

        let html = '';
        for (const [category, actions] of Object.entries(categories)) {
            html += `<div class="shortcut-category">
                <h3>${category}</h3>
                <div class="shortcut-list">`;
            
            for (const [key, shortcut] of Object.entries(this.shortcuts)) {
                if (actions.includes(shortcut.action)) {
                    html += `
                        <div class="shortcut-item">
                            <span class="shortcut-key">${this.formatKey(key)}</span>
                            <span class="shortcut-description">${shortcut.description}</span>
                        </div>
                    `;
                }
            }
            
            html += `</div></div>`;
        }

        return html;
    }

    formatKey(key) {
        return key
            .split('+')
            .map(k => {
                const formatted = {
                    'ctrl': '⌘',
                    'shift': '⇧',
                    'alt': '⌥',
                    'enter': '↵',
                    'esc': 'Esc'
                }[k] || k.toUpperCase();
                return `<kbd>${formatted}</kbd>`;
            })
            .join(' ');
    }

    closeHelp() {
        const modal = document.querySelector('.shortcuts-modal');
        if (modal) {
            modal.remove();
        }
        this.modalOpen = false;
    }
}

// Create global instance
window.keyboardShortcuts = new KeyboardShortcutsManager();

// Export for module usage
export default window.keyboardShortcuts;

/**
 * Register default handlers
 */
window.keyboardShortcuts.register('navigate', (target) => {
    const routes = {
        'chat': '#/chat',
        'agents': '#/agents',
        'providers': '#/providers',
        'settings': '#/settings'
    };
    if (routes[target]) {
        window.location.hash = routes[target];
    }
});

window.keyboardShortcuts.register('newChat', () => {
    window.dispatchEvent(new CustomEvent('chat:new'));
});

window.keyboardShortcuts.register('clearChat', () => {
    if (confirm('Clear current chat?')) {
        window.dispatchEvent(new CustomEvent('chat:clear'));
    }
});

window.keyboardShortcuts.register('toggleSidebar', () => {
    document.body.classList.toggle('sidebar-collapsed');
});

window.keyboardShortcuts.register('search', () => {
    const searchInput = document.querySelector('.search-input');
    if (searchInput) {
        searchInput.focus();
    }
});

window.keyboardShortcuts.register('commandPalette', () => {
    window.dispatchEvent(new CustomEvent('commandPalette:open'));
});

window.keyboardShortcuts.register('openSettings', () => {
    window.location.hash = '#/settings';
});

window.keyboardShortcuts.register('toggleDarkMode', () => {
    const currentTheme = document.documentElement.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
});

window.keyboardShortcuts.register('toggleToolViz', () => {
    const viz = document.querySelector('.tool-execution-viz');
    if (viz) {
        viz.classList.toggle('hidden');
    }
});

window.keyboardShortcuts.register('showShortcuts', () => {
    window.keyboardShortcuts.showHelp();
});

window.keyboardShortcuts.register('showHelp', () => {
    window.open('https://docs.clawmaster.org', '_blank');
});

window.keyboardShortcuts.register('cancel', () => {
    // Close any open modals
    const modals = document.querySelectorAll('.modal, .dialog, .popup');
    modals.forEach(modal => modal.remove());
    
    // Blur active element
    if (document.activeElement) {
        document.activeElement.blur();
    }
});

window.keyboardShortcuts.register('sendMessage', () => {
    window.dispatchEvent(new CustomEvent('chat:send'));
});

window.keyboardShortcuts.register('newLine', () => {
    // Handled by textarea naturally
});

window.keyboardShortcuts.register('providerSwitch', () => {
    window.dispatchEvent(new CustomEvent('provider:switch'));
});

window.keyboardShortcuts.register('toggleBold', () => {
    document.execCommand('bold');
});

window.keyboardShortcuts.register('toggleItalic', () => {
    document.execCommand('italic');
});
