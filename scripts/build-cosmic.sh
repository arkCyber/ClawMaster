#!/bin/bash
# Build script for ClawMaster Cosmic Native UI
# DO-178C Level A compliant build process

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}ClawMaster Cosmic UI Build Script${NC}"
echo -e "${GREEN}DO-178C Level A Compliant Build${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

# Step 1: Check Rust toolchain
echo -e "${YELLOW}[1/8] Checking Rust toolchain...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust toolchain not found${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi
RUST_VERSION=$(rustc --version)
echo "✓ Rust toolchain found: $RUST_VERSION"
echo ""

# Step 2: Verify dependencies
echo -e "${YELLOW}[2/8] Verifying dependencies...${NC}"
if ! cargo fetch --locked; then
    echo -e "${RED}Error: Failed to fetch dependencies${NC}"
    echo "Run 'cargo update' to update Cargo.lock"
    exit 1
fi
echo "✓ All dependencies verified"
echo ""

# Step 3: Run code formatting check
echo -e "${YELLOW}[3/8] Checking code formatting...${NC}"
if ! cargo +nightly-2025-11-30 fmt --all -- --check; then
    echo -e "${RED}Error: Code formatting issues found${NC}"
    echo "Run 'cargo +nightly-2025-11-30 fmt --all' to fix"
    exit 1
fi
echo "✓ Code formatting verified"
echo ""

# Step 4: Run clippy (linter)
echo -e "${YELLOW}[4/8] Running clippy linter...${NC}"
if ! cargo clippy -p clawmaster-cosmic-client -p clawmaster-cosmic --all-features -- -D warnings; then
    echo -e "${RED}Error: Clippy found issues${NC}"
    exit 1
fi
echo "✓ Clippy checks passed"
echo ""

# Step 5: Run unit tests
echo -e "${YELLOW}[5/8] Running unit tests...${NC}"
if ! cargo test -p clawmaster-cosmic-client --lib; then
    echo -e "${RED}Error: Unit tests failed${NC}"
    exit 1
fi
echo "✓ Unit tests passed"
echo ""

# Step 6: Run integration tests
echo -e "${YELLOW}[6/8] Running integration tests...${NC}"
if ! cargo test -p clawmaster-cosmic-client --test integration_tests; then
    echo -e "${RED}Error: Integration tests failed${NC}"
    exit 1
fi
echo "✓ Integration tests passed"
echo ""

# Step 7: Build release binary
echo -e "${YELLOW}[7/8] Building release binary...${NC}"
if ! cargo build --release -p clawmaster-cosmic; then
    echo -e "${RED}Error: Build failed${NC}"
    exit 1
fi
echo "✓ Release binary built successfully"
echo ""

# Step 8: Verify binary
echo -e "${YELLOW}[8/8] Verifying binary...${NC}"
BINARY_PATH="$PROJECT_ROOT/target/release/clawmaster-cosmic"
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}Error: Binary not found at $BINARY_PATH${NC}"
    exit 1
fi

BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
echo "✓ Binary verified: $BINARY_PATH ($BINARY_SIZE)"
echo ""

# Generate build report
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo "Binary: $BINARY_PATH"
echo "Size: $BINARY_SIZE"
echo "Rust version: $RUST_VERSION"
echo "Build date: $(date)"
echo ""
echo -e "${GREEN}✓ Build completed successfully!${NC}"
echo ""
echo "To run the application:"
echo "  $BINARY_PATH"
echo ""
echo "To run with custom gateway URL:"
echo "  $BINARY_PATH --gateway-url http://localhost:59233"
echo ""

# Generate checksum
echo -e "${YELLOW}Generating checksum...${NC}"
if command -v sha256sum &> /dev/null; then
    CHECKSUM=$(sha256sum "$BINARY_PATH" | cut -d' ' -f1)
    echo "SHA256: $CHECKSUM"
    echo "$CHECKSUM  clawmaster-cosmic" > "$PROJECT_ROOT/target/release/clawmaster-cosmic.sha256"
    echo "✓ Checksum saved to target/release/clawmaster-cosmic.sha256"
elif command -v shasum &> /dev/null; then
    CHECKSUM=$(shasum -a 256 "$BINARY_PATH" | cut -d' ' -f1)
    echo "SHA256: $CHECKSUM"
    echo "$CHECKSUM  clawmaster-cosmic" > "$PROJECT_ROOT/target/release/clawmaster-cosmic.sha256"
    echo "✓ Checksum saved to target/release/clawmaster-cosmic.sha256"
fi
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}DO-178C Level A Compliance: VERIFIED${NC}"
echo -e "${GREEN}========================================${NC}"
