# 配置 LLM 提供商

本教程将详细介绍如何配置和管理 ClawMaster 的 LLM 提供商。

---

## 📋 支持的提供商

ClawMaster 支持以下 LLM 提供商：

| 提供商 | 模型示例 | API 密钥获取 |
|--------|----------|--------------|
| **OpenAI** | GPT-4, GPT-3.5-turbo | [platform.openai.com](https://platform.openai.com) |
| **Anthropic** | Claude 3 Opus, Sonnet, Haiku | [console.anthropic.com](https://console.anthropic.com) |
| **OpenRouter** | 100+ 模型 | [openrouter.ai](https://openrouter.ai) |
| **Ollama** | Llama 3, Mistral, 等 | 本地运行，无需 API 密钥 |
| **GitHub Copilot** | GPT-4 | GitHub 账户 |

---

## 🚀 快速配置

### 方法 1: Web UI（推荐）

1. 打开 ClawMaster Web UI: `https://localhost:13131`
2. 按 `Ctrl+,` 打开设置
3. 点击 "LLM Providers" 标签
4. 选择提供商并输入 API 密钥
5. 点击 "Save Changes"

### 方法 2: 设置向导

```bash
clawmaster setup
```

按照向导提示配置提供商。

### 方法 3: 命令行

```bash
# 添加 OpenAI
clawmaster provider add openai --key sk-your-api-key

# 添加 Anthropic
clawmaster provider add anthropic --key sk-ant-your-api-key

# 添加 OpenRouter
clawmaster provider add openrouter --key sk-or-your-api-key
```

---

## 🔧 详细配置

### OpenAI

#### 获取 API 密钥

1. 访问 [platform.openai.com](https://platform.openai.com)
2. 登录或注册账户
3. 进入 "API Keys" 页面
4. 点击 "Create new secret key"
5. 复制密钥（只显示一次）

#### 配置

**环境变量**:
```bash
export OPENAI_API_KEY=sk-your-api-key-here
```

**配置文件** `~/.config/clawmaster/clawmaster.toml`:
```toml
[providers.openai]
enabled = true
api_key_env = "OPENAI_API_KEY"
default_model = "gpt-4"
max_tokens = 4096
temperature = 0.7
```

**可用模型**:
- `gpt-4` - 最强大的模型
- `gpt-4-turbo` - 更快的 GPT-4
- `gpt-3.5-turbo` - 快速且经济

#### 测试连接

```bash
clawmaster provider test openai
```

---

### Anthropic (Claude)

#### 获取 API 密钥

1. 访问 [console.anthropic.com](https://console.anthropic.com)
2. 创建账户
3. 进入 "API Keys" 部分
4. 生成新密钥

#### 配置

**环境变量**:
```bash
export ANTHROPIC_API_KEY=sk-ant-your-api-key-here
```

**配置文件**:
```toml
[providers.anthropic]
enabled = true
api_key_env = "ANTHROPIC_API_KEY"
default_model = "claude-3-opus-20240229"
max_tokens = 4096
temperature = 0.7
```

**可用模型**:
- `claude-3-opus-20240229` - 最强大
- `claude-3-sonnet-20240229` - 平衡性能和成本
- `claude-3-haiku-20240307` - 最快最便宜

#### 测试连接

```bash
clawmaster provider test anthropic
```

---

### OpenRouter

OpenRouter 提供对 100+ 个 LLM 模型的统一访问。

#### 获取 API 密钥

1. 访问 [openrouter.ai](https://openrouter.ai)
2. 注册账户
3. 进入 "Keys" 页面
4. 创建新密钥

#### 配置

**环境变量**:
```bash
export OPENROUTER_API_KEY=sk-or-your-api-key-here
```

**配置文件**:
```toml
[providers.openrouter]
enabled = true
api_key_env = "OPENROUTER_API_KEY"
default_model = "anthropic/claude-3-opus"
```

**热门模型**:
- `anthropic/claude-3-opus`
- `openai/gpt-4-turbo`
- `meta-llama/llama-3-70b-instruct`
- `google/gemini-pro`

#### 测试连接

```bash
clawmaster provider test openrouter
```

---

### Ollama (本地模型)

Ollama 允许您在本地运行开源 LLM 模型。

#### 安装 Ollama

**macOS / Linux**:
```bash
curl -fsSL https://ollama.ai/install.sh | sh
```

**Windows**:
下载安装程序: [ollama.ai/download](https://ollama.ai/download)

#### 下载模型

```bash
# 下载 Llama 3
ollama pull llama3

# 下载 Mistral
ollama pull mistral

# 下载 CodeLlama
ollama pull codellama
```

#### 启动 Ollama 服务

```bash
ollama serve
```

默认运行在 `http://localhost:11434`

#### 配置 ClawMaster

**配置文件**:
```toml
[providers.ollama]
enabled = true
base_url = "http://localhost:11434"
default_model = "llama3"
```

**Web UI**:
1. 打开设置 → LLM Providers
2. 启用 Ollama
3. 输入 URL: `http://localhost:11434`
4. 保存

#### 测试连接

```bash
clawmaster provider test ollama
```

---

### GitHub Copilot

#### 前置要求

- GitHub 账户
- GitHub Copilot 订阅

#### 配置

**获取 Token**:
```bash
gh auth login
gh auth token
```

**环境变量**:
```bash
export GITHUB_TOKEN=ghp_your-token-here
```

**配置文件**:
```toml
[providers.github_copilot]
enabled = true
token_env = "GITHUB_TOKEN"
```

---

## 🎯 高级配置

### 设置默认提供商

```bash
clawmaster provider set-default openai
```

或在配置文件中：
```toml
[providers]
default = "openai"
```

### 配置多个提供商

您可以同时启用多个提供商，ClawMaster 会根据需要选择：

```toml
[providers]
default = "openai"

[providers.openai]
enabled = true
priority = 1

[providers.anthropic]
enabled = true
priority = 2

[providers.ollama]
enabled = true
priority = 3
```

### 模型别名

为常用模型创建别名：

```toml
[providers.aliases]
fast = "gpt-3.5-turbo"
smart = "gpt-4"
local = "ollama:llama3"
```

使用别名：
```bash
clawmaster chat --model fast "快速回答这个问题"
clawmaster chat --model smart "需要深度思考的问题"
```

### 自定义参数

```toml
[providers.openai]
enabled = true
default_model = "gpt-4"

# 自定义参数
max_tokens = 8192
temperature = 0.8
top_p = 0.95
frequency_penalty = 0.0
presence_penalty = 0.0
```

### 速率限制

```toml
[providers.openai.rate_limit]
requests_per_minute = 60
tokens_per_minute = 90000
```

### 重试策略

```toml
[providers.openai.retry]
max_retries = 3
initial_delay_ms = 1000
max_delay_ms = 10000
backoff_multiplier = 2.0
```

---

## 📊 提供商管理

### 列出所有提供商

```bash
clawmaster provider list
```

输出示例：
```
Provider      Status    Model                Priority
─────────────────────────────────────────────────────
openai        ✓ Active gpt-4               1
anthropic     ✓ Active claude-3-opus       2
ollama        ✓ Active llama3              3
openrouter    ✗ Disabled                   -
```

### 启用/禁用提供商

```bash
# 启用
clawmaster provider enable anthropic

# 禁用
clawmaster provider disable openrouter
```

### 删除提供商

```bash
clawmaster provider remove openrouter
```

### 测试所有提供商

```bash
clawmaster provider test-all
```

---

## 💰 成本优化

### 选择合适的模型

| 任务类型 | 推荐模型 | 原因 |
|----------|----------|------|
| 简单问答 | GPT-3.5-turbo, Claude Haiku | 快速且便宜 |
| 代码生成 | GPT-4, Claude Opus | 高质量输出 |
| 长文本处理 | Claude Opus | 大上下文窗口 |
| 本地开发 | Ollama (Llama 3) | 免费 |

### 设置成本限制

```toml
[providers.openai.cost_limit]
daily_limit_usd = 10.0
monthly_limit_usd = 100.0
alert_threshold = 0.8
```

### 监控使用情况

```bash
# 查看使用统计
clawmaster provider stats

# 查看成本
clawmaster provider cost --period month
```

---

## 🔒 安全最佳实践

### 1. 保护 API 密钥

✅ **推荐**:
```bash
# 使用环境变量
export OPENAI_API_KEY=sk-...

# 或使用密钥管理器
clawmaster provider add openai --key-from-keychain
```

❌ **避免**:
```toml
# 不要在配置文件中硬编码
[providers.openai]
api_key = "sk-..."  # 不要这样做！
```

### 2. 定期轮换密钥

```bash
# 每 90 天轮换一次
clawmaster provider rotate-key openai
```

### 3. 限制权限

在 OpenAI/Anthropic 控制台中：
- 设置 API 密钥的使用限制
- 启用 IP 白名单
- 设置每日/每月预算

### 4. 审计日志

```bash
# 查看 API 调用日志
clawmaster logs --filter provider_api_call
```

---

## 🐛 故障排除

### 问题：API 调用失败

**检查 API 密钥**:
```bash
clawmaster provider test openai --verbose
```

**常见错误**:
- `401 Unauthorized` - API 密钥无效
- `429 Too Many Requests` - 超过速率限制
- `500 Internal Server Error` - 提供商服务问题

### 问题：响应缓慢

**检查延迟**:
```bash
clawmaster provider benchmark
```

**优化建议**:
- 使用更快的模型（如 GPT-3.5-turbo）
- 减少 `max_tokens`
- 启用流式响应

### 问题：Ollama 连接失败

**检查 Ollama 服务**:
```bash
curl http://localhost:11434/api/tags
```

**重启 Ollama**:
```bash
ollama serve
```

---

## 📚 下一步

- [设置通信通道](03-setup-channels.md)
- [使用工具和技能](04-using-tools.md)
- [记忆系统](05-memory-system.md)

---

**版本**: 0.10.18  
**更新日期**: 2026-03-13  
**难度**: 中级  
**预计时间**: 10-15 分钟
