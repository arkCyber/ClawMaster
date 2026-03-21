#!/bin/bash
# 启动后端服务器并运行测试

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║   ClawMaster 后端服务器启动和测试                         ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# 检查是否已有后端服务器运行
echo -e "${YELLOW}[1/4] 检查后端服务器状态...${NC}"
if curl -s -f http://localhost:3000/health > /dev/null 2>&1; then
    echo -e "${GREEN}✅ 后端服务器已经在运行${NC}"
    BACKEND_RUNNING=true
else
    echo -e "${YELLOW}⚠️  后端服务器未运行，准备启动...${NC}"
    BACKEND_RUNNING=false
fi
echo ""

# 如果没有运行，启动后端服务器
if [ "$BACKEND_RUNNING" = false ]; then
    echo -e "${YELLOW}[2/4] 启动后端服务器...${NC}"
    echo -e "${CYAN}命令: ./target/release/clawmaster gateway${NC}"
    echo ""
    
    # 在后台启动服务器
    ./target/release/clawmaster gateway > backend_server.log 2>&1 &
    BACKEND_PID=$!
    
    echo -e "${GREEN}✅ 后端服务器已启动 (PID: $BACKEND_PID)${NC}"
    echo -e "${YELLOW}等待服务器初始化...${NC}"
    
    # 等待服务器启动（最多30秒）
    for i in {1..30}; do
        if curl -s -f http://localhost:3000/health > /dev/null 2>&1; then
            echo -e "${GREEN}✅ 后端服务器已就绪！${NC}"
            break
        fi
        echo -n "."
        sleep 1
    done
    echo ""
    
    # 检查是否成功启动
    if ! curl -s -f http://localhost:3000/health > /dev/null 2>&1; then
        echo -e "${RED}❌ 后端服务器启动失败${NC}"
        echo ""
        echo -e "${YELLOW}查看日志:${NC}"
        tail -20 backend_server.log
        exit 1
    fi
else
    echo -e "${YELLOW}[2/4] 跳过启动（服务器已运行）${NC}"
fi
echo ""

# 运行快速测试
echo -e "${YELLOW}[3/4] 运行快速连接测试...${NC}"
./quick_backend_test.sh || {
    echo -e "${RED}❌ 快速测试失败${NC}"
    exit 1
}
echo ""

# 询问是否运行完整测试
echo -e "${YELLOW}[4/4] 准备运行完整测试...${NC}"
echo -e "${CYAN}完整测试将运行 15 个场景，预计需要 5-10 分钟${NC}"
echo ""
read -p "是否继续运行完整测试？(y/n) " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${GREEN}开始运行完整测试...${NC}"
    echo ""
    ./test_backend_cli.sh
else
    echo -e "${YELLOW}跳过完整测试${NC}"
    echo ""
    echo -e "${CYAN}手动运行完整测试:${NC}"
    echo "  ./test_backend_cli.sh"
fi

echo ""
echo -e "${GREEN}完成！${NC}"
