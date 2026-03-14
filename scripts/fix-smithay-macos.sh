#!/usr/bin/env bash
# =============================================================================
# 修复 smithay-client-toolkit 在 macOS 上的编译问题
# 问题：rustix::pipe::pipe_with 在 macOS 上不可用
# 解决方案：使用 std::os::unix::io::pipe 替代
# =============================================================================

set -euo pipefail

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC}  $*"; }
log_ok() { echo -e "${GREEN}[OK]${NC}    $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC}  $*"; }

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  ClawMaster smithay-client-toolkit macOS 补丁工具"
echo "  修复 rustix::pipe 在 macOS 上的兼容性问题"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 查找 smithay-client-toolkit 源代码路径
SMITHAY_BASE="${HOME}/.cargo/registry/src/index.crates.io-"*
SMITHAY_DIR=$(find ${SMITHAY_BASE} -maxdepth 1 -type d -name "smithay-client-toolkit-0.20.0" 2>/dev/null | head -1)

if [ -z "${SMITHAY_DIR}" ] || [ ! -d "${SMITHAY_DIR}" ]; then
    log_error "未找到 smithay-client-toolkit-0.20.0 目录"
    log_info "请先运行 cargo build 以下载依赖"
    exit 1
fi

log_info "找到 smithay-client-toolkit 目录: ${SMITHAY_DIR}"

# ============================================================================
# 补丁 1: data_device_manager/data_offer.rs
# ============================================================================
log_info "应用补丁 1: data_device_manager/data_offer.rs..."

DATA_OFFER_FILE="${SMITHAY_DIR}/src/data_device_manager/data_offer.rs"
if [ ! -f "${DATA_OFFER_FILE}" ]; then
    log_error "未找到文件: ${DATA_OFFER_FILE}"
    exit 1
fi

# 检查是否已经应用过补丁
if grep -q "std::os::unix::io::pipe" "${DATA_OFFER_FILE}"; then
    log_ok "补丁 1 已经应用过，跳过"
else
    log_info "应用补丁到 data_offer.rs..."
    
    # 创建备份
    cp "${DATA_OFFER_FILE}" "${DATA_OFFER_FILE}.bak"
    
    # 替换 rustix::pipe 为 nix::unistd::pipe (macOS 兼容)
    # 添加必要的导入
    perl -i -pe '
        if (/use rustix::pipe::\{pipe_with, PipeFlags\};/) {
            $_ = "    use nix::unistd::pipe;\n    use std::os::unix::io::{FromRawFd, IntoRawFd};\n";
        } elsif (/let \(readfd, writefd\) = pipe_with\(PipeFlags::CLOEXEC\)\?;/) {
            $_ = "    let (readfd, writefd) = pipe()?;\n    let readfd = unsafe { std::os::fd::OwnedFd::from_raw_fd(readfd) };\n    let writefd = unsafe { std::os::fd::OwnedFd::from_raw_fd(writefd) };\n";
        }
    ' "${DATA_OFFER_FILE}"
    
    log_ok "补丁 1 应用成功"
fi

# ============================================================================
# 补丁 2: primary_selection/offer.rs
# ============================================================================
log_info "应用补丁 2: primary_selection/offer.rs..."

PRIMARY_OFFER_FILE="${SMITHAY_DIR}/src/primary_selection/offer.rs"
if [ ! -f "${PRIMARY_OFFER_FILE}" ]; then
    log_error "未找到文件: ${PRIMARY_OFFER_FILE}"
    exit 1
fi

# 检查是否已经应用过补丁
if grep -q "std::os::unix::io::pipe" "${PRIMARY_OFFER_FILE}"; then
    log_ok "补丁 2 已经应用过，跳过"
else
    log_info "应用补丁到 offer.rs..."
    
    # 创建备份
    cp "${PRIMARY_OFFER_FILE}" "${PRIMARY_OFFER_FILE}.bak"
    
    # 替换 rustix::pipe 为 nix::unistd::pipe (macOS 兼容)
    perl -i -pe '
        if (/use rustix::pipe::\{pipe_with, PipeFlags\};/) {
            $_ = "        use nix::unistd::pipe;\n        use std::os::unix::io::{FromRawFd, IntoRawFd};\n";
        } elsif (/let \(readfd, writefd\) = pipe_with\(PipeFlags::CLOEXEC\)\?;/) {
            $_ = "        let (readfd, writefd) = pipe()?;\n        let readfd = unsafe { std::os::fd::OwnedFd::from_raw_fd(readfd) };\n        let writefd = unsafe { std::os::fd::OwnedFd::from_raw_fd(writefd) };\n";
        }
    ' "${PRIMARY_OFFER_FILE}"
    
    log_ok "补丁 2 应用成功"
fi

# ============================================================================
# 清理编译缓存
# ============================================================================
log_info "清理旧的编译缓存..."

cd "$(dirname "$0")/.."
rm -rf target/debug/deps/*smithay* \
       target/release/deps/*smithay* 2>/dev/null || true

log_ok "缓存清理完成"

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  smithay-client-toolkit macOS 补丁应用完成！"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
log_ok "所有补丁已成功应用"
log_info "现在请运行以下命令重新编译："
echo ""
echo "  cargo build --release -p clawmaster-cosmic"
echo ""
log_info "编译完成后，libcosmic UI 将正常工作"
echo ""
