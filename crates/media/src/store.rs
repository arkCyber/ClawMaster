use std::path::{Path, PathBuf};

/// Download and save media to `<data_dir>/media/` with UUID-based naming.
pub async fn save_media_source(url: &str, base_dir: &Path) -> crate::Result<PathBuf> {
    use tokio::io::AsyncWriteExt;

    // Download the file
    let response = reqwest::get(url)
        .await
        .map_err(|e| crate::Error::external("Failed to download media", e))?;

    if !response.status().is_success() {
        return Err(crate::Error::invalid_input(format!(
            "HTTP error {}: {}",
            response.status(),
            url
        )));
    }

    // Get content-type header for MIME detection
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    // Download bytes
    let bytes = response
        .bytes()
        .await
        .map_err(|e| crate::Error::external("Failed to read response body", e))?;

    // Detect MIME type
    let mime = crate::mime::detect_mime(&bytes, content_type.as_deref());
    let ext = crate::mime::extension_for_mime(&mime);

    // Extract filename from URL or use "media"
    let url_path = url.split('/').last().unwrap_or("media");
    let name = url_path.split('?').next().unwrap_or("media");
    let name = name.split('.').next().unwrap_or("media");

    // Generate UUID-based filename
    let uuid = uuid::Uuid::new_v4();
    let filename = format!("{}---{}.{}", name, uuid, ext);

    // Ensure media directory exists
    let media_dir = base_dir.join("media");
    tokio::fs::create_dir_all(&media_dir)
        .await
        .map_err(|e| crate::Error::external("Failed to create media directory", e))?;

    // Save file
    let file_path = media_dir.join(&filename);
    let mut file = tokio::fs::File::create(&file_path)
        .await
        .map_err(|e| crate::Error::external("Failed to create media file", e))?;

    file.write_all(&bytes)
        .await
        .map_err(|e| crate::Error::external("Failed to write media file", e))?;

    file.flush()
        .await
        .map_err(|e| crate::Error::external("Failed to flush media file", e))?;

    Ok(file_path)
}
