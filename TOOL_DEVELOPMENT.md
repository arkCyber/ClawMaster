# ClawMaster Tool 开发指南

**版本**: 1.0.0  
**标准**: DO-178C Level A  
**最后更新**: 2026年3月17日  

---

## 📋 目录

1. [简介](#简介)
2. [Tool 架构](#tool-架构)
3. [开发 Rust Tool](#开发-rust-tool)
4. [开发 MCP Tool](#开发-mcp-tool)
5. [测试要求](#测试要求)
6. [安全规范](#安全规范)
7. [性能要求](#性能要求)
8. [发布流程](#发布流程)

---

## 简介

ClawMaster Tools 是底层执行单元，负责实际执行操作。本指南遵循 DO-178C Level A 标准。

### Tool vs Skill

- **Tool**: 底层执行单元 (如 `bash`, `read_file`)
- **Skill**: 高级能力模块 (使用 Tools 完成任务)

---

## Tool 架构

### Tool Trait

```rust
use async_trait::async_trait;
use serde_json::Value;
use anyhow::Result;

#[async_trait]
pub trait Tool: Send + Sync {
    /// Tool 名称 (唯一标识符)
    fn name(&self) -> &str;
    
    /// Tool 描述
    fn description(&self) -> &str;
    
    /// Tool 参数 schema (JSON Schema)
    fn parameters_schema(&self) -> Value;
    
    /// 执行 Tool
    async fn execute(&self, args: Value) -> Result<Value>;
    
    /// 是否需要用户确认
    fn requires_confirmation(&self) -> bool {
        false
    }
    
    /// 是否为危险操作
    fn is_dangerous(&self) -> bool {
        false
    }
}
```

---

## 开发 Rust Tool

### 步骤 1: 创建 Tool 结构

```rust
// crates/tools/src/my_tool.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use anyhow::Result;

pub struct MyTool;

#[derive(Debug, Deserialize)]
struct MyToolArgs {
    input: String,
    #[serde(default)]
    option: Option<String>,
}

#[derive(Debug, Serialize)]
struct MyToolResult {
    output: String,
    success: bool,
}
```

### 步骤 2: 实现 Tool Trait

```rust
#[async_trait]
impl crate::Tool for MyTool {
    fn name(&self) -> &str {
        "my_tool"
    }
    
    fn description(&self) -> &str {
        "My custom tool that does something useful"
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "input": {
                    "type": "string",
                    "description": "Input string to process"
                },
                "option": {
                    "type": "string",
                    "description": "Optional parameter"
                }
            },
            "required": ["input"]
        })
    }
    
    async fn execute(&self, args: Value) -> Result<Value> {
        // 1. 解析参数
        let args: MyToolArgs = serde_json::from_value(args)?;
        
        // 2. 验证输入
        if args.input.is_empty() {
            anyhow::bail!("input cannot be empty");
        }
        
        // 3. 执行逻辑
        let output = process_input(&args.input, args.option.as_deref());
        
        // 4. 返回结果
        let result = MyToolResult {
            output,
            success: true,
        };
        
        Ok(serde_json::to_value(result)?)
    }
    
    fn requires_confirmation(&self) -> bool {
        false  // 如果是危险操作，返回 true
    }
    
    fn is_dangerous(&self) -> bool {
        false  // 如果会修改系统状态，返回 true
    }
}

fn process_input(input: &str, option: Option<&str>) -> String {
    // 实现你的逻辑
    format!("Processed: {} with option: {:?}", input, option)
}
```

### 步骤 3: 添加测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_my_tool_basic() {
        let tool = MyTool;
        let args = json!({
            "input": "test"
        });
        
        let result = tool.execute(args).await.unwrap();
        assert!(result["success"].as_bool().unwrap());
    }
    
    #[tokio::test]
    async fn test_my_tool_with_option() {
        let tool = MyTool;
        let args = json!({
            "input": "test",
            "option": "custom"
        });
        
        let result = tool.execute(args).await.unwrap();
        assert!(result["success"].as_bool().unwrap());
    }
    
    #[tokio::test]
    async fn test_my_tool_empty_input() {
        let tool = MyTool;
        let args = json!({
            "input": ""
        });
        
        let result = tool.execute(args).await;
        assert!(result.is_err());
    }
}
```

### 步骤 4: 注册 Tool

```rust
// crates/tools/src/lib.rs

mod my_tool;
pub use my_tool::MyTool;

// 在 ToolRegistry 中注册
pub fn register_all_tools(registry: &mut ToolRegistry) {
    registry.register(Box::new(MyTool));
    // ... 其他 tools
}
```

---

## 开发 MCP Tool

### MCP Server 示例 (Node.js)

```javascript
// my-mcp-server.js

const { Server } = require('@modelcontextprotocol/sdk/server/index.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');

const server = new Server({
  name: 'my-mcp-server',
  version: '1.0.0',
}, {
  capabilities: {
    tools: {},
  },
});

// 定义 Tool
server.setRequestHandler('tools/list', async () => {
  return {
    tools: [
      {
        name: 'my_mcp_tool',
        description: 'My MCP tool',
        inputSchema: {
          type: 'object',
          properties: {
            input: {
              type: 'string',
              description: 'Input string',
            },
          },
          required: ['input'],
        },
      },
    ],
  };
});

// 执行 Tool
server.setRequestHandler('tools/call', async (request) => {
  const { name, arguments: args } = request.params;
  
  if (name === 'my_mcp_tool') {
    const result = processInput(args.input);
    return {
      content: [
        {
          type: 'text',
          text: JSON.stringify(result),
        },
      ],
    };
  }
  
  throw new Error(`Unknown tool: ${name}`);
});

function processInput(input) {
  return {
    output: `Processed: ${input}`,
    success: true,
  };
}

// 启动服务器
const transport = new StdioServerTransport();
server.connect(transport);
```

### 配置 MCP Server

```json
// ~/.clawmaster/mcp-servers.json
{
  "my-mcp-server": {
    "command": "node",
    "args": ["/path/to/my-mcp-server.js"],
    "env": {
      "NODE_ENV": "production"
    }
  }
}
```

---

## 测试要求

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tool_name() {
        let tool = MyTool;
        assert_eq!(tool.name(), "my_tool");
    }
    
    #[tokio::test]
    async fn test_tool_description() {
        let tool = MyTool;
        assert!(!tool.description().is_empty());
    }
    
    #[tokio::test]
    async fn test_tool_schema() {
        let tool = MyTool;
        let schema = tool.parameters_schema();
        assert!(schema["type"].as_str().unwrap() == "object");
    }
    
    #[tokio::test]
    async fn test_tool_execute_success() {
        let tool = MyTool;
        let args = json!({"input": "test"});
        let result = tool.execute(args).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_tool_execute_invalid_args() {
        let tool = MyTool;
        let args = json!({"invalid": "field"});
        let result = tool.execute(args).await;
        assert!(result.is_err());
    }
}
```

### 集成测试

```rust
#[tokio::test]
async fn test_tool_in_registry() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(MyTool));
    
    let tool = registry.get("my_tool");
    assert!(tool.is_some());
}

#[tokio::test]
async fn test_tool_execution_flow() {
    let mut registry = ToolRegistry::new();
    registry.register(Box::new(MyTool));
    
    let tool = registry.get("my_tool").unwrap();
    let args = json!({"input": "test"});
    let result = tool.execute(args).await.unwrap();
    
    assert!(result["success"].as_bool().unwrap());
}
```

---

## 安全规范

### 1. 输入验证

```rust
async fn execute(&self, args: Value) -> Result<Value> {
    let args: MyToolArgs = serde_json::from_value(args)?;
    
    // 验证输入长度
    if args.input.len() > 10000 {
        anyhow::bail!("input too long (max 10000 characters)");
    }
    
    // 验证输入格式
    if !args.input.chars().all(|c| c.is_alphanumeric() || c.is_whitespace()) {
        anyhow::bail!("input contains invalid characters");
    }
    
    // 验证路径安全
    if args.input.contains("..") {
        anyhow::bail!("path traversal detected");
    }
    
    // 继续执行...
}
```

### 2. 错误处理

```rust
async fn execute(&self, args: Value) -> Result<Value> {
    // 使用 ? 传播错误
    let args: MyToolArgs = serde_json::from_value(args)?;
    
    // 提供有意义的错误信息
    let result = process_input(&args.input)
        .map_err(|e| anyhow::anyhow!("failed to process input: {}", e))?;
    
    Ok(serde_json::to_value(result)?)
}
```

### 3. 资源限制

```rust
use tokio::time::{timeout, Duration};

async fn execute(&self, args: Value) -> Result<Value> {
    let args: MyToolArgs = serde_json::from_value(args)?;
    
    // 设置超时
    let result = timeout(
        Duration::from_secs(30),
        process_input(&args.input)
    ).await??;
    
    Ok(serde_json::to_value(result)?)
}
```

---

## 性能要求

### 1. 响应时间

- ✅ 简单操作: < 100ms
- ✅ 中等操作: < 1s
- ✅ 复杂操作: < 5s

### 2. 内存使用

- ✅ 单次执行: < 100MB
- ✅ 避免内存泄漏
- ✅ 及时释放资源

### 3. 并发处理

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct MyTool {
    semaphore: Arc<Semaphore>,
}

impl MyTool {
    pub fn new() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(10)), // 最多 10 个并发
        }
    }
}

#[async_trait]
impl Tool for MyTool {
    async fn execute(&self, args: Value) -> Result<Value> {
        let _permit = self.semaphore.acquire().await?;
        
        // 执行逻辑...
        
        Ok(json!({"success": true}))
    }
}
```

---

## 发布流程

### 1. Rust Tool (编译时)

```bash
# 添加到 crates/tools/src/lib.rs
# 提交 PR 到主仓库
git checkout -b add-my-tool
git add crates/tools/src/my_tool.rs
git commit -m "Add my_tool"
git push origin add-my-tool
```

### 2. MCP Tool (运行时)

```bash
# 发布 npm 包
npm publish

# 用户安装
npm install -g my-mcp-server

# 配置
cat >> ~/.clawmaster/mcp-servers.json << 'EOF'
{
  "my-mcp-server": {
    "command": "my-mcp-server"
  }
}
EOF
```

---

## 最佳实践

### 1. 命名规范

- ✅ 使用蛇形命名: `my_tool`
- ✅ 动词开头: `read_file`, `write_file`
- ✅ 简短有意义: `exec` 而不是 `execute_command_in_shell`

### 2. 参数设计

- ✅ 使用 JSON Schema
- ✅ 提供默认值
- ✅ 清晰的描述
- ✅ 验证所有输入

### 3. 错误消息

- ✅ 清晰具体
- ✅ 包含上下文
- ✅ 提供解决方案
- ✅ 避免技术术语

### 4. 文档

- ✅ 完整的 API 文档
- ✅ 使用示例
- ✅ 错误处理说明
- ✅ 性能特性

---

## 示例 Tools

### 示例 1: 文件读取 Tool

```rust
pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }
    
    fn description(&self) -> &str {
        "Read contents of a file"
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path to file"
                }
            },
            "required": ["path"]
        })
    }
    
    async fn execute(&self, args: Value) -> Result<Value> {
        let path: String = args["path"].as_str()
            .ok_or_else(|| anyhow::anyhow!("missing path"))?
            .to_string();
        
        let content = tokio::fs::read_to_string(&path).await?;
        
        Ok(json!({
            "content": content,
            "path": path
        }))
    }
}
```

### 示例 2: HTTP 请求 Tool

```rust
pub struct HttpGetTool {
    client: reqwest::Client,
}

#[async_trait]
impl Tool for HttpGetTool {
    fn name(&self) -> &str {
        "http_get"
    }
    
    fn description(&self) -> &str {
        "Make HTTP GET request"
    }
    
    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "URL to fetch"
                }
            },
            "required": ["url"]
        })
    }
    
    async fn execute(&self, args: Value) -> Result<Value> {
        let url: String = args["url"].as_str()
            .ok_or_else(|| anyhow::anyhow!("missing url"))?
            .to_string();
        
        let response = self.client.get(&url).send().await?;
        let status = response.status().as_u16();
        let body = response.text().await?;
        
        Ok(json!({
            "status": status,
            "body": body
        }))
    }
}
```

---

## 资源链接

- [ClawMaster 官方文档](https://docs.clawmaster.ai)
- [MCP 协议规范](https://modelcontextprotocol.io)
- [Rust async-trait](https://docs.rs/async-trait)
- [JSON Schema](https://json-schema.org)

---

**版本**: 1.0.0  
**标准**: DO-178C Level A  
**最后更新**: 2026年3月17日  
**维护者**: ClawMaster Team
