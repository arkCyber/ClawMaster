# ClawMaster Panic/Unwrap 修复计划
**日期**: 2026-03-14 23:05  
**标准**: DO-178C Level A 航空航天级别  
**目标**: 消除所有可能导致 panic 的代码

---

## 🎯 **修复策略**

### **核心原则**
1. **零 panic!()** - 所有 panic 必须替换为 Result 错误处理
2. **零 unwrap()** - 除测试代码外，禁止使用 unwrap
3. **限制 expect()** - 仅在程序初始化阶段使用
4. **完整错误处理** - 所有错误路径必须有日志和降级策略
5. **资源安全** - 使用 RAII 和 Drop trait 保证资源清理

---

## 📊 **修复优先级**

### **P0 - 立即修复（本周）**

#### **P0-1: 认证模块 (crates/auth)**
**文件**: `crates/auth/src/credential_store.rs`
**问题**: 140+ unwrap()
**影响**: 认证失败导致系统崩溃

**修复步骤**:
1. 定义 `CredentialError` 枚举
2. 所有测试代码的 unwrap() 保持不变（测试允许 panic）
3. 生产代码的 unwrap() 全部替换为 `?` 或 `map_err()`
4. 添加错误日志
5. 实现降级策略（如使用默认值）

**预计时间**: 2 小时

---

#### **P0-2: 会话管理 (crates/sessions)**
**文件**: `crates/sessions/src/metadata.rs` (172+ unwrap)
**文件**: `crates/sessions/src/store.rs` (63+ unwrap)
**影响**: 会话数据损坏导致崩溃

**修复步骤**:
1. 定义 `SessionError` 枚举
2. 修复 metadata.rs 中的 unwrap_or_default() 使用
3. 修复 store.rs 中的数据库操作
4. 添加数据验证和恢复机制
5. 实现会话降级（创建临时会话）

**预计时间**: 3 小时

---

#### **P0-3: 工具执行 (crates/tools)**
**文件**: `crates/tools/src/sandbox.rs` (115+ unwrap)
**文件**: `crates/tools/src/exec.rs` (54+ unwrap)
**影响**: 工具执行失败导致 AI 功能不可用

**修复步骤**:
1. 定义 `ToolError` 枚举
2. 修复 Docker 容器操作的错误处理
3. 修复命令执行的超时和错误处理
4. 实现工具执行重试机制
5. 添加工具执行降级（禁用沙箱）

**预计时间**: 4 小时

---

#### **P0-4: 网关服务 (crates/gateway)**
**文件**: `crates/gateway/src/server.rs` (2 panic, 31+ unwrap, 14+ expect)
**影响**: 服务器崩溃导致整个系统不可用

**修复步骤**:
1. 消除所有 panic!()
2. 定义 `ServerError` 枚举
3. 修复配置加载的错误处理
4. 实现优雅关闭机制
5. 添加健康检查和自动恢复

**预计时间**: 3 小时

---

#### **P0-5: 提供商集成 (crates/providers)**
**文件**: `crates/providers/src/openai_compat.rs` (26 panic)
**文件**: `crates/providers/src/openai.rs` (3 panic, 9 expect)
**影响**: LLM 调用失败导致 AI 对话不可用

**修复步骤**:
1. 消除所有 panic!()
2. 定义 `ProviderError` 枚举
3. 修复响应类型处理
4. 实现提供商回退机制
5. 添加 API 调用重试

**预计时间**: 3 小时

---

### **P1 - 重要修复（下周）**

#### **P1-1: 内存管理 (crates/memory)**
- `manager.rs`: 70+ unwrap
- `tools.rs`: 52+ unwrap
- **预计时间**: 2 小时

#### **P1-2: 配置加载 (crates/config)**
- `loader.rs`: 59+ expect, 37+ unwrap
- **预计时间**: 2 小时

#### **P1-3: 聊天功能 (crates/chat)**
- `lib.rs`: 6 panic, 45+ unwrap, 28+ expect
- **预计时间**: 3 小时

#### **P1-4: Discord 集成 (crates/discord)**
- `config.rs`: 18 panic
- `handler.rs`: 4 panic
- **预计时间**: 2 小时

---

## 🛠️ **错误类型设计**

### **通用错误类型模板**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Operation timeout")]
    Timeout,
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

pub type Result<T> = std::result::Result<T, ModuleError>;
```

---

## 📋 **修复检查清单**

### **每个模块修复后必须完成**:

- [ ] 定义完整的错误枚举
- [ ] 所有 panic!() 已消除
- [ ] 所有 unwrap() 已替换（测试除外）
- [ ] 所有 expect() 已审查（仅保留必要的）
- [ ] 添加错误日志（tracing::error!）
- [ ] 实现降级策略
- [ ] 编写单元测试验证错误处理
- [ ] 更新文档说明错误处理
- [ ] 代码审查通过
- [ ] 编译通过，无警告

---

## 🧪 **测试策略**

### **错误处理测试**
```rust
#[tokio::test]
async fn test_error_handling() {
    // 测试文件不存在
    let result = load_data("nonexistent.json").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ModuleError::NotFound(_)));
    
    // 测试无效数据
    let result = parse_data("invalid json").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ModuleError::Serialization(_)));
    
    // 测试降级策略
    let config = load_config_or_default().await;
    assert!(config.is_valid());
}
```

---

## 📈 **进度跟踪**

### **P0 修复进度**
| 模块 | 状态 | 进度 | 预计完成 |
|------|------|------|----------|
| P0-1: auth | 🔄 进行中 | 0% | 2h |
| P0-2: sessions | ⏳ 待开始 | 0% | 3h |
| P0-3: tools | ⏳ 待开始 | 0% | 4h |
| P0-4: gateway | ⏳ 待开始 | 0% | 3h |
| P0-5: providers | ⏳ 待开始 | 0% | 3h |
| **总计** | | **0%** | **15h** |

---

## 🎯 **成功标准**

### **修复完成的定义**:
1. ✅ 所有 P0 模块的 panic/unwrap 已修复
2. ✅ 编译通过，无警告
3. ✅ 所有单元测试通过
4. ✅ 集成测试通过
5. ✅ 代码审查通过
6. ✅ 文档更新完成
7. ✅ DO-178C Level A 审计通过

---

**开始修复 P0-1: auth 模块！** 🚀
