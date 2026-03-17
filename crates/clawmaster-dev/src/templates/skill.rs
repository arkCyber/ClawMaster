//! Skill templates

pub fn generate_skill_md(name: &str) -> String {
    format!(
        r#"---
name: {}
description: A ClawMaster skill
homepage: https://github.com/yourusername/{}
license: MIT
compatibility: ["clawmaster"]
allowed_tools: []
---

# {}

## Description

This skill provides...

## Usage

### Example 1

Describe how to use this skill...

### Example 2

Another usage example...

## Requirements

- List any requirements here

## Configuration

No configuration required.

## License

MIT License
"#,
        name, name, name
    )
}
