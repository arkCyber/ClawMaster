# Multi-Node

ClawMaster can distribute work across multiple machines. A **node** is a remote
device that connects to your gateway and executes commands on your behalf.
This lets the AI agent run shell commands on a Linux server, query a Raspberry
Pi, or leverage a GPU machine — all from a single chat session.

## How It Works

```
┌──────────────┐    WebSocket     ┌─────────────────┐
│  Your laptop │◄────────────────►│  ClawMaster gateway  │
│  (browser)   │                  │  (clawmaster)        │
└──────────────┘                  └────────┬─────────┘
                                           │ WebSocket
                                  ┌────────▼─────────┐
                                  │  Remote machine   │
                                  │  (clawmaster node)    │
                                  └──────────────────┘
```

1. The gateway runs on your primary machine (or a server).
2. On the remote machine, run `clawmaster node add` to register it with the gateway.
3. The gateway authenticates the node using a **device token** from the pairing flow.
4. Once connected, the agent can execute commands on the node, query its
   telemetry, and discover its LLM providers.

Nodes are **stateless from the gateway's perspective** — they connect and
disconnect freely. There is no start/stop lifecycle managed by the gateway;
a node is available when its process is running and connected.

## Pairing a Node

Before a node can connect, it must be paired with the gateway.

1. Open the **Nodes** page in the web UI (Settings → Nodes).
2. Click **Generate Token** to create a device token.
3. Copy the connection command shown in the UI.
4. Run it on the remote machine.

The pairing flow produces a device token that authenticates the node on every
connection. Tokens can be revoked from the Nodes page at any time.

## Adding a Node

On the remote machine, register it as a node:

```bash
clawmaster node add --host ws://your-gateway:9090/ws --token <device-token> --name "Build Server"
```

This saves the connection parameters to `~/.clawmaster/node.json` and installs an
OS service that starts on boot and reconnects on failure:

| Platform | Service file |
|----------|-------------|
| macOS | `~/Library/LaunchAgents/org.clawmaster.node.plist` |
| Linux | `~/.config/systemd/user/clawmaster-node.service` |

Options:

| Flag | Description | Default |
|------|-------------|---------|
| `--host` | Gateway WebSocket URL | (required) |
| `--token` | Device token from pairing | (required) |
| `--name` | Display name shown in the UI | none |
| `--node-id` | Custom node identifier | random UUID |
| `--working-dir` | Working directory for commands | `$HOME` |
| `--timeout` | Max command timeout in seconds | `300` |
| `--foreground` | Run in the terminal instead of installing a service | off |

You can also set `MOLTIS_GATEWAY_URL` and `MOLTIS_DEVICE_TOKEN` as
environment variables instead of passing `--host` and `--token`.

### Foreground mode

For debugging or one-off use, pass `--foreground` to run the node in the
current terminal session instead of installing a service:

```bash
clawmaster node add --host ws://your-gateway:9090/ws --token <device-token> --foreground
```

Press `Ctrl+C` to disconnect.

## Removing a Node

To disconnect this machine and remove the background service:

```bash
clawmaster node remove
```

This stops the service, removes the service file, and deletes the saved
configuration from `~/.clawmaster/node.json`.

## Checking Status

```bash
clawmaster node status
```

Shows the gateway URL, display name, and whether the background service is
running.

## Logs

```bash
clawmaster node logs
# Tail the log:
tail -f $(clawmaster node logs)
```

## Selecting a Node in Chat

Once a node is connected, you can target it from a chat session:

- **UI dropdown**: The chat toolbar shows a node selector next to the model
  picker. Select a node to route all `exec` commands to it. Select "Local" to
  revert to local execution.
- **Agent tools**: The agent can call `nodes_list`, `nodes_describe`, and
  `nodes_select` to programmatically pick a node based on capabilities or
  telemetry.

The node assignment is per-session and persists across page reloads.

## Node Telemetry

Connected nodes report system telemetry every 30 seconds:

- CPU count and usage
- Memory total and available
- Disk total and available (root partition)
- System uptime
- Installed runtimes (Python, Node.js, Ruby, Go, Rust, Java)
- Available LLM providers (Ollama models, API key presence)

This data is visible on the Nodes page and available to the agent via the
`nodes_describe` tool.

## CLI Reference

| Command | Description |
|---------|-------------|
| `clawmaster node generate-token` | Generate a device token and print the `add` command |
| `clawmaster node list` | List all connected nodes |
| `clawmaster node add --host <url> --token <tok>` | Join this machine to a gateway as a node |
| `clawmaster node add ... --foreground` | Run in the terminal instead of installing a service |
| `clawmaster node remove` | Disconnect this machine and remove the service |
| `clawmaster node status` | Show connection info and service status |
| `clawmaster node logs` | Print log file path |

## Security

- **Device tokens** are SHA-256 hashed before storage. The raw token is shown
  once during pairing and never stored on the gateway.
- **Environment filtering**: When the gateway forwards commands to a node, only
  safe environment variables are forwarded (`TERM`, `LANG`, `LC_*`). Secrets
  like API keys, `DYLD_*`, and `LD_PRELOAD` are always blocked.
- **Token revocation**: Revoke a device token from the Nodes page at any time.
  The node will be disconnected on its next reconnect attempt.
