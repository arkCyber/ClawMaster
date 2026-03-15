// ── Folder Access Control English strings ───────────────────

export default {
	// ── Page title ───────────────────────────────────────────
	title: "Folder Access Control",
	subtitle: "Manage folders and permissions accessible to AI agents",

	// ── Buttons ──────────────────────────────────────────────
	addFolder: "Add Folder",
	refresh: "Refresh",
	edit: "Edit",
	remove: "Remove",
	add: "Add",
	save: "Save",
	cancel: "Cancel",

	// ── Form labels ──────────────────────────────────────────
	folderPath: "Folder Path",
	description: "Description",
	descriptionPlaceholder: "Optional folder description",
	permissions: "Permissions",
	accessCount: "Access Count",
	createdBy: "Created By",

	// ── Permissions ──────────────────────────────────────────
	permRead: "Read",
	permWrite: "Write",
	permExecute: "Execute",
	permDelete: "Delete",

	// ── Messages ─────────────────────────────────────────────
	noFolders: "No folder permissions configured",
	addSuccess: "Folder added successfully",
	updateSuccess: "Permissions updated successfully",
	removeSuccess: "Folder removed successfully",
	confirmRemove: "Are you sure you want to remove this folder permission?",

	// ── Errors ───────────────────────────────────────────────
	errorLoad: "Failed to load folders",
	errorAdd: "Failed to add folder",
	errorUpdate: "Failed to update permissions",
	errorRemove: "Failed to remove folder",
	errorInvalidPath: "Invalid folder path",
	errorNoPermission: "At least one permission must be selected",

	// ── Dialog titles ────────────────────────────────────────
	editFolder: "Edit Folder Permissions",
	viewLogs: "View Access Logs",

	// ── Access logs ──────────────────────────────────────────
	logs: {
		title: "Access Logs",
		operation: "Operation",
		filePath: "File Path",
		success: "Status",
		timestamp: "Time",
		session: "Session",
		noLogs: "No access records",
		succeeded: "Success",
		failed: "Failed",
	},

	// ── Help text ────────────────────────────────────────────
	help: {
		folderPath: "Enter the full path of the folder to authorize access",
		permissions: "Select operations allowed for AI agents",
		read: "Allow reading file contents",
		write: "Allow creating and modifying files",
		execute: "Allow executing scripts and programs",
		delete: "Allow deleting files",
	},
};
