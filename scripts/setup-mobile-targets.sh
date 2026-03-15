#!/usr/bin/env bash
# Install Rust targets for cross-compiling the ClawMaster server (gateway) to iOS and Android.
# Usage: ./scripts/setup-mobile-targets.sh [ios|android|all]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
MODE="${1:-all}"

IOS_TARGETS=(
  aarch64-apple-ios        # iPhone/iPad (arm64)
  x86_64-apple-ios        # iOS Simulator (Intel Mac)
  aarch64-apple-ios-sim    # iOS Simulator (Apple Silicon)
)

ANDROID_TARGETS=(
  aarch64-linux-android   # arm64-v8a
  armv7-linux-androideabi  # armeabi-v7a
  i686-linux-android      # x86
  x86_64-linux-android    # x86_64
)

install_ios_targets() {
  echo "==> Installing Rust targets for iOS..."
  for t in "${IOS_TARGETS[@]}"; do
    rustup target add "$t"
  done
  echo ""
  echo "iOS: Build on macOS with Xcode installed. Simulator builds need a signed run;"
  echo "  device builds typically require an Apple Developer account for code signing."
}

install_android_targets() {
  echo "==> Installing Rust targets for Android..."
  for t in "${ANDROID_TARGETS[@]}"; do
    rustup target add "$t"
  done
  echo ""
  echo "Android: To link the binary you need the Android NDK. Set ANDROID_NDK_HOME"
  echo "  and optionally use cargo-ndk (cargo install cargo-ndk) or configure"
  echo "  .cargo/config.toml with the linker for each target. See docs/src/mobile-server-build.md"
}

case "$MODE" in
  ios)
    install_ios_targets
    ;;
  android)
    install_android_targets
    ;;
  all)
    install_ios_targets
    echo ""
    install_android_targets
    ;;
  *)
    echo "Usage: $0 [ios|android|all]" >&2
    exit 1
    ;;
esac

echo ""
echo "Done. Build with: just build-server-ios  (macOS only) or just build-server-android"
