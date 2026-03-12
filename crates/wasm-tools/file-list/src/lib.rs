//! File listing tool for Moltis AI agents.
//!
//! This tool provides directory listing with:
//! - Path validation
//! - Entry count limits
//! - Hidden file filtering
//! - Metadata extraction
//!
//! # Compliance
//! DO-178C Level A:
//! - §6.3.2: Exception handling - All errors properly handled
//! - §6.3.4: Deterministic behavior - Sorted output
//! - §11.10: Resource management - Entry count limits
//! - §11.13: Initialization - No global state

use std::fs;
use std::path::{Path, PathBuf};

wit_bindgen::generate!({
    world: "file-list",
    exports: {
        world: FileListTool,
    },
});

/// Maximum entries to return
/// DO-178C §11.10: Prevent unbounded resource usage
const MAX_ENTRIES: usize = 1000;

struct FileListTool;

impl Guest for FileListTool {
    /// List directory contents (excluding hidden files).
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors caught and returned
    /// DO-178C §6.3.4: Deterministic - sorted output
    fn list_dir(path: String) -> Result<Vec<FileEntry>, String> {
        Self::list_dir_all(path, false)
    }
    
    /// List directory with option to include hidden files.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: Comprehensive error handling
    /// DO-178C §11.10: Entry count limited
    fn list_dir_all(path: String, include_hidden: bool) -> Result<Vec<FileEntry>, String> {
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        
        let path_obj = Path::new(&path);
        
        // Validate path exists and is a directory
        if !path_obj.exists() {
            return Err(format!("Path does not exist: {}", path));
        }
        
        if !path_obj.is_dir() {
            return Err(format!("Path is not a directory: {}", path));
        }
        
        // Read directory entries
        let entries = fs::read_dir(path_obj)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        let mut file_entries = Vec::new();
        
        for entry in entries {
            // DO-178C §11.10: Check entry limit
            if file_entries.len() >= MAX_ENTRIES {
                return Err(format!("Directory has more than {} entries", MAX_ENTRIES));
            }
            
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy().to_string();
            
            // Skip hidden files if not included
            if !include_hidden && name.starts_with('.') {
                continue;
            }
            
            let metadata = entry
                .metadata()
                .map_err(|e| format!("Failed to read metadata: {}", e))?;
            
            let is_dir = metadata.is_dir();
            let size = if is_dir { 0 } else { metadata.len() };
            
            file_entries.push(FileEntry {
                name,
                path: entry_path.to_string_lossy().to_string(),
                is_dir,
                size,
            });
        }
        
        // DO-178C §6.3.4: Deterministic - sort entries
        file_entries.sort_by(|a, b| a.name.cmp(&b.name));
        
        Ok(file_entries)
    }
    
    /// Check if directory exists.
    ///
    /// # Compliance
    /// DO-178C §6.3.4: Deterministic - no side effects
    fn dir_exists(path: String) -> bool {
        if path.is_empty() {
            return false;
        }
        
        let path_obj = Path::new(&path);
        path_obj.exists() && path_obj.is_dir()
    }
    
    /// Count entries in directory.
    ///
    /// # Compliance
    /// DO-178C §6.3.2: All errors properly handled
    fn count_entries(path: String) -> Result<u32, String> {
        if path.is_empty() {
            return Err("Path cannot be empty".to_string());
        }
        
        let path_obj = Path::new(&path);
        
        if !path_obj.exists() {
            return Err(format!("Path does not exist: {}", path));
        }
        
        if !path_obj.is_dir() {
            return Err(format!("Path is not a directory: {}", path));
        }
        
        let entries = fs::read_dir(path_obj)
            .map_err(|e| format!("Failed to read directory: {}", e))?;
        
        let count = entries.count() as u32;
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;
    
    #[test]
    fn test_list_dir_success() {
        let dir = tempdir().unwrap();
        
        // Create some test files
        File::create(dir.path().join("file1.txt")).unwrap();
        File::create(dir.path().join("file2.txt")).unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();
        
        let result = FileListTool::list_dir(dir.path().to_str().unwrap().to_string());
        assert!(result.is_ok());
        
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
        
        // Check sorted order
        assert_eq!(entries[0].name, "file1.txt");
        assert_eq!(entries[1].name, "file2.txt");
        assert_eq!(entries[2].name, "subdir");
        assert!(entries[2].is_dir);
    }
    
    #[test]
    fn test_list_dir_hidden_files() {
        let dir = tempdir().unwrap();
        
        File::create(dir.path().join("visible.txt")).unwrap();
        File::create(dir.path().join(".hidden")).unwrap();
        
        // Without hidden files
        let result = FileListTool::list_dir(dir.path().to_str().unwrap().to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
        
        // With hidden files
        let result = FileListTool::list_dir_all(
            dir.path().to_str().unwrap().to_string(),
            true,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
    
    #[test]
    fn test_list_dir_not_found() {
        let result = FileListTool::list_dir("/nonexistent/directory".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }
    
    #[test]
    fn test_list_dir_not_directory() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        File::create(&file_path).unwrap();
        
        let result = FileListTool::list_dir(file_path.to_str().unwrap().to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not a directory"));
    }
    
    #[test]
    fn test_dir_exists() {
        let dir = tempdir().unwrap();
        
        assert!(FileListTool::dir_exists(dir.path().to_str().unwrap().to_string()));
        assert!(!FileListTool::dir_exists("/nonexistent".to_string()));
        assert!(!FileListTool::dir_exists(String::new()));
    }
    
    #[test]
    fn test_count_entries() {
        let dir = tempdir().unwrap();
        
        File::create(dir.path().join("file1.txt")).unwrap();
        File::create(dir.path().join("file2.txt")).unwrap();
        File::create(dir.path().join(".hidden")).unwrap();
        
        let result = FileListTool::count_entries(dir.path().to_str().unwrap().to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3); // Includes hidden files
    }
}
