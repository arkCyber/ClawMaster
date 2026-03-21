# 🔥 模型热切换实现方案

**目标**: 实现运行时模型切换，无需重启  
**方案**: 最小可行实现 + 用户操作指南  

---

## 💡 实用解决方案

### 方案 A: 使用 RPC 重新加载（推荐）

**原理**: 通过 RPC 调用触发 provider 重新加载

#### 实现步骤

1. **添加 RPC 端点**
```rust
// crates/gateway/src/rpc/methods/services.rs

reg.register(
    "models.reload",
    Box::new(|ctx| {
        Box::pin(async move {
            let model_id: String = ctx.params.get("model_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow::anyhow!("model_id required"))?
                .to_string();
            
            // 触发 provider 重新加载
            ctx.state.reload_local_model(model_id).await?;
            
            Ok(json!({ "success": true }))
        })
    }),
);
```

2. **前端调用**
```javascript
// models.js
export function selectModel(m) {
    if (m.provider === "local-llm") {
        // 调用重新加载 API
        sendRpc("models.reload", { model_id: m.id })
            .then(() => {
                updateModelComboLabel(m);
                showToast("Model switched successfully", "success");
            })
            .catch(err => {
                showToast("Failed to switch model: " + err.message, "error");
            });
    }
    
    setSessionModel(S.activeSessionKey, m.id);
    closeModelDropdown();
}
```

---

## 🎯 更简单的方案：手动操作

### 方案 B: 快速重启（当前最实用）

**步骤**:
```bash
# 1. 在终端中停止
Ctrl+C

# 2. 立即重启
./target/debug/clawmaster

# 3. 在 WebUI 中选择新模型
```

**时间**: 5-10秒
**优点**: 
- ✅ 简单可靠
- ✅ 无需代码修改
- ✅ 立即可用

---

## 🔧 临时解决方案（立即可用）

### 使用脚本快速切换

创建切换脚本：
```bash
#!/bin/bash
# switch-model.sh

echo "Stopping ClawMaster..."
pkill -f clawmaster

echo "Waiting for cleanup..."
sleep 2

echo "Starting ClawMaster..."
cd /Users/arksong/ClawMaster
./target/debug/clawmaster

echo "ClawMaster restarted. Please select model in WebUI."
```

使用：
```bash
chmod +x switch-model.sh
./switch-model.sh
```

---

## 📊 方案对比

| 方案 | 实现难度 | 切换时间 | 可靠性 | 立即可用 |
|------|----------|----------|--------|----------|
| RPC 重新加载 | 高 | 3-5秒 | 中 | ❌ 需要开发 |
| 快速重启 | 低 | 5-10秒 | 高 | ✅ 立即可用 |
| 切换脚本 | 低 | 5-10秒 | 高 | ✅ 立即可用 |

---

## 🎯 推荐操作

### 现在（立即）
**使用快速重启**:
```
1. 终端按 Ctrl+C
2. 运行 ./target/debug/clawmaster
3. 在 WebUI 选择 Qwen 模型
4. 开始使用
```

### 未来（可选）
实现完整的热切换 API

---

## 💡 为什么快速重启是好方案

### 优点
1. **简单可靠** - 无需复杂代码
2. **快速** - 5-10秒完成
3. **安全** - 完全清理资源
4. **立即可用** - 无需等待开发

### 对比热切换
- 热切换需要复杂的资源管理
- 可能有内存泄漏风险
- 需要大量测试
- 快速重启更可靠

---

## 🚀 立即行动

### 选项 1: 快速重启切换到 Qwen
```bash
# 终端操作
Ctrl+C
./target/debug/clawmaster
# 然后在 WebUI 选择 Qwen
```

### 选项 2: 继续使用 Llama 测试
```
直接测试新闻功能
稍后再切换模型
```

---

**建议：使用快速重启方案，简单可靠且立即可用！** ✅
