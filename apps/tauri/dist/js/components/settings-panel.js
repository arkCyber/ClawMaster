/**
 * Enhanced Settings Panel Component
 * Provides comprehensive configuration UI with multiple options
 */

import { html } from '../vendor/preact-htm.js';
import { useState, useEffect } from '../vendor/preact-hooks.js';
import * as icons from '../icons.js';

/**
 * Settings Panel with tabbed interface
 */
export function SettingsPanel() {
    const [activeTab, setActiveTab] = useState('providers');
    const [settings, setSettings] = useState({
        providers: {},
        channels: {},
        appearance: {},
        advanced: {}
    });
    const [saving, setSaving] = useState(false);
    const [saved, setSaved] = useState(false);

    useEffect(() => {
        loadSettings();
    }, []);

    const loadSettings = async () => {
        try {
            const response = await fetch('/api/settings');
            if (response.ok) {
                const data = await response.json();
                setSettings(data);
            }
        } catch (error) {
            console.error('Failed to load settings:', error);
        }
    };

    const saveSettings = async () => {
        setSaving(true);
        setSaved(false);
        try {
            const response = await fetch('/api/settings', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(settings)
            });
            if (response.ok) {
                setSaved(true);
                setTimeout(() => setSaved(false), 3000);
            }
        } catch (error) {
            console.error('Failed to save settings:', error);
        } finally {
            setSaving(false);
        }
    };

    const updateSetting = (category, key, value) => {
        setSettings(prev => ({
            ...prev,
            [category]: {
                ...prev[category],
                [key]: value
            }
        }));
    };

    const tabs = [
        { id: 'providers', label: 'LLM Providers', icon: icons.brain },
        { id: 'channels', label: 'Channels', icon: icons.messageSquare },
        { id: 'appearance', label: 'Appearance', icon: icons.palette },
        { id: 'p0features', label: 'P0 Features', icon: icons.shield },
        { id: 'advanced', label: 'Advanced', icon: icons.settings }
    ];

    return html`
        <div class="settings-panel">
            <div class="settings-header">
                <h2>${icons.settings} Settings</h2>
                <div class="settings-actions">
                    ${saved && html`
                        <span class="save-indicator success">
                            ${icons.check} Saved
                        </span>
                    `}
                    <button
                        class="btn-primary"
                        onClick=${saveSettings}
                        disabled=${saving}
                    >
                        ${saving ? 'Saving...' : 'Save Changes'}
                    </button>
                </div>
            </div>

            <div class="settings-tabs">
                ${tabs.map(tab => html`
                    <button
                        key=${tab.id}
                        class="tab-button ${activeTab === tab.id ? 'active' : ''}"
                        onClick=${() => setActiveTab(tab.id)}
                    >
                        ${tab.icon}
                        <span>${tab.label}</span>
                    </button>
                `)}
            </div>

            <div class="settings-content">
                ${activeTab === 'providers' && html`
                    <${ProvidersSettings}
                        settings=${settings.providers}
                        onChange=${(key, value) => updateSetting('providers', key, value)}
                    />
                `}
                ${activeTab === 'channels' && html`
                    <${ChannelsSettings}
                        settings=${settings.channels}
                        onChange=${(key, value) => updateSetting('channels', key, value)}
                    />
                `}
                ${activeTab === 'appearance' && html`
                    <${AppearanceSettings}
                        settings=${settings.appearance}
                        onChange=${(key, value) => updateSetting('appearance', key, value)}
                    />
                `}
                ${activeTab === 'p0features' && html`
                    <${P0FeaturesSettings}
                        settings=${settings.p0features || {}}
                        onChange=${(key, value) => updateSetting('p0features', key, value)}
                    />
                `}
                ${activeTab === 'advanced' && html`
                    <${AdvancedSettings}
                        settings=${settings.advanced}
                        onChange=${(key, value) => updateSetting('advanced', key, value)}
                    />
                `}
            </div>
        </div>
    `;
}

/**
 * Providers Settings Tab
 */
function ProvidersSettings({ settings, onChange }) {
    const providers = [
        { id: 'openai', name: 'OpenAI', description: 'GPT-4, GPT-3.5-turbo' },
        { id: 'anthropic', name: 'Anthropic', description: 'Claude 3 Opus, Sonnet, Haiku' },
        { id: 'openrouter', name: 'OpenRouter', description: 'Access to multiple models' },
        { id: 'ollama', name: 'Ollama', description: 'Run models locally' },
        { id: 'github_copilot', name: 'GitHub Copilot', description: 'GitHub\'s AI assistant' }
    ];

    return html`
        <div class="settings-section">
            <h3>LLM Provider Configuration</h3>
            <p class="section-description">
                Enable and configure your preferred LLM providers. You can use multiple providers simultaneously.
            </p>

            <div class="provider-cards">
                ${providers.map(provider => html`
                    <div key=${provider.id} class="provider-card">
                        <div class="card-header">
                            <label class="provider-toggle">
                                <input
                                    type="checkbox"
                                    checked=${settings[`${provider.id}_enabled`] || false}
                                    onChange=${(e) => onChange(`${provider.id}_enabled`, e.target.checked)}
                                />
                                <span class="provider-name">${provider.name}</span>
                            </label>
                        </div>
                        <p class="provider-description">${provider.description}</p>
                        
                        ${settings[`${provider.id}_enabled`] && provider.id !== 'ollama' && html`
                            <div class="provider-config">
                                <label>
                                    API Key
                                    <input
                                        type="password"
                                        placeholder="Enter your API key"
                                        value=${settings[`${provider.id}_key`] || ''}
                                        onChange=${(e) => onChange(`${provider.id}_key`, e.target.value)}
                                    />
                                </label>
                            </div>
                        `}
                        
                        ${settings[`${provider.id}_enabled`] && provider.id === 'ollama' && html`
                            <div class="provider-config">
                                <label>
                                    Ollama URL
                                    <input
                                        type="url"
                                        placeholder="http://localhost:11434"
                                        value=${settings.ollama_url || 'http://localhost:11434'}
                                        onChange=${(e) => onChange('ollama_url', e.target.value)}
                                    />
                                </label>
                            </div>
                        `}
                    </div>
                `)}
            </div>
        </div>
    `;
}

/**
 * Channels Settings Tab
 */
function ChannelsSettings({ settings, onChange }) {
    const channels = [
        { id: 'web', name: 'Web UI', description: 'Browser-based interface (always enabled)', alwaysOn: true },
        { id: 'telegram', name: 'Telegram', description: 'Telegram bot integration' },
        { id: 'discord', name: 'Discord', description: 'Discord bot integration' },
        { id: 'slack', name: 'Slack', description: 'Slack bot integration' }
    ];

    return html`
        <div class="settings-section">
            <h3>Communication Channels</h3>
            <p class="section-description">
                Configure how users can interact with ClawMaster.
            </p>

            <div class="channel-cards">
                ${channels.map(channel => html`
                    <div key=${channel.id} class="channel-card">
                        <div class="card-header">
                            <label class="channel-toggle">
                                <input
                                    type="checkbox"
                                    checked=${channel.alwaysOn || settings[`${channel.id}_enabled`] || false}
                                    disabled=${channel.alwaysOn}
                                    onChange=${(e) => onChange(`${channel.id}_enabled`, e.target.checked)}
                                />
                                <span class="channel-name">${channel.name}</span>
                            </label>
                        </div>
                        <p class="channel-description">${channel.description}</p>
                        
                        ${!channel.alwaysOn && settings[`${channel.id}_enabled`] && html`
                            <div class="channel-config">
                                <label>
                                    Bot Token
                                    <input
                                        type="password"
                                        placeholder="Enter your bot token"
                                        value=${settings[`${channel.id}_token`] || ''}
                                        onChange=${(e) => onChange(`${channel.id}_token`, e.target.value)}
                                    />
                                </label>
                            </div>
                        `}
                    </div>
                `)}
            </div>
        </div>
    `;
}

/**
 * Appearance Settings Tab
 */
function AppearanceSettings({ settings, onChange }) {
    const themes = [
        { id: 'light', name: 'Light', description: 'Clean and bright' },
        { id: 'dark', name: 'Dark', description: 'Easy on the eyes' },
        { id: 'auto', name: 'Auto', description: 'Follow system preference' },
        { id: 'high-contrast', name: 'High Contrast', description: 'Maximum readability' }
    ];

    const fontSizes = [
        { id: 'small', name: 'Small', value: '14px' },
        { id: 'medium', name: 'Medium', value: '16px' },
        { id: 'large', name: 'Large', value: '18px' },
        { id: 'xlarge', name: 'Extra Large', value: '20px' }
    ];

    return html`
        <div class="settings-section">
            <h3>Appearance & Theme</h3>
            
            <div class="setting-group">
                <label class="setting-label">Theme</label>
                <div class="theme-selector">
                    ${themes.map(theme => html`
                        <button
                            key=${theme.id}
                            class="theme-option ${(settings.theme || 'auto') === theme.id ? 'selected' : ''}"
                            onClick=${() => onChange('theme', theme.id)}
                        >
                            <div class="theme-preview theme-preview-${theme.id}"></div>
                            <span class="theme-name">${theme.name}</span>
                            <span class="theme-description">${theme.description}</span>
                        </button>
                    `)}
                </div>
            </div>

            <div class="setting-group">
                <label class="setting-label">Font Size</label>
                <select
                    value=${settings.fontSize || 'medium'}
                    onChange=${(e) => onChange('fontSize', e.target.value)}
                >
                    ${fontSizes.map(size => html`
                        <option key=${size.id} value=${size.id}>
                            ${size.name} (${size.value})
                        </option>
                    `)}
                </select>
            </div>

            <div class="setting-group">
                <label class="checkbox-label">
                    <input
                        type="checkbox"
                        checked=${settings.compactMode || false}
                        onChange=${(e) => onChange('compactMode', e.target.checked)}
                    />
                    Compact Mode
                    <span class="label-hint">Reduce spacing for more content</span>
                </label>
            </div>

            <div class="setting-group">
                <label class="checkbox-label">
                    <input
                        type="checkbox"
                        checked=${settings.showTimestamps !== false}
                        onChange=${(e) => onChange('showTimestamps', e.target.checked)}
                    />
                    Show Timestamps
                    <span class="label-hint">Display message timestamps</span>
                </label>
            </div>

            <div class="setting-group">
                <label class="checkbox-label">
                    <input
                        type="checkbox"
                        checked=${settings.enableAnimations !== false}
                        onChange=${(e) => onChange('enableAnimations', e.target.checked)}
                    />
                    Enable Animations
                    <span class="label-hint">Smooth transitions and effects</span>
                </label>
            </div>
        </div>
    `;
}

/**
 * P0 Features Settings Tab
 */
function P0FeaturesSettings({ settings, onChange }) {
    return html`
        <div class="settings-section">
            <h3>P0 Enterprise Features</h3>
            <p class="section-description">
                Configure DO-178C Level A compliant enterprise features.
            </p>

            <div class="feature-cards">
                <div class="feature-card">
                    <h4>${icons.activity} Health Monitoring</h4>
                    <p>Real-time system health checks and monitoring</p>
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked=${settings.healthCheck !== false}
                            onChange=${(e) => onChange('healthCheck', e.target.checked)}
                        />
                        Enable health monitoring
                    </label>
                    ${settings.healthCheck !== false && html`
                        <label>
                            Check Interval (seconds)
                            <input
                                type="number"
                                min="10"
                                max="300"
                                value=${settings.healthCheckInterval || 30}
                                onChange=${(e) => onChange('healthCheckInterval', parseInt(e.target.value))}
                            />
                        </label>
                    `}
                </div>

                <div class="feature-card">
                    <h4>${icons.shield} Resource Quotas</h4>
                    <p>Rate limiting and resource management</p>
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked=${settings.rateLimiting !== false}
                            onChange=${(e) => onChange('rateLimiting', e.target.checked)}
                        />
                        Enable rate limiting
                    </label>
                    ${settings.rateLimiting !== false && html`
                        <label>
                            Requests per minute
                            <input
                                type="number"
                                min="1"
                                max="1000"
                                value=${settings.rateLimit || 60}
                                onChange=${(e) => onChange('rateLimit', parseInt(e.target.value))}
                            />
                        </label>
                    `}
                </div>

                <div class="feature-card">
                    <h4>${icons.database} Auto Backup</h4>
                    <p>Automatic data backup and recovery</p>
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked=${settings.autoBackup || false}
                            onChange=${(e) => onChange('autoBackup', e.target.checked)}
                        />
                        Enable automatic backups
                    </label>
                    ${settings.autoBackup && html`
                        <label>
                            Backup Interval (hours)
                            <input
                                type="number"
                                min="1"
                                max="168"
                                value=${settings.backupInterval || 24}
                                onChange=${(e) => onChange('backupInterval', parseInt(e.target.value))}
                            />
                        </label>
                    `}
                </div>

                <div class="feature-card">
                    <h4>${icons.fileText} Audit Logging</h4>
                    <p>Comprehensive audit trail with HMAC signatures</p>
                    <label class="checkbox-label">
                        <input
                            type="checkbox"
                            checked=${settings.auditLog !== false}
                            onChange=${(e) => onChange('auditLog', e.target.checked)}
                        />
                        Enable audit logging
                    </label>
                </div>
            </div>
        </div>
    `;
}

/**
 * Advanced Settings Tab
 */
function AdvancedSettings({ settings, onChange }) {
    return html`
        <div class="settings-section">
            <h3>Advanced Configuration</h3>
            
            <div class="setting-group">
                <label class="setting-label">Max Context Length</label>
                <input
                    type="number"
                    min="1000"
                    max="128000"
                    value=${settings.maxContextLength || 8000}
                    onChange=${(e) => onChange('maxContextLength', parseInt(e.target.value))}
                />
                <span class="setting-hint">Maximum tokens in conversation context</span>
            </div>

            <div class="setting-group">
                <label class="setting-label">Tool Execution Timeout (seconds)</label>
                <input
                    type="number"
                    min="5"
                    max="300"
                    value=${settings.toolTimeout || 30}
                    onChange=${(e) => onChange('toolTimeout', parseInt(e.target.value))}
                />
            </div>

            <div class="setting-group">
                <label class="checkbox-label">
                    <input
                        type="checkbox"
                        checked=${settings.enableDebugMode || false}
                        onChange=${(e) => onChange('enableDebugMode', e.target.checked)}
                    />
                    Debug Mode
                    <span class="label-hint">Show detailed logs and diagnostics</span>
                </label>
            </div>

            <div class="setting-group">
                <label class="checkbox-label">
                    <input
                        type="checkbox"
                        checked=${settings.enableTelemetry !== false}
                        onChange=${(e) => onChange('enableTelemetry', e.target.checked)}
                    />
                    Anonymous Telemetry
                    <span class="label-hint">Help improve ClawMaster (no personal data)</span>
                </label>
            </div>

            <div class="setting-group danger-zone">
                <h4>Danger Zone</h4>
                <button class="btn-danger" onClick=${() => {
                    if (confirm('Are you sure you want to reset all settings to defaults?')) {
                        onChange('reset', true);
                    }
                }}>
                    Reset All Settings
                </button>
            </div>
        </div>
    `;
}
