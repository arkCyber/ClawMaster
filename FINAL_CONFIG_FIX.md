# 🔧 最终配置修复

**时间**: 2026年3月18日 10:07  
**问题**: 服务暂时不可用  

---

## 🔍 问题诊断

### 错误信息
WebUI 显示：`服务暂时不可用。请重试。`

### 日志分析
```
INFO clawmaster_providers: local-llm enabled but no models configured. 
Add [providers.local] models = ["..."] to config.

INFO clawmaster_gateway::server: startup model inventory 
model_count=0 provider_count=0

WARN clawmaster_gateway::server: no LLM providers in static startup catalog
```

### 根本原因
配置文件中 `[providers.local-llm]` 缺少 `models` 数组。

---

## ✅ 修复方案

### 配置文件修复
**文件**: `~/.config/clawmaster/clawmaster.toml`

**修复前**:
```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-q4_k_m"  # ❌ 错误：应该是 models 数组
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

**修复后**:
```toml
[providers.local-llm]
enabled = true
models = ["llama-3.1-8b-q4_k_m", "llama-3.2-1b-q4_k_m"]  # ✅ 正确
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

---

## 📊 配置文件结构

### 正确的 local-llm 配置
```toml
[providers.local-llm]
enabled = true                    # 启用本地模型
models = [                        # 模型列表（必需）
    "llama-3.1-8b-q4_k_m",
    "llama-3.2-1b-q4_k_m",
    "qwen2.5-coder-14b-q4_k_m"
]
gpu_layers = 33                   # GPU 层数
temperature = 0.7                 # 温度参数
context_size = 8192               # 上下文大小
```

### 可选配置
```toml
[providers.local-llm]
enabled = true
models = ["llama-3.1-8b-q4_k_m"]
gpu_layers = 33
temperature = 0.7
context_size = 8192
cache_dir = "/custom/path/to/models"  # 可选：自定义模型目录
```

---

## 🔄 配置文件优先级（完整版）

### 1. WebUI 动态配置
```
~/.config/clawmaster/local-llm.json
```
- WebUI 保存的模型配置
- 优先级最高
- 用于单个模型的快速切换

### 2. 主配置文件
```
~/.config/clawmaster/clawmaster.toml
```
- 系统主配置
- 定义可用的模型列表
- 定义全局参数（gpu_layers, temperature 等）

### 3. 备用配置
```
~/.clawmaster/clawmaster.toml
```
- 备用配置位置
- 优先级最低

---

## 📝 配置参数说明

### models（必需）
```toml
models = ["llama-3.1-8b-q4_k_m", "llama-3.2-1b-q4_k_m"]
```
- **类型**: 字符串数组
- **说明**: 可用的模型 ID 列表
- **来源**: 必须是模型注册表中的 ID
- **位置**: `crates/providers/src/local_gguf/models.rs`

### enabled
```toml
enabled = true
```
- **类型**: 布尔值
- **说明**: 是否启用本地模型提供者
- **默认**: false

### gpu_layers
```toml
gpu_layers = 33
```
- **类型**: 整数
- **说明**: 加载到 GPU 的层数
- **范围**: 0-99（0 = 仅 CPU）
- **建议**: 根据显卡内存调整

### temperature
```toml
temperature = 0.7
```
- **类型**: 浮点数
- **说明**: 生成温度（创造性）
- **范围**: 0.0-2.0
- **建议**: 0.7（平衡）

### context_size
```toml
context_size = 8192
```
- **类型**: 整数
- **说明**: 上下文窗口大小（tokens）
- **建议**: 8192 或模型的最大值

---

## 🎯 验证步骤

### 1. 检查配置文件
```bash
cat ~/.config/clawmaster/clawmaster.toml | grep -A 10 "providers.local-llm"
```

**期望输出**:
```toml
[providers.local-llm]
enabled = true
models = ["llama-3.1-8b-q4_k_m", "llama-3.2-1b-q4_k_m"]
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

### 2. 检查服务启动日志
```bash
tail -100 /tmp/clawmaster_fixed2.log | grep "model_count"
```

**期望输出**:
```
INFO clawmaster_gateway::server: startup model inventory 
model_count=2 provider_count=1
```

### 3. 检查 WebUI
访问 https://localhost:59233

**期望行为**:
- 模型选择器显示 2 个模型
- 可以选择模型
- 可以发送消息

### 4. 测试工具调用
输入：`美国新闻`

**期望输出**:
- 调用 `news_search` 工具
- 返回新闻列表
- 没有错误

---

## 💡 常见错误

### 错误 1: model_id 而不是 models
```toml
# ❌ 错误
[providers.local-llm]
model_id = "llama-3.1-8b-q4_k_m"

# ✅ 正确
[providers.local-llm]
models = ["llama-3.1-8b-q4_k_m"]
```

### 错误 2: 模型 ID 不在注册表中
```toml
# ❌ 错误
models = ["custom-llama-3.1-8b-instruct-q4_k_m.gguf"]

# ✅ 正确
models = ["llama-3.1-8b-q4_k_m"]
```

### 错误 3: enabled = false
```toml
# ❌ 错误
[providers.local-llm]
enabled = false
models = ["llama-3.1-8b-q4_k_m"]

# ✅ 正确
[providers.local-llm]
enabled = true
models = ["llama-3.1-8b-q4_k_m"]
```

---

## 📋 完整的修复命令

```bash
# 1. 备份配置
cp ~/.config/clawmaster/clawmaster.toml ~/.config/clawmaster/clawmaster.toml.backup2

# 2. 修复配置（手动编辑或使用 sed）
# 确保包含 models 数组

# 3. 重启服务
pkill -9 -f clawmaster
cd /Users/arksong/ClawMaster
./target/debug/clawmaster > /tmp/clawmaster_fixed2.log 2>&1 &

# 4. 等待启动
sleep 15

# 5. 验证
tail -50 /tmp/clawmaster_fixed2.log | grep -E "(model_count|listening)"

# 6. 测试
# 访问 https://localhost:59233
# 输入: 美国新闻
```

---

## 🚀 预期结果

### 启动日志
```
INFO clawmaster_gateway::server: startup model inventory 
model_count=2 provider_count=1 
provider_model_counts=[("local-llm", 2)]
sample_model_ids=["local-llm::llama-3.1-8b-q4_k_m", "local-llm::llama-3.2-1b-q4_k_m"]

INFO clawmaster_gateway::server: │  llm: 1 provider, 2 models  │
```

### WebUI
- ✅ 模型选择器显示 2 个模型
- ✅ 可以切换模型
- ✅ 可以发送消息
- ✅ 工具调用正常

---

**配置已修复，等待服务启动验证！** 🎉
