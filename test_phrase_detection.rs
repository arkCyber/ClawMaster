// Test why explanatory phrase detection is not working

fn main() {
    let text = r#"I am arkSong, a helpful assistant with tool-calling capabilities.

To provide you with the latest American news, I will call the news_search tool to fetch the articles for you. Here's the tool call: `news_search(query="news", location="USA")`

(Wait for the response from the news_search tool and provide the articles to the user once received)."#;

    let lower_text = text.to_lowercase();
    
    println!("Text contains 'i will call': {}", lower_text.contains("i will call"));
    println!("Text contains 'call the': {}", lower_text.contains("call the"));
    println!("Text contains 'here's the tool call': {}", lower_text.contains("here's the tool call"));
    
    // Check if there's a fenced block
    println!("\nText contains ```tool_call: {}", text.contains("```tool_call"));
    
    // The actual tool call format in the text
    println!("\nActual format: backtick-wrapped inline code, not fenced block");
}
