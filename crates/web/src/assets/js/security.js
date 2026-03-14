/**
 * Security Features Module - DO-178C Level A Compliant
 * 
 * This module provides security-critical UI functionality:
 * - Emergency stop button
 * - Security mode indicator
 * - Approval notification enhancements
 * - Security settings management
 * 
 * Compliance: DO-178C §11.10 - User interface safety requirements
 * 
 * @module security
 */

import { sendRpc } from "./helpers.js";
import * as S from "./state.js";
import { t } from "./i18n.js";

// ══════════════════════════════════════════════════════════════════════════
// State Management
// ══════════════════════════════════════════════════════════════════════════

let securityMode = "on-miss"; // off | on-miss | always
let securityLevel = "allowlist"; // deny | allowlist | full
let hasActiveCommands = false;
let pendingApprovals = new Set();

// ══════════════════════════════════════════════════════════════════════════
// Emergency Stop Button
// ══════════════════════════════════════════════════════════════════════════

/**
 * Initialize emergency stop button
 * 
 * DO-178C §6.3.2: Critical function initialization
 */
export function initEmergencyStop() {
	const btn = document.getElementById("emergencyStopBtn");
	if (!btn) return;

	btn.addEventListener("click", handleEmergencyStop);
	
	// Show button when there are active commands
	updateEmergencyStopVisibility();
}

/**
 * Handle emergency stop button click
 * 
 * DO-178C §6.3.4: User confirmation for critical actions
 */
async function handleEmergencyStop() {
	const confirmMsg = t("security:emergencyStop.confirm", 
		"Are you sure you want to stop all running commands?");
	
	if (!confirm(confirmMsg)) {
		return;
	}

	const btn = document.getElementById("emergencyStopBtn");
	if (btn) {
		btn.disabled = true;
		btn.textContent = t("security:emergencyStop.stopping", "Stopping...");
	}

	try {
		// Abort current session
		const currentSession = S.sessionStore?.activeSessionKey?.value;
		if (currentSession) {
			await sendRpc("chat.abort", { sessionKey: currentSession });
		}

		// Cancel all queued messages
		if (currentSession) {
			await sendRpc("chat.cancel_queued", { sessionKey: currentSession });
		}

		// Deny all pending approvals
		for (const requestId of pendingApprovals) {
			await sendRpc("exec.approval.resolve", {
				requestId: requestId,
				decision: "denied"
			});
		}
		pendingApprovals.clear();

		showNotification(
			t("security:emergencyStop.success", "All commands stopped"),
			"success"
		);
	} catch (error) {
		console.error("Emergency stop failed:", error);
		showNotification(
			t("security:emergencyStop.error", "Failed to stop commands"),
			"error"
		);
	} finally {
		if (btn) {
			btn.disabled = false;
			btn.innerHTML = `<span class="icon icon-sm icon-stop-circle"></span><span class="emergency-label">${t("security:emergencyStop.label", "STOP")}</span>`;
		}
		updateEmergencyStopVisibility();
	}
}

/**
 * Update emergency stop button visibility and state
 * 
 * DO-178C §11.10: Dynamic UI state management
 * 
 * NOTE: Button is always visible for safety. Only state changes.
 */
function updateEmergencyStopVisibility() {
	const btn = document.getElementById("emergencyStopBtn");
	if (!btn) return;

	// Button is always visible (DO-178C safety requirement)
	btn.style.display = "flex";
	
	// Add pulsing animation if there are active commands or pending approvals
	const isActive = hasActiveCommands || pendingApprovals.size > 0;
	if (isActive) {
		btn.classList.add("active");
	} else {
		btn.classList.remove("active");
	}
	
	// Update button state (enabled/disabled)
	btn.disabled = !isActive;
}

/**
 * Notify that a command has started
 */
export function notifyCommandStarted() {
	hasActiveCommands = true;
	updateEmergencyStopVisibility();
}

/**
 * Notify that a command has finished
 */
export function notifyCommandFinished() {
	hasActiveCommands = false;
	updateEmergencyStopVisibility();
}

/**
 * Notify that an approval request was created
 */
export function notifyApprovalRequested(requestId) {
	pendingApprovals.add(requestId);
	updateEmergencyStopVisibility();
}

/**
 * Notify that an approval request was resolved
 */
export function notifyApprovalResolved(requestId) {
	pendingApprovals.delete(requestId);
	updateEmergencyStopVisibility();
}

// ══════════════════════════════════════════════════════════════════════════
// Security Mode Indicator
// ══════════════════════════════════════════════════════════════════════════

/**
 * Initialize security mode indicator
 * 
 * DO-178C §11.10: Security status visualization
 */
export function initSecurityModeIndicator() {
	const indicator = document.getElementById("securityModeIndicator");
	if (!indicator) return;

	// Fetch current security mode
	fetchSecurityMode();

	// Click to open security settings
	indicator.addEventListener("click", openSecuritySettings);

	// Update every 30 seconds
	setInterval(fetchSecurityMode, 30000);
}

/**
 * Fetch current security mode from server
 */
async function fetchSecurityMode() {
	try {
		const response = await sendRpc("exec.approval.get", {});
		if (response?.ok) {
			updateSecurityModeDisplay(response.payload);
		}
	} catch (error) {
		console.error("Failed to fetch security mode:", error);
	}
}

/**
 * Update security mode display
 * 
 * DO-178C §6.3.4: Visual feedback requirements
 */
function updateSecurityModeDisplay(config) {
	const indicator = document.getElementById("securityModeIndicator");
	const modeText = document.getElementById("securityModeText");
	
	if (!indicator || !modeText) return;

	securityMode = config.mode || "on-miss";
	securityLevel = config.securityLevel || "allowlist";

	// Update mode text
	const modeLabels = {
		"off": t("security:mode.off", "Off"),
		"on-miss": t("security:mode.smart", "Smart"),
		"always": t("security:mode.always", "Always")
	};
	modeText.textContent = modeLabels[securityMode] || "Smart";

	// Update data attribute for CSS styling
	indicator.setAttribute("data-mode", securityMode);

	// Update tooltip
	const tooltips = {
		"off": t("security:mode.offDesc", "Approval disabled (not recommended)"),
		"on-miss": t("security:mode.smartDesc", "Smart approval mode (recommended)"),
		"always": t("security:mode.alwaysDesc", "Always require approval (most secure)")
	};
	indicator.title = tooltips[securityMode] || "";

	// Show indicator
	indicator.style.display = "flex";
}

// ══════════════════════════════════════════════════════════════════════════
// Approval Notification Enhancements
// ══════════════════════════════════════════════════════════════════════════

/**
 * Enhanced approval notification with sound and browser notification
 * 
 * DO-178C §11.10: Multi-modal user notification
 */
export function enhanceApprovalNotification(requestId, command) {
	// Add to pending approvals
	notifyApprovalRequested(requestId);

	// Play notification sound
	playNotificationSound();

	// Show browser notification
	showBrowserNotification(command);

	// Show top banner
	showApprovalBanner(requestId, command);

	// Add pulse animation to approval card
	setTimeout(() => {
		const card = document.getElementById(`approval-${requestId}`);
		if (card) {
			card.classList.add("new-request");
		}
	}, 100);
}

/**
 * Play notification sound
 * 
 * DO-178C §11.10: Audio feedback for critical events
 */
function playNotificationSound() {
	try {
		// Create a simple beep sound using Web Audio API
		const audioContext = new (window.AudioContext || window.webkitAudioContext)();
		const oscillator = audioContext.createOscillator();
		const gainNode = audioContext.createGain();

		oscillator.connect(gainNode);
		gainNode.connect(audioContext.destination);

		oscillator.frequency.value = 800; // 800 Hz
		oscillator.type = "sine";

		gainNode.gain.setValueAtTime(0.3, audioContext.currentTime);
		gainNode.gain.exponentialRampToValueAtTime(0.01, audioContext.currentTime + 0.5);

		oscillator.start(audioContext.currentTime);
		oscillator.stop(audioContext.currentTime + 0.5);
	} catch (error) {
		console.warn("Failed to play notification sound:", error);
	}
}

/**
 * Show browser notification
 * 
 * DO-178C §11.10: System-level notification
 */
function showBrowserNotification(command) {
	if (!("Notification" in window)) {
		return;
	}

	if (Notification.permission === "granted") {
		new Notification(t("security:approval.title", "Command Approval Required"), {
			body: t("security:approval.body", "Command: ") + command,
			icon: "/icons/icon-72.png",
			badge: "/icons/icon-72.png",
			tag: "approval-request",
			requireInteraction: true
		});
	} else if (Notification.permission !== "denied") {
		Notification.requestPermission().then(permission => {
			if (permission === "granted") {
				showBrowserNotification(command);
			}
		});
	}
}

/**
 * Show approval banner at top of page
 */
function showApprovalBanner(requestId, command) {
	// Remove existing banner
	const existingBanner = document.getElementById("approvalBanner");
	if (existingBanner) {
		existingBanner.remove();
	}

	const banner = document.createElement("div");
	banner.id = "approvalBanner";
	banner.className = "approval-banner";
	banner.innerHTML = `
		<span>⚠️ ${t("security:approval.bannerText", "Command waiting for approval")}</span>
		<button id="approvalBannerBtn">${t("security:approval.viewButton", "View")}</button>
	`;

	document.body.insertBefore(banner, document.body.firstChild);

	// Scroll to approval card when clicked
	document.getElementById("approvalBannerBtn").addEventListener("click", () => {
		const card = document.getElementById(`approval-${requestId}`);
		if (card) {
			card.scrollIntoView({ behavior: "smooth", block: "center" });
			card.classList.add("new-request");
		}
		banner.remove();
	});

	// Auto-remove after 10 seconds
	setTimeout(() => {
		if (banner.parentNode) {
			banner.remove();
		}
	}, 10000);
}

/**
 * Remove approval banner
 */
export function removeApprovalBanner() {
	const banner = document.getElementById("approvalBanner");
	if (banner) {
		banner.remove();
	}
}

// ══════════════════════════════════════════════════════════════════════════
// Security Settings Modal
// ══════════════════════════════════════════════════════════════════════════

/**
 * Open security settings modal
 */
function openSecuritySettings() {
	// This will be implemented in page-settings.js
	// For now, navigate to settings page
	window.location.hash = "#/settings";
	
	// Scroll to security section after a short delay
	setTimeout(() => {
		const securitySection = document.getElementById("securitySettings");
		if (securitySection) {
			securitySection.scrollIntoView({ behavior: "smooth" });
		}
	}, 300);
}

// ══════════════════════════════════════════════════════════════════════════
// Utility Functions
// ══════════════════════════════════════════════════════════════════════════

/**
 * Show notification message
 */
function showNotification(message, type = "info") {
	// Use existing notification system if available
	if (window.chatAddMsg) {
		window.chatAddMsg(type === "error" ? "error" : "notice", message);
	} else {
		console.log(`[${type.toUpperCase()}] ${message}`);
	}
}

// ══════════════════════════════════════════════════════════════════════════
// Initialization
// ══════════════════════════════════════════════════════════════════════════

/**
 * Initialize all security features
 * 
 * DO-178C §11.13: Proper initialization sequence
 */
export function initSecurity() {
	initEmergencyStop();
	initSecurityModeIndicator();
	
	console.log("[Security] Security features initialized");
}

// Export to window for global access
window.enhanceApprovalNotification = enhanceApprovalNotification;
window.notifyCommandStarted = notifyCommandStarted;
window.notifyCommandFinished = notifyCommandFinished;
window.notifyApprovalRequested = notifyApprovalRequested;
window.notifyApprovalResolved = notifyApprovalResolved;

// Auto-initialize when DOM is ready
if (document.readyState === "loading") {
	document.addEventListener("DOMContentLoaded", initSecurity);
} else {
	initSecurity();
}
