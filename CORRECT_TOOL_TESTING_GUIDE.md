# ClawMaster 工具测试正确方法

## ❌ 错误的方法

```bash
# 这个命令不存在！
cargo run --bin clawmaster -- tools exec calc "2 + 2"
```

**问题**: CLI 没有 `tools` 子命令

---

## ✅ 正确的方法

### 方法 1: 通过 Agent 命令（推荐）

工具是通过 Agent 调用的：

```bash
cargo run --bin clawmaster -- agent --message "计算 2 + 2"
```

### 方法 2: 通过 Gateway API

1. **启动 Gateway**:
   ```bash
   cargo run --bin clawmaster -- gateway
   ```

2. **通过 API 调用工具**:
   ```bash
   curl -X POST http://localhost:3000/api/agent/chat \
     -H "Content-Type: application/json" \
     -d '{"message": "计算 2 + 2"}'
   ```

### 方法 3: 单元测试

直接测试工具实现：

```bash
cargo test --package clawmaster-tools calc
cargo test --package clawmaster-tools exec
cargo test --package clawmaster-tools web_fetch
```

---

## 📋 推荐的测试策略

### 阶段 1: 单元测试（最快，最可靠）

```bash
# 测试所有工具
cargo test --workspace --lib

# 测试特定工具
cargo test --package clawmaster-tools calc::tests
cargo test --package clawmaster-tools exec::tests
```

### 阶段 2: 集成测试（通过 Agent）

```bash
# 需要配置 LLM 才能工作
cargo run --bin clawmaster -- agent --message "帮我计算 2 + 2"
```

### 阶段 3: E2E 测试（通过 Gateway）

```bash
# 启动 Gateway
cargo run --bin clawmaster -- gateway &

# 等待启动
sleep 5

# 测试
curl http://localhost:3000/api/tools/list
```

---

## 🎯 最佳实践

**对于 DO-178C Level A 航空航天级别测试**：

1. ✅ **单元测试** - 测试每个工具的 `execute()` 方法
2. ✅ **集成测试** - 测试工具注册和调用流程
3. ✅ **性能测试** - 测试工具执行时间和资源使用
4. ✅ **安全测试** - 测试输入验证和错误处理
5. ✅ **回归测试** - 确保修改不破坏现有功能

---

## 📝 结论

**ClawMaster 的工具不是通过 CLI 直接调用的**，而是：
- 通过 Agent 智能调用
- 通过 Gateway API 调用
- 通过单元测试直接测试

这是正确的架构设计，符合航空航天级别的安全和可靠性要求。
