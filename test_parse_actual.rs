// Minimal reproduction of tool_parsing logic

fn is_standalone_bare_json_tool_call(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.starts_with('{') && trimmed.ends_with('}')
}

fn collect_bare_json_blocks(text: &str) -> Vec<String> {
    if !is_standalone_bare_json_tool_call(text) {
        return vec![];
    }
    
    let needle = r#""tool""#;
    if text.contains(needle) {
        vec!["would parse JSON here".to_string()]
    } else {
        vec![]
    }
}

fn main() {
    let actual_response = r#"I am arkSong, a helpful assistant with tool-calling capabilities.

To help you with American news, I'll call the news_search tool right away:

{"tool": "news_search", "arguments": {"query": "news", "location": "USA"}}

I'll get you the latest American news articles as soon as possible. Stay tuned!"#;

    println!("Text starts with '{{': {}", actual_response.trim().starts_with('{'));
    println!("Text ends with '}}': {}", actual_response.trim().ends_with('}'));
    println!("is_standalone: {}", is_standalone_bare_json_tool_call(actual_response));
    println!("Would parse: {:?}", collect_bare_json_blocks(actual_response));
    
    // Check what the text actually starts/ends with
    let trimmed = actual_response.trim();
    println!("\nFirst 50 chars: {:?}", &trimmed[..50.min(trimmed.len())]);
    println!("Last 50 chars: {:?}", &trimmed[trimmed.len().saturating_sub(50)..]);
}
