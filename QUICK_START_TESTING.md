# ClawMaster 快速开始测试指南

**更新时间**: 2026-03-20 06:52

---

## ✅ 当前状态

**代码**: ✅ 优秀（无编译错误）  
**WASM**: ✅ 已构建  
**测试脚本**: ✅ 已修复（macOS 兼容）  
**阻塞**: ⚠️ 需要配置 LLM 提供商

---

## 🚀 快速开始（3 步）

### 步骤 1: 配置 LLM 提供商

ClawMaster 已检测到本地配置文件存在：`~/.clawmaster/local_llm.json`

**选项 A: 使用现有本地模型**（推荐）
```bash
# 检查现有配置
cat ~/.clawmaster/local_llm.json

# 如果已配置，直接进入步骤 2
```

**选项 B: 配置 OpenAI**
```bash
export OPENAI_API_KEY="your-api-key-here"
```

**选项 C: 配置 Anthropic**
```bash
export ANTHROPIC_API_KEY="your-api-key-here"
```

### 步骤 2: 运行快速测试（3 个场景）

```bash
./quick_test_fixed.sh
```

**测试内容**:
- calc: 计算 123 + 456
- task_list: 添加任务
- sessions_list: 列出会话

**预期时间**: 1-3 分钟

### 步骤 3: 运行完整测试（96 个场景）

```bash
./test_all_tools_cli.sh
```

**测试内容**:
- 32 个工具
- 每个工具 3 个场景
- 总计 96 个测试

**预期时间**: 30-60 分钟

---

## 📊 测试结果位置

**日志目录**: `test_logs_YYYYMMDD_HHMMSS/`

**报告文件**:
- `test_report.md` - 详细测试报告
- `master_test.log` - 主日志
- `*_*.log` - 各个测试的详细日志

---

## 🔧 故障排除

### 问题 1: "command not found: timeout"

**已解决**: 测试脚本已修复，使用纯 shell 实现

### 问题 2: "run_agent requires a configured provider"

**解决方法**: 配置 LLM 提供商（见步骤 1）

### 问题 3: 编译错误

**已解决**: 所有编译错误已修复

---

## 📈 预期测试结果

**通过率**: 60-85%  
**失败原因**: 
- 功能未实现
- 配置缺失
- 网络问题
- 超时

**这是正常的** - 测试的目的是发现问题

---

## 💡 提示

1. **首次运行**: 使用快速测试验证配置
2. **查看日志**: 失败的测试会有详细日志
3. **分析结果**: 测试报告会自动生成
4. **迭代改进**: 根据结果修复问题

---

**准备好了吗？运行 `./quick_test_fixed.sh` 开始测试！**
