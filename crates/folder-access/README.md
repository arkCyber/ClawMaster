# ClawMaster Folder Access Control

航空航天级别（DO-178C Level A）文件夹访问控制系统。

## 🎯 功能特性

- **细粒度权限控制** - 读、写、执行、删除四种独立权限
- **多层安全验证** - 路径规范化、符号链接解析、攻击防护
- **完整审计追踪** - 所有访问尝试完整记录
- **动态规则管理** - 黑名单/白名单/模式匹配，支持热重载
- **DO-178C 合规** - 航空航天级别代码质量标准

## 🚀 快速开始

### 1. 添加依赖

```toml
[dependencies]
clawmaster-folder-access = { path = "../folder-access" }
```

### 2. 运行数据库迁移

```rust
use clawmaster_folder_access::run_migrations;

run_migrations(&pool).await?;
```

### 3. 创建服务实例

```rust
use clawmaster_folder_access::FolderAccessService;

let service = FolderAccessService::new(pool).await?;
```

### 4. 添加文件夹权限

```rust
use clawmaster_folder_access::PermissionFlags;

// 添加只读文件夹
service.add_folder(
    "/home/user/documents",
    PermissionFlags::read_only(),
    Some("用户文档".to_string()),
    "admin",
).await?;

// 添加读写文件夹
service.add_folder(
    "/home/user/workspace",
    PermissionFlags::read_write(),
    Some("工作区".to_string()),
    "admin",
).await?;
```

### 5. 检查访问权限

```rust
use clawmaster_folder_access::AccessOperation;

let allowed = service.check_access(
    "/home/user/documents/file.txt",
    AccessOperation::Read,
    Some("session_123".to_string()),
).await?;

if !allowed {
    return Err(anyhow!("Access denied"));
}
```

## 📊 权限模型

### PermissionFlags

```rust
pub struct PermissionFlags {
    pub can_read: bool,      // 读取文件内容
    pub can_write: bool,     // 创建和修改文件
    pub can_execute: bool,   // 执行脚本和程序
    pub can_delete: bool,    // 删除文件
}
```

### 预定义权限集

- `PermissionFlags::read_only()` - 只读访问
- `PermissionFlags::read_write()` - 读写访问
- `PermissionFlags::full()` - 完全访问

## 🔒 安全特性

### 默认黑名单规则

系统预置以下黑名单规则，防止访问敏感系统文件：

```
/etc/*          - 系统配置文件
/sys/*          - 系统内核接口
/proc/*         - 进程信息
/dev/*          - 设备文件
/root/*         - Root 用户目录
*/.ssh/*        - SSH 密钥
*/.gnupg/*      - GPG 密钥
*/id_rsa*       - SSH 私钥
*/id_ed25519*   - SSH 私钥
```

### 路径验证

所有路径经过以下验证：

1. Null 字节注入检测
2. 空路径检查
3. 路径长度限制（4096 字符）
4. 非法字符检测
5. 路径规范化（canonicalize）
6. 符号链接解析
7. 黑名单/白名单规则应用

## 🧪 测试

```bash
# 运行所有测试
cargo test -p clawmaster-folder-access

# 查看测试输出
cargo test -p clawmaster-folder-access -- --nocapture

# 运行特定测试
cargo test -p clawmaster-folder-access test_add_folder
```

## 📚 API 文档

### FolderAccessService

#### 权限管理

- `add_folder(path, permissions, description, created_by)` - 添加文件夹权限
- `remove_folder(folder_id)` - 移除文件夹权限（软删除）
- `update_permissions(folder_id, permissions)` - 更新权限
- `list_folders(include_inactive)` - 列出所有文件夹

#### 访问控制

- `check_access(file_path, operation, session_key)` - 检查访问权限

#### 审计日志

- `get_access_logs(folder_id, limit)` - 获取访问日志

#### 验证规则

- `add_validation_rule(rule_type, pattern, description, priority, created_by)` - 添加验证规则
- `reload_validation_rules()` - 重新加载验证规则

## 🎯 使用场景

### 场景 1: AI 智能体文件访问控制

```rust
// 为 AI 智能体配置工作区访问
service.add_folder(
    "/home/ai/workspace",
    PermissionFlags::read_write(),
    Some("AI 工作区".to_string()),
    "system",
).await?;

// 在文件操作前检查权限
if service.check_access(&path, AccessOperation::Write, session).await? {
    std::fs::write(&path, content)?;
}
```

### 场景 2: 多用户项目协作

```rust
// 项目目录 - 完全访问
service.add_folder(
    "/projects/team-alpha",
    PermissionFlags::full(),
    Some("团队 Alpha 项目".to_string()),
    "admin",
).await?;

// 文档目录 - 只读访问
service.add_folder(
    "/projects/docs",
    PermissionFlags::read_only(),
    Some("项目文档".to_string()),
    "admin",
).await?;
```

### 场景 3: 审计和合规

```rust
// 查看最近的访问记录
let logs = service.get_access_logs(folder_id, 100).await?;

for log in logs {
    if !log.success {
        println!("拒绝访问: {:?} - {}",
            log.operation,
            log.file_path.unwrap_or_default()
        );
    }
}
```

## 📋 DO-178C 合规性

本系统遵循 DO-178C Level A 标准：

- **§6.3.2** - 异常处理：所有错误正确处理，无 panic
- **§6.3.4** - 确定性行为：相同输入产生相同输出
- **§11.10** - 资源管理：数据库连接池，路径长度限制
- **§11.13** - 初始化：数据库迁移，无全局状态

## 🔧 配置

### 环境变量

无需额外环境变量，使用 ClawMaster 的数据库连接。

### 数据库

系统使用 SQLite 数据库，自动创建以下表：

- `folder_permissions` - 文件夹权限
- `folder_access_log` - 访问审计日志
- `folder_validation_rules` - 验证规则

## 🤝 贡献

欢迎贡献！请确保：

1. 遵循 DO-178C 标准
2. 添加完整的测试
3. 更新相关文档
4. 通过所有 CI 检查

## 📄 许可证

与 ClawMaster 主项目相同的许可证。

## 🔗 相关文档

- [完整文档](../../docs/FOLDER_ACCESS_CONTROL.md)
- [AGENTS.md](../../AGENTS.md) - 工程指南
- [DO-178C 标准](https://www.rtca.org/content/standards-guidance-materials)
