# Multi-stage Dockerfile for clawmaster
# Builds a minimal debian-based image with the clawmaster gateway
#
# Moltis uses Docker/Podman for sandboxed command execution. To enable this,
# mount the container runtime socket when running:
#
#   Docker:    -v /var/run/docker.sock:/var/run/docker.sock
#   Podman:    -v /run/podman/podman.sock:/var/run/docker.sock
#   OrbStack:  -v /var/run/docker.sock:/var/run/docker.sock (same as Docker)
#
# See README.md for detailed instructions.

# Build stage — nightly required for wacore-binary (portable_simd)
FROM rust:bookworm AS builder

WORKDIR /build

# Switch to nightly (pinned for reproducibility; wacore-binary needs portable_simd)
RUN rustup install nightly-2025-11-30 && rustup default nightly-2025-11-30

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY apps/courier ./apps/courier
COPY wit ./wit

ENV DEBIAN_FRONTEND=noninteractive
# Install build dependencies for llama-cpp-sys-2
RUN apt-get update -qq && \
    apt-get install -yqq --no-install-recommends cmake build-essential libclang-dev pkg-config git && \
    rm -rf /var/lib/apt/lists/*

# Build Tailwind CSS (style.css is gitignored — must be generated before cargo build)
RUN ARCH=$(uname -m) && \
    case "$ARCH" in x86_64) TW="tailwindcss-linux-x64";; aarch64) TW="tailwindcss-linux-arm64";; esac && \
    curl -sLO "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/$TW" && \
    chmod +x "$TW" && \
    cd crates/web/ui && TAILWINDCSS="../../../$TW" ./build.sh

# Install WASM target and build WASM components (embedded via include_bytes!)
RUN rustup target add wasm32-wasip2 && \
    cargo build --target wasm32-wasip2 -p clawmaster-wasm-calc -p clawmaster-wasm-web-fetch -p clawmaster-wasm-web-search --release

# Build release binary (exclude local-llm-metal: Metal is macOS-only)
RUN cargo build --release -p clawmaster --no-default-features --features "\
agent,caldav,code-splitter,file-watcher,graphql,jemalloc,local-llm,\
mdns,metrics,openclaw-import,prometheus,push-notifications,qmd,\
tailscale,tls,trusted-network,vault,voice,wasm,web-ui,whatsapp"

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies:
# - ca-certificates: for HTTPS connections to LLM providers
# - chromium: headless browser for the browser tool (web search/fetch)
# - curl: makes it possible to run healthchecks from docker
# - sudo: allows clawmaster user to install packages at runtime (passwordless)
# - docker-ce-cli + docker-buildx-plugin: Docker CLI for sandbox execution
#   (talks to mounted socket, no daemon in-container)
# - tmux: terminal multiplexer available in deployed container
# - vim-tiny: lightweight terminal text editor
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update -qq && \
    apt-get install -yqq --no-install-recommends \
        ca-certificates \
        chromium \
        curl \
        gnupg \
        libgomp1 \
        sudo \
        tmux \
        vim-tiny && \
    install -m 0755 -d /etc/apt/keyrings && \
    curl -fsSL https://download.docker.com/linux/debian/gpg \
        | gpg --dearmor -o /etc/apt/keyrings/docker.gpg && \
    chmod a+r /etc/apt/keyrings/docker.gpg && \
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian $(. /etc/os-release && echo \"$VERSION_CODENAME\") stable" \
        > /etc/apt/sources.list.d/docker.list && \
    apt-get update -qq && \
    apt-get install -yqq --no-install-recommends \
        docker-buildx-plugin \
        docker-ce-cli && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user and add to docker group for socket access.
# Grant passwordless sudo so clawmaster can install host packages at startup.
RUN groupadd -f docker && \
    useradd --create-home --user-group clawmaster && \
    usermod -aG docker clawmaster && \
    echo "clawmaster ALL=(ALL) NOPASSWD:ALL" > /etc/sudoers.d/clawmaster

# Copy binary from builder
COPY --from=builder /build/target/release/clawmaster /usr/local/bin/clawmaster
COPY --from=builder /build/crates/web/src/assets /usr/share/clawmaster/web
COPY --from=builder /build/target/wasm32-wasip2/release/clawmaster_wasm_calc.wasm /usr/share/clawmaster/wasm/
COPY --from=builder /build/target/wasm32-wasip2/release/clawmaster_wasm_web_fetch.wasm /usr/share/clawmaster/wasm/
COPY --from=builder /build/target/wasm32-wasip2/release/clawmaster_wasm_web_search.wasm /usr/share/clawmaster/wasm/

# Create config and data directories
RUN mkdir -p /home/clawmaster/.config/clawmaster /home/clawmaster/.clawmaster && \
    chown -R clawmaster:clawmaster /home/clawmaster/.config /home/clawmaster/.clawmaster

# Volume mount points for persistence and container runtime
VOLUME ["/home/clawmaster/.config/clawmaster", "/home/clawmaster/.clawmaster", "/var/run/docker.sock"]

USER clawmaster
WORKDIR /home/clawmaster

# Expose gateway port (HTTPS), HTTP port for CA certificate download (gateway port + 1),
# and OAuth callback port (used by providers with pre-registered redirect URIs).
EXPOSE 13131 13132 1455

# Bind 0.0.0.0 so Docker port forwarding works (localhost only binds to
# the container's loopback, making the port unreachable from the host).
ENTRYPOINT ["clawmaster"]
CMD ["--bind", "0.0.0.0", "--port", "13131"]
