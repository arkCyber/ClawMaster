# ClawMaster 集成指南

**日期**: 2026-03-13  
**版本**: 0.10.18  
**目标**: 集成所有新实现的功能到主系统

---

## 🎯 概述

本指南提供了将新实现的功能集成到 ClawMaster 主系统的详细步骤。

**新功能**:
1. SOUL.md 个性化系统
2. 配置模板系统
3. Agentic Loop 智能体循环
4. Chat Catchup 群聊追赶

---

## 📋 集成清单

### 1. SOUL.md 个性化系统 ✅

**状态**: 已完成，可直接使用

**集成步骤**:

```rust
// 在 clawmaster-agents 中使用
use clawmaster_soul::Soul;

async fn create_agent_with_personality() -> Result<()> {
    // 加载 SOUL.md
    let soul = Soul::from_file("SOUL.md").await?;
    
    // 获取系统提示词
    let system_prompt = soul.get_system_prompt();
    
    // 在 LLM 调用中使用
    let response = llm_client.chat(system_prompt, user_message).await?;
    
    Ok(())
}
```

**使用示例**:

```rust
// 创建默认 SOUL.md
Soul::create_default("SOUL.md").await?;

// 解析现有文件
let soul = Soul::from_file("SOUL.md").await?;

// 获取个性特征
let style = soul.get_personality_trait("style");
let tone = soul.get_personality_trait("tone");

// 获取行为规则
let always_do = soul.get_behavior_rules("always");
let never_do = soul.get_behavior_rules("never");

// 生成系统提示词
let prompt = soul.get_system_prompt();

// 热重载
soul.reload().await?;
```

### 2. 配置模板系统 ✅

**状态**: 已完成，集成到 setup-wizard

**集成步骤**:

```rust
// 在设置向导中使用
use clawmaster_setup_wizard::{ConfigTemplate, WizardState};

fn select_template() -> ConfigTemplate {
    // 用户选择模板
    let template = ConfigTemplate::Production;
    
    // 获取推荐配置
    let providers = template.recommended_providers();
    let channels = template.recommended_channels();
    
    // 应用到配置
    config.providers = providers;
    config.channels = channels;
    
    template
}
```

**可用模板**:

```rust
// 1. Custom - 自定义配置
ConfigTemplate::Custom

// 2. Basic - 快速开始
ConfigTemplate::Basic
// 推荐: OpenAI + WebUI

// 3. Development - 开发环境
ConfigTemplate::Development
// 推荐: OpenAI + Ollama + WebUI + Telegram

// 4. Production - 生产环境
ConfigTemplate::Production
// 推荐: OpenAI + Anthropic + WebUI + Telegram + Slack

// 5. Minimal - 最小配置
ConfigTemplate::Minimal
// 推荐: Ollama + WebUI

// 6. Enterprise - 企业配置
ConfigTemplate::Enterprise
// 推荐: OpenAI + Anthropic + Gemini + WebUI + Telegram + Slack + Discord
```

### 3. Agentic Loop 智能体循环 ✅

**状态**: 已完成，待集成到 agents

**集成步骤**:

#### 步骤 1: 添加依赖

在 `crates/agents/Cargo.toml` 中添加:

```toml
[dependencies]
clawmaster-agentic-loop = { workspace = true }
```

#### 步骤 2: 集成到 Agent

```rust
use clawmaster_agentic_loop::{AgenticLoop, AgenticLoopConfig, ReasoningResult, ToolCall};

pub struct Agent {
    // 现有字段...
    agentic_loop: Option<AgenticLoop>,
}

impl Agent {
    pub fn with_agentic_loop(mut self, config: AgenticLoopConfig) -> Self {
        let agentic_loop = AgenticLoop::new(config);
        
        // 注册 ClawMaster 工具
        self.register_tools(&agentic_loop);
        
        self.agentic_loop = Some(agentic_loop);
        self
    }
    
    fn register_tools(&self, agentic_loop: &AgenticLoop) {
        // 注册文件操作工具
        agentic_loop.registry().register(Box::new(ReadFileTool));
        agentic_loop.registry().register(Box::new(WriteFileTool));
        
        // 注册 Web 工具
        agentic_loop.registry().register(Box::new(WebSearchTool));
        agentic_loop.registry().register(Box::new(WebFetchTool));
        
        // 注册系统工具
        agentic_loop.registry().register(Box::new(BashTool));
        agentic_loop.registry().register(Box::new(GlobTool));
    }
    
    pub async fn execute_task_with_loop(&self, task: &str) -> Result<String> {
        if let Some(loop_instance) = &self.agentic_loop {
            loop_instance.run_until_complete(task, |ctx| {
                // 使用 LLM 进行推理
                self.reason_with_llm(ctx)
            }).await
        } else {
            Err(anyhow!("Agentic loop not configured"))
        }
    }
    
    fn reason_with_llm(&self, ctx: &ExecutionContext) -> Result<ReasoningResult> {
        // 构建提示词
        let prompt = format!(
            "Task: {}\n\nPrevious thoughts:\n{}\n\nLast tool result: {:?}\n\nWhat should I do next?",
            ctx.task,
            ctx.thoughts.join("\n"),
            ctx.get_last_tool_result()
        );
        
        // 调用 LLM
        let response = self.llm_client.chat(&prompt)?;
        
        // 解析响应
        self.parse_llm_response(response)
    }
}
```

#### 步骤 3: 实现工具

```rust
use clawmaster_agentic_loop::Tool;
use async_trait::async_trait;

struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }
    
    fn description(&self) -> &str {
        "Read contents of a file"
    }
    
    async fn execute(&self, args: serde_json::Value) -> Result<String> {
        let path = args["path"].as_str().ok_or(anyhow!("Missing path"))?;
        let content = tokio::fs::read_to_string(path).await?;
        Ok(content)
    }
}

struct WebSearchTool;

#[async_trait]
impl Tool for WebSearchTool {
    fn name(&self) -> &str {
        "web_search"
    }
    
    fn description(&self) -> &str {
        "Search the web for information"
    }
    
    async fn execute(&self, args: serde_json::Value) -> Result<String> {
        let query = args["query"].as_str().ok_or(anyhow!("Missing query"))?;
        // 使用现有的 web search 实现
        let results = search_web(query).await?;
        Ok(serde_json::to_string(&results)?)
    }
}
```

#### 步骤 4: 使用示例

```rust
// 创建带有 agentic loop 的 agent
let config = AgenticLoopConfig {
    max_iterations: 10,
    timeout_seconds: 300,
    enable_memory: true,
};

let agent = Agent::new()
    .with_agentic_loop(config)
    .build()?;

// 执行复杂任务
let result = agent.execute_task_with_loop(
    "Search for Rust async programming best practices and create a summary document"
).await?;

println!("Task completed: {}", result);
```

### 4. Chat Catchup 群聊追赶 ✅

**状态**: 已集成，待完善测试

**集成步骤**:

#### 步骤 1: 添加依赖

在需要使用的 crate 中添加:

```toml
[dependencies]
clawmaster-chat-catchup = { workspace = true }
```

#### 步骤 2: 集成到通道处理

```rust
use clawmaster_chat_catchup::{ChatCatchup, CatchupConfig, ChatCatchupInterface};

pub struct ChannelHandler {
    catchup: Arc<dyn ChatCatchupInterface>,
}

impl ChannelHandler {
    pub fn new() -> Result<Self> {
        let config = CatchupConfig {
            max_messages_per_batch: 100,
            max_lookback_period: Duration::from_secs(86400), // 24 hours
            catchup_timeout: Duration::from_secs(30),
            enable_clustering: true,
            enable_summarization: true,
            max_context_length: 10000,
            message_filter: Default::default(),
            strategy: CatchupStrategy::Adaptive {
                summary_threshold: 20,
                cluster_threshold: 10,
                old_message_threshold: Duration::from_secs(3600),
            },
        };
        
        let message_store = Arc::new(MessageStoreImpl::new());
        let session_store = Arc::new(SessionStoreImpl::new());
        
        let catchup = Arc::new(ChatCatchup::new(config, message_store, session_store)?);
        
        Ok(Self { catchup })
    }
    
    pub async fn handle_user_return(&self, channel_id: &str, user_id: &str) -> Result<()> {
        // 用户返回时执行追赶
        let result = self.catchup.catch_up(channel_id, user_id).await?;
        
        if result.had_unread {
            // 生成追赶摘要
            let summary = format!(
                "Welcome back! You missed {} messages. Here's what happened:\n\n{}",
                result.messages_processed,
                result.context.summary
            );
            
            // 发送摘要给用户
            self.send_message(channel_id, user_id, &summary).await?;
        }
        
        Ok(())
    }
}
```

#### 步骤 3: 实现存储接口

```rust
use clawmaster_chat_catchup::catchup_engine::{MessageStore, SessionStore};

struct MessageStoreImpl {
    db: Arc<Database>,
}

#[async_trait]
impl MessageStore for MessageStoreImpl {
    async fn get_messages_since(
        &self,
        channel_id: &str,
        since: DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<ChatMessage>, CatchupError> {
        // 从数据库获取消息
        let messages = self.db.query_messages(channel_id, since, limit).await?;
        Ok(messages)
    }
    
    async fn get_message_count(
        &self,
        channel_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<usize, CatchupError> {
        let count = self.db.count_messages(channel_id, from, to).await?;
        Ok(count)
    }
}

struct SessionStoreImpl {
    db: Arc<Database>,
}

#[async_trait]
impl SessionStore for SessionStoreImpl {
    async fn get_last_read_timestamp(
        &self,
        channel_id: &str,
        user_id: &str,
    ) -> Result<Option<DateTime<Utc>>, CatchupError> {
        let timestamp = self.db.get_last_read(channel_id, user_id).await?;
        Ok(timestamp)
    }
    
    async fn update_last_read_timestamp(
        &self,
        channel_id: &str,
        user_id: &str,
        timestamp: DateTime<Utc>,
    ) -> Result<(), CatchupError> {
        self.db.update_last_read(channel_id, user_id, timestamp).await?;
        Ok(())
    }
}
```

---

## 🔧 完整集成示例

### 示例 1: 带有所有功能的 Agent

```rust
use clawmaster_soul::Soul;
use clawmaster_agentic_loop::{AgenticLoop, AgenticLoopConfig};
use clawmaster_chat_catchup::{ChatCatchup, CatchupConfig};

pub struct EnhancedAgent {
    soul: Soul,
    agentic_loop: AgenticLoop,
    chat_catchup: Arc<dyn ChatCatchupInterface>,
}

impl EnhancedAgent {
    pub async fn new() -> Result<Self> {
        // 加载个性化配置
        let soul = Soul::from_file("SOUL.md").await?;
        
        // 创建 agentic loop
        let loop_config = AgenticLoopConfig::default();
        let agentic_loop = AgenticLoop::new(loop_config);
        
        // 注册工具
        Self::register_all_tools(&agentic_loop);
        
        // 创建 chat catchup
        let catchup_config = CatchupConfig::default();
        let message_store = Arc::new(MessageStoreImpl::new());
        let session_store = Arc::new(SessionStoreImpl::new());
        let chat_catchup = Arc::new(ChatCatchup::new(
            catchup_config,
            message_store,
            session_store,
        )?);
        
        Ok(Self {
            soul,
            agentic_loop,
            chat_catchup,
        })
    }
    
    fn register_all_tools(agentic_loop: &AgenticLoop) {
        // 文件操作
        agentic_loop.registry().register(Box::new(ReadFileTool));
        agentic_loop.registry().register(Box::new(WriteFileTool));
        
        // Web 操作
        agentic_loop.registry().register(Box::new(WebSearchTool));
        agentic_loop.registry().register(Box::new(WebFetchTool));
        
        // 系统操作
        agentic_loop.registry().register(Box::new(BashTool));
    }
    
    pub async fn handle_message(
        &self,
        channel_id: &str,
        user_id: &str,
        message: &str,
    ) -> Result<String> {
        // 1. 检查是否需要追赶
        let catchup_result = self.chat_catchup.catch_up(channel_id, user_id).await?;
        
        let mut context = String::new();
        if catchup_result.had_unread {
            context = format!("Recent context: {}\n\n", catchup_result.context.summary);
        }
        
        // 2. 使用个性化系统提示词
        let system_prompt = self.soul.get_system_prompt();
        
        // 3. 使用 agentic loop 处理复杂任务
        let task = format!("{}{}", context, message);
        let response = self.agentic_loop.run_until_complete(&task, |ctx| {
            self.reason_with_personality(ctx, &system_prompt)
        }).await?;
        
        Ok(response)
    }
    
    fn reason_with_personality(
        &self,
        ctx: &ExecutionContext,
        system_prompt: &str,
    ) -> Result<ReasoningResult> {
        // 结合个性化和上下文进行推理
        let prompt = format!(
            "{}\n\nTask: {}\n\nContext: {:?}",
            system_prompt,
            ctx.task,
            ctx.get_summary()
        );
        
        // 调用 LLM 并解析响应
        // ...
        
        Ok(ReasoningResult {
            thought: "Analyzing task...".to_string(),
            tool_call: None,
            is_complete: false,
            final_answer: None,
        })
    }
}
```

### 示例 2: 配置模板在设置向导中的使用

```rust
use clawmaster_setup_wizard::{Wizard, WizardConfig, ConfigTemplate};

async fn run_setup_wizard() -> Result<()> {
    let mut wizard = Wizard::new()?;
    
    // 用户选择模板
    let template = wizard.select_template()?;
    
    match template {
        ConfigTemplate::Basic => {
            println!("Setting up basic configuration...");
            wizard.config.providers = vec![Provider::OpenAI];
            wizard.config.channels = vec![Channel::WebUI];
        }
        ConfigTemplate::Production => {
            println!("Setting up production configuration...");
            wizard.config.providers = vec![Provider::OpenAI, Provider::Anthropic];
            wizard.config.channels = vec![Channel::WebUI, Channel::Telegram, Channel::Slack];
        }
        ConfigTemplate::Enterprise => {
            println!("Setting up enterprise configuration...");
            wizard.config.providers = vec![
                Provider::OpenAI,
                Provider::Anthropic,
                Provider::Gemini,
            ];
            wizard.config.channels = vec![
                Channel::WebUI,
                Channel::Telegram,
                Channel::Slack,
                Channel::Discord,
            ];
        }
        _ => {
            // 其他模板...
        }
    }
    
    wizard.save_config().await?;
    Ok(())
}
```

---

## 🧪 测试集成

### 测试所有新功能

```bash
# 测试 SOUL.md
cargo test -p clawmaster-soul

# 测试配置模板
cargo test -p clawmaster-setup-wizard

# 测试 Agentic Loop
cargo test -p clawmaster-agentic-loop

# 测试 Chat Catchup
cargo test -p clawmaster-chat-catchup --test basic_tests

# 测试完整集成
cargo test --workspace
```

---

## 📋 集成检查清单

### SOUL.md 个性化系统
- [x] Crate 创建完成
- [x] 核心功能实现
- [x] 测试通过（4/4）
- [x] 文档完成
- [ ] 集成到 agents
- [ ] 使用示例创建

### 配置模板系统
- [x] 功能实现完成
- [x] UI 实现完成
- [x] 测试通过（12/12）
- [x] 集成到 setup-wizard
- [ ] 用户指南创建

### Agentic Loop
- [x] Crate 创建完成
- [x] 核心功能实现
- [x] 测试通过（14/14）
- [x] 文档完成
- [ ] 集成到 agents
- [ ] 工具注册实现
- [ ] 使用示例创建

### Chat Catchup
- [x] Crate 集成到 workspace
- [x] 基础测试创建
- [x] 部分测试通过（5/7）
- [ ] 修复剩余测试
- [ ] 集成到通道处理
- [ ] 存储接口实现

---

## 🚀 下一步行动

### 立即可做
1. 修复 Chat Catchup 剩余 2 个测试
2. 在 agents crate 中集成 Agentic Loop
3. 实现工具注册系统
4. 创建完整的使用示例

### 本周计划
1. 完成所有集成工作
2. 创建端到端测试
3. 更新用户文档
4. 性能优化

---

## 📚 相关文档

1. [SOUL.md README](crates/soul/README.md)
2. [Agentic Loop README](crates/agentic-loop/README.md)
3. [Chat Catchup README](crates/chat-catchup/README.md)
4. [NEXT_PHASE_ROADMAP.md](NEXT_PHASE_ROADMAP.md)

---

**创建日期**: 2026-03-13  
**版本**: 0.10.18  
**状态**: 准备集成
