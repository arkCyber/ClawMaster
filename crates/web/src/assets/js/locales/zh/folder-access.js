// ── Folder Access Control Chinese (Simplified) strings ──────

export default {
	// ── Page title ───────────────────────────────────────────
	title: "文件夹访问控制",
	subtitle: "管理 AI 智能体可以访问的文件夹和权限",

	// ── Buttons ──────────────────────────────────────────────
	addFolder: "添加文件夹",
	refresh: "刷新",
	edit: "编辑",
	remove: "删除",
	add: "添加",
	save: "保存",
	cancel: "取消",

	// ── Form labels ──────────────────────────────────────────
	folderPath: "文件夹路径",
	description: "描述",
	descriptionPlaceholder: "可选的文件夹描述",
	permissions: "权限",
	accessCount: "访问次数",
	createdBy: "创建者",

	// ── Permissions ──────────────────────────────────────────
	permRead: "读取",
	permWrite: "写入",
	permExecute: "执行",
	permDelete: "删除",

	// ── Messages ─────────────────────────────────────────────
	noFolders: "暂无文件夹权限配置",
	addSuccess: "文件夹添加成功",
	updateSuccess: "权限更新成功",
	removeSuccess: "文件夹删除成功",
	confirmRemove: "确定要删除此文件夹权限吗？",

	// ── Errors ───────────────────────────────────────────────
	errorLoad: "加载文件夹失败",
	errorAdd: "添加文件夹失败",
	errorUpdate: "更新权限失败",
	errorRemove: "删除文件夹失败",
	errorInvalidPath: "无效的文件夹路径",
	errorNoPermission: "至少需要选择一个权限",

	// ── Dialog titles ────────────────────────────────────────
	editFolder: "编辑文件夹权限",
	viewLogs: "查看访问日志",

	// ── Access logs ──────────────────────────────────────────
	logs: {
		title: "访问日志",
		operation: "操作",
		filePath: "文件路径",
		success: "状态",
		timestamp: "时间",
		session: "会话",
		noLogs: "暂无访问记录",
		succeeded: "成功",
		failed: "失败",
	},

	// ── Help text ────────────────────────────────────────────
	help: {
		folderPath: "输入要授权访问的文件夹完整路径",
		permissions: "选择允许 AI 智能体执行的操作",
		read: "允许读取文件内容",
		write: "允许创建和修改文件",
		execute: "允许执行脚本和程序",
		delete: "允许删除文件",
	},
};
