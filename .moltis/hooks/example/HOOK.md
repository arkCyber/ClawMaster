+++
name = "example"
description = "Skeleton hook — edit this to build your own"
emoji = "🪝"
events = ["BeforeToolCall"]
# command = "./handler.sh"
# timeout = 10
# priority = 0

# [requires]
# os = ["darwin", "linux"]
# bins = ["jq", "curl"]
# env = ["SLACK_WEBHOOK_URL"]
+++

# Example Hook

This is a skeleton hook to help you get started. It subscribes to
`BeforeToolCall` but has no `command`, so it won't execute anything.

## Quick start

1. Uncomment the `command` line above and point it at your script
2. Create `handler.sh` (or any executable) in this directory
3. Click **Reload** in the Hooks UI (or restart moltis)

## How hooks work

Your script receives the event payload as **JSON on stdin** and communicates
its decision via **exit code** and **stdout**:

| Exit code | Stdout | Action |
|-----------|--------|--------|
| 0 | *(empty)* | **Continue** — let the action proceed |
| 0 | `{"action":"modify","data":{...}}` | **Modify** — alter the payload |
| 1 | *(stderr used as reason)* | **Block** — prevent the action |

## Example handler (bash)

```bash
#!/usr/bin/env bash
# handler.sh — log every tool call to a file
payload=$(cat)
tool=$(echo "$payload" | jq -r '.tool_name // "unknown"')
echo "$(date -Iseconds) tool=$tool" >> /tmp/moltis-hook.log
# Exit 0 with no stdout = Continue
```

## Available events

**Can modify or block (sequential dispatch):**
- `BeforeAgentStart` — before a new agent run begins
- `BeforeToolCall` — before executing a tool (inspect/modify arguments)
- `BeforeCompaction` — before compacting chat history
- `MessageSending` — before sending a message to the LLM
- `ToolResultPersist` — before persisting a tool result

**Read-only (parallel dispatch, Block/Modify ignored):**
- `AgentEnd` — after an agent run completes
- `AfterToolCall` — after a tool finishes (observe result)
- `AfterCompaction` — after compaction completes
- `MessageReceived` — after receiving an LLM response
- `MessageSent` — after a message is sent
- `SessionStart` / `SessionEnd` — session lifecycle
- `GatewayStart` / `GatewayStop` — server lifecycle

## Frontmatter reference

```toml
name = "my-hook"           # unique identifier
description = "What it does"
emoji = "🔧"               # optional, shown in UI
events = ["BeforeToolCall"] # which events to subscribe to
command = "./handler.sh"    # script to run (relative to this dir)
timeout = 10                # seconds before kill (default: 10)
priority = 0                # higher runs first (default: 0)

[requires]
os = ["darwin", "linux"]    # skip on other OSes
bins = ["jq"]               # required binaries in PATH
env = ["MY_API_KEY"]        # required environment variables
```
