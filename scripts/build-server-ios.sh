#!/usr/bin/env bash
# Cross-compile the ClawMaster server binary for iOS (aarch64-apple-ios).
# Run on macOS with Xcode (or command line tools) installed.
# Usage: ./scripts/build-server-ios.sh [--simulator]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$REPO_ROOT"

SIMULATOR=""
if [[ "${1:-}" == "--simulator" ]]; then
  if [[ "$(uname -m)" == "arm64" ]]; then
    TARGET="aarch64-apple-ios-sim"
  else
    TARGET="x86_64-apple-ios"
  fi
  SIMULATOR=" (simulator)"
else
  TARGET="aarch64-apple-ios"
fi

rustup target add "$TARGET" 2>/dev/null || true

echo "==> Building clawmaster for ${TARGET}${SIMULATOR}..."
echo "    Using lightweight feature set (no jemalloc on iOS)."
cargo build -p clawmaster --release --target "$TARGET" \
  --no-default-features \
  --features "agent,code-splitter,file-watcher,graphql,mdns,metrics,push-notifications,tls,trusted-network,vault,web-ui"

echo ""
echo "Binary: target/${TARGET}/release/clawmaster"
echo "Note: On a real device, running a long-lived server is restricted by iOS;"
echo "      this build is mainly for embedding or experimentation."
