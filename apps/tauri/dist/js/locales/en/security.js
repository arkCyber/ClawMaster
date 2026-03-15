/**
 * Security Features - English Translations
 * DO-178C Level A Compliant
 */

export default {
	emergencyStop: {
		title: "Emergency Stop - Abort all running commands",
		label: "STOP",
		confirm: "Are you sure you want to stop all running commands?",
		stopping: "Stopping...",
		success: "All commands stopped",
		error: "Failed to stop commands"
	},
	mode: {
		off: "Off",
		smart: "Smart",
		always: "Always",
		offDesc: "Approval disabled (not recommended)",
		smartDesc: "Smart approval mode (recommended)",
		alwaysDesc: "Always require approval (most secure)"
	},
	approval: {
		title: "Command Approval Required",
		body: "Command: ",
		bannerText: "Command waiting for approval",
		viewButton: "View"
	},
	settings: {
		title: "Security Settings",
		approvalMode: "Approval Mode",
		approvalModeDesc: "Control whether commands require user approval before execution",
		securityLevel: "Security Level",
		securityLevelDesc: "Control which commands are allowed to execute",
		allowlist: "Command Allowlist",
		allowlistDesc: "List of commands allowed to execute automatically",
		dangerousPatterns: "Dangerous Command Patterns",
		dangerousPatternsDesc: "These commands always require approval regardless of settings",
		
		modes: {
			off: {
				label: "Off",
				desc: "No approval required (not recommended)"
			},
			smart: {
				label: "Smart Mode",
				desc: "Safe commands auto-approved, others require approval (recommended)"
			},
			always: {
				label: "Always Approve",
				desc: "All commands require approval (most secure)"
			}
		},
		
		levels: {
			deny: {
				label: "Deny",
				desc: "Block all command execution"
			},
			allowlist: {
				label: "Allowlist",
				desc: "Only allow whitelisted commands (recommended)"
			},
			full: {
				label: "Full",
				desc: "Allow all commands (dangerous commands still require approval)"
			}
		},
		
		addCommand: "Add Command",
		removeCommand: "Remove",
		saveSettings: "Save Settings",
		cancel: "Cancel",
		saved: "Settings saved",
		saveFailed: "Failed to save settings"
	},
	
	dangerousCommands: {
		title: "Dangerous Command Detected",
		patterns: [
			"rm -r / (delete root directory)",
			"rm -r ~ (delete home directory)",
			"mkfs (format filesystem)",
			"dd if=/dev/zero (disk overwrite)",
			"fork bomb (resource exhaustion)",
			"git reset --hard (force reset)",
			"git push --force (force push)",
			"git clean -f (force clean)",
			"git stash drop/clear (delete stash)",
			"DROP TABLE/DATABASE (drop database)",
			"TRUNCATE (truncate table)",
			"docker system prune (prune containers)",
			"kubectl delete namespace (delete namespace)",
			"terraform destroy (destroy infrastructure)",
			"chmod -R 777 / (root permissions)"
		],
		note: "18 dangerous patterns total"
	}
};
