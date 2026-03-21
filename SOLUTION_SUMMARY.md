# ClawMaster 工具调用问题 - 最终总结

**时间**: 2026-03-20 21:18  
**问题**: 工具调用成功率 0%，系统未切换到 Qwen 3.5 9B

---

## 核心问题

### 1. 模型未切换
**现象**: 系统仍使用 Llama 3.1 8B  
**证据**: `model="local-llm::llama-3.1-8b-q4_k_m"`  
**原因**: 配置文件未被读取或 provider 注册逻辑有问题

### 2. 工具调用模式错误
**现象**: `native_tools=true`  
**期望**: `native_tools=false`（文本模式）  
**影响**: Llama 3.1 8B 不支持原生工具调用，导致失败

### 3. 响应缓存问题
**现象**: 所有请求返回相同结果 "结果是 300"  
**原因**: 会话缓存或 WebSocket 连接复用

---

## 已尝试的解决方案

### ✅ 成功完成
1. 下载 Qwen 3.5 9B 模型（6.6 GB）
2. 修改代码设置 `supports_tools=false`
3. 添加 Qwen 3.5 模型定义
4. 创建配置文件指定模型路径

### ❌ 未生效
1. 配置文件 `~/.config/clawmaster/local-llm.json` 未被读取
2. 系统仍注册并使用 Llama 3.1 8B
3. `native_tools` 仍为 `true`

---

## 根本原因分析

### 问题 1: Provider 注册顺序
系统可能在启动时注册了多个模型：
1. Llama 3.1 8B（默认）
2. Mistral 7B
3. Qwen 2.5 Coder 32B
4. Qwen 3.5 9B（新添加）

**默认选择**: 第一个注册的模型（Llama 3.1 8B）

### 问题 2: 配置加载时机
配置文件可能在 provider 注册**之后**才加载，导致无效

### 问题 3: 模型 ID 不匹配
- 配置中: `qwen3.5:9b`
- 系统期望: 可能需要不同的格式

---

## 最终解决方案

### 方案 1: 使用 Ollama Provider（最简单）

**步骤**:
1. 不使用 local-llm provider
2. 改用 Ollama provider
3. Ollama 原生支持 `qwen3.5:9b`

**配置**: `~/.config/clawmaster/clawmaster.toml`
```toml
[providers.ollama]
base_url = "http://localhost:11434"
models = ["qwen3.5:9b"]

[chat]
# 让系统自动选择 Ollama 的 Qwen 3.5
```

**优势**:
- 无需修改代码
- Ollama 自动管理模型
- 配置简单直接

### 方案 2: 修改 Provider 注册逻辑（需要代码修改）

**步骤**:
1. 找到 provider 注册代码
2. 确保 Qwen 3.5 是第一个注册的
3. 或者修改默认模型选择逻辑

**位置**: `crates/gateway/src/server.rs` 或 `crates/providers/src/lib.rs`

### 方案 3: 清空其他模型配置（临时方案）

**步骤**:
1. 编辑 `~/.config/clawmaster/local-llm.json`
2. 只保留 Qwen 3.5 9B
3. 删除其他模型配置

```json
{
  "models": [
    {
      "model_id": "qwen3.5:9b",
      "model_path": "/Users/arksong/.ollama/models/blobs/sha256-dec52a44569a2a25341c4e4d3fee25846eed4f6f0b936278e3a3c900bb99d37c",
      "gpu_layers": 0,
      "backend": "GGUF"
    }
  ]
}
```

---

## 推荐行动

### 立即执行（方案 1）
1. 检查 Ollama 是否运行: `ollama list`
2. 创建 `clawmaster.toml` 配置
3. 重启后端测试

### 如果方案 1 失败
1. 执行方案 3（清空其他模型）
2. 检查代码中的 provider 注册逻辑
3. 修改为强制使用 Qwen 3.5

---

## 预期结果

成功后应该看到：
```
✅ model="ollama::qwen3.5:9b" 或 "local-llm::qwen3.5:9b"
✅ native_tools=false
✅ tool_calls_count > 0
✅ 工具调用成功率 > 60%
```

---

## 关键经验教训

1. **模型能力是基础**: Llama 3.1 8B 对工具调用支持有限
2. **配置生效验证**: 必须验证配置是否真的被读取
3. **日志是关键**: 通过日志确认实际使用的模型
4. **Provider 选择**: Ollama provider 可能比 local-llm 更简单

---

**当前状态**: 🔴 需要切换到 Ollama Provider 或修改注册逻辑  
**推荐方案**: 方案 1（Ollama Provider）  
**预计时间**: 5-10 分钟
