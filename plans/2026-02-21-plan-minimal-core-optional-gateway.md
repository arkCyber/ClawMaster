# Plan: Minimal Core with Optional Gateway

**Goal**: Make `clawmaster-gateway` optional so the CLI can run a headless agent loop
(or a TUI client) without compiling 50.9K lines of gateway code and its 20
workspace dependencies. Cut the default binary from 44 MB / ~150K LoC to
~20 MB / ~70K LoC for constrained devices and headless deployments.

---

## Current State

### Dependency graph (simplified)

```
clawmaster (cli) 3.0K
├── clawmaster-gateway 50.9K ← THE BOTTLENECK (20 workspace deps)
│   ├── agents, tools, config, sessions, plugins, common, skills
│   ├── browser, canvas, channels, cron, mcp, media, memory
│   ├── oauth, onboarding, projects, protocol, routing, telegram
│   └── optional: metrics, qmd, voice
├── clawmaster-agents 24.8K
├── clawmaster-tools 15.9K
│   ├── agents, browser*, cron*, common, config, sessions, skills
│   └── optional: metrics
├── clawmaster-plugins 1.8K
├── clawmaster-sessions 3.2K
├── clawmaster-memory 5.1K
├── clawmaster-config 6.1K
├── clawmaster-common 1.0K
├── clawmaster-skills 3.7K
├── clawmaster-browser 4.8K
├── clawmaster-cron 3.8K
├── clawmaster-oauth 2.1K
├── clawmaster-onboarding 0.8K
└── clawmaster-projects 1.6K
```

### What the TUI branch does

`origin/tui-interface` adds `crates/tui/` (7.7K LoC) — a **WebSocket client**
that connects to a running gateway. It does NOT replace the gateway; it replaces
the web UI as the user interface. The TUI depends on:

- `clawmaster-config` (resolve gateway URL)
- `clawmaster-protocol` (message types)
- ratatui, crossterm, tokio-tungstenite

This means the TUI still needs a gateway process running somewhere. However, it
opens the door to a split architecture: headless gateway + TUI client.

---

## Proposed Architecture

### Two binary profiles

1. **`clawmaster`** (full, default) — everything as today
2. **`clawmaster --no-default-features --features headless`** — agent loop + tools,
   no HTTP server, no web UI, no TLS stack

### Three build tiers

| Tier | Features | Est. binary | Est. LoC | Use case |
|------|----------|-------------|----------|----------|
| **Headless** | `headless` | ~15 MB | ~55K | Raspberry Pi, CI, containers, scripts |
| **TUI** | `headless,tui` | ~18 MB | ~63K | Terminal-only interactive use |
| **Full** | default (all) | ~44 MB | ~150K | Web UI, cloud deploy, channels |

---

## Implementation Plan

### Phase 1: Decouple tools from browser and cron

**Why**: `clawmaster-tools` is core, but it hard-depends on `clawmaster-browser` (4.8K)
and `clawmaster-cron` (3.8K). These are only used in `browser.rs` and
`cron_tool.rs` respectively — easy to feature-gate.

**Files**:
- `crates/tools/Cargo.toml`
- `crates/tools/src/browser.rs`
- `crates/tools/src/cron_tool.rs`
- `crates/tools/src/lib.rs` (conditional module inclusion)

**Changes**:
1. Make `clawmaster-browser` and `clawmaster-cron` optional deps in tools:
   ```toml
   [dependencies]
   clawmaster-browser = { workspace = true, optional = true }
   clawmaster-cron    = { workspace = true, optional = true }

   [features]
   default = ["browser", "cron"]
   browser = ["dep:clawmaster-browser"]
   cron    = ["dep:clawmaster-cron"]
   ```
2. Gate `mod browser` and `mod cron_tool` with `#[cfg(feature = "browser")]` /
   `#[cfg(feature = "cron")]`
3. Gate tool registration in the tool registry (likely in `lib.rs` or wherever
   tools are registered)
4. Propagate features from gateway and cli Cargo.toml

**Tests**: `cargo test -p clawmaster-tools --no-default-features` must pass.

---

### Phase 2: Make gateway optional in the CLI

**Why**: The CLI currently hard-depends on `clawmaster-gateway`. Making it optional
lets us build without the entire HTTP/WS/web-UI stack.

**Files**:
- `crates/cli/Cargo.toml`
- `crates/cli/src/main.rs`
- `crates/cli/src/lib.rs` (if exists)

**Changes**:
1. Make `clawmaster-gateway` optional:
   ```toml
   [dependencies]
   clawmaster-gateway = { workspace = true, optional = true }

   [features]
   default = ["gateway", "browser", "cron", ...]
   gateway = [
     "dep:clawmaster-gateway",
     "clawmaster-gateway/voice",
     "clawmaster-gateway/web-ui",
     "clawmaster-gateway/tls",
     ...
   ]
   headless = []  # intentionally empty — it's the absence of gateway
   ```
2. Gate the `gateway` subcommand (which is also the default command) behind
   `#[cfg(feature = "gateway")]`
3. Add a `run` or `agent` subcommand for headless agent execution:
   - Loads config, initializes providers, tools, hooks
   - Reads from stdin or accepts a prompt argument
   - Runs the agent loop once (or in a REPL)
   - Prints response to stdout
   - No HTTP server, no WebSocket, no auth middleware
4. Make the default subcommand conditional:
   - With `gateway` feature: default to `gateway` (current behavior)
   - Without `gateway` feature: default to `agent` REPL

**Tests**: `cargo build -p clawmaster --no-default-features --features headless`
must compile and produce a working binary.

---

### Phase 3: Make other gateway-only crates optional in CLI

**Why**: Several crates are only needed because the gateway needs them. Without
the gateway, the CLI doesn't need them directly.

**Crates to make optional (dep of gateway, not core)**:
- `clawmaster-onboarding` — only used by gateway for web onboarding wizard
- `clawmaster-oauth` — needed by agents (for Copilot/Codex providers) but could be
  feature-gated if those providers aren't needed
- `clawmaster-browser` — only for browser automation tool
- `clawmaster-projects` — only for project UI in gateway

**Files**: `crates/cli/Cargo.toml`

**Changes**: Move these to optional deps gated behind `gateway` feature. The
headless profile only gets: agents, tools (no browser/cron), config, sessions,
plugins, common, skills, memory.

---

### Phase 4: Add headless agent loop

**Why**: Without the gateway, we need a way to actually run the agent.

**Files**:
- `crates/cli/src/agent_cmd.rs` (new)

**Design**:
```
clawmaster agent "What is 2+2?"              # one-shot
clawmaster agent --session my-session        # resume session
echo "Fix the bug" | clawmaster agent        # pipe stdin
clawmaster agent --repl                      # interactive REPL
```

**Implementation**:
1. Load config, discover providers, register tools, discover hooks/skills
2. Create or resume a session
3. Build system prompt (reuse existing logic from gateway's chat service)
4. Call `run_agent_loop_streaming()` directly
5. Stream response to stdout (markdown or plain text)
6. Persist session

**Key concern**: The gateway currently owns a lot of orchestration logic in
`chat.rs` (`LiveChatService::run_with_tools`, `run_streaming`). The headless
agent command needs to replicate or extract the essential parts:
- Provider resolution
- Tool registry setup
- System prompt construction
- Session persistence
- Memory context injection

This is the most significant refactoring. Options:
- **A**: Extract a `ChatEngine` from gateway that both gateway and CLI use
- **B**: Duplicate the minimal path in the CLI (simpler, less clean)
- **C**: Make gateway's chat service usable without HTTP (extract from axum)

Option A is cleanest long-term. Create `clawmaster-engine` or put it in
`clawmaster-agents` as a higher-level `AgentSession` that handles the full
orchestration without any HTTP dependency.

---

### Phase 5: Integrate TUI as optional feature

**Why**: The TUI branch is nearly complete. Once gateway is optional, TUI
becomes a natural middle-ground tier.

**Files**:
- `crates/cli/Cargo.toml`
- `crates/cli/src/main.rs`
- Merge `origin/tui-interface` into main

**Changes**:
1. Add `tui` feature in cli:
   ```toml
   tui = ["dep:clawmaster-tui"]
   ```
2. Add `tui` subcommand gated behind the feature
3. The TUI still connects to a gateway — so `clawmaster tui` is a client command,
   not a replacement for `clawmaster gateway`
4. For fully headless + interactive: `clawmaster agent --repl` (no gateway needed)

---

### Phase 6: Finish TUI branch (separate effort)

Current TUI state (`origin/tui-interface`, 7.7K LoC):
- [x] App framework (ratatui + crossterm)
- [x] WebSocket connection to gateway
- [x] Chat view with markdown rendering
- [x] Input handling with tui-textarea
- [x] Session list sidebar
- [x] Status bar
- [x] Onboarding modal workflow
- [ ] Tool call rendering (thinking, tool start/end)
- [ ] Session management (new, rename, delete, branch)
- [ ] Settings / provider config
- [ ] Voice integration (mic capture → gateway)
- [ ] File upload / image display
- [ ] Reconnection handling
- [ ] Error handling polish

This is a separate workstream but becomes more valuable once the headless tier
exists — users on constrained devices can run `clawmaster gateway --headless` +
`clawmaster tui` for a lightweight full experience.

---

## Dependency Map After Changes

### Headless tier (~55K LoC)

```
clawmaster (cli)
├── clawmaster-agents      24.8K  (providers, agent loop)
├── clawmaster-tools       15.9K  (tool exec, sandbox — no browser/cron)
├── clawmaster-config       6.1K  (configuration)
├── clawmaster-memory       5.1K  (embeddings, search)
├── clawmaster-skills       3.7K  (skill loading)
├── clawmaster-sessions     3.2K  (persistence)
├── clawmaster-plugins      1.8K  (hook dispatch)
└── clawmaster-common       1.0K  (shared utils)
```

### TUI tier (~63K LoC) — adds:

```
├── clawmaster-tui          7.7K  (terminal UI client)
└── clawmaster-protocol     0.3K  (message types)
```

### Full tier (~150K LoC) — adds:

```
├── clawmaster-gateway     50.9K  (HTTP/WS server, web UI, auth)
├── clawmaster-browser      4.8K
├── clawmaster-cron         3.8K
├── clawmaster-telegram     5.7K
├── clawmaster-channels     0.7K
├── clawmaster-voice        6.0K
├── clawmaster-mcp          3.7K
├── clawmaster-oauth        2.1K
├── clawmaster-onboarding   0.8K
├── clawmaster-qmd          0.7K
├── clawmaster-routing      0.03K
├── clawmaster-projects     1.6K
├── clawmaster-media        0.4K
├── clawmaster-canvas       0.01K
├── clawmaster-auto-reply   0.1K
├── clawmaster-metrics      1.7K
└── clawmaster-protocol     0.3K
```

---

## Risks and Open Questions

1. **Chat orchestration extraction (Phase 4)** is the hardest part. The gateway's
   `LiveChatService` is tightly coupled to WebSocket broadcasting, session
   management, and the gateway's state. Extracting a reusable `ChatEngine`
   requires careful API design.

2. **Tool registry** currently registers all tools unconditionally. Headless mode
   needs conditional registration (no browser tool if no browser feature, no
   cron tool if no cron feature).

3. **Memory manager** startup is currently done in gateway's `server.rs`. For
   headless mode, the CLI agent command needs to do this itself.

4. **Hook dispatch** depends on the gateway event bus for some events
   (`GatewayStart`, `GatewayStop`). Headless mode needs equivalent lifecycle
   events or these hooks simply don't fire.

5. **MCP servers** are currently started by the gateway. If headless mode wants
   MCP tools, we need to start MCP servers outside the gateway.

6. **OAuth flows** require a callback HTTP server. Headless mode can use
   device-code flow (already supported for Codex) but not browser-based OAuth.

---

## Suggested Order of Work

| Phase | Effort | Blocks |
|-------|--------|--------|
| 1. Feature-gate browser/cron in tools | Small (1-2h) | Nothing |
| 2. Make gateway optional in CLI | Medium (4-6h) | Phase 1 |
| 3. Make gateway-only crates optional | Small (1-2h) | Phase 2 |
| 4. Add headless agent loop | Large (1-2 days) | Phases 1-3 |
| 5. Integrate TUI feature | Small (2-3h) | Phase 2 |
| 6. Finish TUI branch | Large (3-5 days) | Phase 5 |

Phases 1-3 are mechanical Cargo.toml + cfg changes. Phase 4 is the real
architecture work. Phases 5-6 can happen in parallel with phase 4.
