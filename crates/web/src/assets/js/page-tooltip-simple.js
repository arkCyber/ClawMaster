// ── Simple Tooltip Test ───────────────────────────────────────────
// Basic tooltip functionality test

import { html } from "htm/preact";
import { render } from "preact";
import { registerPage } from "./router.js";
import { SimpleTooltip, ButtonTooltip, TooltipPosition, TooltipTheme } from "./tooltip.js";

function SimpleTooltipTest() {
	return html`
		<div style="padding: 40px; max-width: 800px; margin: 0 auto;">
			<h1 style="font-size: 2rem; margin-bottom: 30px; color: var(--text, #fff);">
				🎯 Simple Tooltip Test
			</h1>
			
			<div style="margin-bottom: 40px;">
				<h2 style="font-size: 1.3rem; margin-bottom: 20px; color: var(--text, #fff);">
					Basic Tooltips
				</h2>
				<div style="display: flex; gap: 20px; flex-wrap: wrap;">
					<${SimpleTooltip} text="This is a simple tooltip">
						<button style="padding: 10px 20px; background: var(--accent, #667eea); color: white; border: none; border-radius: 6px; cursor: pointer;">
							Hover me
						</button>
					</${SimpleTooltip}>
					
					<${ButtonTooltip} text="Success!" theme=${TooltipTheme.SUCCESS}>
						<button style="padding: 10px 20px; background: var(--success, #22c55e); color: white; border: none; border-radius: 6px; cursor: pointer;">
							✅ Success
						</button>
					</${ButtonTooltip}>
					
					<${ButtonTooltip} text="Warning!" theme=${TooltipTheme.WARNING}>
						<button style="padding: 10px 20px; background: var(--warning, #f59e0b); color: white; border: none; border-radius: 6px; cursor: pointer;">
							⚠️ Warning
						</button>
					</${ButtonTooltip}>
					
					<${ButtonTooltip} text="Error!" theme=${TooltipTheme.ERROR}>
						<button style="padding: 10px 20px; background: var(--error, #ef4444); color: white; border: none; border-radius: 6px; cursor: pointer;">
							❌ Error
						</button>
					</${ButtonTooltip}>
				</div>
			</div>
			
			<div style="margin-bottom: 40px;">
				<h2 style="font-size: 1.3rem; margin-bottom: 20px; color: var(--text, #fff);">
					Position Test
				</h2>
				<div style="display: flex; gap: 20px; flex-wrap: wrap;">
					<${SimpleTooltip} text="Top position" position=${TooltipPosition.TOP}>
						<button style="padding: 10px 20px; background: var(--accent, #667eea); color: white; border: none; border-radius: 6px; cursor: pointer;">
							Top
						</button>
					</${SimpleTooltip}>
					
					<${SimpleTooltip} text="Bottom position" position=${TooltipPosition.BOTTOM}>
						<button style="padding: 10px 20px; background: var(--accent, #667eea); color: white; border: none; border-radius: 6px; cursor: pointer;">
							Bottom
						</button>
					</${SimpleTooltip}>
					
					<${SimpleTooltip} text="Left position" position=${TooltipPosition.LEFT}>
						<button style="padding: 10px 20px; background: var(--accent, #667eea); color: white; border: none; border-radius: 6px; cursor: pointer;">
							Left
						</button>
					</${SimpleTooltip}>
					
					<${SimpleTooltip} text="Right position" position=${TooltipPosition.RIGHT}>
						<button style="padding: 10px 20px; background: var(--accent, #667eea); color: white; border: none; border-radius: 6px; cursor: pointer;">
							Right
						</button>
					</${SimpleTooltip}>
				</div>
			</div>
			
			<div style="margin-bottom: 40px;">
				<h2 style="font-size: 1.3rem; margin-bottom: 20px; color: var(--text, #fff);">
					Icon Tooltips
				</h2>
				<div style="display: flex; gap: 20px; font-size: 2rem;">
					<${SimpleTooltip} text="Settings">
						<span style="cursor: pointer;">⚙️</span>
					</${SimpleTooltip}>
					
					<${SimpleTooltip} text="Download">
						<span style="cursor: pointer;">📥</span>
					</${SimpleTooltip}>
					
					<${SimpleTooltip} text="Help">
						<span style="cursor: pointer;">❓</span>
					</${SimpleTooltip}>
					
					<${SimpleTooltip} text="Information">
						<span style="cursor: pointer;">ℹ️</span>
					</${SimpleTooltip}>
				</div>
			</div>
			
			<div style="padding: 20px; background: var(--surface2, #2a2a2a); border-radius: 8px; border: 1px solid var(--border, #333);">
				<h3 style="color: var(--text, #fff); margin-bottom: 10px;">✅ Tooltip Status</h3>
				<p style="color: var(--muted, #888); margin: 0;">
					If you can see tooltips when hovering over the buttons above, the tooltip system is working correctly!
				</p>
			</div>
		</div>
	`;
}

function initSimpleTooltipTest(container) {
	render(html`<${SimpleTooltipTest} />`, container);
}

function teardownSimpleTooltipTest() {
	// Cleanup
}

// Register the simple test page
registerPage("/tooltip-simple", initSimpleTooltipTest, teardownSimpleTooltipTest);

export { SimpleTooltipTest };
