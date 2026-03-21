# Qwen 3.5 9B 集成总结

**日期**: 2026-03-20  
**目标**: 将 Qwen 3.5 9B 集成到 ClawMaster 以提升工具调用能力  
**当前状态**: 🟡 模型已加载，调试 Ollama 集成问题

---

## 完成的工作

### ✅ 1. 模型准备
- **下载**: Qwen 3.5 9B GGUF (6.6 GB)
- **位置**: `~/.ollama/models/blobs/sha256-dec52a...`
- **格式**: GGUF Q4_K_M
- **参数**: 9.7B

### ✅ 2. Ollama 配置
```bash
ollama list
# qwen3.5:9b    6488c96fa5fa    6.6 GB    8 days ago
```

### ✅ 3. ClawMaster 配置
```toml
[providers.ollama]
base_url = "http://localhost:11434"
models = ["qwen3.5:9b"]

[chat]
default_model = "ollama::qwen3.5:9b"
```

### ✅ 4. 代码修改
- **文件**: `crates/providers/src/local_gguf/mod.rs`
  - `supports_tools()` → `false`
  - `tool_mode()` → `ToolMode::Text`
  
- **文件**: `crates/providers/src/local_gguf/models.rs`
  - 添加 Qwen 3.5 9B 模型定义

- **文件**: `crates/cli/src/agent_client.rs`
  - 移除硬编码的 `llama-3.1-8b-q4_k_m`
  - 改为自动选择第一个可用模型

### ✅ 5. 后端验证
```
model="ollama::qwen3.5:9b"
native_tools=true
tools_count=37
provider="ollama"
```

---

## 遇到的问题

### 问题 1: local-llm provider 加载失败 ❌
**错误**: `unknown model 'qwen3.5:9b'. Use model_path for custom GGUF files.`  
**原因**: 模型不在 local-llm 注册表中  
**解决**: 切换到 Ollama provider

### 问题 2: 会话元数据缓存 ✅
**错误**: `model 'local-llm::llama-3.1-8b-q4_k_m' not found`  
**原因**: 数据库中存储了旧模型 ID  
**解决**: 清除 `~/.clawmaster/*.db*`

### 问题 3: CLI 硬编码模型 ✅
**错误**: CLI 请求旧模型  
**原因**: `agent_client.rs` 硬编码  
**解决**: 移除硬编码

### 问题 4: Ollama HTTP 404 🔄
**错误**: `HTTP 404: 404 page not found`  
**状态**: 正在调试  
**可能原因**:
- Ollama API 端点不匹配
- 模型名称格式问题
- Provider 配置问题

---

## 技术架构

### 推理路径
```
CLI → Gateway (65518) → Ollama Provider → Ollama (11434) → Qwen 3.5 9B
```

### Provider 对比

| Provider | 优势 | 劣势 | 状态 |
|----------|------|------|------|
| **local-llm** | 直接加载 GGUF | 需要注册表定义 | ❌ 失败 |
| **Ollama** | 模型管理简单 | 需要额外服务 | 🔄 调试中 |

---

## 下一步行动

1. 🔄 调试 Ollama HTTP 404 错误
2. ⏳ 验证 Ollama API 端点
3. ⏳ 测试直接 Ollama API 调用
4. ⏳ 运行完整工具调用测试
5. ⏳ 生成性能对比报告

---

## 关键经验

1. **模型注册表很重要**: local-llm provider 需要在代码中定义模型
2. **会话缓存需要清理**: 数据库会存储模型 ID
3. **Provider 选择**: Ollama 提供更好的模型管理
4. **配置优先级**: `clawmaster.toml` > `local-llm.json`
5. **端口动态分配**: Gateway 端口可能不是固定的 59233

---

**当前焦点**: 解决 Ollama HTTP 404 错误  
**预期时间**: 10-15 分钟  
**成功标准**: CLI 测试返回正确响应，工具调用成功率 > 60%
