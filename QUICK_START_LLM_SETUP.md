# ClawMaster WebUI 快速配置指南

**目标**: 解决 "No LLMs Connected" 问题，让 WebUI 完全可用

---

## 🚀 快速开始

### 当前状态
- ✅ WebUI 服务运行中: https://localhost:59233
- ✅ 所有按钮功能正常
- ✅ 所有页面路由已注册
- ⚠️ 需要配置 LLM 提供商

---

## 📝 配置 LLM 提供商（必需）

### 方法 1: 通过 Web 界面配置

#### 步骤 1: 访问设置页面
1. 打开浏览器访问: https://localhost:59233
2. 点击顶部的 **"Settings"** 按钮
3. 或直接访问: https://localhost:59233/settings/llms

#### 步骤 2: 选择提供商
支持的提供商：
```
推荐（免费或易获取）:
- Ollama (本地运行，完全免费)
- LM Studio (本地运行，完全免费)
- OpenRouter (按使用付费，支持多模型)

商业提供商:
- OpenAI (GPT-4, GPT-3.5)
- Anthropic (Claude)
- Google (Gemini)
- Mistral
- Groq (快速推理)
- DeepSeek
- Together
- Fireworks
- Cohere
```

#### 步骤 3: 配置 API 密钥

**选项 A: 使用 Ollama (推荐，本地免费)**
```bash
# 1. 安装 Ollama
brew install ollama  # macOS
# 或访问 https://ollama.ai 下载

# 2. 启动 Ollama
ollama serve

# 3. 下载模型
ollama pull llama2
# 或其他模型: qwen, mistral, codellama 等

# 4. 在 WebUI 中配置
- 提供商: Ollama
- Base URL: http://localhost:11434
- 模型: llama2 (或你下载的模型)
- 无需 API 密钥
```

**选项 B: 使用 OpenAI**
```
1. 访问 https://platform.openai.com/api-keys
2. 创建新的 API 密钥
3. 在 WebUI 中输入:
   - 提供商: OpenAI
   - API Key: sk-...
   - 模型: gpt-4 或 gpt-3.5-turbo
```

**选项 C: 使用 OpenRouter**
```
1. 访问 https://openrouter.ai/settings/keys
2. 创建 API 密钥
3. 在 WebUI 中输入:
   - 提供商: OpenRouter
   - API Key: sk-or-...
   - 模型: 选择任意支持的模型
```

#### 步骤 4: 测试连接
1. 输入 API 密钥后点击 **"Test Connection"**
2. 看到 ✅ 成功提示
3. 点击 **"Save"** 保存配置

#### 步骤 5: 开始使用
1. 返回聊天页面: https://localhost:59233/chats
2. "No LLMs Connected" 应该消失
3. 可以开始聊天了！

---

### 方法 2: 通过配置文件配置

#### 编辑配置文件
```bash
# 打开配置文件
nano ~/.config/clawmaster/clawmaster.toml
```

#### 添加提供商配置
```toml
# Ollama 配置示例
[[providers]]
id = "ollama"
type = "ollama"
base_url = "http://localhost:11434"
models = ["llama2", "codellama"]

# OpenAI 配置示例
[[providers]]
id = "openai"
type = "openai"
api_key = "sk-your-api-key-here"
models = ["gpt-4", "gpt-3.5-turbo"]

# OpenRouter 配置示例
[[providers]]
id = "openrouter"
type = "openrouter"
api_key = "sk-or-your-api-key-here"
base_url = "https://openrouter.ai/api/v1"
```

#### 重启服务
```bash
# 停止当前服务 (Ctrl+C)
# 重新启动
cargo run --bin clawmaster
```

---

## 🔧 验证配置

### 检查提供商状态
1. 访问 https://localhost:59233/settings/llms
2. 应该看到配置的提供商
3. 状态应该显示为 "Connected" 或绿色指示器

### 测试聊天功能
1. 访问 https://localhost:59233/chats
2. 在输入框输入消息: "Hello, how are you?"
3. 按 Enter 或点击发送按钮
4. 应该收到 AI 回复

---

## 📊 所有可用功能

### 已验证的功能
1. ✅ **聊天** - 与 AI 对话
2. ✅ **监控** - 查看系统指标
3. ✅ **设置** - 配置系统
4. ✅ **定时任务** - 创建 cron 任务
5. ✅ **项目管理** - 组织会话
6. ✅ **技能管理** - 添加技能
7. ✅ **通道配置** - 连接消息平台
8. ✅ **多语言** - 6 种语言支持
9. ✅ **主题切换** - 浅色/深色模式
10. ✅ **紧急停止** - 安全控制

### 所有按钮都有对应页面
- 顶部导航栏: 7 个按钮 ✅
- 侧边栏: 3 个主要按钮 ✅
- 设置子页面: 7 个页面 ✅
- 总计: 19 个页面全部可用 ✅

---

## 🎯 推荐配置

### 开发/测试环境
```
提供商: Ollama
模型: llama2 或 qwen
优点: 免费、本地、快速
```

### 生产环境
```
提供商: OpenAI 或 Anthropic
模型: gpt-4 或 claude-3
优点: 高质量、稳定、支持好
```

### 预算有限
```
提供商: OpenRouter
模型: 选择便宜的模型
优点: 按使用付费、多模型选择
```

---

## 🐛 常见问题

### Q: "No LLMs Connected" 不消失
**A**: 
1. 确认 API 密钥正确
2. 检查网络连接
3. 查看浏览器控制台错误
4. 刷新页面 (Ctrl+R 或 Cmd+R)

### Q: Ollama 连接失败
**A**:
1. 确认 Ollama 服务运行: `ollama serve`
2. 检查端口: `lsof -i :11434`
3. 确认模型已下载: `ollama list`

### Q: API 密钥无效
**A**:
1. 重新生成 API 密钥
2. 检查密钥格式（不要有空格）
3. 确认账户有余额（商业提供商）

### Q: 模型列表为空
**A**:
1. 等待提供商连接完成
2. 刷新页面
3. 检查提供商 API 状态

---

## 📚 相关文档

- **完整审计报告**: `WEBUI_BUTTON_FUNCTIONALITY_AUDIT.md`
- **测试执行报告**: `WEBUI_TEST_EXECUTION_REPORT.md`
- **按钮功能审计**: `TAURI_BUTTON_FUNCTIONALITY_AUDIT.md`

---

## ✅ 配置检查清单

- [ ] WebUI 服务运行中
- [ ] 可以访问 https://localhost:59233
- [ ] 已选择 LLM 提供商
- [ ] 已配置 API 密钥或本地服务
- [ ] 测试连接成功
- [ ] 保存配置
- [ ] "No LLMs Connected" 消失
- [ ] 可以发送消息并收到回复
- [ ] 所有按钮功能正常
- [ ] 页面导航正常

---

**配置完成后，ClawMaster WebUI 即可完全使用！** 🚀✅
