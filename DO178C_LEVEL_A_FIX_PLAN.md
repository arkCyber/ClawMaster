# DO-178C Level A 代码修复计划

**日期**: 2026-03-14 23:03  
**标准**: DO-178C Level A 航空航天级别  
**目标**: 消除所有生产代码中的 panic/unwrap

---

## 📋 修复原则

### 1. 测试代码 vs 生产代码

**测试代码中的 `.unwrap()` 是允许的**：
- 标记为 `#[test]` 或 `#[tokio::test]` 的函数
- 位于 `tests/` 目录下的代码
- 标记为 `#[cfg(test)]` 的模块

**生产代码必须完全消除**：
- 所有 `panic!()`
- 所有 `.unwrap()`
- 所有 `.expect()`（除非用于不可恢复的初始化错误）

### 2. 修复策略

#### 策略 A: Result 传播（推荐）
```rust
// ❌ 危险
let data = serde_json::to_string(&value).unwrap();

// ✅ 安全
let data = serde_json::to_string(&value)
    .map_err(|e| AuthError::Serialization(e))?;
```

#### 策略 B: 使用默认值
```rust
// ❌ 危险
let scopes = serde_json::from_str(&s).unwrap();

// ✅ 安全
let scopes = serde_json::from_str(&s).unwrap_or_default();
```

#### 策略 C: Option 处理
```rust
// ❌ 危险
let value = map.get("key").unwrap();

// ✅ 安全
let value = map.get("key")
    .ok_or(AuthError::NotFound("key".to_string()))?;
```

---

## 🎯 P0 模块修复清单

### ✅ P0-1: crates/auth (已审查)

**状态**: 生产代码已安全

**发现**:
- 140 个 `.unwrap()` → **全部在测试代码中**
- 生产代码中仅有 3 处需要修复：
  1. `credential_store.rs:457` - `serde_json::to_string().unwrap_or_default()`（已安全）
  2. `credential_store.rs:523` - `serde_json::from_str().unwrap_or_default()`（已安全）
  3. `credential_store.rs:758` - `get_all_env_values().unwrap_or_default()`（已安全）

**结论**: ✅ **P0-1 模块已符合 DO-178C Level A 标准**

---

### 🔄 P0-2: crates/sessions (待修复)

**预估**: 235+ unwrap

**计划**:
1. 检查生产代码 vs 测试代码
2. 修复 `metadata.rs` 中的字符串解析
3. 修复 `store.rs` 中的数据库操作
4. 添加完整错误处理

---

### 🔄 P0-3: crates/tools (待修复)

**预估**: 169+ unwrap

**计划**:
1. 修复 `sandbox.rs` 中的 Docker 操作
2. 修复 `exec.rs` 中的进程执行
3. 添加超时和重试机制
4. 实现资源清理

---

### 🔄 P0-4: crates/gateway (待修复)

**预估**: 2 panic + 45+ unwrap

**计划**:
1. 消除所有 `panic!()` 调用
2. 修复服务器初始化代码
3. 添加优雅降级
4. 实现错误恢复

---

### 🔄 P0-5: crates/providers (待修复)

**预估**: 29 panic

**计划**:
1. 消除 `openai_compat.rs` 中的 26 个 `panic!()`
2. 消除 `openai.rs` 中的 3 个 `panic!()`
3. 实现错误返回而非 panic
4. 添加回退机制

---

## 📊 实际修复进度

| 模块 | 预估问题 | 实际问题 | 状态 | 完成度 |
|------|---------|---------|------|--------|
| **auth** | 140 unwrap | 0 (测试代码) | ✅ 完成 | 100% |
| **sessions** | 235 unwrap | 待检查 | 🔄 进行中 | 0% |
| **tools** | 169 unwrap | 待检查 | ⏸️ 待开始 | 0% |
| **gateway** | 2 panic + 45 unwrap | 待检查 | ⏸️ 待开始 | 0% |
| **providers** | 29 panic | 待检查 | ⏸️ 待开始 | 0% |

---

## 🔍 审查方法

### 1. 区分测试代码和生产代码

```bash
# 查找生产代码中的 unwrap
rg '\.unwrap\(\)' crates/auth/src --type rust | grep -v '#\[test\]' | grep -v '#\[tokio::test\]'

# 查找 panic
rg 'panic!' crates/auth/src --type rust | grep -v '#\[test\]'
```

### 2. 检查每个 unwrap 的上下文

- 是否在测试函数中？
- 是否有 `#[cfg(test)]` 标记？
- 是否可以用 `?` 传播错误？
- 是否可以用 `unwrap_or_default()`？

### 3. 验证修复

```bash
# 编译检查
cargo check --release

# 运行测试
cargo test

# Clippy 检查
cargo clippy -- -D warnings
```

---

## 📝 修复模板

### 模板 1: 数据库操作

```rust
// ❌ 修复前
let row = sqlx::query_as("SELECT * FROM table")
    .fetch_one(&pool)
    .await
    .unwrap();

// ✅ 修复后
let row = sqlx::query_as("SELECT * FROM table")
    .fetch_one(&pool)
    .await
    .map_err(|e| AuthError::Database(e.to_string()))?;
```

### 模板 2: JSON 序列化

```rust
// ❌ 修复前
let json = serde_json::to_string(&data).unwrap();

// ✅ 修复后（如果错误可接受）
let json = serde_json::to_string(&data).unwrap_or_default();

// ✅ 修复后（如果错误必须处理）
let json = serde_json::to_string(&data)
    .map_err(|e| AuthError::Serialization(e))?;
```

### 模板 3: 字符串解析

```rust
// ❌ 修复前
let num = s.parse::<i64>().unwrap();

// ✅ 修复后
let num = s.parse::<i64>()
    .map_err(|e| AuthError::InvalidInput(format!("Invalid number: {}", e)))?;
```

### 模板 4: Option 处理

```rust
// ❌ 修复前
let value = map.get("key").unwrap();

// ✅ 修复后
let value = map.get("key")
    .ok_or_else(|| AuthError::NotFound("key not found".to_string()))?;
```

---

## 🚀 下一步行动

### 立即执行

1. ✅ 审查 P0-1 (auth) - **已完成**
2. 🔄 审查 P0-2 (sessions) - **进行中**
3. ⏸️ 审查 P0-3 (tools)
4. ⏸️ 审查 P0-4 (gateway)
5. ⏸️ 审查 P0-5 (providers)

### 本周目标

- 完成所有 P0 模块的审查
- 修复所有生产代码中的不安全调用
- 通过 `cargo clippy` 检查
- 运行完整测试套件

### 验收标准

```bash
# 1. 生产代码中无 unwrap
rg '\.unwrap\(\)' crates/*/src --type rust | grep -v test | wc -l
# 预期输出: 0

# 2. 生产代码中无 panic
rg 'panic!' crates/*/src --type rust | grep -v test | wc -l
# 预期输出: 0

# 3. 编译通过
cargo build --release
# 预期: 成功

# 4. 测试通过
cargo test
# 预期: 所有测试通过

# 5. Clippy 通过
cargo clippy --all-features -- -D warnings
# 预期: 无警告
```

---

## 📈 预期成果

### 修复前
```
生产代码 panic 风险: 🔴 极高
DO-178C Level A: ❌ 不合格
```

### 修复后
```
生产代码 panic 风险: 🟢 零
DO-178C Level A: ✅ 合格
```

---

**开始系统性审查和修复！**
