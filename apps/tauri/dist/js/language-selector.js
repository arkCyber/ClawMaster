// ── Language Selector Component ────────────────────────────
// Dropdown component for selecting UI language

import { html } from "htm/preact";
import { useState, useEffect, useRef } from "preact/hooks";
import { useComputed } from "@preact/signals";
import { locale, setLocale, supportedLocales, localeNames } from "./i18n.js";

/**
 * Language selector dropdown component
 * Replaces the "Report issue" button in the header
 */
export function LanguageSelector() {
	var [isOpen, setIsOpen] = useState(false);
	var dropdownRef = useRef(null);
	// Use computed to reactively track locale changes
	var currentLocale = useComputed(() => locale.value);

	// Close dropdown when clicking outside
	useEffect(() => {
		if (!isOpen) return;

		function handleClickOutside(event) {
			if (dropdownRef.current && !dropdownRef.current.contains(event.target)) {
				setIsOpen(false);
			}
		}

		document.addEventListener("mousedown", handleClickOutside);
		return () => document.removeEventListener("mousedown", handleClickOutside);
	}, [isOpen]);

	// Close dropdown on Escape key
	useEffect(() => {
		if (!isOpen) return;

		function handleEscape(event) {
			if (event.key === "Escape") {
				setIsOpen(false);
			}
		}

		document.addEventListener("keydown", handleEscape);
		return () => document.removeEventListener("keydown", handleEscape);
	}, [isOpen]);

	function handleLanguageChange(lng) {
		setLocale(lng).then(() => {
			setIsOpen(false);
		});
	}

	function toggleDropdown() {
		setIsOpen(!isOpen);
	}

	// Get language flag emoji (approximate)
	function getLanguageFlag(lng) {
		var flags = {
			en: "🇬🇧",
			zh: "🇨🇳",
			es: "🇪🇸",
			fr: "🇫🇷",
			de: "🇩🇪",
			ja: "🇯🇵",
			ko: "🇰🇷",
			ru: "🇷🇺",
			pt: "🇵🇹",
			it: "🇮🇹",
			ar: "🇸🇦",
			hi: "🇮🇳",
			tr: "🇹🇷",
			nl: "🇳🇱",
			pl: "🇵🇱",
			vi: "🇻🇳",
		};
		return flags[lng] || "🌐";
	}

	return html`
		<div class="language-selector" ref=${dropdownRef}>
			<button
				class="language-selector-button"
				onClick=${toggleDropdown}
				title="Select language"
				aria-label="Select language"
				aria-expanded=${isOpen}
			>
				<span class="language-selector-flag">${getLanguageFlag(currentLocale.value)}</span>
				<span class="language-selector-name">${localeNames[currentLocale.value]}</span>
				<svg
					class="language-selector-arrow ${isOpen ? 'language-selector-arrow-open' : ''}"
					width="12"
					height="12"
					viewBox="0 0 12 12"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<path
						d="M2 4L6 8L10 4"
						stroke="currentColor"
						stroke-width="1.5"
						stroke-linecap="round"
						stroke-linejoin="round"
					/>
				</svg>
			</button>

			${isOpen && html`
				<div class="language-selector-dropdown">
					<div class="language-selector-list">
						${supportedLocales.map((lng) => html`
							<button
								key=${lng}
								class="language-selector-item ${lng === currentLocale.value ? 'language-selector-item-active' : ''}"
								onClick=${() => handleLanguageChange(lng)}
							>
								<span class="language-selector-item-flag">${getLanguageFlag(lng)}</span>
								<span class="language-selector-item-name">${localeNames[lng]}</span>
								${lng === currentLocale.value && html`
									<svg
										class="language-selector-item-check"
										width="16"
										height="16"
										viewBox="0 0 16 16"
										fill="none"
										xmlns="http://www.w3.org/2000/svg"
									>
										<path
											d="M13 4L6 11L3 8"
											stroke="currentColor"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
										/>
									</svg>
								`}
							</button>
						`)}
					</div>
				</div>
			`}
		</div>
	`;
}
