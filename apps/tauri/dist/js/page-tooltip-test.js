// ── Tooltip Test Page ─────────────────────────────────────────────
// Demo page to showcase tooltip functionality

import { html } from "htm/preact";
import { render } from "preact";
import { useEffect, useState } from "preact/hooks";
import { registerPage } from "./router.js";
import { 
	Tooltip, 
	SimpleTooltip, 
	ButtonTooltip, 
	IconTooltip, 
	RichTooltip,
	TooltipPosition,
	TooltipTheme 
} from "./tooltip.js";

function TooltipTestPage() {
	const [showCode, setShowCode] = useState(false);

	const codeExamples = {
		basic: `<SimpleTooltip text="This is a simple tooltip">
	<button>Hover me</button>
</SimpleTooltip>`,
		
		positioned: `<Tooltip 
	content="I appear on the right!" 
	position={TooltipPosition.RIGHT}
>
	<button>Right Tooltip</button>
</Tooltip>`,
		
		themed: `<ButtonTooltip 
	text="Warning message" 
	theme={TooltipTheme.WARNING}
>
	<button>⚠️ Warning</button>
</ButtonTooltip>`,
		
		rich: `<RichTooltip 
	title="Detailed Information"
	content="This is a rich tooltip with multiple sections and detailed content."
	footer="Click to learn more"
>
	<button>Rich Tooltip</button>
</RichTooltip>`
	};

	return html`
		<div class="tooltip-test-container">
			<div class="tooltip-test-header">
				<h1>🎯 Tooltip Component Demo</h1>
				<p>Test all tooltip variations and configurations</p>
			</div>

			<div class="tooltip-test-section">
				<h2>Basic Tooltips</h2>
				<div class="tooltip-demo-grid">
					<${SimpleTooltip} text="Simple text tooltip">
						<button class="demo-btn">Simple Tooltip</button>
					</${SimpleTooltip}>
					
					<${Tooltip} content="Tooltip with custom delay" delay={500}>
						<button class="demo-btn">Delayed Tooltip</button>
					</${Tooltip}>
					
					<${Tooltip} content="I appear on the right!" position=${TooltipPosition.RIGHT}>
						<button class="demo-btn">Right Position</button>
					</${Tooltip}>
					
					<${Tooltip} content="I appear below!" position=${TooltipPosition.BOTTOM}>
						<button class="demo-btn">Bottom Position</button>
					</${Tooltip}>
				</div>
			</div>

			<div class="tooltip-test-section">
				<h2>Themed Tooltips</h2>
				<div class="tooltip-demo-grid">
					<${ButtonTooltip} text="Success operation!" theme=${TooltipTheme.SUCCESS}>
						<button class="demo-btn success">✅ Success</button>
					</${ButtonTooltip}>
					
					<${ButtonTooltip} text="Warning message" theme=${TooltipTheme.WARNING}>
						<button class="demo-btn warning">⚠️ Warning</button>
					</${ButtonTooltip}>
					
					<${ButtonTooltip} text="Error occurred" theme=${TooltipTheme.ERROR}>
						<button class="demo-btn error">❌ Error</button>
					</${ButtonTooltip}>
					
					<${ButtonTooltip} text="Information" theme=${TooltipTheme.INFO}>
						<button class="demo-btn info">ℹ️ Info</button>
					</${ButtonTooltip}>
					
					<${ButtonTooltip} text="Dark theme" theme=${TooltipTheme.DARK}>
						<button class="demo-btn">🌙 Dark</button>
					</${ButtonTooltip}>
					
					<${ButtonTooltip} text="Light theme" theme=${TooltipTheme.LIGHT}>
						<button class="demo-btn">☀️ Light</button>
					</${ButtonTooltip}>
				</div>
			</div>

			<div class="tooltip-test-section">
				<h2>Icon Tooltips</h2>
				<div class="tooltip-demo-grid">
					<${IconTooltip} text="Settings configuration">
						<span class="demo-icon">⚙️</span>
					</${IconTooltip}>
					
					<${IconTooltip} text="Download file" size="medium">
						<span class="demo-icon">📥</span>
					</${IconTooltip}>
					
					<${IconTooltip} text="Delete item" size="large">
						<span class="demo-icon">🗑️</span>
					</${IconTooltip}>
					
					<${IconTooltip} text="Help documentation">
						<span class="demo-icon">❓</span>
					</${IconTooltip}>
				</div>
			</div>

			<div class="tooltip-test-section">
				<h2>Rich Content Tooltips</h2>
				<div class="tooltip-demo-grid">
					<${RichTooltip} 
						title="User Profile"
						content="This user has administrative privileges and can access all system features."
						footer="Last active: 2 minutes ago"
					>
						<button class="demo-btn">User Info</button>
					</${RichTooltip}>
					
					<${RichTooltip} 
						title="System Status"
						content=${html`
							<div style="color: #22c55e;">✅ All systems operational</div>
							<div style="margin-top: 4px;">CPU: 45% | Memory: 2.3GB</div>
						`}
						footer="Updated: Just now"
					>
						<button class="demo-btn">System Status</button>
					</${RichTooltip}>
					
					<${Tooltip} 
						content=${html`
							<div style="display: flex; align-items: center; gap: 8px;">
								<span>🚀</span>
								<span><strong>Pro tip:</strong> Press Ctrl+K for quick commands</span>
							</div>
						`}
						maxWidth={400}
					>
						<button class="demo-btn">HTML Content</button>
					</${Tooltip}>
				</div>
			</div>

			<div class="tooltip-test-section">
				<h2>Advanced Features</h2>
				<div class="tooltip-demo-grid">
					<${Tooltip} 
						content="This tooltip has no arrow" 
						arrow={false}
					>
						<button class="demo-btn">No Arrow</button>
					</${Tooltip}>
					
					<${Tooltip} 
						content="This is a very long tooltip that demonstrates how the system handles text wrapping and ensures the tooltip stays within the viewport boundaries without overflowing or getting cut off." 
						maxWidth={200}
					>
						<button class="demo-btn">Long Content</button>
					</${Tooltip}>
					
					<${Tooltip} 
						content="Disabled tooltip" 
						disabled={true}
					>
						<button class="demo-btn" disabled>Disabled</button>
					</${Tooltip}>
					
					<${Tooltip} 
						content="Click me!" 
						onClick=${() => alert('Tooltip clicked!')}
					>
						<button class="demo-btn">Clickable</button>
					</${Tooltip}>
				</div>
			</div>

			<div class="tooltip-test-section">
				<h2>Code Examples</h2>
				<div class="code-examples">
					${Object.entries(codeExamples).map(([key, code]) => html`
						<div class="code-example">
							<h3>${key.charAt(0).toUpperCase() + key.slice(1)}</h3>
							<button 
								class="code-toggle"
								onClick=${() => setShowCode(showCode === key ? null : key)}
							>
								${showCode === key ? 'Hide' : 'Show'} Code
							</button>
							${showCode === key && html`
								<pre class="code-block"><code>${code}</code></pre>
							`}
						</div>
					`)}
				</div>
			</div>

			<div class="tooltip-test-section">
				<h2>Position Variations</h2>
				<div class="position-demo">
					<div class="position-grid">
						${[
							TooltipPosition.TOP,
							TooltipPosition.TOP_START,
							TooltipPosition.TOP_END,
							TooltipPosition.BOTTOM,
							TooltipPosition.BOTTOM_START,
							TooltipPosition.BOTTOM_END,
							TooltipPosition.LEFT,
							TooltipPosition.LEFT_START,
							TooltipPosition.LEFT_END,
							TooltipPosition.RIGHT,
							TooltipPosition.RIGHT_START,
							TooltipPosition.RIGHT_END,
						].map(position => html`
							<${Tooltip} 
								content=${position.replace('-', ' ')} 
								position=${position}
							>
								<button class="demo-btn small">${position.split('-').map(word => 
									word.charAt(0).toUpperCase() + word.slice(1)
								).join(' ')}</button>
							</${Tooltip}>
						`)}
					</div>
				</div>
			</div>
		</div>
	`;
}

function initTooltipTest(container) {
	render(html`<${TooltipTestPage} />`, container);
}

function teardownTooltipTest() {
	// Cleanup if needed
}

// Register the test page
registerPage("/tooltip-test", initTooltipTest, teardownTooltipTest);

export { TooltipTestPage };
