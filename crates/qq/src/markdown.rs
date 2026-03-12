//! Markdown to QQ CQ code conversion.

/// Convert Markdown to QQ CQ code format.
pub fn markdown_to_qq(markdown: &str) -> String {
    let mut result = markdown.to_string();

    // Bold: **text** or __text__ -> [CQ:face,id=text]
    // Note: QQ doesn't have native bold, we'll keep it as is
    // result = result.replace("**", "");
    // result = result.replace("__", "");

    // Italic: *text* or _text_ -> keep as is
    // result = result.replace("*", "");
    // result = result.replace("_", "");

    // Code: `code` -> keep as is
    // result = result.replace("`", "");

    // Links: [text](url) -> text (url)
    result = regex::Regex::new(r"\[([^\]]+)\]\(([^)]+)\)")
        .unwrap()
        .replace_all(&result, "$1 ($2)")
        .to_string();

    // Images: ![alt](url) -> [CQ:image,file=url]
    result = regex::Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)")
        .unwrap()
        .replace_all(&result, "[CQ:image,file=$2]")
        .to_string();

    // Headers: # text -> text
    result = regex::Regex::new(r"^#+\s+(.+)$")
        .unwrap()
        .replace_all(&result, "$1")
        .to_string();

    // Lists: - item or * item -> • item
    result = regex::Regex::new(r"^[\-\*]\s+(.+)$")
        .unwrap()
        .replace_all(&result, "• $1")
        .to_string();

    // Code blocks: ```code``` -> code
    result = regex::Regex::new(r"```[^\n]*\n(.*?)\n```")
        .unwrap()
        .replace_all(&result, "$1")
        .to_string();

    result
}

/// Convert QQ CQ code to plain text.
pub fn qq_to_plain_text(cq_code: &str) -> String {
    // Remove CQ codes
    regex::Regex::new(r"\[CQ:[^\]]+\]")
        .unwrap()
        .replace_all(cq_code, "")
        .to_string()
}

/// Escape special characters for QQ.
pub fn escape_qq(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('[', "&#91;")
        .replace(']', "&#93;")
        .replace(',', "&#44;")
}

/// Unescape special characters from QQ.
pub fn unescape_qq(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&#91;", "[")
        .replace("&#93;", "]")
        .replace("&#44;", ",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_qq_links() {
        let markdown = "[Click here](https://example.com)";
        let qq = markdown_to_qq(markdown);
        assert_eq!(qq, "Click here (https://example.com)");
    }

    #[test]
    fn test_markdown_to_qq_images() {
        let markdown = "![Image](https://example.com/image.jpg)";
        let qq = markdown_to_qq(markdown);
        assert_eq!(qq, "[CQ:image,file=https://example.com/image.jpg]");
    }

    #[test]
    fn test_qq_to_plain_text() {
        let cq = "Hello [CQ:face,id=1] World [CQ:image,file=test.jpg]";
        let plain = qq_to_plain_text(cq);
        assert_eq!(plain, "Hello  World ");
    }

    #[test]
    fn test_escape_qq() {
        let text = "Hello & [World], test";
        let escaped = escape_qq(text);
        assert_eq!(escaped, "Hello &amp; &#91;World&#93;&#44; test");
    }

    #[test]
    fn test_unescape_qq() {
        let text = "Hello &amp; &#91;World&#93;&#44; test";
        let unescaped = unescape_qq(text);
        assert_eq!(unescaped, "Hello & [World], test");
    }
}
