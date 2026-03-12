# Running ClawMaster in Docker

ClawMaster is available as a multi-architecture Docker image supporting both
`linux/amd64` and `linux/arm64`. The image is published to GitHub Container
Registry on every release.

## Quick Start

```bash
docker run -d \
  --name clawmaster \
  -p 13131:13131 \
  -p 13132:13132 \
  -p 1455:1455 \
  -v clawmaster-config:/home/clawmaster/.config/clawmaster \
  -v clawmaster-data:/home/clawmaster/.clawmaster \
  -v /var/run/docker.sock:/var/run/docker.sock \
  ghcr.io/clawmaster-org/clawmaster:latest
```

Open https://localhost:13131 in your browser and configure your LLM provider to start chatting.

### Ports

| Port | Purpose |
|------|---------|
| 13131 | Gateway (HTTPS) — web UI, API, WebSocket |
| 13132 | HTTP — CA certificate download for TLS trust |
| 1455 | OAuth callback — required for OpenAI Codex and other providers with pre-registered redirect URIs |

### Trusting the TLS certificate

ClawMaster generates a self-signed CA on first run. Browsers will show a security
warning until you trust this CA. Port 13132 serves the certificate over plain
HTTP so you can download it:

```bash
# Download the CA certificate
curl -o clawmaster-ca.pem http://localhost:13132/certs/ca.pem

# macOS — add to system Keychain and trust it
sudo security add-trusted-cert -d -r trustRoot \
  -k /Library/Keychains/System.keychain clawmaster-ca.pem

# Linux (Debian/Ubuntu)
sudo cp clawmaster-ca.pem /usr/local/share/ca-certificates/clawmaster-ca.crt
sudo update-ca-certificates
```

After trusting the CA, restart your browser. The warning will not appear again
(the CA persists in the mounted config volume).

```admonish note
When accessing from localhost, no authentication is required. If you access ClawMaster from a different machine (e.g., over the network), a setup code is printed to the container logs for authentication setup:

~~~bash
docker logs clawmaster
~~~
```

## Volume Mounts

ClawMaster uses two directories that should be persisted:

| Path | Contents |
|------|----------|
| `/home/clawmaster/.config/clawmaster` | Configuration files: `clawmaster.toml`, `credentials.json`, `mcp-servers.json` |
| `/home/clawmaster/.clawmaster` | Runtime data: databases, sessions, memory files, logs |

You can use named volumes (as shown above) or bind mounts to local directories
for easier access to configuration files:

```bash
docker run -d \
  --name clawmaster \
  -p 13131:13131 \
  -p 13132:13132 \
  -p 1455:1455 \
  -v ./config:/home/clawmaster/.config/clawmaster \
  -v ./data:/home/clawmaster/.clawmaster \
  -v /var/run/docker.sock:/var/run/docker.sock \
  ghcr.io/clawmaster-org/clawmaster:latest
```

With bind mounts, you can edit `config/clawmaster.toml` directly on the host.

## Docker Socket (Sandbox Execution)

ClawMaster runs LLM-generated shell commands inside isolated containers for
security. When ClawMaster itself runs in a container, it needs access to the host's
container runtime to create these sandbox containers.

```bash
# Recommended for full container isolation
-v /var/run/docker.sock:/var/run/docker.sock
```

**Without the socket mount**, ClawMaster automatically falls back to the
[restricted-host sandbox](sandbox.md#restricted-host-sandbox), which provides
lightweight isolation by clearing environment variables, restricting `PATH`,
and applying resource limits via `ulimit`. Commands will execute successfully
inside the ClawMaster container but without filesystem or network isolation.

For full container-level isolation (filesystem boundaries, network policies),
mount the Docker socket.

### Security Consideration

Mounting the Docker socket gives the container full access to the Docker
daemon. This is equivalent to root access on the host for practical purposes.
Only run ClawMaster containers from trusted sources (official images from
`ghcr.io/clawmaster-org/clawmaster`).

## Docker Compose

See [`examples/docker-compose.yml`](../examples/docker-compose.yml) for a
complete example:

```yaml
services:
  clawmaster:
    image: ghcr.io/clawmaster-org/clawmaster:latest
    container_name: clawmaster
    restart: unless-stopped
    ports:
      - "13131:13131"
      - "13132:13132"
      - "1455:1455"   # OAuth callback (OpenAI Codex, etc.)
    volumes:
      - ./config:/home/clawmaster/.config/clawmaster
      - ./data:/home/clawmaster/.clawmaster
      - /var/run/docker.sock:/var/run/docker.sock
```

### Coolify (Hetzner/VPS)

For Coolify service stacks, use
[`examples/docker-compose.coolify.yml`](../examples/docker-compose.coolify.yml).
It is preconfigured for reverse-proxy deployments (`--no-tls`) and includes
the Docker socket mount for sandboxed command execution.

Key points:

- Set `MOLTIS_PASSWORD` in the Coolify UI before first deploy.
- Set `SERVICE_FQDN_MOLTIS_13131` to your app domain.
- Keep ClawMaster in `--no-tls` mode behind Coolify's reverse proxy. If requests
  are redirected to `:13131`, check that TLS is disabled in ClawMaster.
- Keep `/var/run/docker.sock:/var/run/docker.sock` mounted if you want sandbox
  isolation for exec tools.

Start with:

```bash
docker compose up -d
docker compose logs -f clawmaster  # watch for startup messages
```

## Browser Sandbox in Docker

When ClawMaster runs inside Docker and launches a sandboxed browser, the browser
container is a sibling container on the host. By default, ClawMaster connects to
`127.0.0.1` which only reaches its own loopback, not the browser.

Add `container_host` to your `clawmaster.toml` so ClawMaster can reach the browser
container through the host's port mapping:

```toml
[tools.browser]
container_host = "host.docker.internal"
```

On Linux, add `--add-host` to the ClawMaster container so `host.docker.internal`
resolves to the host:

```bash
docker run -d \
  --name clawmaster \
  --add-host=host.docker.internal:host-gateway \
  -p 13131:13131 \
  -p 13132:13132 \
  -p 1455:1455 \
  -v clawmaster-config:/home/clawmaster/.config/clawmaster \
  -v clawmaster-data:/home/clawmaster/.clawmaster \
  -v /var/run/docker.sock:/var/run/docker.sock \
  ghcr.io/clawmaster-org/clawmaster:latest
```

Alternatively, use the Docker bridge gateway IP directly
(`container_host = "172.17.0.1"` on most Linux setups).

## Podman Support

ClawMaster works with Podman using its Docker-compatible API. Mount the Podman
socket instead of the Docker socket:

```bash
# Podman rootless
podman run -d \
  --name clawmaster \
  -p 13131:13131 \
  -p 13132:13132 \
  -p 1455:1455 \
  -v clawmaster-config:/home/clawmaster/.config/clawmaster \
  -v clawmaster-data:/home/clawmaster/.clawmaster \
  -v /run/user/$(id -u)/podman/podman.sock:/var/run/docker.sock \
  ghcr.io/clawmaster-org/clawmaster:latest

# Podman rootful
podman run -d \
  --name clawmaster \
  -p 13131:13131 \
  -p 13132:13132 \
  -p 1455:1455 \
  -v clawmaster-config:/home/clawmaster/.config/clawmaster \
  -v clawmaster-data:/home/clawmaster/.clawmaster \
  -v /run/podman/podman.sock:/var/run/docker.sock \
  ghcr.io/clawmaster-org/clawmaster:latest
```

You may need to enable the Podman socket service first:

```bash
# Rootless
systemctl --user enable --now podman.socket

# Rootful
sudo systemctl enable --now podman.socket
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `MOLTIS_CONFIG_DIR` | Override config directory (default: `~/.config/clawmaster`) |
| `MOLTIS_DATA_DIR` | Override data directory (default: `~/.clawmaster`) |

Example:

```bash
docker run -d \
  --name clawmaster \
  -p 13131:13131 \
  -p 13132:13132 \
  -p 1455:1455 \
  -e MOLTIS_CONFIG_DIR=/config \
  -e MOLTIS_DATA_DIR=/data \
  -v ./config:/config \
  -v ./data:/data \
  -v /var/run/docker.sock:/var/run/docker.sock \
  ghcr.io/clawmaster-org/clawmaster:latest
```

### API Keys and the `[env]` Section

Features like web search (Brave), embeddings, and LLM provider API calls read
keys from process environment variables (`std::env::var`). In Docker, there are
two ways to provide these:

**Option 1: `docker -e` flags** (takes precedence)

```bash
docker run -d \
  --name clawmaster \
  -e BRAVE_API_KEY=your-key \
  -e OPENROUTER_API_KEY=sk-or-... \
  ...
  ghcr.io/clawmaster-org/clawmaster:latest
```

**Option 2: `[env]` section in `clawmaster.toml`**

Add an `[env]` section to your config file. These variables are injected into
the ClawMaster process at startup, making them available to all features:

```toml
[env]
BRAVE_API_KEY = "your-brave-key"
OPENROUTER_API_KEY = "sk-or-..."
```

If a variable is set both via `docker -e` and `[env]`, the Docker/host
environment value wins — `[env]` never overwrites existing variables.

```admonish info title="Settings UI env vars"
Environment variables set through the Settings UI (Settings > Environment)
are stored in SQLite. At startup, ClawMaster injects them into the process
environment so they are available to all features (search, embeddings,
provider API calls), not just sandbox commands.

Precedence order (highest wins):
1. Host / `docker -e` environment variables
2. Config file `[env]` section
3. Settings UI environment variables
```

## Building Locally

To build the Docker image from source:

```bash
# Single architecture (current platform)
docker build -t clawmaster:local .

# Multi-architecture (requires buildx)
docker buildx build --platform linux/amd64,linux/arm64 -t clawmaster:local .
```

## OrbStack

OrbStack on macOS works identically to Docker — use the same socket path
(`/var/run/docker.sock`). OrbStack's lightweight Linux VM provides good
isolation with lower resource usage than Docker Desktop.

## Troubleshooting

### "Cannot connect to Docker daemon"

The Docker socket is not mounted or the ClawMaster user doesn't have permission
to access it. Verify:

```bash
docker exec clawmaster ls -la /var/run/docker.sock
```

### Setup code not appearing in logs (for network access)

The setup code only appears when accessing from a non-localhost address. If you're accessing from the same machine via `localhost`, no setup code is needed. For network access, wait a few seconds for the gateway to start, then check logs:

```bash
docker logs clawmaster 2>&1 | grep -i setup
```

### OAuth authentication error (OpenAI Codex)

If clicking **Connect** for OpenAI Codex shows "unknown_error" on OpenAI's
page, port 1455 is not reachable from your browser. Make sure you published it:

```bash
-p 1455:1455
```

If you're running ClawMaster on a remote server (cloud VM, VPS) and accessing it
over the network, `localhost:1455` on the browser side points to your local
machine — not the server. In that case, authenticate via the CLI instead:

```bash
docker exec -it clawmaster clawmaster auth login --provider openai-codex
```

The CLI opens a browser on the machine where you run the command and handles
the OAuth callback locally. If automatic callback capture fails, ClawMaster prompts
you to paste the callback URL (or `code#state`) into the terminal. Tokens are
saved to the config volume and picked up by the running gateway automatically.

### Permission denied on bind mounts

When using bind mounts, ensure the directories exist and are writable:

```bash
mkdir -p ./config ./data
chmod 755 ./config ./data
```

The container runs as user `clawmaster` (UID 1000). If you see permission errors,
you may need to adjust ownership:

```bash
sudo chown -R 1000:1000 ./config ./data
```
