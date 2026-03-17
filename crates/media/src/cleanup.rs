use std::time::{SystemTime, UNIX_EPOCH};

/// TTL-based media cleanup (default 2 minutes).
pub async fn clean_old_media(media_dir: &std::path::Path, ttl_secs: u64) -> crate::Result<u64> {
    if !media_dir.exists() {
        return Ok(0);
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cutoff_time = now.saturating_sub(ttl_secs);
    let mut deleted_count = 0u64;

    // Read directory entries
    let mut entries = tokio::fs::read_dir(media_dir)
        .await
        .map_err(|e| crate::Error::external("Failed to read media directory", e))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| crate::Error::external("Failed to read directory entry", e))?
    {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // Get file metadata
        let metadata = match tokio::fs::metadata(&path).await {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!("Failed to get metadata for {:?}: {}", path, e);
                continue;
            },
        };

        // Get modified time
        let modified = match metadata.modified() {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!("Failed to get modified time for {:?}: {}", path, e);
                continue;
            },
        };

        let modified_secs = modified
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Delete if older than TTL
        if modified_secs < cutoff_time {
            match tokio::fs::remove_file(&path).await {
                Ok(_) => {
                    tracing::debug!("Deleted old media file: {:?}", path);
                    deleted_count += 1;
                },
                Err(e) => {
                    tracing::warn!("Failed to delete {:?}: {}", path, e);
                },
            }
        }
    }

    if deleted_count > 0 {
        tracing::info!("Cleaned up {} old media files", deleted_count);
    }

    Ok(deleted_count)
}

#[cfg(test)]
mod tests {
    use {super::*, std::time::Duration};

    #[tokio::test]
    async fn test_clean_old_media_empty_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = clean_old_media(temp_dir.path(), 120).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_clean_old_media_nonexistent_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nonexistent = temp_dir.path().join("nonexistent");
        let result = clean_old_media(&nonexistent, 120).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_clean_old_media_with_files() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create test files
        let file1 = temp_dir.path().join("old_file.txt");
        let file2 = temp_dir.path().join("new_file.txt");

        tokio::fs::write(&file1, b"old content").await.unwrap();
        tokio::fs::write(&file2, b"new content").await.unwrap();

        // Wait a bit to ensure file timestamps are set
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Clean with TTL of 0 (should delete all files)
        let result = clean_old_media(temp_dir.path(), 0).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);

        // Verify files are deleted
        assert!(!file1.exists());
        assert!(!file2.exists());
    }

    #[tokio::test]
    async fn test_clean_old_media_respects_ttl() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create a test file
        let file = temp_dir.path().join("test_file.txt");
        tokio::fs::write(&file, b"content").await.unwrap();

        // Clean with very long TTL (should not delete)
        let result = clean_old_media(temp_dir.path(), 86400).await; // 1 day
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);

        // Verify file still exists
        assert!(file.exists());
    }

    #[tokio::test]
    async fn test_clean_old_media_skips_directories() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create a subdirectory
        let subdir = temp_dir.path().join("subdir");
        tokio::fs::create_dir(&subdir).await.unwrap();

        // Clean with TTL of 0
        let result = clean_old_media(temp_dir.path(), 0).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0); // Should not count directories

        // Verify directory still exists
        assert!(subdir.exists());
    }
}
