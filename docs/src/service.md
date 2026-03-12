# Service Management

ClawMaster can be installed as an OS service so it starts automatically on boot
and restarts after crashes.

## Install

```bash
clawmaster service install
```

This creates a service definition and starts it immediately:

| Platform | Service file | Init system |
|----------|-------------|-------------|
| macOS | `~/Library/LaunchAgents/org.clawmaster.gateway.plist` | launchd (user agent) |
| Linux | `~/.config/systemd/user/clawmaster.service` | systemd (user unit) |

Both configurations:

- **Start on boot** (`RunAtLoad` / `WantedBy=default.target`)
- **Restart on failure** with a 10-second cooldown
- **Log to** `~/.clawmaster/clawmaster.log`

### Options

You can pass `--bind`, `--port`, and `--log-level` to bake them into the
service definition:

```bash
clawmaster service install --bind 0.0.0.0 --port 8080 --log-level debug
```

These flags are written into the service file. The service reads the rest of
its configuration from `~/.clawmaster/clawmaster.toml` as usual.

## Manage

```bash
clawmaster service status     # Show running/stopped/not-installed and PID
clawmaster service stop       # Stop the service
clawmaster service restart    # Restart the service
clawmaster service logs       # Print the log file path
```

To tail the logs:

```bash
tail -f $(clawmaster service logs)
```

## Uninstall

```bash
clawmaster service uninstall
```

This stops the service, removes the service file, and cleans up.

## CLI Reference

| Command | Description |
|---------|-------------|
| `clawmaster service install` | Install and start the service |
| `clawmaster service uninstall` | Stop and remove the service |
| `clawmaster service status` | Show service status and PID |
| `clawmaster service stop` | Stop the service |
| `clawmaster service restart` | Restart the service |
| `clawmaster service logs` | Print log file path |

## How It Differs from `clawmaster node add`

`clawmaster service install` manages the **gateway** — the main ClawMaster server
that hosts the web UI, chat sessions, and API.

`clawmaster node add` registers a **headless node** — a client process on a
remote machine that connects back to a gateway for command execution. See
[Multi-Node](nodes.md) for details.

| | `clawmaster service` | `clawmaster node` |
|---|---|---|
| What it runs | The gateway server | A node client |
| Needs `--host`/`--token` | No | Yes |
| Config source | `~/.clawmaster/clawmaster.toml` | `~/.clawmaster/node.json` |
| launchd label | `org.clawmaster.gateway` | `org.clawmaster.node` |
| systemd unit | `clawmaster.service` | `clawmaster-node.service` |
