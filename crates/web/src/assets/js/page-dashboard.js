// ── Dashboard Page ──────────────────────────────────────────
// System overview, active sessions, recent activity, and quick actions

import { html } from "htm/preact";
import { render } from "preact";
import { useEffect, useState } from "preact/hooks";
import { registerPage } from "./router.js";
import { routes } from "./routes.js";
import { sendRpc } from "./helpers.js";
import { t } from "./i18n.js";
import * as S from "./state.js";
import { sessionStore } from "./stores/session-store.js";
import { modelStore } from "./stores/model-store.js";

/**
 * Dashboard Page Component
 * 
 * Displays:
 * - System status and health
 * - Active sessions
 * - Recent activity
 * - Quick actions
 * - Emergency controls
 */
function DashboardPage() {
	const [systemStatus, setSystemStatus] = useState(null);
	const [activeCommands, setActiveCommands] = useState(0);
	const [pendingApprovals, setPendingApprovals] = useState(0);
	const [recentSessions, setRecentSessions] = useState([]);
	const [memoryUsage, setMemoryUsage] = useState(null);
	const [uptime, setUptime] = useState(null);
	const [llmUsage, setLlmUsage] = useState(null);
	const [loading, setLoading] = useState(true);

	useEffect(() => {
		loadDashboardData();
		const interval = setInterval(loadDashboardData, 5000); // Refresh every 5s
		return () => clearInterval(interval);
	}, []);

	async function loadDashboardData() {
		try {
			// Get sessions from store
			const sessions = sessionStore.sessions?.value || [];
			setRecentSessions(sessions.slice(0, 5)); // Top 5 recent sessions

			// Get system status from gon data
			const gonData = window.__MOLTIS__ || {};
			const status = {
				connected: S.wsConnected?.value || false,
				models: modelStore.models?.value?.length || 0,
				sessions: sessions.length,
				providers: gonData.counts?.providers || 0,
				channels: gonData.counts?.channels || 0,
				skills: gonData.counts?.skills || 0,
				connectedClients: 1, // WebSocket connection
				httpRequests: 0,
			};
			setSystemStatus(status);
			
			// Get memory usage
			if (gonData.mem) {
				const memPercent = ((gonData.mem.process / gonData.mem.total) * 100).toFixed(1);
				const memMB = Math.round(gonData.mem.process / (1024 * 1024));
				setMemoryUsage({
					process: gonData.mem.process,
					total: gonData.mem.total,
					percent: memPercent,
					mb: memMB
				});
			}
			
			// Calculate uptime
			if (gonData.started_at) {
				const uptimeMs = Date.now() - gonData.started_at;
				const uptimeMinutes = Math.floor(uptimeMs / 60000);
				const uptimeHours = Math.floor(uptimeMinutes / 60);
				const uptimeDays = Math.floor(uptimeHours / 24);
				
				let uptimeStr;
				if (uptimeDays > 0) {
					uptimeStr = `${uptimeDays}d`;
				} else if (uptimeHours > 0) {
					uptimeStr = `${uptimeHours}h`;
				} else {
					uptimeStr = `${uptimeMinutes}m`;
				}
				
				setUptime(uptimeStr);
			}
			
			// LLM Usage statistics (placeholder - would come from metrics API)
			setLlmUsage({
				completions: 0,
				inputTokens: 0,
				outputTokens: 0,
				cacheTokens: 0
			});

			setLoading(false);
		} catch (error) {
			console.error("Failed to load dashboard data:", error);
			setLoading(false);
		}
	}

	async function handleEmergencyStop() {
		if (!confirm(t("security:emergencyStop.confirm", "Stop all running commands and cancel all pending operations?"))) {
			return;
		}

		try {
			const sessionKey = sessionStore.activeSessionKey?.value;
			if (sessionKey) {
				await sendRpc("chat.abort", { sessionKey });
				await sendRpc("chat.cancel_queued", { sessionKey });
			}
			alert(t("security:emergencyStop.success", "All operations stopped successfully."));
		} catch (error) {
			console.error("Emergency stop failed:", error);
			alert(t("security:emergencyStop.error", "Failed to stop operations: ") + error.message);
		}
	}

	if (loading) {
		return html`<div class="flex-1 flex items-center justify-center">
			<div class="text-sm text-[var(--muted)]">${t("common:status.loading", "Loading...")}</div>
		</div>`;
	}

	return html`<div class="flex-1 flex flex-col min-w-0 overflow-y-auto">
		<div class="dashboard-container">
			<!-- Header -->
			<div class="dashboard-header">
				<div class="flex items-center gap-3">
					<h1 class="dashboard-title">
						${t("dashboard:title", "Monitoring")}
					</h1>
					<div class="flex items-center gap-2 px-3 py-1 rounded-full" style="background: rgba(34, 197, 94, 0.1); color: #22c55e;">
						<div style="width: 8px; height: 8px; border-radius: 50%; background: #22c55e;"></div>
						<span class="text-sm font-medium">Live</span>
					</div>
				</div>
				<div class="flex gap-2">
					<button class="px-4 py-2 rounded-lg border border-[var(--border)] bg-[var(--surface)] text-[var(--text)] hover:bg-[var(--surface2)] transition-colors">
						Overview
					</button>
					<button class="px-4 py-2 rounded-lg border border-[var(--border)] bg-transparent text-[var(--muted)] hover:bg-[var(--surface)] transition-colors">
						Charts
					</button>
				</div>
			</div>

			<!-- SYSTEM Section -->
			<div class="mb-6">
				<h2 class="text-sm font-semibold text-[var(--muted)] uppercase tracking-wider mb-4">SYSTEM</h2>
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
					<!-- Uptime -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">UPTIME</div>
						<div class="monitoring-card-value">${uptime || '0m'}</div>
					</div>
					
					<!-- Connected Clients -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">CONNECTED CLIENTS</div>
						<div class="monitoring-card-value">${systemStatus?.connectedClients || 0}</div>
					</div>
					
					<!-- Active Sessions -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">ACTIVE SESSIONS</div>
						<div class="monitoring-card-value">${systemStatus?.sessions || 0}</div>
					</div>
					
					<!-- HTTP Requests -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">HTTP REQUESTS</div>
						<div class="monitoring-card-value">${systemStatus?.httpRequests || 0}</div>
					</div>
					
					<!-- Process Memory -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">PROCESS MEMORY</div>
						<div class="monitoring-card-value">${memoryUsage ? memoryUsage.mb + 'MB' : 'N/A'}</div>
					</div>
				</div>
			</div>
			
			<!-- LLM USAGE Section -->
			<div class="mb-6">
				<h2 class="text-sm font-semibold text-[var(--muted)] uppercase tracking-wider mb-4">LLM USAGE</h2>
				<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
					<!-- Completions -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">COMPLETIONS</div>
						<div class="monitoring-card-value">${llmUsage?.completions || 0}</div>
					</div>
					
					<!-- Input Tokens -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">INPUT TOKENS</div>
						<div class="monitoring-card-value">${llmUsage?.inputTokens || 0}</div>
					</div>
					
					<!-- Output Tokens -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">OUTPUT TOKENS</div>
						<div class="monitoring-card-value">${llmUsage?.outputTokens || 0}</div>
					</div>
					
					<!-- Cache Tokens -->
					<div class="monitoring-card">
						<div class="monitoring-card-label">CACHE TOKENS</div>
						<div class="monitoring-card-value">${llmUsage?.cacheTokens || 0}</div>
					</div>
				</div>
			</div>

			<!-- Emergency Controls -->
			<div class="dashboard-card dashboard-emergency-card">
				<div class="dashboard-section-header">
					<h2 class="dashboard-section-title">
						${t("dashboard:emergency.title", "Emergency Controls")}
					</h2>
				</div>
				<button 
					class="dashboard-emergency-button"
					onClick=${handleEmergencyStop}>
					🛑 ${t("security:emergencyStop.button", "EMERGENCY STOP")}
				</button>
				<div class="dashboard-emergency-description">
					${t("dashboard:emergency.description", "Stop all running commands and cancel pending operations")}
				</div>
			</div>

			<!-- Recent Sessions -->
			<div class="dashboard-card">
				<div class="dashboard-section-header">
					<h2 class="dashboard-section-title">
						${t("dashboard:recentSessions.title", "Recent Sessions")}
					</h2>
					<a href="${routes.chats}" class="dashboard-section-link">
						${t("dashboard:recentSessions.viewAll", "View all")} →
					</a>
				</div>
				${recentSessions.length > 0
					? html`<div class="dashboard-session-list">
						${recentSessions.map(session => html`
							<a 
								href="/chats/${session.key.replace(/:/g, '/')}" 
								class="dashboard-session-item"
								key=${session.key}>
								<div class="dashboard-session-icon">
									💬
								</div>
								<div class="dashboard-session-info">
									<div class="dashboard-session-title">
										${session.title || session.key}
									</div>
									<div class="dashboard-session-time">
										${session.updated_at ? new Date(session.updated_at).toLocaleString() : ''}
									</div>
								</div>
							</a>
						`)}
					</div>`
					: html`<div class="dashboard-empty-state">
						${t("dashboard:recentSessions.empty", "No recent sessions")}
					</div>`
				}
			</div>

			<!-- Quick Actions -->
			<div class="dashboard-card">
				<div class="dashboard-section-header">
					<h2 class="dashboard-section-title">
						${t("dashboard:quickActions.title", "Quick Actions")}
					</h2>
				</div>
				<div class="dashboard-quick-actions">
					<a href="${routes.chats}" class="dashboard-quick-action">
						<div class="dashboard-quick-action-icon">💬</div>
						<div class="dashboard-quick-action-label">${t("dashboard:quickActions.newChat", "New Chat")}</div>
					</a>
					<a href="${routes.settings}" class="dashboard-quick-action">
						<div class="dashboard-quick-action-icon">⚙️</div>
						<div class="dashboard-quick-action-label">${t("dashboard:quickActions.settings", "Settings")}</div>
					</a>
					<a href="${routes.security}" class="dashboard-quick-action">
						<div class="dashboard-quick-action-icon">🛡️</div>
						<div class="dashboard-quick-action-label">${t("dashboard:quickActions.security", "Security")}</div>
					</a>
					<a href="${routes.logs}" class="dashboard-quick-action">
						<div class="dashboard-quick-action-icon">📋</div>
						<div class="dashboard-quick-action-label">${t("dashboard:quickActions.logs", "Logs")}</div>
					</a>
				</div>
			</div>
		</div>
	</div>`;
}

/**
 * Initialize dashboard page
 */
function initDashboard(container) {
	render(html`<${DashboardPage} />`, container);
}

/**
 * Teardown dashboard page
 */
function teardownDashboard() {
	// Cleanup if needed
}

// Register dashboard route
registerPage("/dashboard", initDashboard, teardownDashboard);
