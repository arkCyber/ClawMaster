/**
 * Command Palette
 * Quick access to all features with fuzzy search
 */

import { html, render } from './vendor/preact-htm.js';
import { useState, useEffect, useRef } from './vendor/preact-hooks.js';
import * as icons from './icons.js';

/**
 * Available commands
 */
const COMMANDS = [
    // Navigation
    { id: 'nav-chat', label: 'Go to Chat', icon: icons.messageSquare, action: () => window.location.hash = '#/chat', category: 'Navigation' },
    { id: 'nav-agents', label: 'Go to Agents', icon: icons.users, action: () => window.location.hash = '#/agents', category: 'Navigation' },
    { id: 'nav-providers', label: 'Go to Providers', icon: icons.brain, action: () => window.location.hash = '#/providers', category: 'Navigation' },
    { id: 'nav-settings', label: 'Go to Settings', icon: icons.settings, action: () => window.location.hash = '#/settings', category: 'Navigation' },
    { id: 'nav-skills', label: 'Go to Skills', icon: icons.zap, action: () => window.location.hash = '#/skills', category: 'Navigation' },
    { id: 'nav-mcp', label: 'Go to MCP Servers', icon: icons.server, action: () => window.location.hash = '#/mcp', category: 'Navigation' },
    
    // Chat Actions
    { id: 'chat-new', label: 'New Chat', icon: icons.plus, action: () => window.dispatchEvent(new CustomEvent('chat:new')), category: 'Chat' },
    { id: 'chat-clear', label: 'Clear Current Chat', icon: icons.trash, action: () => window.dispatchEvent(new CustomEvent('chat:clear')), category: 'Chat' },
    { id: 'chat-export', label: 'Export Chat', icon: icons.download, action: () => window.dispatchEvent(new CustomEvent('chat:export')), category: 'Chat' },
    { id: 'chat-search', label: 'Search Messages', icon: icons.search, action: () => window.dispatchEvent(new CustomEvent('chat:search')), category: 'Chat' },
    
    // Provider Actions
    { id: 'provider-switch', label: 'Switch Provider', icon: icons.repeat, action: () => window.dispatchEvent(new CustomEvent('provider:switch')), category: 'Providers' },
    { id: 'provider-add', label: 'Add Provider', icon: icons.plus, action: () => window.location.hash = '#/providers?action=add', category: 'Providers' },
    { id: 'provider-test', label: 'Test Provider Connection', icon: icons.activity, action: () => window.dispatchEvent(new CustomEvent('provider:test')), category: 'Providers' },
    
    // UI Actions
    { id: 'ui-theme-toggle', label: 'Toggle Dark Mode', icon: icons.moon, action: () => {
        const current = document.documentElement.getAttribute('data-theme');
        const newTheme = current === 'dark' ? 'light' : 'dark';
        document.documentElement.setAttribute('data-theme', newTheme);
        localStorage.setItem('theme', newTheme);
    }, category: 'UI' },
    { id: 'ui-sidebar-toggle', label: 'Toggle Sidebar', icon: icons.sidebar, action: () => document.body.classList.toggle('sidebar-collapsed'), category: 'UI' },
    { id: 'ui-tool-viz', label: 'Toggle Tool Visualization', icon: icons.tool, action: () => {
        const viz = document.querySelector('.tool-execution-viz');
        if (viz) viz.classList.toggle('hidden');
    }, category: 'UI' },
    { id: 'ui-fullscreen', label: 'Toggle Fullscreen', icon: icons.maximize, action: () => {
        if (!document.fullscreenElement) {
            document.documentElement.requestFullscreen();
        } else {
            document.exitFullscreen();
        }
    }, category: 'UI' },
    
    // P0 Features
    { id: 'p0-health', label: 'View System Health', icon: icons.activity, action: () => window.open('/api/p0/health', '_blank'), category: 'P0 Features' },
    { id: 'p0-metrics', label: 'View System Metrics', icon: icons.barChart, action: () => window.open('/api/p0/metrics', '_blank'), category: 'P0 Features' },
    { id: 'p0-backup', label: 'Create Backup', icon: icons.database, action: () => window.dispatchEvent(new CustomEvent('p0:backup')), category: 'P0 Features' },
    { id: 'p0-audit', label: 'View Audit Log', icon: icons.fileText, action: () => window.location.hash = '#/audit', category: 'P0 Features' },
    
    // Help & Documentation
    { id: 'help-shortcuts', label: 'Show Keyboard Shortcuts', icon: icons.keyboard, action: () => window.keyboardShortcuts?.showHelp(), category: 'Help' },
    { id: 'help-docs', label: 'Open Documentation', icon: icons.book, action: () => window.open('https://docs.clawmaster.org', '_blank'), category: 'Help' },
    { id: 'help-github', label: 'Open GitHub Repository', icon: icons.github, action: () => window.open('https://github.com/arksong/ClawMaster', '_blank'), category: 'Help' },
    { id: 'help-about', label: 'About ClawMaster', icon: icons.info, action: () => window.dispatchEvent(new CustomEvent('help:about')), category: 'Help' },
];

/**
 * Command Palette Component
 */
export function CommandPalette() {
    const [isOpen, setIsOpen] = useState(false);
    const [query, setQuery] = useState('');
    const [selectedIndex, setSelectedIndex] = useState(0);
    const inputRef = useRef(null);

    useEffect(() => {
        const handleOpen = () => {
            setIsOpen(true);
            setQuery('');
            setSelectedIndex(0);
        };

        const handleKeyDown = (e) => {
            // Ctrl+P or Cmd+P to open
            if ((e.ctrlKey || e.metaKey) && e.key === 'p') {
                e.preventDefault();
                handleOpen();
            }
        };

        window.addEventListener('commandPalette:open', handleOpen);
        document.addEventListener('keydown', handleKeyDown);

        return () => {
            window.removeEventListener('commandPalette:open', handleOpen);
            document.removeEventListener('keydown', handleKeyDown);
        };
    }, []);

    useEffect(() => {
        if (isOpen && inputRef.current) {
            inputRef.current.focus();
        }
    }, [isOpen]);

    const filteredCommands = query
        ? fuzzySearch(COMMANDS, query)
        : COMMANDS;

    const groupedCommands = groupByCategory(filteredCommands);

    const handleKeyDown = (e) => {
        if (e.key === 'Escape') {
            setIsOpen(false);
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            setSelectedIndex(prev => Math.min(prev + 1, filteredCommands.length - 1));
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            setSelectedIndex(prev => Math.max(prev - 1, 0));
        } else if (e.key === 'Enter') {
            e.preventDefault();
            if (filteredCommands[selectedIndex]) {
                executeCommand(filteredCommands[selectedIndex]);
            }
        }
    };

    const executeCommand = (command) => {
        command.action();
        setIsOpen(false);
        setQuery('');
        setSelectedIndex(0);
    };

    if (!isOpen) return null;

    return html`
        <div class="command-palette-overlay" onClick=${() => setIsOpen(false)}>
            <div class="command-palette" onClick=${(e) => e.stopPropagation()}>
                <div class="command-palette-header">
                    <input
                        ref=${inputRef}
                        type="text"
                        class="command-palette-input"
                        placeholder="Type a command or search..."
                        value=${query}
                        onInput=${(e) => {
                            setQuery(e.target.value);
                            setSelectedIndex(0);
                        }}
                        onKeyDown=${handleKeyDown}
                    />
                </div>

                <div class="command-palette-results">
                    ${Object.entries(groupedCommands).map(([category, commands]) => html`
                        <div key=${category} class="command-category">
                            <div class="category-label">${category}</div>
                            ${commands.map((command, index) => {
                                const globalIndex = filteredCommands.indexOf(command);
                                return html`
                                    <div
                                        key=${command.id}
                                        class="command-item ${globalIndex === selectedIndex ? 'selected' : ''}"
                                        onClick=${() => executeCommand(command)}
                                        onMouseEnter=${() => setSelectedIndex(globalIndex)}
                                    >
                                        <span class="command-icon">${command.icon}</span>
                                        <span class="command-label">${highlightMatch(command.label, query)}</span>
                                    </div>
                                `;
                            })}
                        </div>
                    `)}

                    ${filteredCommands.length === 0 && html`
                        <div class="command-empty">
                            <div class="empty-icon">${icons.search}</div>
                            <p>No commands found</p>
                            <p class="empty-hint">Try a different search term</p>
                        </div>
                    `}
                </div>

                <div class="command-palette-footer">
                    <div class="footer-hint">
                        <kbd>↑</kbd><kbd>↓</kbd> Navigate
                        <kbd>↵</kbd> Select
                        <kbd>Esc</kbd> Close
                    </div>
                </div>
            </div>
        </div>
    `;
}

/**
 * Fuzzy search implementation
 */
function fuzzySearch(commands, query) {
    const lowerQuery = query.toLowerCase();
    
    return commands
        .map(command => {
            const label = command.label.toLowerCase();
            let score = 0;
            let lastIndex = -1;

            for (const char of lowerQuery) {
                const index = label.indexOf(char, lastIndex + 1);
                if (index === -1) return null;
                
                // Bonus for consecutive matches
                if (index === lastIndex + 1) score += 2;
                else score += 1;
                
                // Bonus for word start matches
                if (index === 0 || label[index - 1] === ' ') score += 3;
                
                lastIndex = index;
            }

            return { command, score };
        })
        .filter(result => result !== null)
        .sort((a, b) => b.score - a.score)
        .map(result => result.command);
}

/**
 * Group commands by category
 */
function groupByCategory(commands) {
    const grouped = {};
    
    for (const command of commands) {
        if (!grouped[command.category]) {
            grouped[command.category] = [];
        }
        grouped[command.category].push(command);
    }
    
    return grouped;
}

/**
 * Highlight matching characters
 */
function highlightMatch(text, query) {
    if (!query) return text;
    
    const lowerText = text.toLowerCase();
    const lowerQuery = query.toLowerCase();
    const parts = [];
    let lastIndex = 0;
    
    for (const char of lowerQuery) {
        const index = lowerText.indexOf(char, lastIndex);
        if (index === -1) break;
        
        if (index > lastIndex) {
            parts.push(html`<span>${text.substring(lastIndex, index)}</span>`);
        }
        parts.push(html`<mark>${text[index]}</mark>`);
        lastIndex = index + 1;
    }
    
    if (lastIndex < text.length) {
        parts.push(html`<span>${text.substring(lastIndex)}</span>`);
    }
    
    return parts;
}

/**
 * Mount the command palette
 */
export function mountCommandPalette(container) {
    render(html`<${CommandPalette} />`, container);
}

// Auto-mount on page load
if (typeof document !== 'undefined') {
    document.addEventListener('DOMContentLoaded', () => {
        const container = document.createElement('div');
        container.id = 'command-palette-root';
        document.body.appendChild(container);
        mountCommandPalette(container);
    });
}
