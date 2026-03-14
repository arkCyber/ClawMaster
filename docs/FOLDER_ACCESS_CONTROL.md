# 航空航天级别文件夹访问控制系统

## 🎯 系统概述

基于 **DO-178C Level A** 标准实现的企业级文件夹访问控制系统，为 AI 智能体提供安全、可审计的文件系统访问管理。

## ✅ 核心功能

### 1. 细粒度权限控制
- **读取权限** (Read) - 允许读取文件内容
- **写入权限** (Write) - 允许创建和修改文件
- **执行权限** (Execute) - 允许执行脚本和程序
- **删除权限** (Delete) - 允许删除文件

### 2. 多层安全验证
- 路径规范化和符号链接解析
- 路径遍历攻击防护 (../)
- Null 字节注入检测
- 黑名单/白名单规则
- 路径长度和字符验证

### 3. 完整审计追踪
- 所有访问尝试记录
- 成功/失败状态
- 会话和上下文信息
- 时间戳和统计数据

### 4. 动态规则管理
- 热重载验证规则
- 优先级排序
- 黑名单/白名单/模式匹配

## 🔒 安全特性

### 默认黑名单规则
```
/etc/*          - 系统配置文件
/sys/*          - 系统内核接口
/proc/*         - 进程信息
/dev/*          - 设备文件
/root/*         - Root 用户目录
*/.ssh/*        - SSH 密钥
*/.gnupg/*      - GPG 密钥
*/id_rsa*       - SSH 私钥
```

### 路径验证流程
```
用户输入路径
    ↓
Null 字节检查
    ↓
空路径检查
    ↓
长度限制检查 (4096 字符)
    ↓
非法字符检查
    ↓
路径规范化 (canonicalize)
    ↓
应用黑名单/白名单规则
    ↓
返回验证结果
```

## 📊 数据库架构

### folder_permissions 表
```sql
- id: 主键
- folder_path: 文件夹路径（唯一）
- folder_hash: SHA-256 完整性哈希
- can_read/write/execute/delete: 权限标志
- description: 描述信息
- created_at/updated_at: 时间戳
- created_by: 创建者
- is_active: 激活状态
- last_accessed_at: 最后访问时间
- access_count: 访问计数
```

### folder_access_log 表
```sql
- id: 主键
- folder_id: 关联文件夹
- operation: 操作类型 (read/write/execute/delete)
- file_path: 具体文件路径
- success: 成功/失败
- session_key: 会话标识
- error_message: 错误信息
- timestamp: 时间戳
```

### folder_validation_rules 表
```sql
- id: 主键
- rule_type: 规则类型 (blacklist/whitelist/pattern)
- pattern: 匹配模式
- description: 描述
- is_active: 激活状态
- priority: 优先级
- created_at: 创建时间
- created_by: 创建者
```

## 💻 使用示例

### 添加文件夹权限
```rust
use clawmaster_folder_access::{FolderAccessService, PermissionFlags};

let service = FolderAccessService::new(pool).await?;

// 添加只读文件夹
service.add_folder(
    "/home/user/documents",
    PermissionFlags::read_only(),
    Some("用户文档目录".to_string()),
    "admin",
).await?;

// 添加读写文件夹
service.add_folder(
    "/home/user/workspace",
    PermissionFlags::read_write(),
    Some("工作区".to_string()),
    "admin",
).await?;

// 添加完全权限文件夹
service.add_folder(
    "/home/user/projects",
    PermissionFlags::full(),
    Some("项目目录".to_string()),
    "admin",
).await?;
```

### 检查访问权限
```rust
use clawmaster_folder_access::AccessOperation;

// 检查读取权限
let allowed = service.check_access(
    "/home/user/documents/file.txt",
    AccessOperation::Read,
    Some("session_123".to_string()),
).await?;

if allowed {
    // 执行文件读取操作
    let content = std::fs::read_to_string("/home/user/documents/file.txt")?;
} else {
    // 拒绝访问
    return Err(anyhow!("Access denied"));
}
```

### 更新权限
```rust
// 将只读文件夹升级为读写
service.update_permissions(
    folder_id,
    PermissionFlags::read_write(),
).await?;
```

### 查看审计日志
```rust
// 获取最近 100 条访问记录
let logs = service.get_access_logs(folder_id, 100).await?;

for log in logs {
    println!("{:?} - {} - {}",
        log.operation,
        log.file_path.unwrap_or_default(),
        if log.success { "成功" } else { "失败" }
    );
}
```

### 添加验证规则
```rust
use clawmaster_folder_access::RuleType;

// 添加黑名单规则
service.add_validation_rule(
    RuleType::Blacklist,
    "/tmp/sensitive/*".to_string(),
    Some("敏感临时文件".to_string()),
    100,
    "admin",
).await?;
```

## 🧪 测试

### 运行测试
```bash
# 运行所有测试
cargo test -p clawmaster-folder-access

# 运行特定测试
cargo test -p clawmaster-folder-access test_add_folder

# 查看测试覆盖率
cargo test -p clawmaster-folder-access -- --nocapture
```

### 测试覆盖
- ✅ 权限模型测试
- ✅ 路径验证测试
- ✅ 访问控制测试
- ✅ 审计日志测试
- ✅ 安全攻击防护测试

## 📋 DO-178C 合规性

### §6.3.2 - 异常处理
- ✅ 所有错误正确处理
- ✅ Result 类型返回
- ✅ 详细错误消息
- ✅ 无 panic/unwrap

### §6.3.4 - 确定性行为
- ✅ 相同输入产生相同输出
- ✅ 排序结果
- ✅ 明确的权限模型
- ✅ 枚举操作类型

### §11.10 - 资源管理
- ✅ 数据库连接池
- ✅ 路径长度限制
- ✅ 日志条目限制
- ✅ 访问统计

### §11.13 - 初始化
- ✅ 数据库迁移
- ✅ 默认规则加载
- ✅ 验证器初始化
- ✅ 无全局状态

## 🔧 集成指南

### 1. 添加依赖
```toml
[dependencies]
clawmaster-folder-access = { path = "../folder-access" }
```

### 2. 初始化服务
```rust
use clawmaster_folder_access::{FolderAccessService, run_migrations};

// 运行数据库迁移
run_migrations(&pool).await?;

// 创建服务实例
let folder_access = FolderAccessService::new(pool).await?;
```

### 3. 集成到文件工具
```rust
// 在文件读取前检查权限
if !folder_access.check_access(&path, AccessOperation::Read, session).await? {
    return Err(anyhow!("Access denied"));
}

// 执行文件操作
let content = std::fs::read_to_string(&path)?;
```

## 🎯 最佳实践

### 1. 最小权限原则
- 默认拒绝所有访问
- 仅授予必要的权限
- 定期审查权限设置

### 2. 审计日志管理
- 定期归档旧日志
- 监控异常访问模式
- 保留足够的审计记录

### 3. 验证规则维护
- 定期更新黑名单规则
- 测试新规则的影响
- 使用优先级管理规则冲突

### 4. 性能优化
- 使用数据库索引
- 缓存常用路径验证结果
- 批量操作时合并权限检查

## 🚀 未来增强

- [ ] 用户/角色权限系统
- [ ] 时间限制访问
- [ ] 文件配额管理
- [ ] 访问速率限制
- [ ] Web UI 管理界面
- [ ] 实时访问监控
- [ ] 权限继承机制
- [ ] 批量权限操作

## 📚 相关文档

- [DO-178C 标准](https://www.rtca.org/content/standards-guidance-materials)
- [AGENTS.md](../AGENTS.md) - 工程指南
- [安全最佳实践](./SECURITY.md)

## 🤝 贡献

欢迎提交问题和改进建议！请确保：
- 遵循 DO-178C 标准
- 添加完整的测试
- 更新相关文档
- 通过所有 CI 检查
