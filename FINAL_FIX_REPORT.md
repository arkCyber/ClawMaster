# 🎉 AI 模型切换问题修复报告

**时间**: 2026年3月18日 09:35  
**状态**: ✅ 已修复  

---

## 📋 问题诊断

### 错误现象
```
Auto-compact failed: compact summarization failed: 
failed to load model: unknown model 'custom-llama-3.1-8b-instruct-q4_k_m.gguf'
Use model_path for custom GGUF files.
Provider: local-llm
```

### 根本原因
配置文件中使用了错误的 `model_id`：
- ❌ **错误**: `custom-llama-3.1-8b-instruct-q4_k_m`
- ✅ **正确**: `llama-3.1-8b-q4_k_m`

---

## 🔍 代码审计发现

### 1. 模型注册表位置
`crates/providers/src/local_gguf/models.rs` 第166-174行

```rust
GgufModelDef {
    id: "llama-3.1-8b-q4_k_m",  // ← 正确的 model_id
    display_name: "Llama 3.1 8B (Q4_K_M)",
    hf_repo: "bartowski/Meta-Llama-3.1-8B-Instruct-GGUF",
    hf_filename: "Meta-Llama-3.1-8B-Instruct-Q4_K_M.gguf",
    min_ram_gb: 12,
    context_window: 128_000,
    chat_template: ChatTemplateHint::Llama3,
    backend: ModelBackend::Gguf,
}
```

### 2. 模型查找逻辑
`crates/providers/src/local_gguf/mod.rs` 第147-152行

```rust
let Some(def) = find_model(&config.model_id) else {
    bail!(
        "unknown model '{}'. Use model_path for custom GGUF files.",
        config.model_id
    );
};
```

**问题**: 
- 如果 `model_id` 不在注册表中，直接报错
- 没有回退到使用文件名匹配

### 3. 模型文件实际位置
```bash
~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf  # 4.6GB
```

---

## ✅ 修复方案

### 方案 1: 使用注册表 ID（已实施）

**修改配置文件**:
```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-q4_k_m"  # ← 使用注册表中的正确 ID
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

**优点**:
- ✅ 自动下载（如果文件不存在）
- ✅ 自动配置（context_size, chat_template）
- ✅ 已验证兼容性

---

### 方案 2: 使用 model_path（备选）

如果要使用自定义模型：

```toml
[providers.local-llm]
enabled = true
model_id = "my-custom-llama"  # 任意名称
model_path = "/Users/arksong/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf"
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

**优点**:
- ✅ 支持任意 GGUF 文件
- ✅ 不需要在注册表中

---

## 🔧 完善的动态模型切换功能

### 当前实现分析

#### 1. 热切换机制
`crates/providers/src/local_gguf/mod.rs` 第235-261行

```rust
pub async fn reload(old_provider: Option<Self>, config: LocalGgufConfig) -> Result<Self> {
    info!(
        old_model = old_provider.as_ref().map(|p| p.model_id.as_str()),
        new_model = %config.model_id,
        "Starting model hot-swap"
    );

    // 1. 释放旧模型
    if let Some(old) = old_provider {
        info!(model = %old.model_id, "Dropping old model");
        drop(old);
        
        // 2. 等待资源释放
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // 3. 加载新模型
    info!(model = %config.model_id, "Loading new model");
    let new_provider = Self::from_config(config).await?;
    
    info!(
        model = %new_provider.model_id,
        "Model hot-swap completed successfully"
    );
    
    Ok(new_provider)
}
```

**优点**:
- ✅ 自动释放旧模型资源
- ✅ 等待 500ms 确保资源释放
- ✅ 详细的日志记录

**改进建议**:
1. 增加重试机制（如果加载失败）
2. 添加模型验证（加载后测试生成）
3. 支持回滚到旧模型（如果新模型失败）

---

### 建议的改进代码

```rust
pub async fn reload(old_provider: Option<Self>, config: LocalGgufConfig) -> Result<Self> {
    let old_model_id = old_provider.as_ref().map(|p| p.model_id.clone());
    
    info!(
        old_model = old_model_id.as_deref(),
        new_model = %config.model_id,
        "Starting model hot-swap"
    );

    // 1. 尝试加载新模型（保留旧模型作为备份）
    let new_provider = match Self::from_config(config.clone()).await {
        Ok(provider) => provider,
        Err(e) => {
            warn!(
                error = %e,
                new_model = %config.model_id,
                "Failed to load new model, keeping old model"
            );
            
            // 如果有旧模型，返回旧模型
            if let Some(old) = old_provider {
                return Ok(old);
            }
            
            return Err(e);
        }
    };
    
    // 2. 新模型加载成功，释放旧模型
    if let Some(old) = old_provider {
        info!(model = %old.model_id, "Dropping old model");
        drop(old);
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    
    info!(
        model = %new_provider.model_id,
        "Model hot-swap completed successfully"
    );
    
    Ok(new_provider)
}
```

---

## 📊 完整的模型注册表

### 按内存分类

#### 4GB 内存（Tiny）
- `qwen2.5-coder-1.5b-q4_k_m` - Qwen 2.5 Coder 1.5B
- `llama-3.2-1b-q4_k_m` - Llama 3.2 1B

#### 8GB 内存（Small）
- `qwen2.5-coder-7b-q4_k_m` - Qwen 2.5 Coder 7B
- `llama-3.2-3b-q4_k_m` - Llama 3.2 3B
- `deepseek-coder-6.7b-q4_k_m` - DeepSeek Coder 6.7B

#### 16GB 内存（Medium）
- `qwen2.5-coder-14b-q4_k_m` - Qwen 2.5 Coder 14B
- `codestral-22b-q4_k_m` - Codestral 22B
- `mistral-7b-q5_k_m` - Mistral 7B
- **`llama-3.1-8b-q4_k_m`** - Llama 3.1 8B ← **当前使用**

#### 32GB+ 内存（Large）
- `qwen2.5-coder-32b-q4_k_m` - Qwen 2.5 Coder 32B
- `deepseek-coder-33b-q4_k_m` - DeepSeek Coder 33B
- `llama-3.1-70b-q2_k` - Llama 3.1 70B

---

## 🎯 工具调用验证

### 预期行为
使用 Llama 3.1 8B 后，应该能够正确调用工具：

**测试查询**: `美国新闻`

**预期日志**:
```
resolved effective tool mode
  effective_mode = Text
  native_tools = false

tool mode configuration for this request
  tool_mode = Text
  native_tools = false

streaming LLM response complete
  tool_calls_count = 1  ← 关键！
```

**预期输出**:
```
```tool_call
{"tool": "news_search", "arguments": {"query": "news", "location": "USA"}}
```
```

---

## 📝 修复步骤总结

### 1. 备份配置
```bash
cp ~/.config/clawmaster/clawmaster.toml ~/.config/clawmaster/clawmaster.toml.backup
```

### 2. 修复 model_id
```bash
sed -i '' 's/model_id = "custom-llama-3.1-8b-instruct-q4_k_m"/model_id = "llama-3.1-8b-q4_k_m"/' ~/.config/clawmaster/clawmaster.toml
```

### 3. 重启服务
```bash
pkill -9 -f clawmaster
./target/debug/clawmaster > /tmp/clawmaster_fixed.log 2>&1 &
```

### 4. 验证
```bash
# 检查模型加载
tail -100 /tmp/clawmaster_fixed.log | grep -E "(llama-3.1|loading|model)"

# 测试工具调用
# 访问 https://localhost:59233
# 输入: 美国新闻
```

---

## 🔄 动态切换使用方法

### 方法 1: WebUI（推荐）
1. 访问 https://localhost:59233
2. 点击模型选择器（左上角）
3. 选择新模型
4. 系统自动热切换

### 方法 2: RPC API
```bash
curl -X POST https://localhost:59233/api/rpc \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "services.reload_model",
    "params": {
      "model_id": "local-llm::llama-3.2-3b-q4_k_m"
    },
    "id": 1
  }'
```

### 方法 3: 修改配置文件
```bash
# 1. 修改配置
vim ~/.config/clawmaster/clawmaster.toml

# 2. 重启服务
pkill -9 -f clawmaster
./target/debug/clawmaster > /tmp/clawmaster.log 2>&1 &
```

---

## ✅ 验证清单

- [x] 模型文件已下载（4.6GB）
- [x] 配置文件已修复（使用正确的 model_id）
- [x] ClawMaster 已重启
- [x] 模型注册表已审计
- [x] 动态切换功能已分析
- [ ] 工具调用功能待测试

---

## 🎓 经验教训

### 1. model_id 必须精确匹配
- 注册表中的 ID 是唯一标识
- 不能使用文件名或自定义名称
- 如需自定义，使用 `model_path`

### 2. 模型注册表是权威来源
- 位置: `crates/providers/src/local_gguf/models.rs`
- 包含所有预定义模型
- 定义了 ID、下载地址、配置

### 3. 两种模型加载方式
- **注册表模式**: 使用 `model_id`，自动下载和配置
- **自定义模式**: 使用 `model_path`，手动管理

### 4. 热切换需要时间
- 释放旧模型资源需要时间
- 当前等待 500ms
- 建议增加重试和回滚机制

---

## 📚 相关文档

已创建的文档：
1. `MODEL_SWITCHING_GUIDE.md` - 完整的模型切换指南
2. `FINAL_FIX_REPORT.md` - 本文档

相关代码文件：
1. `crates/providers/src/local_gguf/models.rs` - 模型注册表
2. `crates/providers/src/local_gguf/mod.rs` - 模型加载和切换逻辑
3. `~/.config/clawmaster/clawmaster.toml` - 配置文件

---

## 🚀 下一步

### 1. 测试工具调用
访问 https://localhost:59233，输入：`美国新闻`

### 2. 验证日志
```bash
tail -100 /tmp/clawmaster_fixed.log | grep -E "(tool_calls_count|tool_mode)"
```

### 3. 如果成功
- `tool_calls_count = 1` → 🎉 完全成功！
- 新闻工具正常工作
- 问题彻底解决

### 4. 如果失败
- 检查 `native_tools` 是否为 `false`
- 检查 `tool_mode` 是否为 `Text`
- 可能需要进一步调试 prompt

---

**修复完成！请测试工具调用功能。** 🎉
