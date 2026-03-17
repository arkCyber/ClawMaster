/// MIME detection via buffer sniffing with header fallback.
pub fn detect_mime(buffer: &[u8], headers: Option<&str>) -> String {
    // Magic byte detection (first few bytes)
    if buffer.len() >= 4 {
        match &buffer[0..4] {
            // PNG: 89 50 4E 47
            [0x89, 0x50, 0x4E, 0x47] => return "image/png".to_string(),
            // GIF: 47 49 46 38
            [0x47, 0x49, 0x46, 0x38] => return "image/gif".to_string(),
            // WebP: 52 49 46 46 (RIFF)
            [0x52, 0x49, 0x46, 0x46] if buffer.len() >= 12 && &buffer[8..12] == b"WEBP" => {
                return "image/webp".to_string();
            },
            _ => {},
        }
    }

    // JPEG: FF D8 FF
    if buffer.len() >= 3 && buffer[0] == 0xFF && buffer[1] == 0xD8 && buffer[2] == 0xFF {
        return "image/jpeg".to_string();
    }

    // MP4: ftyp at offset 4
    if buffer.len() >= 12 && &buffer[4..8] == b"ftyp" {
        return "video/mp4".to_string();
    }

    // OGG: 4F 67 67 53
    if buffer.len() >= 4 && &buffer[0..4] == b"OggS" {
        return "audio/ogg".to_string();
    }

    // MP3: ID3 or FF FB/FF F3 (MPEG sync)
    if buffer.len() >= 3 {
        if &buffer[0..3] == b"ID3" {
            return "audio/mpeg".to_string();
        }
        if buffer[0] == 0xFF && (buffer[1] == 0xFB || buffer[1] == 0xF3) {
            return "audio/mpeg".to_string();
        }
    }

    // Fall back to Content-Type header
    if let Some(ct) = headers {
        // Extract MIME type from Content-Type (e.g., "image/jpeg; charset=utf-8")
        let mime = ct.split(';').next().unwrap_or("").trim();
        if !mime.is_empty() {
            return mime.to_string();
        }
    }

    // Default to binary
    "application/octet-stream".to_string()
}

pub fn extension_for_mime(mime: &str) -> &str {
    match mime {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "audio/ogg" => "ogg",
        "audio/mpeg" => "mp3",
        "video/mp4" => "mp4",
        _ => "bin",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_mime_png() {
        let png_bytes = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(detect_mime(&png_bytes, None), "image/png");
    }

    #[test]
    fn test_detect_mime_jpeg() {
        let jpeg_bytes = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10];
        assert_eq!(detect_mime(&jpeg_bytes, None), "image/jpeg");
    }

    #[test]
    fn test_detect_mime_gif() {
        let gif_bytes = vec![0x47, 0x49, 0x46, 0x38, 0x39, 0x61];
        assert_eq!(detect_mime(&gif_bytes, None), "image/gif");
    }

    #[test]
    fn test_detect_mime_webp() {
        let webp_bytes = vec![
            0x52, 0x49, 0x46, 0x46, // RIFF
            0x00, 0x00, 0x00, 0x00, // size
            0x57, 0x45, 0x42, 0x50, // WEBP
        ];
        assert_eq!(detect_mime(&webp_bytes, None), "image/webp");
    }

    #[test]
    fn test_detect_mime_mp4() {
        let mp4_bytes = vec![
            0x00, 0x00, 0x00, 0x20, // size
            0x66, 0x74, 0x79, 0x70, // ftyp
            0x69, 0x73, 0x6F, 0x6D, // isom
        ];
        assert_eq!(detect_mime(&mp4_bytes, None), "video/mp4");
    }

    #[test]
    fn test_detect_mime_ogg() {
        let ogg_bytes = vec![0x4F, 0x67, 0x67, 0x53, 0x00, 0x02];
        assert_eq!(detect_mime(&ogg_bytes, None), "audio/ogg");
    }

    #[test]
    fn test_detect_mime_mp3_id3() {
        let mp3_bytes = vec![0x49, 0x44, 0x33, 0x03, 0x00, 0x00];
        assert_eq!(detect_mime(&mp3_bytes, None), "audio/mpeg");
    }

    #[test]
    fn test_detect_mime_mp3_sync() {
        let mp3_bytes = vec![0xFF, 0xFB, 0x90, 0x00];
        assert_eq!(detect_mime(&mp3_bytes, None), "audio/mpeg");
    }

    #[test]
    fn test_detect_mime_from_header() {
        let unknown_bytes = vec![0x00, 0x01, 0x02, 0x03];
        let mime = detect_mime(&unknown_bytes, Some("image/jpeg; charset=utf-8"));
        assert_eq!(mime, "image/jpeg");
    }

    #[test]
    fn test_detect_mime_default() {
        let unknown_bytes = vec![0x00, 0x01, 0x02, 0x03];
        assert_eq!(
            detect_mime(&unknown_bytes, None),
            "application/octet-stream"
        );
    }

    #[test]
    fn test_detect_mime_empty_buffer() {
        let empty_bytes = vec![];
        assert_eq!(detect_mime(&empty_bytes, None), "application/octet-stream");
    }

    #[test]
    fn test_extension_for_mime() {
        assert_eq!(extension_for_mime("image/jpeg"), "jpg");
        assert_eq!(extension_for_mime("image/png"), "png");
        assert_eq!(extension_for_mime("image/gif"), "gif");
        assert_eq!(extension_for_mime("image/webp"), "webp");
        assert_eq!(extension_for_mime("audio/ogg"), "ogg");
        assert_eq!(extension_for_mime("audio/mpeg"), "mp3");
        assert_eq!(extension_for_mime("video/mp4"), "mp4");
        assert_eq!(extension_for_mime("unknown/type"), "bin");
    }
}
