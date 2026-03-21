// 临时测试文件：检查 prompt 实际输出格式
use clawmaster_agents::prompt::*;
use clawmaster_agents::tool_registry::ToolRegistry;

fn main() {
    let tools = ToolRegistry::new();
    let runtime = PromptRuntimeContext {
        host: PromptHostRuntimeContext {
            host: Some("clawmaster-devbox".into()),
            os: Some("macos".into()),
            arch: Some("aarch64".into()),
            shell: Some("zsh".into()),
            time: Some("2026-02-17 16:18:00 CET".into()),
            today: Some("2026-02-17".into()),
            user_datetime: Some("2026-02-17 16:18:00 CET".into()),
            provider: Some("openai".into()),
            model: Some("gpt-5".into()),
            data_dir: Some("/home/moltis/.moltis".into()),
            sudo_non_interactive: Some(true),
            sudo_status: Some("passwordless".into()),
            timezone: Some("Europe/Paris".into()),
            accept_language: Some("en-US,fr;q=0.9".into()),
            remote_ip: Some("203.0.113.42".into()),
        },
        sandbox: Some(PromptSandboxRuntimeContext {
            enabled: true,
            image: Some("ubuntu:22.04".into()),
        }),
        nodes: None,
    };
    
    let prompt = build_system_prompt_full(
        &tools,
        true,
        None,
        None,
        None,
        None,
        None,
        Some(&runtime),
        None,
    );
    
    println!("=== PROMPT OUTPUT ===");
    println!("{}", prompt);
    println!("\n=== CHECKING FOR PATTERNS ===");
    println!("Contains 'Host: host=': {}", prompt.contains("Host: host="));
    println!("Contains 'Host:': {}", prompt.contains("Host:"));
    println!("Contains 'clawmaster-devbox': {}", prompt.contains("clawmaster-devbox"));
}
