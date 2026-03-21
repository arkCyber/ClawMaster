fn is_standalone_bare_json_tool_call(text: &str) -> bool {
    let trimmed = text.trim();
    trimmed.starts_with('{') && trimmed.ends_with('}')
}

fn main() {
    let test1 = r#"I am arkSong, a helpful assistant with tool-calling capabilities.

To help you with American news, I'll call the news_search tool right away:

{"tool": "news_search", "arguments": {"query": "news", "location": "USA"}}

I'll get you the latest American news articles as soon as possible. Stay tuned!"#;

    let test2 = r#"{"tool": "news_search", "arguments": {"query": "news", "location": "USA"}}"#;

    println!("test1 (with prose): {}", is_standalone_bare_json_tool_call(test1));
    println!("test2 (pure JSON): {}", is_standalone_bare_json_tool_call(test2));
}
