#!/bin/bash
# 快速测试 CLI 到后端服务器的连接

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

GATEWAY_URL="${CLAWMASTER_GATEWAY_URL:-http://localhost:3000}"

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   ClawMaster CLI 后端连接快速测试                         ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 检查后端服务器
echo -e "${YELLOW}[1/3] 检查后端服务器...${NC}"
if curl -s -f "$GATEWAY_URL/health" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ 后端服务器正在运行: $GATEWAY_URL${NC}"
else
    echo -e "${RED}❌ 无法连接到后端服务器: $GATEWAY_URL${NC}"
    echo ""
    echo -e "${YELLOW}请先启动后端服务器:${NC}"
    echo "  clawmaster gateway"
    echo ""
    exit 1
fi
echo ""

# 检查 CLI 二进制文件
echo -e "${YELLOW}[2/3] 检查 CLI 二进制文件...${NC}"
if [ -f "./target/release/clawmaster" ]; then
    echo -e "${GREEN}✅ CLI 二进制文件存在${NC}"
else
    echo -e "${RED}❌ CLI 二进制文件不存在${NC}"
    echo ""
    echo -e "${YELLOW}请先编译:${NC}"
    echo "  cargo build --release --bin clawmaster"
    echo ""
    exit 1
fi
echo ""

# 测试 CLI 连接
echo -e "${YELLOW}[3/3] 测试 CLI 连接到后端...${NC}"
echo -e "${CYAN}发送测试消息: 计算 2 + 2${NC}"
echo ""

CLAWMASTER_GATEWAY_URL="$GATEWAY_URL" timeout 30s ./target/release/clawmaster agent --message "计算 2 + 2" 2>&1 || {
    exit_code=$?
    echo ""
    if [ $exit_code -eq 124 ]; then
        echo -e "${YELLOW}⏱️  测试超时（30秒）${NC}"
        echo ""
        echo -e "${YELLOW}可能的原因:${NC}"
        echo "  - LLM 推理时间过长"
        echo "  - 后端服务器响应缓慢"
        echo "  - 网络连接问题"
    else
        echo -e "${RED}❌ 测试失败（退出码: $exit_code）${NC}"
        echo ""
        echo -e "${YELLOW}请检查:${NC}"
        echo "  1. 后端服务器日志"
        echo "  2. LLM 配置是否正确"
        echo "  3. 网络连接是否正常"
    fi
    exit $exit_code
}

echo ""
echo -e "${GREEN}✅ CLI 连接测试成功！${NC}"
echo ""
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}所有检查通过！可以开始全面测试了。${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}运行全面测试:${NC}"
echo "  ./test_backend_cli.sh"
echo ""
