//! Storage backend for Wasm files.
//!
//! This module provides an abstraction over different storage backends
//! (local filesystem, S3, etc.) for storing Wasm tool files.

use {
    crate::error::{Error, Result},
    std::path::{Path, PathBuf},
    tokio::{fs, io::AsyncWriteExt},
};

/// Storage backend trait.
#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    /// Store a Wasm file.
    ///
    /// # Arguments
    /// * `name` - Tool name
    /// * `version` - Tool version
    /// * `wasm_bytes` - Wasm file bytes
    ///
    /// # Returns
    /// URL to download the file.
    async fn store(&self, name: &str, version: &str, wasm_bytes: &[u8]) -> Result<String>;

    /// Retrieve a Wasm file.
    ///
    /// # Arguments
    /// * `name` - Tool name
    /// * `version` - Tool version
    ///
    /// # Returns
    /// Wasm file bytes.
    async fn retrieve(&self, name: &str, version: &str) -> Result<Vec<u8>>;

    /// Delete a Wasm file.
    async fn delete(&self, name: &str, version: &str) -> Result<()>;
}

/// Local filesystem storage.
///
/// Stores Wasm files in a local directory structure:
/// `{base_dir}/{name}/{version}/tool.wasm`
pub struct LocalStorage {
    base_dir: PathBuf,
}

impl LocalStorage {
    /// Create a new local storage backend.
    ///
    /// # Arguments
    /// * `base_dir` - Base directory for storing files
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        }
    }

    fn tool_path(&self, name: &str, version: &str) -> PathBuf {
        self.base_dir.join(name).join(version).join("tool.wasm")
    }
}

#[async_trait::async_trait]
impl Storage for LocalStorage {
    async fn store(&self, name: &str, version: &str, wasm_bytes: &[u8]) -> Result<String> {
        let path = self.tool_path(name, version);

        // Create parent directories
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write file
        let mut file = fs::File::create(&path).await?;
        file.write_all(wasm_bytes).await?;
        file.sync_all().await?;

        // Return download URL (relative)
        Ok(format!("/tools/{}/{}/download", name, version))
    }

    async fn retrieve(&self, name: &str, version: &str) -> Result<Vec<u8>> {
        let path = self.tool_path(name, version);
        fs::read(&path).await.map_err(Into::into)
    }

    async fn delete(&self, name: &str, version: &str) -> Result<()> {
        let path = self.tool_path(name, version);
        fs::remove_file(&path).await.map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, tempfile::tempdir};

    #[tokio::test]
    async fn test_local_storage() {
        let dir = tempdir().unwrap();
        let storage = LocalStorage::new(dir.path());

        let wasm_bytes = b"\x00asm\x01\x00\x00\x00";

        // Store
        let url = storage
            .store("test-tool", "1.0.0", wasm_bytes)
            .await
            .unwrap();
        assert_eq!(url, "/tools/test-tool/1.0.0/download");

        // Retrieve
        let retrieved = storage.retrieve("test-tool", "1.0.0").await.unwrap();
        assert_eq!(retrieved, wasm_bytes);

        // Delete
        storage.delete("test-tool", "1.0.0").await.unwrap();
        assert!(storage.retrieve("test-tool", "1.0.0").await.is_err());
    }
}
