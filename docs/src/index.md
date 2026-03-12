# ClawMaster

```admonish warning title="Alpha software: use with care"
Running an AI assistant on your own machine or server is still new territory. Treat ClawMaster as alpha software: run it in isolated environments, review enabled tools/providers, keep secrets scoped and rotated, and avoid exposing it publicly without strong authentication and network controls.
```

<div style="text-align: center; margin: 2em 0;">
<strong style="font-size: 1.2em;">A personal AI gateway written in Rust.<br>One binary, no runtime, no npm.</strong>
</div>

ClawMaster compiles your entire AI gateway — web UI, LLM providers, tools, and all assets — into a single self-contained executable. There's no Node.js to babysit, no `node_modules` to sync, no V8 garbage collector introducing latency spikes.

```bash
# Quick install (macOS / Linux)
curl -fsSL https://www.clawmaster.org/install.sh | sh
```

## Why ClawMaster?

| Feature | ClawMaster | Other Solutions |
|---------|--------|-----------------|
| **Deployment** | Single binary | Node.js + dependencies |
| **Memory Safety** | Rust ownership | Garbage collection |
| **Secret Handling** | Zeroed on drop | "Eventually collected" |
| **Sandbox** | Docker + Apple Container | Docker only |
| **Startup** | Milliseconds | Seconds |

## Key Features

- **Multiple LLM Providers** — Anthropic, OpenAI, Google Gemini, DeepSeek, Mistral, Groq, xAI, OpenRouter, Ollama, Local LLM, and more
- **Streaming-First** — Responses appear as tokens arrive, not after completion
- **Sandboxed Execution** — Commands run in isolated containers (Docker or Apple Container)
- **MCP Support** — Connect to Model Context Protocol servers for extended capabilities
- **Multi-Channel** — Web UI, Telegram, Discord, API access with synchronized responses
- **Built-in Throttling** — Per-IP endpoint limits with strict login protection
- **Long-Term Memory** — Embeddings-powered knowledge base with hybrid search
- **Hook System** — Observe, modify, or block actions at any lifecycle point
- **Compile-Time Safety** — Misconfigurations caught by `cargo check`, not runtime crashes

See the full list of [supported providers](providers.md).

## Quick Start

```bash
# Install
curl -fsSL https://www.clawmaster.org/install.sh | sh

# Run
clawmaster
```

On first launch:
1. Open the URL shown in your browser (e.g., `http://localhost:13131`)
2. Add your LLM API key
3. Start chatting!

```admonish note
Authentication is only required when accessing ClawMaster from a non-localhost address. On localhost, you can start using it immediately.
```

→ [Full Quickstart Guide](quickstart.md)

## How It Works

```
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│  Web UI  │  │ Telegram │  │ Discord  │  │   API    │
└────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘
     │             │             │             │
     └─────────────┴─────────┬───┴─────────────┘
                             │
                             ▼
        ┌───────────────────────────────┐
        │       ClawMaster Gateway          │
        │   ┌─────────┐ ┌───────────┐   │
        │   │  Agent  │ │   Tools   │   │
        │   │  Loop   │◄┤  Registry │   │
        │   └────┬────┘ └───────────┘   │
        │        │                      │
        │   ┌────▼────────────────┐     │
        │   │  Provider Registry  │     │
        │   │ Anthropic·OpenAI·Gemini… │   │
        │   └─────────────────────┘     │
        └───────────────────────────────┘
                        │
                ┌───────▼───────┐
                │    Sandbox    │
                │ Docker/Apple  │
                └───────────────┘
```

## Documentation

### Getting Started
- **[Quickstart](quickstart.md)** — Up and running in 5 minutes
- **[Installation](installation.md)** — All installation methods
- **[Configuration](configuration.md)** — `clawmaster.toml` reference
- **[End-to-End Testing](e2e-testing.md)** — Browser regression coverage for the web UI

### Features
- **[Providers](providers.md)** — Configure LLM providers
- **[MCP Servers](mcp.md)** — Extend with Model Context Protocol
- **[Hooks](hooks.md)** — Lifecycle hooks for customization
- **[Local LLMs](local-llm.md)** — Run models on your machine

### Deployment
- **[Docker](docker.md)** — Container deployment

### Architecture
- **[Streaming](streaming.md)** — How real-time streaming works
- **[Metrics & Tracing](metrics-and-tracing.md)** — Observability

## Security

ClawMaster applies defense in depth:

- **Authentication** — Password or passkey (WebAuthn) required for non-localhost access
- **SSRF Protection** — Blocks requests to internal networks
- **Secret Handling** — `secrecy::Secret` zeroes memory on drop
- **Sandboxed Execution** — Commands never run on the host
- **Origin Validation** — Prevents Cross-Site WebSocket Hijacking
- **No Unsafe Code** — `unsafe` is denied workspace-wide

## Community

- **GitHub**: [github.com/clawmaster-org/clawmaster](https://github.com/clawmaster-org/clawmaster)
- **Issues**: [Report bugs](https://github.com/clawmaster-org/clawmaster/issues)
- **Discussions**: [Ask questions](https://github.com/clawmaster-org/clawmaster/discussions)

## License

MIT — Free for personal and commercial use.
