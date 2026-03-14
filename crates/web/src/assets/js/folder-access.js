// ── Folder Access Control UI ────────────────────────────────

import { sendRpc } from "./helpers.js";
import { t } from "./i18n.js";

/**
 * Initialize folder access control UI
 */
export function initFolderAccess() {
	var container = document.getElementById("folderAccessContainer");
	if (!container) return;

	loadFolders();
	setupEventListeners();
}

/**
 * Load and display folders
 */
async function loadFolders() {
	try {
		var response = await sendRpc("folder_access.list", {
			include_inactive: false,
		});

		if (response.ok && response.folders) {
			renderFolders(response.folders);
		}
	} catch (error) {
		console.error("Failed to load folders:", error);
		showError("Failed to load folders");
	}
}

/**
 * Render folders list
 */
function renderFolders(folders) {
	var container = document.getElementById("foldersList");
	if (!container) return;

	container.innerHTML = "";

	if (folders.length === 0) {
		container.innerHTML = `
			<div class="empty-state">
				<p>${t("folderAccess.noFolders")}</p>
			</div>
		`;
		return;
	}

	for (var folder of folders) {
		var card = createFolderCard(folder);
		container.appendChild(card);
	}
}

/**
 * Create folder card element
 */
function createFolderCard(folder) {
	var card = document.createElement("div");
	card.className = "folder-card";
	card.dataset.folderId = folder.id;

	var permissions = [];
	if (folder.permissions.can_read) permissions.push("Read");
	if (folder.permissions.can_write) permissions.push("Write");
	if (folder.permissions.can_execute) permissions.push("Execute");
	if (folder.permissions.can_delete) permissions.push("Delete");

	card.innerHTML = `
		<div class="folder-card-header">
			<div class="folder-path">
				<span class="folder-icon">📁</span>
				<span class="path-text">${escapeHtml(folder.folder_path)}</span>
			</div>
			<div class="folder-actions">
				<button class="btn-icon edit-folder" title="${t("folderAccess.edit")}">
					✏️
				</button>
				<button class="btn-icon remove-folder" title="${t("folderAccess.remove")}">
					🗑️
				</button>
			</div>
		</div>
		<div class="folder-card-body">
			<div class="folder-permissions">
				<span class="label">${t("folderAccess.permissions")}:</span>
				<div class="permission-badges">
					${permissions.map(p => `<span class="permission-badge">${p}</span>`).join("")}
				</div>
			</div>
			${folder.description ? `
				<div class="folder-description">
					<span class="label">${t("folderAccess.description")}:</span>
					<span>${escapeHtml(folder.description)}</span>
				</div>
			` : ""}
			<div class="folder-stats">
				<span class="stat">
					<span class="label">${t("folderAccess.accessCount")}:</span>
					<span class="value">${folder.access_count}</span>
				</span>
				<span class="stat">
					<span class="label">${t("folderAccess.createdBy")}:</span>
					<span class="value">${escapeHtml(folder.created_by)}</span>
				</span>
			</div>
		</div>
	`;

	// Add event listeners
	var editBtn = card.querySelector(".edit-folder");
	editBtn.addEventListener("click", () => editFolder(folder));

	var removeBtn = card.querySelector(".remove-folder");
	removeBtn.addEventListener("click", () => removeFolder(folder.id));

	return card;
}

/**
 * Setup event listeners
 */
function setupEventListeners() {
	var addBtn = document.getElementById("addFolderBtn");
	if (addBtn) {
		addBtn.addEventListener("click", showAddFolderDialog);
	}

	var refreshBtn = document.getElementById("refreshFoldersBtn");
	if (refreshBtn) {
		refreshBtn.addEventListener("click", loadFolders);
	}
}

/**
 * Show add folder dialog
 */
function showAddFolderDialog() {
	var dialog = document.getElementById("addFolderDialog");
	if (!dialog) {
		dialog = createAddFolderDialog();
		document.body.appendChild(dialog);
	}

	dialog.style.display = "flex";
}

/**
 * Create add folder dialog
 */
function createAddFolderDialog() {
	var dialog = document.createElement("div");
	dialog.id = "addFolderDialog";
	dialog.className = "modal-overlay";

	dialog.innerHTML = `
		<div class="modal-content">
			<div class="modal-header">
				<h2>${t("folderAccess.addFolder")}</h2>
				<button class="modal-close">&times;</button>
			</div>
			<div class="modal-body">
				<form id="addFolderForm">
					<div class="form-group">
						<label for="folderPath">${t("folderAccess.folderPath")}</label>
						<input type="text" id="folderPath" required 
							placeholder="/home/user/workspace">
					</div>
					<div class="form-group">
						<label for="folderDescription">${t("folderAccess.description")}</label>
						<input type="text" id="folderDescription" 
							placeholder="${t("folderAccess.descriptionPlaceholder")}">
					</div>
					<div class="form-group">
						<label>${t("folderAccess.permissions")}</label>
						<div class="permission-checkboxes">
							<label class="checkbox-label">
								<input type="checkbox" id="permRead" checked>
								<span>Read</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" id="permWrite">
								<span>Write</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" id="permExecute">
								<span>Execute</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" id="permDelete">
								<span>Delete</span>
							</label>
						</div>
					</div>
					<div class="form-actions">
						<button type="button" class="btn-secondary cancel-btn">
							${t("common.cancel")}
						</button>
						<button type="submit" class="btn-primary">
							${t("folderAccess.add")}
						</button>
					</div>
				</form>
			</div>
		</div>
	`;

	// Event listeners
	var closeBtn = dialog.querySelector(".modal-close");
	closeBtn.addEventListener("click", () => {
		dialog.style.display = "none";
	});

	var cancelBtn = dialog.querySelector(".cancel-btn");
	cancelBtn.addEventListener("click", () => {
		dialog.style.display = "none";
	});

	var form = dialog.querySelector("#addFolderForm");
	form.addEventListener("submit", async (e) => {
		e.preventDefault();
		await handleAddFolder();
		dialog.style.display = "none";
	});

	return dialog;
}

/**
 * Handle add folder submission
 */
async function handleAddFolder() {
	var path = document.getElementById("folderPath").value;
	var description = document.getElementById("folderDescription").value;
	var canRead = document.getElementById("permRead").checked;
	var canWrite = document.getElementById("permWrite").checked;
	var canExecute = document.getElementById("permExecute").checked;
	var canDelete = document.getElementById("permDelete").checked;

	try {
		var response = await sendRpc("folder_access.add", {
			folder_path: path,
			can_read: canRead,
			can_write: canWrite,
			can_execute: canExecute,
			can_delete: canDelete,
			description: description || null,
		});

		if (response.ok) {
			showSuccess(t("folderAccess.addSuccess"));
			loadFolders();
			// Reset form
			document.getElementById("addFolderForm").reset();
		} else {
			showError(response.error || "Failed to add folder");
		}
	} catch (error) {
		console.error("Failed to add folder:", error);
		showError(error.message || "Failed to add folder");
	}
}

/**
 * Edit folder permissions
 */
function editFolder(folder) {
	var dialog = document.getElementById("editFolderDialog");
	if (!dialog) {
		dialog = createEditFolderDialog();
		document.body.appendChild(dialog);
	}

	// Populate form
	document.getElementById("editFolderId").value = folder.id;
	document.getElementById("editFolderPath").textContent = folder.folder_path;
	document.getElementById("editPermRead").checked = folder.permissions.can_read;
	document.getElementById("editPermWrite").checked = folder.permissions.can_write;
	document.getElementById("editPermExecute").checked = folder.permissions.can_execute;
	document.getElementById("editPermDelete").checked = folder.permissions.can_delete;

	dialog.style.display = "flex";
}

/**
 * Create edit folder dialog
 */
function createEditFolderDialog() {
	var dialog = document.createElement("div");
	dialog.id = "editFolderDialog";
	dialog.className = "modal-overlay";

	dialog.innerHTML = `
		<div class="modal-content">
			<div class="modal-header">
				<h2>${t("folderAccess.editFolder")}</h2>
				<button class="modal-close">&times;</button>
			</div>
			<div class="modal-body">
				<form id="editFolderForm">
					<input type="hidden" id="editFolderId">
					<div class="form-group">
						<label>${t("folderAccess.folderPath")}</label>
						<div id="editFolderPath" class="readonly-field"></div>
					</div>
					<div class="form-group">
						<label>${t("folderAccess.permissions")}</label>
						<div class="permission-checkboxes">
							<label class="checkbox-label">
								<input type="checkbox" id="editPermRead">
								<span>Read</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" id="editPermWrite">
								<span>Write</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" id="editPermExecute">
								<span>Execute</span>
							</label>
							<label class="checkbox-label">
								<input type="checkbox" id="editPermDelete">
								<span>Delete</span>
							</label>
						</div>
					</div>
					<div class="form-actions">
						<button type="button" class="btn-secondary cancel-btn">
							${t("common.cancel")}
						</button>
						<button type="submit" class="btn-primary">
							${t("common.save")}
						</button>
					</div>
				</form>
			</div>
		</div>
	`;

	// Event listeners
	var closeBtn = dialog.querySelector(".modal-close");
	closeBtn.addEventListener("click", () => {
		dialog.style.display = "none";
	});

	var cancelBtn = dialog.querySelector(".cancel-btn");
	cancelBtn.addEventListener("click", () => {
		dialog.style.display = "none";
	});

	var form = dialog.querySelector("#editFolderForm");
	form.addEventListener("submit", async (e) => {
		e.preventDefault();
		await handleEditFolder();
		dialog.style.display = "none";
	});

	return dialog;
}

/**
 * Handle edit folder submission
 */
async function handleEditFolder() {
	var folderId = parseInt(document.getElementById("editFolderId").value);
	var canRead = document.getElementById("editPermRead").checked;
	var canWrite = document.getElementById("editPermWrite").checked;
	var canExecute = document.getElementById("editPermExecute").checked;
	var canDelete = document.getElementById("editPermDelete").checked;

	try {
		var response = await sendRpc("folder_access.update_permissions", {
			folder_id: folderId,
			can_read: canRead,
			can_write: canWrite,
			can_execute: canExecute,
			can_delete: canDelete,
		});

		if (response.ok) {
			showSuccess(t("folderAccess.updateSuccess"));
			loadFolders();
		} else {
			showError(response.error || "Failed to update permissions");
		}
	} catch (error) {
		console.error("Failed to update permissions:", error);
		showError(error.message || "Failed to update permissions");
	}
}

/**
 * Remove folder
 */
async function removeFolder(folderId) {
	if (!confirm(t("folderAccess.confirmRemove"))) {
		return;
	}

	try {
		var response = await sendRpc("folder_access.remove", {
			folder_id: folderId,
		});

		if (response.ok) {
			showSuccess(t("folderAccess.removeSuccess"));
			loadFolders();
		} else {
			showError(response.error || "Failed to remove folder");
		}
	} catch (error) {
		console.error("Failed to remove folder:", error);
		showError(error.message || "Failed to remove folder");
	}
}

/**
 * Show success message
 */
function showSuccess(message) {
	// TODO: Implement toast notification
	console.log("Success:", message);
	alert(message);
}

/**
 * Show error message
 */
function showError(message) {
	// TODO: Implement toast notification
	console.error("Error:", message);
	alert(message);
}

/**
 * Escape HTML to prevent XSS
 */
function escapeHtml(text) {
	var div = document.createElement("div");
	div.textContent = text;
	return div.innerHTML;
}
