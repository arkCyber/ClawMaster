# 🔄 AI 模型动态切换完整指南

---

## 📋 问题诊断

### 错误信息
```
failed to load model: unknown model 'custom-llama-3.1-8b-instruct-q4_k_m.gguf'
```

### 根本原因
配置文件中的 `model_id` 不在模型注册表中。

---

## ✅ 解决方案

### 1. 使用注册表中的模型（推荐）

**模型注册表位置**: `crates/providers/src/local_gguf/models.rs`

**可用的 Llama 模型**:
- `llama-3.2-1b-q4_k_m` - Llama 3.2 1B (约 0.9GB)
- `llama-3.2-3b-q4_k_m` - Llama 3.2 3B (约 2GB)
- `llama-3.1-8b-q4_k_m` - Llama 3.1 8B (约 4.9GB) ← **推荐**
- `llama-3.1-70b-q2_k` - Llama 3.1 70B (约 28GB)

**配置示例**:
```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-q4_k_m"  # 使用注册表中的 ID
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

---

### 2. 使用自定义模型文件（高级）

如果要使用不在注册表中的模型，使用 `model_path`:

```toml
[providers.local-llm]
enabled = true
model_id = "my-custom-model"  # 任意名称
model_path = "/Users/arksong/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

**注意**: 使用 `model_path` 时，`model_id` 可以是任意名称，但文件必须存在。

---

## 🔍 模型注册表详解

### 代码位置
`crates/providers/src/local_gguf/models.rs` 第78-305行

### Llama 3.1 8B 定义
```rust
GgufModelDef {
    id: "llama-3.1-8b-q4_k_m",  // ← 这是正确的 model_id
    display_name: "Llama 3.1 8B (Q4_K_M)",
    hf_repo: "bartowski/Meta-Llama-3.1-8B-Instruct-GGUF",
    hf_filename: "Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf",
    min_ram_gb: 12,
    context_window: 128_000,
    chat_template: ChatTemplateHint::Llama3,
    backend: ModelBackend::Gguf,
}
```

### 模型查找逻辑
`crates/providers/src/local_gguf/mod.rs` 第141-152行:

```rust
if let Some(path) = &config.model_path {
    // 使用自定义路径
    (path, find_model(&config.model_id))
} else {
    // 从注册表查找
    let Some(def) = find_model(&config.model_id) else {
        bail!(
            "unknown model '{}'. Use model_path for custom GGUF files.",
            config.model_id
        );
    };
    // ... 自动下载逻辑
}
```

---

## 🚀 动态模型切换功能

### 当前实现
`crates/providers/src/local_gguf/mod.rs` 第235-261行

```rust
pub async fn reload(old_provider: Option<Self>, config: LocalGgufConfig) -> Result<Self> {
    // 1. 释放旧模型
    if let Some(old) = old_provider {
        drop(old);
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    // 2. 加载新模型
    let new_provider = Self::from_config(config).await?;
    
    Ok(new_provider)
}
```

### 使用方式

**通过 WebUI**:
1. 访问 https://localhost:59233
2. 点击模型选择器
3. 选择新模型
4. 系统自动热切换

**通过 RPC**:
```json
{
  "jsonrpc": "2.0",
  "method": "services.reload_model",
  "params": {
    "model_id": "local-llm::llama-3.1-8b-q4_k_m"
  },
  "id": 1
}
```

---

## 📝 完整的模型列表

### 4GB 内存（Tiny）
- `qwen2.5-coder-1.5b-q4_k_m` - Qwen 2.5 Coder 1.5B
- `llama-3.2-1b-q4_k_m` - Llama 3.2 1B

### 8GB 内存（Small）
- `qwen2.5-coder-7b-q4_k_m` - Qwen 2.5 Coder 7B
- `llama-3.2-3b-q4_k_m` - Llama 3.2 3B
- `deepseek-coder-6.7b-q4_k_m` - DeepSeek Coder 6.7B

### 16GB 内存（Medium）
- `qwen2.5-coder-14b-q4_k_m` - Qwen 2.5 Coder 14B
- `codestral-22b-q4_k_m` - Codestral 22B
- `mistral-7b-q5_k_m` - Mistral 7B
- `llama-3.1-8b-q4_k_m` - Llama 3.1 8B ← **推荐用于工具调用**

### 32GB+ 内存（Large）
- `qwen2.5-coder-32b-q4_k_m` - Qwen 2.5 Coder 32B
- `deepseek-coder-33b-q4_k_m` - DeepSeek Coder 33B
- `llama-3.1-70b-q2_k` - Llama 3.1 70B

---

## 🔧 故障排除

### 问题 1: "unknown model 'xxx'"

**原因**: `model_id` 不在注册表中

**解决**:
- 方案 A: 使用注册表中的 ID（见上面列表）
- 方案 B: 添加 `model_path` 指向实际文件

### 问题 2: "model file not found"

**原因**: `model_path` 指向的文件不存在

**解决**:
```bash
# 检查文件
ls -lh ~/.clawmaster/models/

# 确保路径正确
[providers.local-llm]
model_path = "/Users/arksong/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"
```

### 问题 3: 模型切换后仍使用旧模型

**原因**: 配置文件未生效或进程未重启

**解决**:
```bash
# 1. 修改配置
vim ~/.config/clawmaster/clawmaster.toml

# 2. 完全重启
pkill -9 -f clawmaster
./target/debug/clawmaster > /tmp/clawmaster.log 2>&1 &

# 3. 验证
tail -50 /tmp/clawmaster.log | grep "model="
```

---

## 💡 最佳实践

### 1. 使用注册表模型（推荐）
- ✅ 自动下载
- ✅ 自动配置
- ✅ 已验证兼容性

### 2. 自定义模型（高级用户）
- 需要手动下载
- 需要指定 `model_path`
- 需要确保格式兼容

### 3. 模型选择建议
- **工具调用**: Llama 3.1 8B 或 Llama 3.2 3B
- **代码生成**: Qwen 2.5 Coder 系列
- **通用对话**: Llama 3.1/3.2 系列

---

## 📊 当前配置验证

```bash
# 查看当前配置
grep -A 10 "\[providers.local-llm\]" ~/.config/clawmaster/clawmaster.toml

# 查看可用模型文件
ls -lh ~/.clawmaster/models/

# 查看运行日志
tail -100 /tmp/clawmaster_fixed.log | grep -E "(model|loading)"
```

---

## ✅ 修复总结

### 修复前
```toml
model_id = "custom-llama-3.1-8b-instruct-q4_k_m"  # ❌ 错误
```

### 修复后
```toml
model_id = "llama-3.1-8b-q4_k_m"  # ✅ 正确
```

或使用自定义路径：
```toml
model_id = "my-llama-3.1-8b"
model_path = "/Users/arksong/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"
```

---

**现在模型应该可以正常加载了！** 🎉
