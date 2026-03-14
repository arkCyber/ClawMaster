# clawmaster-soul

SOUL.md personality system for ClawMaster - Define AI personality, behavior, and constraints.

## Overview

The `clawmaster-soul` crate provides a system for defining and managing AI personality through a human-readable `SOUL.md` file. This allows users to customize how the AI assistant behaves, communicates, and makes decisions.

## Features

- **Personality Traits**: Define style, tone, and expertise
- **Behavior Rules**: Specify what the AI should always/never do
- **Safety Constraints**: Set security and privacy boundaries
- **System Prompt Generation**: Automatically convert SOUL.md to LLM prompts
- **Hot Reload**: Update personality without restarting
- **Custom Sections**: Add any custom configuration sections

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
clawmaster-soul = { path = "../soul" }
```

## Usage

### Basic Usage

```rust
use clawmaster_soul::Soul;

#[tokio::main]
async fn main() -> Result<()> {
    // Load SOUL.md (creates default if not exists)
    let soul = Soul::load().await?;
    
    // Get system prompt for LLM
    let system_prompt = soul.get_system_prompt();
    
    // Use in your LLM calls
    let messages = vec![
        Message::system(system_prompt),
        Message::user("Hello!"),
    ];
    
    Ok(())
}
```

### Reload on Changes

```rust
let mut soul = Soul::load().await?;

// Later, when SOUL.md is modified
soul.reload().await?;
let updated_prompt = soul.get_system_prompt();
```

### Access Specific Sections

```rust
let soul = Soul::load().await?;

// Get personality traits
let personality = soul.personality();
println!("Style: {:?}", personality.style);
println!("Tone: {:?}", personality.tone);
println!("Expertise: {:?}", personality.expertise);

// Get behavior rules
let behavior = soul.behavior();
println!("Always do: {:?}", behavior.always_do);
println!("Never do: {:?}", behavior.never_do);

// Get constraints
let constraints = soul.constraints();
println!("Safety: {:?}", constraints.safety);
```

## SOUL.md File Format

The SOUL.md file uses Markdown with specific section headers:

```markdown
# SOUL.md - AI Personality Configuration

## Personality
- Professional yet approachable
- Helpful and proactive
- Clear and concise communication

## Tone
- Friendly but not overly casual
- Respectful and patient
- Encouraging and supportive

## Expertise
- Rust programming
- System architecture
- DevOps and deployment

## Behavior
- Always provide code examples when relevant
- Explain technical decisions and trade-offs
- Suggest improvements and best practices

## Never Do
- Execute dangerous operations without confirmation
- Access or modify sensitive data without permission
- Make assumptions about user requirements

## Safety
- Require confirmation for destructive operations
- Validate all inputs before processing
- Follow security best practices

## Confirmation Required
- Deleting files or data
- Modifying system configurations
- Installing dependencies
```

## Supported Sections

### Standard Sections

| Section | Description | Maps To |
|---------|-------------|---------|
| `Personality` / `Style` | Communication style | `personality.style` |
| `Tone` | Voice and manner | `personality.tone` |
| `Expertise` | Areas of knowledge | `personality.expertise` |
| `Behavior` / `Always Do` | Required behaviors | `behavior.always_do` |
| `Never Do` | Prohibited actions | `behavior.never_do` |
| `Preferences` | Optional preferences | `behavior.preferences` |
| `Safety` | Security constraints | `constraints.safety` |
| `Privacy` | Privacy rules | `constraints.privacy` |
| `Confirmation Required` | Actions needing approval | `constraints.confirmation_required` |

### Custom Sections

Any section not matching the standard names will be stored as a custom section:

```rust
let soul = Soul::load().await?;
for section in soul.custom_sections {
    println!("{}: {}", section.title, section.content);
}
```

## File Location

The SOUL.md file is stored at:

```
~/.config/clawmaster/SOUL.md
```

On first run, a default SOUL.md is created automatically.

## API Reference

### `Soul`

Main struct for managing SOUL.md.

```rust
pub struct Soul {
    pub async fn load() -> Result<Self>
    pub async fn reload(&mut self) -> Result<()>
    pub fn content(&self) -> &str
    pub fn path(&self) -> &Path
    pub fn personality(&self) -> &PersonalityTraits
    pub fn behavior(&self) -> &BehaviorRules
    pub fn constraints(&self) -> &Constraints
    pub fn get_system_prompt(&self) -> String
}
```

### `PersonalityTraits`

```rust
pub struct PersonalityTraits {
    pub style: Vec<String>,
    pub tone: Vec<String>,
    pub expertise: Vec<String>,
}
```

### `BehaviorRules`

```rust
pub struct BehaviorRules {
    pub always_do: Vec<String>,
    pub never_do: Vec<String>,
    pub preferences: Vec<String>,
}
```

### `Constraints`

```rust
pub struct Constraints {
    pub safety: Vec<String>,
    pub privacy: Vec<String>,
    pub confirmation_required: Vec<String>,
}
```

## Examples

### Example 1: Professional Assistant

```markdown
# SOUL.md

## Personality
- Professional and formal
- Detail-oriented
- Precise and accurate

## Tone
- Formal and respectful
- Clear and unambiguous

## Expertise
- Technical documentation
- Code review
- Best practices

## Behavior
- Always cite sources
- Provide detailed explanations
- Use industry-standard terminology

## Never Do
- Use casual language
- Make assumptions
- Skip error handling
```

### Example 2: Friendly Helper

```markdown
# SOUL.md

## Personality
- Friendly and approachable
- Patient and understanding
- Encouraging

## Tone
- Warm and supportive
- Conversational
- Positive

## Expertise
- Beginner-friendly explanations
- Step-by-step guidance
- Troubleshooting

## Behavior
- Break down complex topics
- Provide examples
- Encourage questions

## Never Do
- Use jargon without explanation
- Rush through explanations
- Criticize mistakes
```

## Testing

Run the test suite:

```bash
cargo test -p clawmaster-soul
```

Tests cover:
- Default file creation
- SOUL.md parsing
- System prompt generation
- File reloading
- Custom sections

## Integration with ClawMaster

The SOUL.md system integrates with ClawMaster's LLM providers:

```rust
use clawmaster_soul::Soul;
use clawmaster_providers::Provider;

async fn create_llm_request(user_message: &str) -> Result<()> {
    let soul = Soul::load().await?;
    let system_prompt = soul.get_system_prompt();
    
    let provider = Provider::new();
    let response = provider.chat(vec![
        Message::system(system_prompt),
        Message::user(user_message),
    ]).await?;
    
    Ok(())
}
```

## Comparison with OpenClaw

| Feature | OpenClaw | ClawMaster |
|---------|----------|------------|
| SOUL.md Support | ✅ | ✅ |
| Structured Parsing | ❌ | ✅ |
| Type-Safe API | ❌ | ✅ |
| Hot Reload | ❌ | ✅ |
| Custom Sections | ❌ | ✅ |
| System Prompt Gen | ✅ | ✅ |

## Best Practices

1. **Be Specific**: Use concrete examples in your SOUL.md
2. **Keep It Updated**: Reload when behavior needs change
3. **Test Changes**: Verify new personality traits work as expected
4. **Version Control**: Track SOUL.md changes in git
5. **Document Decisions**: Explain why certain rules exist

## Troubleshooting

### SOUL.md Not Found

The file is automatically created at `~/.config/clawmaster/SOUL.md` on first run.

### Changes Not Applied

Make sure to call `reload()` after modifying the file:

```rust
soul.reload().await?;
```

### Parsing Errors

Ensure your SOUL.md follows the format:
- Use `## Section Name` for headers
- Use `- Item` for list items
- Keep one blank line between sections

## Contributing

Contributions are welcome! Please ensure:
- All tests pass
- Code is formatted with `cargo fmt`
- Documentation is updated

## License

MIT OR Apache-2.0

---

**Version**: 0.10.18  
**Status**: ✅ Production Ready  
**Tests**: 4/4 passing
