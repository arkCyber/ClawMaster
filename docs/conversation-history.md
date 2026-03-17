# 会话记录系统 (Conversation History System)

## 概述

ClawMaster 会话记录系统提供完整的问答对话追踪、时间戳记录、问题解决状态管理和未解决问题处理机制。

## 核心功能

### 1. 会话记录 (Conversation Turns)
- ✅ **完整的问答对记录**：每次用户提问和助手回答都被完整记录
- ✅ **精确时间戳**：记录问题提出时间和回答完成时间
- ✅ **响应时间追踪**：计算并记录每次回答的耗时（毫秒级）
- ✅ **问题解决状态**：标记每个问题是否已解决（resolved/unresolved/partial）
- ✅ **用户反馈**：支持用户对回答进行正面/负面反馈
- ✅ **上下文信息**：记录使用的模型、提供商、工具调用等元数据
- ✅ **Token 使用统计**：追踪输入和输出 token 数量
- ✅ **错误追踪**：记录是否发生错误及错误详情

### 2. 未解决问题追踪 (Unresolved Issues)
- ✅ **自动问题检测**：识别未能成功回答的问题
- ✅ **优先级管理**：支持 low/normal/high/critical 四级优先级
- ✅ **状态追踪**：open/in_progress/resolved/abandoned 状态流转
- ✅ **重试机制**：记录尝试次数和最后尝试时间
- ✅ **解决方法记录**：记录问题如何被解决（retry/escalation/manual/alternative）
- ✅ **问题摘要**：自动生成问题摘要便于快速浏览

### 3. 会话元数据 (Conversation Metadata)
- ✅ **统计信息**：总轮次、已解决/未解决问题数量
- ✅ **时间范围**：首次和最后一次消息时间
- ✅ **性能指标**：平均响应时间、总 token 使用量
- ✅ **质量评估**：错误次数、正面/负面反馈统计

### 4. 搜索和检索
- ✅ **全文搜索**：在用户消息和助手回答中搜索关键词
- ✅ **会话过滤**：按会话 key 过滤搜索结果
- ✅ **时间排序**：按创建时间倒序返回结果
- ✅ **结果限制**：支持限制返回数量避免过载

## 数据库架构

### conversation_turns 表
```sql
CREATE TABLE conversation_turns (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    session_key         TEXT    NOT NULL,
    turn_number         INTEGER NOT NULL,
    user_message        TEXT    NOT NULL,
    assistant_response  TEXT    NOT NULL,
    created_at          INTEGER NOT NULL,  -- Unix timestamp
    response_at         INTEGER NOT NULL,  -- Unix timestamp
    duration_ms         INTEGER NOT NULL,
    
    -- 问题解决追踪
    is_resolved         INTEGER NOT NULL DEFAULT 0,
    resolution_status   TEXT,
    user_feedback       TEXT,
    feedback_at         INTEGER,
    
    -- 上下文和元数据
    model_id            TEXT    NOT NULL,
    provider_name       TEXT    NOT NULL,
    tool_calls_count    INTEGER NOT NULL DEFAULT 0,
    tool_calls_json     TEXT,
    
    -- Token 使用
    input_tokens        INTEGER NOT NULL DEFAULT 0,
    output_tokens       INTEGER NOT NULL DEFAULT 0,
    
    -- 错误追踪
    had_error           INTEGER NOT NULL DEFAULT 0,
    error_message       TEXT,
    
    UNIQUE(session_key, turn_number)
);
```

### unresolved_issues 表
```sql
CREATE TABLE unresolved_issues (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_turn_id INTEGER NOT NULL,
    session_key         TEXT    NOT NULL,
    issue_summary       TEXT    NOT NULL,
    user_query          TEXT    NOT NULL,
    failed_response     TEXT,
    created_at          INTEGER NOT NULL,
    last_attempt_at     INTEGER,
    resolved_at         INTEGER,
    attempt_count       INTEGER NOT NULL DEFAULT 1,
    priority            TEXT    NOT NULL DEFAULT 'normal',
    status              TEXT    NOT NULL DEFAULT 'open',
    resolution_method   TEXT,
    resolution_notes    TEXT,
    resolved_by         TEXT,
    
    FOREIGN KEY (conversation_turn_id) REFERENCES conversation_turns(id)
);
```

### conversation_metadata 表
```sql
CREATE TABLE conversation_metadata (
    session_key         TEXT    PRIMARY KEY,
    total_turns         INTEGER NOT NULL DEFAULT 0,
    resolved_count      INTEGER NOT NULL DEFAULT 0,
    unresolved_count    INTEGER NOT NULL DEFAULT 0,
    first_message_at    INTEGER NOT NULL,
    last_message_at     INTEGER NOT NULL,
    avg_response_time_ms INTEGER NOT NULL DEFAULT 0,
    total_tokens        INTEGER NOT NULL DEFAULT 0,
    error_count         INTEGER NOT NULL DEFAULT 0,
    positive_feedback   INTEGER NOT NULL DEFAULT 0,
    negative_feedback   INTEGER NOT NULL DEFAULT 0
);
```

## RPC API 接口

### 1. 记录会话轮次
```javascript
// 请求
{
    "method": "conversation.recordTurn",
    "params": {
        "session_key": "session-123",
        "turn_number": 1,
        "user_message": "如何使用 Rust 编写高性能代码？",
        "assistant_response": "Rust 提供了零成本抽象...",
        "model_id": "gpt-4",
        "provider_name": "openai",
        "tool_calls_count": 0,
        "input_tokens": 50,
        "output_tokens": 200,
        "duration_ms": 1500
    }
}

// 响应
{
    "turn_id": 123,
    "success": true
}
```

### 2. 获取会话历史
```javascript
// 请求
{
    "method": "conversation.getHistory",
    "params": {
        "session_key": "session-123",
        "limit": 50
    }
}

// 响应
[
    {
        "id": 123,
        "session_key": "session-123",
        "turn_number": 1,
        "user_message": "如何使用 Rust...",
        "assistant_response": "Rust 提供了...",
        "created_at": 1700000000,
        "response_at": 1700000002,
        "duration_ms": 1500,
        "is_resolved": true,
        "model_id": "gpt-4",
        "provider_name": "openai",
        ...
    }
]
```

### 3. 更新解决状态
```javascript
{
    "method": "conversation.updateResolution",
    "params": {
        "turn_id": 123,
        "is_resolved": true,
        "status": "resolved"
    }
}
```

### 4. 添加用户反馈
```javascript
{
    "method": "conversation.addFeedback",
    "params": {
        "turn_id": 123,
        "feedback": "非常有帮助！",
        "is_positive": true
    }
}
```

### 5. 记录未解决问题
```javascript
{
    "method": "conversation.recordIssue",
    "params": {
        "conversation_turn_id": 123,
        "session_key": "session-123",
        "issue_summary": "无法连接数据库",
        "user_query": "帮我修复数据库连接错误",
        "failed_response": "尝试了多种方法但都失败了",
        "priority": "high"
    }
}
```

### 6. 获取未解决问题列表
```javascript
{
    "method": "conversation.getIssues",
    "params": {
        "status": "open",
        "priority": "high",
        "limit": 20
    }
}
```

### 7. 更新问题状态
```javascript
{
    "method": "conversation.updateIssue",
    "params": {
        "issue_id": 456,
        "status": "resolved",
        "resolution_method": "retry",
        "notes": "通过重启服务解决"
    }
}
```

### 8. 搜索会话历史
```javascript
{
    "method": "conversation.searchHistory",
    "params": {
        "query": "Rust 性能",
        "session_key": "session-123",  // 可选
        "limit": 50
    }
}
```

### 9. 获取会话元数据
```javascript
{
    "method": "conversation.getMetadata",
    "params": {
        "session_key": "session-123"
    }
}

// 响应
{
    "session_key": "session-123",
    "total_turns": 25,
    "resolved_count": 23,
    "unresolved_count": 2,
    "first_message_at": 1700000000,
    "last_message_at": 1700010000,
    "avg_response_time_ms": 1800,
    "total_tokens": 5000,
    "error_count": 1,
    "positive_feedback": 20,
    "negative_feedback": 2
}
```

## 使用示例

### Rust 代码集成

```rust
use clawmaster_gateway::conversation_history::{
    ConversationHistoryStore, SqliteConversationHistory, ConversationTurn,
};
use sqlx::SqlitePool;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化数据库连接
    let pool = SqlitePool::connect("sqlite:clawmaster.db").await?;
    
    // 运行迁移
    clawmaster_gateway::run_migrations(&pool).await?;
    
    // 创建会话历史存储
    let store: Arc<dyn ConversationHistoryStore> = 
        Arc::new(SqliteConversationHistory::new(pool));
    
    // 记录一次会话
    let turn = ConversationTurn {
        id: 0,
        session_key: "my-session".to_string(),
        turn_number: 1,
        user_message: "你好！".to_string(),
        assistant_response: "你好！我能帮你什么？".to_string(),
        created_at: 1700000000,
        response_at: 1700000001,
        duration_ms: 1000,
        is_resolved: true,
        resolution_status: Some("resolved".to_string()),
        user_feedback: None,
        feedback_at: None,
        model_id: "gpt-4".to_string(),
        provider_name: "openai".to_string(),
        tool_calls_count: 0,
        tool_calls_json: None,
        input_tokens: 10,
        output_tokens: 20,
        had_error: false,
        error_message: None,
    };
    
    let turn_id = store.record_turn(&turn).await?;
    println!("记录了会话轮次，ID: {}", turn_id);
    
    // 获取会话历史
    let history = store.get_session_history("my-session", Some(10)).await?;
    println!("会话历史共 {} 条", history.len());
    
    // 添加反馈
    store.add_feedback(turn_id, "很有帮助！", true).await?;
    
    // 获取元数据
    if let Some(metadata) = store.get_metadata("my-session").await? {
        println!("总轮次: {}", metadata.total_turns);
        println!("平均响应时间: {}ms", metadata.avg_response_time_ms);
    }
    
    Ok(())
}
```

### 未解决问题处理流程

```rust
use clawmaster_gateway::conversation_history::{
    UnresolvedIssue, IssueStatus, IssuePriority,
};

// 1. 检测到问题时记录
let issue = UnresolvedIssue {
    id: 0,
    conversation_turn_id: turn_id,
    session_key: "my-session".to_string(),
    issue_summary: "工具执行失败".to_string(),
    user_query: "帮我运行这个脚本".to_string(),
    failed_response: Some("脚本执行超时".to_string()),
    created_at: 1700000000,
    last_attempt_at: None,
    resolved_at: None,
    attempt_count: 1,
    priority: "high".to_string(),
    status: "open".to_string(),
    resolution_method: None,
    resolution_notes: None,
    resolved_by: None,
};

let issue_id = store.record_unresolved_issue(&issue).await?;

// 2. 获取所有未解决的高优先级问题
let high_priority_issues = store.get_unresolved_issues(
    Some(IssueStatus::Open),
    Some(IssuePriority::High),
    Some(20)
).await?;

// 3. 处理问题后更新状态
store.update_issue_status(
    issue_id,
    IssueStatus::Resolved,
    Some("retry"),
    Some("增加超时时间后重试成功")
).await?;
```

## 性能优化

### 索引策略
```sql
-- 会话查询索引
CREATE INDEX idx_conversation_turns_session 
    ON conversation_turns(session_key, turn_number DESC);

-- 时间范围查询索引
CREATE INDEX idx_conversation_turns_created 
    ON conversation_turns(created_at DESC);

-- 未解决问题快速查询
CREATE INDEX idx_conversation_turns_unresolved 
    ON conversation_turns(is_resolved, created_at DESC) 
    WHERE is_resolved = 0;

-- 问题状态查询索引
CREATE INDEX idx_unresolved_issues_status 
    ON unresolved_issues(status, priority, created_at DESC);
```

### 查询优化建议
1. **限制返回数量**：始终使用 `limit` 参数避免返回过多数据
2. **会话级查询**：优先使用 `session_key` 过滤减少扫描范围
3. **时间窗口过滤**：对历史数据查询添加时间范围限制
4. **定期归档**：将旧会话数据归档到单独的表或文件

## 测试覆盖

### 单元测试
- ✅ `test_record_and_retrieve_turn` - 记录和检索会话轮次
- ✅ `test_unresolved_issue_tracking` - 未解决问题追踪
- ✅ `test_feedback_tracking` - 用户反馈追踪
- ✅ `test_search_history` - 历史搜索功能

### RPC 测试
- ✅ `test_record_turn_rpc` - RPC 记录接口
- ✅ `test_get_history_rpc` - RPC 查询接口
- ✅ `test_record_and_get_issues_rpc` - RPC 问题管理接口

### 测试运行
```bash
# 运行所有会话历史测试
cargo test -p clawmaster-gateway conversation_history

# 运行 RPC 测试
cargo test -p clawmaster-gateway conversation_history_rpc

# 查看详细输出
cargo test -p clawmaster-gateway conversation_history -- --nocapture
```

## 监控和可观测性

### Tracing 日志
```rust
// 记录会话时的日志
tracing::debug!(
    session_key = %turn.session_key,
    turn_number = turn.turn_number,
    turn_id = turn_id,
    "recorded conversation turn"
);

// 记录未解决问题时的警告
tracing::warn!(
    issue_id,
    session_key = %issue.session_key,
    priority = %issue.priority,
    "recorded unresolved issue"
);
```

### Metrics 指标
建议添加以下 metrics：
- `conversation_turns_total` - 总会话轮次计数
- `unresolved_issues_total` - 未解决问题计数（按优先级分组）
- `conversation_response_time_ms` - 响应时间直方图
- `conversation_feedback_total` - 反馈计数（正面/负面）

## 最佳实践

### 1. 及时记录
在每次助手回答完成后立即记录会话轮次，确保数据完整性。

### 2. 异步处理
会话记录操作应该异步执行，不阻塞主要的聊天流程。

### 3. 错误处理
记录失败不应导致整个聊天流程失败，应该记录错误日志并继续。

### 4. 隐私保护
敏感信息（如密码、API key）不应记录到会话历史中。

### 5. 数据清理
定期清理或归档旧的会话数据，避免数据库过大影响性能。

## 未来扩展

### 计划中的功能
- [ ] 会话摘要生成
- [ ] 智能问题分类
- [ ] 自动重试机制
- [ ] 问题优先级自动调整
- [ ] 会话质量评分
- [ ] 导出为多种格式（JSON, CSV, Markdown）
- [ ] 可视化仪表板

## 故障排查

### 常见问题

**Q: 会话记录没有保存？**
A: 检查数据库连接和迁移是否正确执行。运行 `clawmaster_gateway::run_migrations(&pool).await?`

**Q: 搜索结果为空？**
A: 确认搜索关键词正确，SQLite 的 LIKE 查询区分大小写。

**Q: 未解决问题没有被追踪？**
A: 确保在检测到错误时调用 `record_unresolved_issue` 方法。

**Q: 性能下降？**
A: 检查数据库大小，考虑添加索引或归档旧数据。

## 贡献指南

欢迎贡献代码和建议！请遵循以下步骤：
1. Fork 仓库
2. 创建功能分支
3. 编写测试
4. 提交 Pull Request

## 许可证

与 ClawMaster 主项目相同的许可证。
