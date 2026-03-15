// ── Tooltips Component ────────────────────────────────────────────
// A flexible tooltip system for ClawMaster WebUI
// Supports multiple positions, themes, and rich content

import { signal } from "@preact/signals";
import { html } from "htm/preact";
import { render } from "preact";
import { useEffect, useRef, useState } from "preact/hooks";

// Global tooltip state
var activeTooltip = signal(null);
var tooltipId = 0;

// Tooltip positions
export var TooltipPosition = {
	TOP: "top",
	BOTTOM: "bottom",
	LEFT: "left",
	RIGHT: "left",
	TOP_START: "top-start",
	TOP_END: "top-end",
	BOTTOM_START: "bottom-start",
	BOTTOM_END: "bottom-end",
	LEFT_START: "left-start",
	LEFT_END: "left-end",
	RIGHT_START: "right-start",
	RIGHT_END: "right-end",
};

// Tooltip themes
export var TooltipTheme = {
	DEFAULT: "default",
	DARK: "dark",
	LIGHT: "light",
	SUCCESS: "success",
	WARNING: "warning",
	ERROR: "error",
	INFO: "info",
};

// Tooltip component
export function Tooltip(props) {
	var {
		children,
		content,
		position = TooltipPosition.TOP,
		theme = TooltipTheme.DEFAULT,
		delay = 300,
		hideDelay = 100,
		disabled = false,
		arrow = true,
		maxWidth = 300,
		className = "",
		onClick,
		...restProps
	} = props;

	var [visible, setVisible] = useState(false);
	var [coords, setCoords] = useState({ x: 0, y: 0 });
	var tooltipRef = useRef(null);
	var triggerRef = useRef(null);
	var timeoutRef = useRef(null);
	var hideTimeoutRef = useRef(null);

	var tooltipId = useRef(++tooltipId);

	function showTooltip(e) {
		if (disabled) return;
		
		// Clear any existing timeouts
		if (timeoutRef.current) clearTimeout(timeoutRef.current);
		if (hideTimeoutRef.current) clearTimeout(hideTimeoutRef.current);

		timeoutRef.current = setTimeout(() => {
			if (!triggerRef.current) return;

			var rect = triggerRef.current.getBoundingClientRect();
			var scrollX = window.pageXOffset;
			var scrollY = window.pageYOffset;

			var tooltipCoords = calculateTooltipPosition(
				rect,
				position,
				arrow ? 8 : 0,
				maxWidth
			);

			setCoords(tooltipCoords);
			setVisible(true);
			activeTooltip.value = tooltipId.current;
		}, delay);
	}

	function hideTooltip() {
		if (timeoutRef.current) clearTimeout(timeoutRef.current);
		
		hideTimeoutRef.current = setTimeout(() => {
			if (activeTooltip.value === tooltipId.current) {
				setVisible(false);
				activeTooltip.value = null;
			}
		}, hideDelay);
	}

	function calculateTooltipPosition(rect, position, arrowSize, maxWidth) {
		var tooltipHeight = 40; // Estimated height
		var tooltipWidth = Math.min(maxWidth, 200); // Estimated width

		var x = 0;
		var y = 0;

		switch (position) {
			case TooltipPosition.TOP:
				x = rect.left + rect.width / 2 - tooltipWidth / 2;
				y = rect.top - tooltipHeight - arrowSize;
				break;
			case TooltipPosition.BOTTOM:
				x = rect.left + rect.width / 2 - tooltipWidth / 2;
				y = rect.bottom + arrowSize;
				break;
			case TooltipPosition.LEFT:
				x = rect.left - tooltipWidth - arrowSize;
				y = rect.top + rect.height / 2 - tooltipHeight / 2;
				break;
			case TooltipPosition.RIGHT:
				x = rect.right + arrowSize;
				y = rect.top + rect.height / 2 - tooltipHeight / 2;
				break;
			case TooltipPosition.TOP_START:
				x = rect.left;
				y = rect.top - tooltipHeight - arrowSize;
				break;
			case TooltipPosition.TOP_END:
				x = rect.right - tooltipWidth;
				y = rect.top - tooltipHeight - arrowSize;
				break;
			case TooltipPosition.BOTTOM_START:
				x = rect.left;
				y = rect.bottom + arrowSize;
				break;
			case TooltipPosition.BOTTOM_END:
				x = rect.right - tooltipWidth;
				y = rect.bottom + arrowSize;
				break;
			case TooltipPosition.LEFT_START:
				x = rect.left - tooltipWidth - arrowSize;
				y = rect.top;
				break;
			case TooltipPosition.LEFT_END:
				x = rect.left - tooltipWidth - arrowSize;
				y = rect.bottom - tooltipHeight;
				break;
			case TooltipPosition.RIGHT_START:
				x = rect.right + arrowSize;
				y = rect.top;
				break;
			case TooltipPosition.RIGHT_END:
				x = rect.right + arrowSize;
				y = rect.bottom - tooltipHeight;
				break;
		}

		// Keep tooltip within viewport
		var viewportWidth = window.innerWidth;
		var viewportHeight = window.innerHeight;

		if (x < 10) x = 10;
		if (x + tooltipWidth > viewportWidth - 10) x = viewportWidth - tooltipWidth - 10;
		if (y < 10) y = 10;
		if (y + tooltipHeight > viewportHeight - 10) y = viewportHeight - tooltipHeight - 10;

		return { x, y };
	}

	function handleClick(e) {
		if (onClick) onClick(e);
		hideTooltip();
	}

	// Hide tooltip when clicking outside
	useEffect(() => {
		function handleClickOutside(e) {
			if (triggerRef.current && !triggerRef.current.contains(e.target)) {
				hideTooltip();
			}
		}

		if (visible) {
			document.addEventListener("mousedown", handleClickOutside);
			return () => document.removeEventListener("mousedown", handleClickOutside);
		}
	}, [visible]);

	// Hide tooltip when scrolling
	useEffect(() => {
		function handleScroll() {
			hideTooltip();
		}

		if (visible) {
			window.addEventListener("scroll", handleScroll);
			window.addEventListener("resize", handleScroll);
			return () => {
				window.removeEventListener("scroll", handleScroll);
				window.removeEventListener("resize", handleScroll);
			};
		}
	}, [visible]);

	// Cleanup timeouts
	useEffect(() => {
		return () => {
			if (timeoutRef.current) clearTimeout(timeoutRef.current);
			if (hideTimeoutRef.current) clearTimeout(hideTimeoutRef.current);
		};
	}, []);

	var tooltipClasses = [
		"tooltip",
		`tooltip-${theme}`,
		`tooltip-${position}`,
		visible ? "tooltip-visible" : "tooltip-hidden",
		arrow ? "tooltip-with-arrow" : "tooltip-no-arrow",
		className,
	].filter(Boolean).join(" ");

	var tooltipStyle = {
		position: "fixed",
		left: `${coords.x}px`,
		top: `${coords.y}px`,
		maxWidth: `${maxWidth}px`,
		zIndex: 9999,
	};

	var arrowStyle = getArrowStyle(position);

	return html`
		<span
			ref=${triggerRef}
			class="tooltip-trigger"
			onMouseEnter=${showTooltip}
			onMouseLeave=${hideTooltip}
			onClick=${handleClick}
			...${restProps}
		>
			${children}
			${visible && html`
				<div
					ref=${tooltipRef}
					class=${tooltipClasses}
					style=${tooltipStyle}
					role="tooltip"
					aria-hidden=${!visible}
				>
					${typeof content === "string" ? html`<span>${content}</span>` : content}
					${arrow && html`
						<div class="tooltip-arrow" style=${arrowStyle}></div>
					`}
				</div>
			`}
		</span>
	`;
}

function getArrowStyle(position) {
	var baseStyle = {
		position: "absolute",
		width: "8px",
		height: "8px",
		transform: "rotate(45deg)",
		background: "inherit",
	};

	switch (position) {
		case TooltipPosition.TOP:
		case TooltipPosition.TOP_START:
		case TooltipPosition.TOP_END:
			return { ...baseStyle, bottom: "-4px", left: "50%", marginLeft: "-4px" };
		case TooltipPosition.BOTTOM:
		case TooltipPosition.BOTTOM_START:
		case TooltipPosition.BOTTOM_END:
			return { ...baseStyle, top: "-4px", left: "50%", marginLeft: "-4px" };
		case TooltipPosition.LEFT:
		case TooltipPosition.LEFT_START:
		case TooltipPosition.LEFT_END:
			return { ...baseStyle, right: "-4px", top: "50%", marginTop: "-4px" };
		case TooltipPosition.RIGHT:
		case TooltipPosition.RIGHT_START:
		case TooltipPosition.RIGHT_END:
			return { ...baseStyle, left: "-4px", top: "50%", marginTop: "-4px" };
		default:
			return baseStyle;
	}
}

// Convenience wrapper for simple text tooltips
export function SimpleTooltip(props) {
	var { text, ...restProps } = props;
	return html`<${Tooltip} content=${text} ...${restProps}>${props.children}<//>`;
}

// Tooltip for buttons
export function ButtonTooltip(props) {
	var { text, position = TooltipPosition.TOP, theme = TooltipTheme.DEFAULT, ...restProps } = props;
	return html`
		<${Tooltip} 
			content=${text} 
			position=${position} 
			theme=${theme}
			delay=${200}
			...${restProps}
		>
			${props.children}
		<//>
	`;
}

// Tooltip for icons
export function IconTooltip(props) {
	var { text, size = "small", ...restProps } = props;
	var sizeClasses = {
		small: "tooltip-icon-small",
		medium: "tooltip-icon-medium",
		large: "tooltip-icon-large",
	};

	return html`
		<${Tooltip} 
			content=${text} 
			position=${TooltipPosition.TOP} 
			theme=${TooltipTheme.DARK}
			delay=${150}
			className=${sizeClasses[size]}
			...${restProps}
		>
			${props.children}
		<//>
	`;
}

// Rich content tooltip with HTML
export function RichTooltip(props) {
	var { title, content, footer, ...restProps } = props;

	return html`
		<${Tooltip} 
			...${restProps}
			content=${html`
				<div class="tooltip-rich-content">
					${title && html`<div class="tooltip-title">${title}</div>`}
					<div class="tooltip-body">${content}</div>
					${footer && html`<div class="tooltip-footer">${footer}</div>`}
				</div>
			`}
		>
			${props.children}
		<//>
	`;
}

// Export signal for external access
export { activeTooltip };
