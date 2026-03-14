# ClawMaster Health Check System

**DO-178C Level A Compliant System Health Monitoring**

## 概述

健康检查系统提供全面的系统监控和诊断功能，符合 DO-178C Level A 航空航天级别标准。

## 功能特性

### 核心功能
- ✅ 组件健康检查（数据库、WebSocket、资源等）
- ✅ 资源使用监控（CPU、内存、磁盘）
- ✅ 系统就绪验证
- ✅ 健康状态聚合
- ✅ 超时保护（5秒）
- ✅ 历史记录存储

### DO-178C 合规性
- §11.10 - 运行时监控和诊断 ✅
- §6.3.3 - 故障容错 ✅
- §11.13 - 配置管理 ✅

## 使用方法

### 基本用法

```rust
use clawmaster_health_check::{
    HealthCheckService, DatabaseHealthCheck, MemoryHealthCheck,
    CpuHealthCheck, DiskHealthCheck,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 创建健康检查服务
    let mut service = HealthCheckService::new();
    
    // 注册检查
    service.register(Arc::new(MemoryHealthCheck::new()));
    service.register(Arc::new(CpuHealthCheck::new()));
    service.register(Arc::new(DiskHealthCheck::new()));
    
    // 运行健康检查
    let health = service.check_health().await;
    
    println!("System status: {:?}", health.status);
    println!("Ready: {}", health.is_ready());
}
```

### 数据库健康检查

```rust
use sqlx::SqlitePool;

// 创建数据库连接池
let pool = SqlitePool::connect("sqlite::memory:").await?;

// 注册数据库健康检查
service.register(Arc::new(DatabaseHealthCheck::new(pool)));
```

### 自定义健康检查

```rust
use async_trait::async_trait;
use clawmaster_health_check::{HealthCheck, HealthStatus, Criticality};

struct CustomCheck;

#[async_trait]
impl HealthCheck for CustomCheck {
    async fn check(&self) -> HealthStatus {
        // 实现检查逻辑
        HealthStatus::Healthy
    }
    
    fn name(&self) -> &str {
        "custom"
    }
    
    fn criticality(&self) -> Criticality {
        Criticality::Important
    }
}

service.register(Arc::new(CustomCheck));
```

## 健康状态

### HealthStatus

```rust
pub enum HealthStatus {
    Healthy,                        // 完全正常
    Degraded { reason: String },    // 功能降级
    Unhealthy { reason: String },   // 不可用
}
```

### Criticality

```rust
pub enum Criticality {
    Critical,   // 关键组件（失败导致系统失败）
    Important,  // 重要组件（失败导致功能降级）
    Optional,   // 可选组件（失败影响最小）
}
```

## 聚合规则

系统健康状态根据以下规则聚合：

1. **任何关键组件不健康** → 系统不健康
2. **任何关键组件降级** → 系统降级
3. **任何重要组件不健康** → 系统降级
4. **其他情况** → 系统健康

## 资源阈值

### 内存
- **健康**: < 80%
- **降级**: 80% - 90%
- **不健康**: > 90%

### 磁盘
- **健康**: < 90%
- **降级**: 90% - 95%
- **不健康**: > 95%

### CPU
- **健康**: < 85%
- **降级**: 85% - 95%
- **不健康**: > 95%

## HTTP 端点

### /health
返回完整的健康检查结果

```json
{
  "status": "healthy",
  "checks": [
    {
      "name": "database",
      "status": "healthy",
      "criticality": "critical",
      "timestamp": "2026-03-13T08:00:00Z",
      "duration_ms": 10
    }
  ],
  "timestamp": "2026-03-13T08:00:00Z",
  "total_duration_ms": 50
}
```

### /ready
返回系统就绪状态（简化版）

```json
{
  "ready": true
}
```

## 测试

```bash
# 运行所有测试
cargo test -p clawmaster-health-check

# 运行特定测试
cargo test -p clawmaster-health-check test_health_check_service

# 查看测试覆盖率
cargo tarpaulin -p clawmaster-health-check
```

## 性能

- **检查超时**: 5 秒
- **典型延迟**: < 100ms
- **内存占用**: < 10MB
- **CPU 影响**: 最小

## 最佳实践

### 1. 定期检查
```rust
// 每 30 秒检查一次
let mut interval = tokio::time::interval(Duration::from_secs(30));
loop {
    interval.tick().await;
    let health = service.check_health().await;
    // 处理结果
}
```

### 2. 监控告警
```rust
let health = service.check_health().await;
if health.status.is_unhealthy() {
    // 发送告警
    alert_system.send_alert(&health).await;
}
```

### 3. 优雅降级
```rust
if !service.is_ready().await {
    // 拒绝新请求
    return Err("System not ready");
}
```

## 集成示例

### Kubernetes 探针

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 3000
  initialDelaySeconds: 30
  periodSeconds: 10

readinessProbe:
  httpGet:
    path: /ready
    port: 3000
  initialDelaySeconds: 5
  periodSeconds: 5
```

### Prometheus 指标

```rust
// 导出健康检查指标
let health = service.check_health().await;
metrics::gauge!("system_health_status", 
    if health.status.is_healthy() { 1.0 } else { 0.0 }
);
```

## 故障排查

### 问题：健康检查超时
**原因**: 组件响应慢或死锁
**解决**: 检查组件日志，增加超时时间

### 问题：内存使用过高
**原因**: 内存泄漏或负载过高
**解决**: 分析内存使用，优化或扩容

### 问题：磁盘空间不足
**原因**: 日志或数据增长
**解决**: 清理旧数据，增加磁盘空间

## 架构

```
┌─────────────────────────────────────┐
│   HealthCheckService                │
├─────────────────────────────────────┤
│  - checks: Vec<HealthCheck>         │
│  - last_result: SystemHealth        │
├─────────────────────────────────────┤
│  + register(check)                  │
│  + check_health() -> SystemHealth   │
│  + is_ready() -> bool               │
└─────────────────────────────────────┘
           │
           ├─── DatabaseHealthCheck
           ├─── MemoryHealthCheck
           ├─── CpuHealthCheck
           ├─── DiskHealthCheck
           └─── CustomHealthCheck
```

## 依赖

- `tokio` - 异步运行时
- `sysinfo` - 系统信息
- `sqlx` - 数据库（可选）
- `async-trait` - 异步 trait
- `serde` - 序列化

## 许可证

MIT OR Apache-2.0

## 贡献

欢迎贡献！请确保：
1. 所有测试通过
2. 代码覆盖率 > 90%
3. 符合 DO-178C Level A 标准
4. 添加适当的文档和注释
