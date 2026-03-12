#!/usr/bin/env bash
# Verify Day 1 Wasm implementation
# 验证 Day 1 Wasm 实施成果

set -euo pipefail

echo "🔍 Verifying Day 1 Wasm Implementation"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0

# Helper functions
pass() {
    echo -e "${GREEN}✅ PASS${NC}: $1"
    ((PASSED++))
}

fail() {
    echo -e "${RED}❌ FAIL${NC}: $1"
    ((FAILED++))
}

warn() {
    echo -e "${YELLOW}⚠️  WARN${NC}: $1"
}

# Check if file exists
check_file() {
    if [ -f "$1" ]; then
        pass "File exists: $1"
        return 0
    else
        fail "File missing: $1"
        return 1
    fi
}

# Check if directory exists
check_dir() {
    if [ -d "$1" ]; then
        pass "Directory exists: $1"
        return 0
    else
        fail "Directory missing: $1"
        return 1
    fi
}

echo "📁 Checking code files..."
echo ""

# Check code files
check_file "crates/tools/src/fuel_calibrator.rs"
check_file "crates/tools/src/wasm_engine.rs"
check_file "crates/wasm-precompile/src/main.rs"
check_file "justfile"

echo ""
echo "📚 Checking documentation files..."
echo ""

# Check documentation files
check_file "WASM_SECURITY_AUDIT.md"
check_file "WASM_OPTIMIZATION_GUIDE.md"
check_file "WASM_TOOLS_AND_CLAWHUB.md"
check_file "WASM_CLAWHUB_IMPLEMENTATION.md"
check_file "WASM_IMPLEMENTATION_STATUS.md"
check_file "IMPLEMENTATION_PROGRESS.md"
check_file "PROGRESS_SUMMARY.md"
check_file "DAY1_COMPLETE.md"
check_file "WASM_AUDIT_COMPLETE.txt"
check_file "WASM_PROJECT_INDEX.md"
check_file "WASM_README.md"

echo ""
echo "🧪 Running tests..."
echo ""

# Test Fuel Calibrator
echo "Testing Fuel Calibrator..."
if cargo test -p clawmaster-tools fuel_calibrator --quiet 2>&1 | grep -q "test result: ok"; then
    pass "Fuel calibrator tests passed"
else
    fail "Fuel calibrator tests failed"
fi

# Test Wasm Engine
echo "Testing Wasm Engine..."
if cargo test -p clawmaster-tools wasm_engine --quiet 2>&1 | grep -q "test result: ok"; then
    pass "Wasm engine tests passed"
else
    fail "Wasm engine tests failed"
fi

echo ""
echo "🔧 Checking build system..."
echo ""

# Check justfile recipes
if grep -q "precompile:" justfile; then
    pass "justfile has 'precompile' recipe"
else
    fail "justfile missing 'precompile' recipe"
fi

if grep -q "clean-precompiled:" justfile; then
    pass "justfile has 'clean-precompiled' recipe"
else
    fail "justfile missing 'clean-precompiled' recipe"
fi

echo ""
echo "📊 Checking code quality..."
echo ""

# Check for unwrap/expect in fuel_calibrator.rs (should be minimal in production code)
UNWRAP_COUNT=$(grep -c "unwrap()" crates/tools/src/fuel_calibrator.rs || true)
if [ "$UNWRAP_COUNT" -eq 0 ]; then
    pass "No unwrap() in fuel_calibrator.rs production code"
else
    warn "Found $UNWRAP_COUNT unwrap() in fuel_calibrator.rs (check if in tests)"
fi

# Check for DO-178C compliance comments
if grep -q "DO-178C" crates/tools/src/fuel_calibrator.rs; then
    pass "DO-178C compliance comments found in fuel_calibrator.rs"
else
    warn "No DO-178C compliance comments in fuel_calibrator.rs"
fi

if grep -q "DO-178C" crates/tools/src/wasm_engine.rs; then
    pass "DO-178C compliance comments found in wasm_engine.rs"
else
    warn "No DO-178C compliance comments in wasm_engine.rs"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Verification Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "${GREEN}Passed: $PASSED${NC}"
echo -e "${RED}Failed: $FAILED${NC}"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All verifications passed!${NC}"
    echo ""
    echo "🎉 Day 1 implementation is complete and verified!"
    echo ""
    echo "Next steps:"
    echo "  1. Read WASM_README.md for quick start"
    echo "  2. Run 'cargo test -p clawmaster-tools' for full test suite"
    echo "  3. Run 'just wasm-tools' to build and precompile"
    echo ""
    exit 0
else
    echo -e "${RED}❌ Some verifications failed!${NC}"
    echo ""
    echo "Please check the failed items above."
    echo ""
    exit 1
fi
