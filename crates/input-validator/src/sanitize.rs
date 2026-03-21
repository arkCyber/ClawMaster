//! Output Encoding and Sanitization Utilities
//!
//! DO-178C Level A Compliant Output Encoding

use {
    html_escape::{encode_quoted_attribute, encode_text},
    percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode},
};

/// Characters to percent-encode in URLs
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

/// Encode HTML text content
///
/// DO-178C §6.3.1: Output encoding
pub fn encode_html(text: &str) -> String {
    encode_text(text).to_string()
}

/// Encode HTML attribute value
///
/// DO-178C §6.3.1: Attribute encoding
pub fn encode_html_attribute(text: &str) -> String {
    encode_quoted_attribute(text).to_string()
}

/// Encode URL component
///
/// DO-178C §6.3.1: URL encoding
pub fn encode_url(text: &str) -> String {
    utf8_percent_encode(text, FRAGMENT).to_string()
}

/// Encode for JavaScript string
///
/// DO-178C §6.3.1: JavaScript encoding
pub fn encode_javascript(text: &str) -> String {
    let mut result = String::new();

    for ch in text.chars() {
        match ch {
            '\'' => result.push_str(r"\'"),
            '"' => result.push_str(r#"\""#),
            '\\' => result.push_str(r"\\"),
            '\n' => result.push_str(r"\n"),
            '\r' => result.push_str(r"\r"),
            '\t' => result.push_str(r"\t"),
            '<' => result.push_str(r"\x3C"),
            '>' => result.push_str(r"\x3E"),
            '&' => result.push_str(r"\x26"),
            _ => result.push(ch),
        }
    }

    result
}

/// Encode for JSON string
///
/// DO-178C §6.3.1: JSON encoding
pub fn encode_json(text: &str) -> String {
    serde_json::to_string(text).unwrap_or_else(|_| "\"\"".to_string())
}

/// Strip HTML tags from text
///
/// DO-178C §6.3.1: HTML stripping
pub fn strip_html_tags(text: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for ch in text.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {},
        }
    }

    result
}

/// Sanitize for safe display (combine multiple encodings)
///
/// DO-178C §6.3.1: Safe display
pub fn sanitize_for_display(text: &str) -> String {
    // First strip any HTML tags
    let stripped = strip_html_tags(text);

    // Then encode remaining HTML entities
    encode_html(&stripped)
}

/// Truncate text to maximum length with ellipsis
///
/// DO-178C §6.3.1: Length limiting
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let truncated = &text[..max_length.saturating_sub(3)];
        format!("{}...", truncated)
    }
}

/// Remove control characters
///
/// DO-178C §6.3.1: Control character removal
pub fn remove_control_chars(text: &str) -> String {
    text.chars()
        .filter(|&c| !c.is_control() || c == '\n' || c == '\r' || c == '\t')
        .collect()
}

/// Normalize whitespace
///
/// DO-178C §6.3.1: Whitespace normalization
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_html() {
        assert_eq!(encode_html("Hello <world>"), "Hello &lt;world&gt;");
        assert_eq!(encode_html("A & B"), "A &amp; B");
    }

    #[test]
    fn test_encode_html_attribute() {
        let encoded = encode_html_attribute("value with \"quotes\"");
        assert!(encoded.contains("&quot;"));
    }

    #[test]
    fn test_encode_url() {
        assert_eq!(encode_url("hello world"), "hello%20world");
        assert_eq!(encode_url("a<b>c"), "a%3Cb%3Ec");
    }

    #[test]
    fn test_encode_javascript() {
        assert_eq!(encode_javascript("alert('test')"), r"alert(\'test\')");
        assert_eq!(encode_javascript("line1\nline2"), r"line1\nline2");
        assert_eq!(encode_javascript("<script>"), r"\x3Cscript\x3E");
    }

    #[test]
    fn test_encode_json() {
        let encoded = encode_json("hello \"world\"");
        assert!(encoded.contains(r#"\""#));
    }

    #[test]
    fn test_strip_html_tags() {
        assert_eq!(strip_html_tags("Hello <b>world</b>"), "Hello world");
        assert_eq!(
            strip_html_tags("<script>alert('xss')</script>"),
            "alert('xss')"
        );
    }

    #[test]
    fn test_sanitize_for_display() {
        let sanitized = sanitize_for_display("<script>alert('xss')</script>");
        assert!(!sanitized.contains("<script>"));
        assert!(!sanitized.contains("</script>"));
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("hello", 10), "hello");
        assert_eq!(truncate_text("hello world", 8), "hello...");
    }

    #[test]
    fn test_remove_control_chars() {
        let text = "hello\x00world\x01test";
        let cleaned = remove_control_chars(text);
        assert_eq!(cleaned, "helloworldtest");
    }

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("hello   world"), "hello world");
        assert_eq!(normalize_whitespace("  hello  \n  world  "), "hello world");
    }
}
