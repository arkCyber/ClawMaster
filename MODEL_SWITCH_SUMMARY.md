# Qwen 3.5 9B 模型切换总结

**时间**: 2026-03-20 21:11  
**目标**: 将默认模型从 Llama 3.1 8B 切换到 Qwen 3.5 9B

---

## 当前状态

### ❌ 问题
系统仍在使用 **Llama 3.1 8B**，未能成功切换到 Qwen 3.5 9B

### 证据
```
model="local-llm::llama-3.1-8b-q4_k_m"  ← 错误
native_tools=true                        ← 错误
```

### 测试结果
- **工具调用成功率**: 0%
- **响应质量**: 混乱（返回缓存的旧结果）
- **模型加载**: 未确认 Qwen 3.5 是否加载

---

## 已完成的操作

### 1. 模型下载 ✅
```bash
ollama list
# qwen3.5:9b    6488c96fa5fa    6.6 GB    8 days ago
```

### 2. 配置文件修改 ✅
**位置**: `~/.config/clawmaster/local-llm.json`

**内容**:
```json
{
  "models": [
    {
      "model_id": "qwen3.5:9b",
      "gpu_layers": 0,
      "backend": "GGUF"
    }
  ]
}
```

### 3. 代码修改 ✅
- 添加了 Qwen 3.5 模型定义到 `models.rs`
- 修改了 `supports_tools()` 为 `false`
- 设置 `tool_mode()` 为 `Text`

---

## 问题分析

### 可能原因

#### 1. 模型路径问题
Qwen 3.5 通过 Ollama 下载，可能路径与 ClawMaster 期望的不同：
- Ollama 路径: `~/.ollama/models/`
- ClawMaster 期望: `~/.clawmaster/models/` 或 HuggingFace 下载

#### 2. 模型 ID 格式问题
配置中使用 `qwen3.5:9b`（Ollama 格式），但 ClawMaster 可能期望：
- `qwen-3.5-9b-q4_k_m`（内部格式）
- 完整路径

#### 3. Provider 注册问题
系统可能在启动时注册了多个模型，默认选择了第一个（Llama 3.1 8B）

#### 4. 模型未在注册表中
`qwen3.5:9b` 可能不在 `MODEL_REGISTRY` 中，导致无法识别

---

## 解决方案

### 方案 A: 使用 Ollama Provider（推荐）
不使用 local-llm，改用 Ollama provider：

**配置**: `~/.config/clawmaster/clawmaster.toml`
```toml
[providers.ollama]
base_url = "http://localhost:11434"
models = ["qwen3.5:9b"]

[chat]
# 不指定 provider，让系统自动选择
```

### 方案 B: 指定模型路径
找到 Ollama 中 Qwen 3.5 的实际 GGUF 文件路径：

```json
{
  "models": [
    {
      "model_id": "qwen3.5:9b",
      "model_path": "/Users/arksong/.ollama/models/blobs/sha256-xxx",
      "gpu_layers": 0,
      "backend": "GGUF"
    }
  ]
}
```

### 方案 C: 修改代码强制使用 Qwen
直接在代码中硬编码默认模型：

**位置**: `crates/providers/src/local_gguf/mod.rs`
```rust
impl Default for LocalGgufConfig {
    fn default() -> Self {
        Self {
            model_id: "qwen3.5:9b".to_string(),  // 强制使用 Qwen
            model_path: None,
            context_size: None,
            gpu_layers: 0,
            temperature: 0.7,
        }
    }
}
```

---

## 下一步行动

### 立即执行（方案 A）
1. 检查 Ollama 是否运行
2. 配置使用 Ollama provider
3. 重启后端测试

### 如果方案 A 失败
1. 查找 Qwen 3.5 的实际 GGUF 文件路径
2. 使用方案 B 指定完整路径
3. 重新测试

### 最后手段（方案 C）
1. 修改代码强制默认模型
2. 重新编译
3. 测试验证

---

## 预期结果

成功切换后应该看到：
```
model="local-llm::qwen3.5:9b"  或  model="ollama::qwen3.5:9b"
native_tools=false
tool_calls_count>0
```

---

**当前状态**: 🔴 模型未成功切换  
**推荐方案**: 方案 A（使用 Ollama Provider）  
**下一步**: 配置 Ollama provider 并测试
