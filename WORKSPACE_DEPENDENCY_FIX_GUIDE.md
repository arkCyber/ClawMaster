# ClawMaster Workspace 依赖修复指南

**问题**: 缺少 3 个 workspace 依赖导致编译失败  
**影响**: 无法编译和测试新实现的 5 个文件系统工具  
**解决方案**: 添加缺失的依赖到 workspace Cargo.toml

---

## 🔧 快速修复

### 步骤 1: 编辑 `/Users/arksong/ClawMaster/Cargo.toml`

在 `[workspace.dependencies]` 部分（约第 160-330 行）添加以下依赖：

```toml
# 在现有依赖中按字母顺序添加
glob = "0.3"                    # 文件 glob 模式匹配
notify-debouncer-full = "0.4"   # 文件系统监控
portable-pty = "0.8"            # 跨平台 PTY
```

### 步骤 2: 验证修复

```bash
cd /Users/arksong/ClawMaster
cargo build --package clawmaster-tools
```

应该看到：
```
   Compiling clawmaster-tools v0.10.18
   Finished `dev` profile [unoptimized + debuginfo] target(s) in X.XXs
```

---

## 📋 详细修复说明

### 缺失的依赖

| 依赖 | 版本 | 用途 | 需要的 crate |
|------|------|------|-------------|
| `glob` | 0.3 | Glob 模式文件搜索 | clawmaster-tools (search_files.rs) |
| `notify-debouncer-full` | 0.4 | 文件系统变化监控 | clawmaster-skills |
| `portable-pty` | 0.8 | 跨平台伪终端 | clawmaster-gateway |

### 在 Cargo.toml 中的位置

找到 `[workspace.dependencies]` 部分，在合适的位置（按字母顺序）插入：

```toml
[workspace.dependencies]
# ... 现有依赖 ...
genai        = "0.5"
glob         = "0.3"              # ← 添加这行
google-cloud-auth = "0.18"
# ... 更多依赖 ...
native-tls         = "0.2"
notify-debouncer-full = "0.4"     # ← 添加这行
open               = "5.3"
# ... 更多依赖 ...
portable-pty = "0.8"              # ← 添加这行（在 p 开头的依赖附近）
# ... 更多依赖 ...
```

---

## ✅ 验证步骤

### 1. 编译验证

```bash
cargo build --package clawmaster-tools
```

预期输出：
```
   Compiling clawmaster-tools v0.10.18
   Finished `dev` profile
```

### 2. 运行文件系统工具测试

```bash
# 测试 ReadFileTool
cargo test --package clawmaster-tools --lib read_file::tests

# 测试 WriteFileTool
cargo test --package clawmaster-tools --lib write_file::tests

# 测试 ListDirectoryTool
cargo test --package clawmaster-tools --lib list_directory::tests

# 测试 SearchFilesTool
cargo test --package clawmaster-tools --lib search_files::tests

# 测试 GrepTool
cargo test --package clawmaster-tools --lib grep_tool::tests
```

预期输出（每个工具）：
```
running X tests
test ... ok
test ... ok
...
test result: ok. X passed; 0 failed; 0 ignored
```

### 3. 运行所有工具测试

```bash
cargo test --package clawmaster-tools
```

预期：
- ✅ 26 个新测试通过（文件系统工具）
- ✅ 577+ 个现有测试通过
- ✅ 总通过率 100%

---

## 🚀 修复后的下一步

### 1. 注册新工具到工具注册表

编辑 `crates/gateway/src/tool_registry_setup.rs` 或相关文件，添加：

```rust
use clawmaster_tools::{
    read_file::{ReadFileTool, ReadFileConfig},
    write_file::{WriteFileTool, WriteFileConfig},
    list_directory::{ListDirectoryTool, ListDirectoryConfig},
    search_files::{SearchFilesTool, SearchFilesConfig},
    grep_tool::{GrepTool, GrepConfig},
};

// 在工具注册函数中添加
registry.register_builtin(
    "read_file",
    Arc::new(ReadFileTool::new(ReadFileConfig::default())),
)?;

registry.register_builtin(
    "write_file",
    Arc::new(WriteFileTool::new(WriteFileConfig::default())),
)?;

registry.register_builtin(
    "list_directory",
    Arc::new(ListDirectoryTool::new(ListDirectoryConfig::default())),
)?;

registry.register_builtin(
    "search_files",
    Arc::new(SearchFilesTool::new(SearchFilesConfig::default())),
)?;

registry.register_builtin(
    "grep",
    Arc::new(GrepTool::new(GrepConfig::default())),
)?;
```

### 2. 运行完整测试套件

```bash
cargo test --workspace
```

### 3. 生成 WASM 组件（可选）

```bash
# 为文件系统工具生成 WASM 组件
cargo build --target wasm32-wasip1 --package clawmaster-tools --features wasm
```

---

## 📊 预期结果

修复完成后，ClawMaster 将拥有：

- ✅ **31 个工具**（26 个现有 + 5 个新增）
- ✅ **603+ 个测试**（577 个现有 + 26 个新增）
- ✅ **100% 测试通过率**
- ✅ **完整的文件系统操作能力**
- ✅ **与 OpenClaw 功能对等**

---

## ⚠️ 常见问题

### Q: 编译时出现 "duplicate key" 错误

A: 检查是否有重复的依赖定义。每个依赖在 `[workspace.dependencies]` 中只能出现一次。

### Q: 测试失败

A: 确保：
1. 所有依赖都已正确添加
2. 运行 `cargo clean` 清理缓存
3. 重新编译 `cargo build --package clawmaster-tools`

### Q: WASM 编译失败

A: 确保安装了 WASM 目标：
```bash
rustup target add wasm32-wasip1
```

---

**修复指南生成时间**: 2026-03-21 12:10  
**预计修复时间**: 5 分钟  
**影响范围**: 5 个新工具，26 个新测试
