// ── Tooltip i18n Integration ──────────────────────────────────────
// Dynamically updates tooltip text when language changes

import { locale } from "./i18n.js";
import { t } from "./i18n.js";

// Tooltip mapping: element ID/selector -> translation key
const TOOLTIP_MAP = {
	// Top navigation
	titleLink: "common:tooltips.home",
	metricsBtn: "common:tooltips.monitor",
	settingsBtn: "common:tooltips.settings",
	emergencyStopBtn: "common:tooltips.emergencyStop",
	logoutBtn: "common:tooltips.signOut",
	
	// Theme buttons
	'[data-theme-val="light"]': "common:tooltips.themeLight",
	'[data-theme-val="system"]': "common:tooltips.themeSystem",
	'[data-theme-val="dark"]': "common:tooltips.themeDark",
	
	// Sidebar
	newSessionBtn: "common:tooltips.newSession",
	'[data-tab="sessions"]': "common:tooltips.sessionsTab",
	'[data-tab="cron"]': "common:tooltips.cronTab",
	sessionsToggle: "common:tooltips.toggleSessions",
	projectFilterBtn: "common:tooltips.projectFilter",
	
	// Mobile menu
	mobileMenuBtn: "common:tooltips.mobileMenu",
	mobileMenuSessionsBtn: "common:tooltips.mobileSessions",
	mobileMenuSettingsBtn: "common:tooltips.mobileSettings",
	mobileMenuLogoutBtn: "common:tooltips.signOut",
};

/**
 * Update all tooltip texts based on current language
 */
export function updateTooltips() {
	Object.entries(TOOLTIP_MAP).forEach(([selector, key]) => {
		// Try as ID first
		let element = document.getElementById(selector);
		
		// If not found, try as CSS selector
		if (!element) {
			element = document.querySelector(selector);
		}
		
		if (element) {
			const translatedText = t(key);
			if (translatedText && translatedText !== key) {
				element.setAttribute("title", translatedText);
			}
		}
	});
}

/**
 * Initialize tooltip i18n system
 * Sets up automatic updates when language changes
 */
export function initTooltipI18n() {
	// Update tooltips initially
	updateTooltips();
	
	// Subscribe to locale changes
	locale.subscribe(() => {
		// Wait a bit for i18next to finish loading new language
		setTimeout(updateTooltips, 100);
	});
	
	// Also update on DOM content loaded (for initial page load)
	if (document.readyState === "loading") {
		document.addEventListener("DOMContentLoaded", updateTooltips);
	}
}

/**
 * Add a new tooltip mapping dynamically
 * Useful for dynamically created elements
 */
export function registerTooltip(elementOrSelector, translationKey) {
	let element;
	
	if (typeof elementOrSelector === "string") {
		element = document.getElementById(elementOrSelector) || 
		          document.querySelector(elementOrSelector);
	} else {
		element = elementOrSelector;
	}
	
	if (element) {
		const translatedText = t(translationKey);
		if (translatedText && translatedText !== translationKey) {
			element.setAttribute("title", translatedText);
		}
		
		// Store for future updates
		const selector = element.id || elementOrSelector;
		TOOLTIP_MAP[selector] = translationKey;
	}
}

/**
 * Remove a tooltip mapping
 */
export function unregisterTooltip(selector) {
	delete TOOLTIP_MAP[selector];
}

// Export for debugging
if (typeof window !== "undefined") {
	window.__tooltipI18n = {
		updateTooltips,
		registerTooltip,
		unregisterTooltip,
		getMap: () => TOOLTIP_MAP,
	};
}
