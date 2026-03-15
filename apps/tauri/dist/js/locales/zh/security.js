/**
 * Security Features - Chinese (Simplified) Translations
 * DO-178C Level A Compliant
 */

export default {
	emergencyStop: {
		title: "紧急停止 - 中止所有运行中的命令",
		label: "停止",
		confirm: "确定要停止所有正在运行的命令吗？",
		stopping: "正在停止...",
		success: "所有命令已停止",
		error: "停止命令失败"
	},
	mode: {
		off: "关闭",
		smart: "智能",
		always: "总是",
		offDesc: "审批已禁用（不推荐）",
		smartDesc: "智能审批模式（推荐）",
		alwaysDesc: "总是需要审批（最安全）"
	},
	approval: {
		title: "需要命令审批",
		body: "命令：",
		bannerText: "命令等待审批",
		viewButton: "查看"
	},
	settings: {
		title: "安全设置",
		approvalMode: "审批模式",
		approvalModeDesc: "控制命令执行前是否需要用户审批",
		securityLevel: "安全等级",
		securityLevelDesc: "控制允许执行的命令范围",
		allowlist: "命令白名单",
		allowlistDesc: "允许自动执行的命令列表",
		dangerousPatterns: "危险命令模式",
		dangerousPatternsDesc: "这些命令无论配置如何都会强制要求审批",
		
		modes: {
			off: {
				label: "关闭",
				desc: "不需要审批（不推荐）"
			},
			smart: {
				label: "智能模式",
				desc: "安全命令自动通过，其他需要审批（推荐）"
			},
			always: {
				label: "总是审批",
				desc: "所有命令都需要审批（最安全）"
			}
		},
		
		levels: {
			deny: {
				label: "拒绝",
				desc: "禁止所有命令执行"
			},
			allowlist: {
				label: "白名单",
				desc: "只允许白名单中的命令（推荐）"
			},
			full: {
				label: "完全",
				desc: "允许所有命令（危险命令仍需审批）"
			}
		},
		
		addCommand: "添加命令",
		removeCommand: "删除",
		saveSettings: "保存设置",
		cancel: "取消",
		saved: "设置已保存",
		saveFailed: "保存设置失败"
	},
	
	dangerousCommands: {
		title: "检测到危险命令",
		patterns: [
			"rm -r / (删除根目录)",
			"rm -r ~ (删除用户目录)",
			"mkfs (格式化文件系统)",
			"dd if=/dev/zero (磁盘覆写)",
			"fork bomb (系统资源耗尽)",
			"git reset --hard (强制重置)",
			"git push --force (强制推送)",
			"git clean -f (强制清理)",
			"git stash drop/clear (删除暂存)",
			"DROP TABLE/DATABASE (删除数据库)",
			"TRUNCATE (清空表)",
			"docker system prune (清理容器)",
			"kubectl delete namespace (删除命名空间)",
			"terraform destroy (销毁基础设施)",
			"chmod -R 777 / (根目录权限)"
		],
		note: "共 18 种危险模式"
	}
};
