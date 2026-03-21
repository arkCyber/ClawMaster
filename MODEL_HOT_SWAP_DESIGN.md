# 🔥 模型热切换设计方案

**目标**: 实现运行时动态切换模型，无需重启系统  
**时间**: 2026年3月17日 22:28  

---

## 🔍 技术分析

### 当前问题
```rust
// LlamaBackend::init() 只能调用一次
let backend = LlamaBackend::init()?; // ✅ 第一次成功
let backend2 = LlamaBackend::init()?; // ❌ 第二次失败
```

### 解决方案
**关键**: 在切换前显式释放旧的 backend 和 model

```rust
// 1. Drop 旧的 provider（释放 backend 和 model）
drop(old_provider);

// 2. 等待资源完全释放
tokio::time::sleep(Duration::from_millis(100)).await;

// 3. 重新初始化新的 backend
let new_backend = LlamaBackend::init()?;
let new_model = LlamaModel::load_from_file(...)?;
```

---

## 🏗️ 实现架构

### 方案 1: 在 Provider 层实现（推荐）

#### 优点
- ✅ 封装良好
- ✅ 不影响其他代码
- ✅ 易于测试

#### 实现
```rust
// crates/providers/src/local_gguf/mod.rs

impl LocalGgufProvider {
    /// 卸载当前模型，释放资源
    pub fn unload(self) {
        // Drop self，自动释放 Arc<SendSyncBackend> 和 Arc<Mutex<LlamaModel>>
        drop(self);
    }
    
    /// 热切换到新模型
    pub async fn hot_swap(old_provider: Self, new_config: LocalGgufConfig) -> Result<Self> {
        info!("Starting model hot swap");
        
        // 1. 卸载旧模型
        drop(old_provider);
        
        // 2. 等待资源释放
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        // 3. 加载新模型
        Self::from_config(new_config).await
    }
}
```

### 方案 2: 在 Gateway 层实现

#### 实现
```rust
// crates/gateway/src/server.rs

// 全局 provider 管理
static LOCAL_LLM_PROVIDER: OnceCell<RwLock<Option<Arc<LocalGgufProvider>>>> = OnceCell::new();

async fn switch_local_model(new_model_id: String) -> Result<()> {
    let provider_lock = LOCAL_LLM_PROVIDER.get().unwrap();
    let mut provider_guard = provider_lock.write().await;
    
    // 1. 取出旧 provider
    let old_provider = provider_guard.take();
    
    // 2. 释放旧 provider
    if let Some(old) = old_provider {
        drop(old);
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    
    // 3. 加载新 provider
    let config = LocalGgufConfig {
        model_id: new_model_id,
        ..Default::default()
    };
    
    let new_provider = LocalGgufProvider::from_config(config).await?;
    *provider_guard = Some(Arc::new(new_provider));
    
    Ok(())
}
```

---

## 🔧 具体实现步骤

### 步骤 1: 添加 Drop 实现（可选）
```rust
impl Drop for LocalGgufProvider {
    fn drop(&mut self) {
        info!(model = %self.model_id, "Unloading GGUF model");
        // Arc 会自动清理，但我们可以添加日志
    }
}
```

### 步骤 2: 添加热切换 API
```rust
// crates/gateway/src/rpc/models.rs

pub async fn switch_model(params: Value) -> Result<Value> {
    let model_id: String = serde_json::from_value(params.get("model_id"))?;
    
    // 调用热切换逻辑
    switch_local_model(model_id).await?;
    
    Ok(json!({ "success": true }))
}
```

### 步骤 3: 前端调用
```javascript
// models.js
export function selectModel(m) {
    // 如果是 local-llm，调用热切换 API
    if (m.provider === "local-llm") {
        sendRpc("models.switch", { model_id: m.id });
    }
    
    // 更新 session
    setSessionModel(S.activeSessionKey, m.id);
    closeModelDropdown();
}
```

---

## ⚠️ 注意事项

### 1. 内存管理
```rust
// 确保 Arc 引用计数为 0
assert_eq!(Arc::strong_count(&backend), 1);
drop(backend);
```

### 2. 等待时间
```rust
// 给系统时间释放资源
tokio::time::sleep(Duration::from_millis(200)).await;
```

### 3. 错误处理
```rust
// 如果新模型加载失败，恢复旧模型
match load_new_model().await {
    Ok(new) => new,
    Err(e) => {
        warn!("Failed to load new model, keeping old one");
        return Err(e);
    }
}
```

---

## 🧪 测试计划

### 测试 1: 基本切换
```
1. 启动 Llama 3.2 1B
2. 切换到 Qwen 14B
3. 验证切换成功
4. 发送查询测试
```

### 测试 2: 快速切换
```
1. Llama → Qwen
2. 立即 Qwen → Llama
3. 验证无崩溃
```

### 测试 3: 错误恢复
```
1. 切换到不存在的模型
2. 验证保持原模型
3. 错误信息清晰
```

---

## 📊 预期效果

| 操作 | 修复前 | 修复后 |
|------|--------|--------|
| 切换模型 | ❌ 需要重启 | ✅ 热切换 |
| 切换时间 | 30-60秒 | 5-10秒 |
| 用户体验 | 差 | 优秀 |

---

## 💡 实现优先级

### 立即实现（最小可行）
```rust
// 简单版本：在 sessions.patch 中添加
if model_changed {
    drop(old_provider);
    tokio::time::sleep(Duration::from_millis(200)).await;
    load_new_provider().await?;
}
```

### 完整实现（推荐）
- 添加专门的 RPC 端点
- 完善错误处理
- 添加进度通知
- 实现回滚机制

---

**现在开始实现最小可行版本！** 🚀
