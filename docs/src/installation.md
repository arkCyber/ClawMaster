# Installation

ClawMaster is distributed as a single self-contained binary. Choose the installation method that works best for your setup.

## Quick Install (Recommended)

The fastest way to get started on macOS or Linux:

```bash
curl -fsSL https://www.clawmaster.org/install.sh | sh
```

This downloads the latest release for your platform and installs it to `~/.local/bin`.

## Package Managers

### Homebrew (macOS / Linux)

```bash
brew install clawmaster-org/tap/clawmaster
```

## Linux Packages

### Debian / Ubuntu (.deb)

```bash
# Download the latest .deb package
curl -LO https://github.com/clawmaster-org/clawmaster/releases/latest/download/clawmaster_amd64.deb

# Install
sudo dpkg -i clawmaster_amd64.deb
```

### Fedora / RHEL (.rpm)

```bash
# Download the latest .rpm package
curl -LO https://github.com/clawmaster-org/clawmaster/releases/latest/download/clawmaster.x86_64.rpm

# Install
sudo rpm -i clawmaster.x86_64.rpm
```

### Arch Linux (.pkg.tar.zst)

```bash
# Download the latest package
curl -LO https://github.com/clawmaster-org/clawmaster/releases/latest/download/clawmaster.pkg.tar.zst

# Install
sudo pacman -U clawmaster.pkg.tar.zst
```

### Snap

```bash
sudo snap install clawmaster
```

### AppImage

```bash
# Download
curl -LO https://github.com/clawmaster-org/clawmaster/releases/latest/download/clawmaster.AppImage
chmod +x clawmaster.AppImage

# Run
./clawmaster.AppImage
```

## Docker

Multi-architecture images (amd64/arm64) are published to GitHub Container Registry:

```bash
docker pull ghcr.io/clawmaster-org/clawmaster:latest
```

See [Docker Deployment](docker.md) for full instructions on running ClawMaster in a container.

## Build from Source

### Prerequisites

- Rust 1.91 or later
- A C compiler (for some dependencies)
- [just](https://github.com/casey/just) (command runner)
- Node.js (for building Tailwind CSS)

### Clone and Build

```bash
git clone https://github.com/clawmaster-org/clawmaster.git
cd clawmaster
just build-css           # Build Tailwind CSS for the web UI
just build-release       # Build in release mode
```

For a full release build including WASM sandbox tools:

```bash
just build-release-with-wasm
```

The binary will be at `target/release/clawmaster`.

### Install via Cargo

```bash
cargo install clawmaster --git https://github.com/clawmaster-org/clawmaster
```

## First Run

After installation, start ClawMaster:

```bash
clawmaster
```

On first launch:

1. Open `http://localhost:<port>` in your browser (the port is shown in the terminal output)
2. Configure your LLM provider (API key)
3. Start chatting!

```admonish tip
ClawMaster picks a random available port on first install to avoid conflicts. The port is saved in your config and reused on subsequent runs.
```

```admonish note
Authentication is only required when accessing ClawMaster from a non-localhost address (e.g., over the network). When this happens, a one-time setup code is printed to the terminal for initial authentication setup.
```

## Verify Installation

```bash
clawmaster --version
```

## Updating

### Homebrew

```bash
brew upgrade clawmaster
```

### From Source

```bash
cd clawmaster
git pull
just build-css
just build-release
```

## Uninstalling

### Homebrew

```bash
brew uninstall clawmaster
```

### Remove Data

ClawMaster stores data in two directories:

```bash
# Configuration
rm -rf ~/.config/clawmaster

# Data (sessions, databases, memory)
rm -rf ~/.clawmaster
```

```admonish warning
Removing these directories deletes all your conversations, memory, and settings permanently.
```
