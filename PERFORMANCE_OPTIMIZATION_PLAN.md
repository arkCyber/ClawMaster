# ClawMaster 性能优化计划

**创建时间**: 2026-03-21  
**目标**: 减少启动时间 40-60%，优化运行时性能

---

## 🎯 优化目标

### 启动性能
- **当前**: 未测量
- **目标**: 减少 40-60%
- **重点**: 延迟加载、并行初始化

### 运行时性能
- **目标**: 减少内存占用 20-30%
- **重点**: 缓存优化、资源复用

---

## 📊 性能分析

### 已发现的优化机会

#### 1. 延迟加载机会
- ✅ 已使用 `lazy_static` / `once_cell` 的模块：
  - `crates/providers/src/local_llm/mod.rs` (3 处)
  - `crates/web/src/templates.rs` (3 处)
  - `crates/providers/src/lib.rs` (2 处)
  - `crates/web/src/assets.rs` (2 处)

- 🔍 可以添加延迟加载的模块：
  - 工具注册表初始化
  - 技能系统加载
  - MCP 服务器连接
  - 通道插件加载

#### 2. 并行初始化机会
- 数据库迁移（可并行）
- 配置文件加载
- 提供商初始化
- 通道启动

#### 3. 缓存机会
- 配置文件解析结果
- 技能元数据
- 工具 schema
- 模板渲染结果

---

## 🚀 实施计划

### Phase 3.1: 延迟加载优化（2-3 天）

**优先级**: 🔴 高

**目标模块**:
1. **工具注册表** (`crates/agents/src/tool_registry.rs`)
   - 延迟加载内置工具
   - 按需注册 MCP 工具
   - 预期收益: 启动时间 -15%

2. **技能系统** (`crates/skills/src/`)
   - 延迟扫描技能目录
   - 按需加载技能元数据
   - 预期收益: 启动时间 -10%

3. **提供商初始化** (`crates/providers/src/`)
   - 延迟连接 LLM 提供商
   - 按需建立 WebSocket 连接
   - 预期收益: 启动时间 -10%

**实施步骤**:
```rust
// 示例：延迟加载工具注册表
use std::sync::OnceLock;

static TOOL_REGISTRY: OnceLock<ToolRegistry> = OnceLock::new();

pub fn get_tool_registry() -> &'static ToolRegistry {
    TOOL_REGISTRY.get_or_init(|| {
        // 延迟初始化
        ToolRegistry::new()
    })
}
```

---

### Phase 3.2: 并行初始化（2-3 天）

**优先级**: 🟡 中

**目标**:
1. **数据库迁移并行化**
   ```rust
   tokio::join!(
       run_projects_migrations(pool),
       run_sessions_migrations(pool),
       run_gateway_migrations(pool),
   );
   ```

2. **配置加载并行化**
   ```rust
   let (config, identity, user) = tokio::join!(
       load_config(),
       load_identity(),
       load_user_profile(),
   );
   ```

3. **通道启动并行化**
   ```rust
   let channels = vec![telegram, discord, slack];
   futures::future::join_all(
       channels.into_iter().map(|ch| ch.start())
   ).await;
   ```

**预期收益**: 启动时间 -20-30%

---

### Phase 3.3: 缓存机制（1-2 天）

**优先级**: 🟢 低

**目标**:
1. **配置缓存**
   - 缓存解析后的配置对象
   - 文件修改时自动失效

2. **技能元数据缓存**
   - 缓存 SKILL.md 解析结果
   - 减少重复文件读取

3. **工具 Schema 缓存**
   - 缓存工具参数 schema
   - 减少 JSON schema 生成开销

**实施**:
```rust
use std::sync::Arc;
use dashmap::DashMap;

pub struct SchemaCache {
    cache: Arc<DashMap<String, Value>>,
}

impl SchemaCache {
    pub fn get_or_insert<F>(&self, key: &str, f: F) -> Value
    where
        F: FnOnce() -> Value,
    {
        self.cache.entry(key.to_string())
            .or_insert_with(f)
            .clone()
    }
}
```

**预期收益**: 运行时性能 +10-15%

---

## 📈 性能测试计划

### 测试指标

1. **启动时间**
   - 冷启动（首次运行）
   - 热启动（已有缓存）
   - 各阶段耗时分布

2. **内存占用**
   - 启动时内存
   - 稳定运行内存
   - 峰值内存

3. **响应时间**
   - 工具调用延迟
   - LLM 响应延迟
   - WebSocket 消息延迟

### 测试方法

```bash
# 启动时间测试
time cargo run --release

# 内存占用测试
/usr/bin/time -l cargo run --release

# 性能分析
cargo flamegraph --bin clawmaster
```

---

## 🎯 预期收益总结

| 优化项 | 预期收益 | 优先级 | 工作量 |
|--------|----------|--------|--------|
| 延迟加载 | 启动时间 -35% | 🔴 高 | 2-3 天 |
| 并行初始化 | 启动时间 -25% | 🟡 中 | 2-3 天 |
| 缓存机制 | 运行时 +15% | 🟢 低 | 1-2 天 |
| **总计** | **启动 -40-60%** | - | **5-8 天** |

---

## 📝 实施检查清单

### Phase 3.1: 延迟加载
- [ ] 分析当前启动流程
- [ ] 识别可延迟加载的模块
- [ ] 实现工具注册表延迟加载
- [ ] 实现技能系统延迟加载
- [ ] 实现提供商延迟初始化
- [ ] 测试启动时间改善
- [ ] 确保功能正常

### Phase 3.2: 并行初始化
- [ ] 分析依赖关系
- [ ] 实现数据库迁移并行化
- [ ] 实现配置加载并行化
- [ ] 实现通道启动并行化
- [ ] 测试并发安全性
- [ ] 测试启动时间改善

### Phase 3.3: 缓存机制
- [ ] 设计缓存架构
- [ ] 实现配置缓存
- [ ] 实现技能元数据缓存
- [ ] 实现工具 Schema 缓存
- [ ] 实现缓存失效机制
- [ ] 测试缓存命中率
- [ ] 测试内存占用

---

## 🔍 监控和验证

### 性能监控
```rust
use std::time::Instant;

let start = Instant::now();
// 执行操作
let duration = start.elapsed();
tracing::info!("Operation took {:?}", duration);
```

### 基准测试
```rust
#[cfg(test)]
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn bench_tool_registry_init(c: &mut Criterion) {
        c.bench_function("tool_registry_init", |b| {
            b.iter(|| {
                black_box(ToolRegistry::new())
            });
        });
    }

    criterion_group!(benches, bench_tool_registry_init);
    criterion_main!(benches);
}
```

---

## 🎉 成功标准

### 必须达成
- ✅ 启动时间减少至少 40%
- ✅ 所有测试通过
- ✅ 功能完全正常
- ✅ 无性能回退

### 期望达成
- 🎯 启动时间减少 50-60%
- 🎯 内存占用减少 20%
- 🎯 响应时间改善 10%

---

**下一步**: 开始实施 Phase 3.1 - 延迟加载优化
