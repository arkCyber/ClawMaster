# ClawHub - Wasm Tool Plugin Marketplace

ClawHub is a secure, decentralized marketplace for Wasm-based AI agent tools built with aerospace-grade standards (DO-178C Level A).

## Features

- 🔒 **Security First**: Ed25519 signature verification and automated security scanning
- 📦 **Tool Registry**: SQLite-based registry with full-text search
- 🗄️ **Flexible Storage**: Pluggable storage backend (local filesystem, S3-compatible)
- ✅ **Type Safe**: Strongly typed metadata and validation
- 🧪 **Well Tested**: Comprehensive test coverage
- 📖 **Well Documented**: Complete API documentation

## Architecture

```
ClawHub
├── Registry (SQLite)
│   ├── Tool metadata
│   ├── Version tracking
│   └── Full-text search
├── Storage Backend
│   ├── Local filesystem
│   └── S3-compatible (planned)
├── Security
│   ├── Ed25519 signing
│   ├── SHA-256 hashing
│   └── Automated scanning
└── API (planned)
    ├── HTTP REST API
    └── CLI tool
```

## Quick Start

### Basic Usage

```rust
use clawmaster_clawhub::registry::Registry;
use clawmaster_clawhub::types::{ToolMetadata, ToolType, SecurityStatus};
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create registry
    let registry = Registry::new("clawhub.db").await?;
    
    // Publish a tool
    let metadata = ToolMetadata {
        name: "my-tool".to_string(),
        version: "1.0.0".to_string(),
        description: "My awesome tool".to_string(),
        // ... other fields
    };
    registry.publish(metadata).await?;
    
    // Retrieve a tool
    let tool = registry.get_tool("my-tool", "1.0.0").await?;
    println!("Tool: {}", tool.name);
    
    Ok(())
}
```

### Run Example

```bash
cargo run --example basic_usage
```

## API Reference

### Registry

```rust
impl Registry {
    /// Create a new registry
    pub async fn new<P: AsRef<Path>>(database_path: P) -> Result<Self>;
    
    /// Publish a tool
    pub async fn publish(&self, metadata: ToolMetadata) -> Result<()>;
    
    /// Get tool metadata
    pub async fn get_tool(&self, name: &str, version: &str) -> Result<ToolMetadata>;
    
    /// Search for tools
    pub async fn search(&self, query: SearchQuery) -> Result<(Vec<ToolMetadata>, u64)>;
    
    /// Increment download count
    pub async fn increment_downloads(&self, name: &str, version: &str) -> Result<()>;
}
```

### Security

```rust
/// Verify Ed25519 signature
pub fn verify_signature(
    wasm_bytes: &[u8],
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<()>;

/// Compute SHA-256 hash
pub fn compute_wasm_hash(wasm_bytes: &[u8]) -> String;

/// Basic security scan
pub fn basic_security_scan(wasm_bytes: &[u8]) -> Result<()>;
```

### Storage

```rust
#[async_trait]
pub trait Storage: Send + Sync {
    async fn store(&self, name: &str, version: &str, wasm_bytes: &[u8]) -> Result<String>;
    async fn retrieve(&self, name: &str, version: &str) -> Result<Vec<u8>>;
    async fn delete(&self, name: &str, version: &str) -> Result<()>;
}

// Local filesystem storage
let storage = LocalStorage::new("./tools");
```

## Metadata Validation

### Tool Name Rules

- 3-64 characters
- Lowercase letters, numbers, hyphens only
- Must start with a letter
- Cannot end with a hyphen

```rust
use clawmaster_clawhub::metadata::validate_tool_name;

validate_tool_name("my-tool")?;  // ✅ Valid
validate_tool_name("MyTool")?;   // ❌ Invalid (uppercase)
validate_tool_name("ab")?;       // ❌ Invalid (too short)
```

### Version Rules

- Semantic versioning: MAJOR.MINOR.PATCH
- All components must be numbers

```rust
use clawmaster_clawhub::metadata::validate_version;

validate_version("1.0.0")?;   // ✅ Valid
validate_version("v1.0.0")?;  // ❌ Invalid (prefix)
validate_version("1.0")?;     // ❌ Invalid (missing patch)
```

## Database Schema

```sql
CREATE TABLE tools (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT NOT NULL,
    -- ... other fields
    UNIQUE(name, version)
);

-- Full-text search
CREATE VIRTUAL TABLE tools_fts USING fts5(
    name, description, keywords
);
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_registry_creation

# Run with output
cargo test -- --nocapture
```

## Compliance

This crate follows DO-178C Level A standards:

- ✅ §6.3.2: Exception handling - All errors properly handled
- ✅ §6.3.4: Deterministic behavior - No random behavior
- ✅ §11.10: Resource management - Proper cleanup
- ✅ §11.13: Initialization - Deterministic startup

## Roadmap

### Phase 1: Core (✅ Complete)
- [x] Registry implementation
- [x] Metadata types
- [x] Security verification
- [x] Storage backend
- [x] Database schema
- [x] Tests

### Phase 2: API (In Progress)
- [ ] HTTP REST API
- [ ] Authentication
- [ ] Search implementation
- [ ] Rate limiting

### Phase 3: CLI
- [ ] `claw publish` - Publish tools
- [ ] `claw search` - Search tools
- [ ] `claw install` - Install tools
- [ ] `claw list` - List installed tools

### Phase 4: Advanced
- [ ] S3 storage backend
- [ ] CDN integration
- [ ] Web UI
- [ ] Community features

## License

MIT

## Contributing

Contributions welcome! Please ensure:

1. All tests pass
2. Code follows DO-178C standards
3. Documentation is updated
4. No `unwrap`/`expect` in production code

## Security

For security issues, please email security@example.com

## Links

- [Documentation](https://docs.clawhub.io)
- [GitHub](https://github.com/clawmaster-org/clawmaster)
- [Issues](https://github.com/clawmaster-org/clawmaster/issues)
