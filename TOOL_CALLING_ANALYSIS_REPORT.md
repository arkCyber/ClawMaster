# ClawMaster 工具调用能力分析报告

**测试时间**: 2026-03-20  
**测试目的**: 验证 CLI 接口通过自然语言命令实际调用后端工具的能力  
**后端模型**: Llama 3.1 8B (GGUF Q4_K_M)

---

## 执行摘要

**核心发现**: 本地 LLM 模型（Llama 3.1 8B）对工具调用的支持**不完整**。部分工具能够被正确调用并返回实际结果，但大部分工具只是被解释而非执行。

**实际执行率**: **40%** (4/10 测试通过)

---

## 测试结果详情

### ✅ 成功调用的工具 (4个)

| 工具 | 测试命令 | 结果 | 说明 |
|------|---------|------|------|
| `calc` | "计算 123 + 456" | ✅ 返回 "579" | 完美执行 |
| `calc` | "计算 2 的 10 次方" | ✅ 返回 "1024" | 完美执行 |
| `memory_save` | "记住：我喜欢 Rust" | ✅ 确认保存 | 实际调用工具 |
| `memory_search` | "我喜欢什么编程语言" | ✅ 返回 "Rust" | 实际调用工具 |

### ❌ 失败的工具 (6个 - 只解释不执行)

| 工具 | 测试命令 | 实际响应 | 问题 |
|------|---------|---------|------|
| `news_search` | "搜索最新科技新闻" | "你可以使用 `news_search category=\"tech\"`" | 只解释，未调用 |
| `exec` | "列出当前目录的文件" | "你可以使用 `sandbox ls`" | 只解释，未调用 |
| `task_list` | "显示所有任务" | "你可以使用 `task_list`" | 只解释，未调用 |
| `sessions_list` | "显示所有会话" | "你可以使用 `sessions_list`" | 只解释，未调用 |
| `web_search` | "搜索 Rust 教程" | "你可以使用 `memory_search query=\"Rust\"`" | 只解释，未调用 |
| `browser` | "打开 https://www.rust-lang.org" | "你可以使用 `browser open ...`" | 只解释，未调用 |

---

## 根本原因分析

### 1. 模型能力限制

**Llama 3.1 8B Q4_K_M** 是一个量化的小型模型：
- **参数量**: 8B (量化后约 4.5GB)
- **工具调用能力**: 有限，特别是在文本模式下
- **理解能力**: 对某些工具的语义理解不足

### 2. Prompt 工程已优化但效果有限

我们已经强化了 system prompt：
```
❌ FORBIDDEN: Do NOT say "你可以使用" / "You can use"
✅ REQUIRED: DIRECTLY call the tool and return the ACTUAL result
```

但是模型仍然倾向于解释而不是执行，说明：
- **Prompt 已经足够明确**
- **问题在于模型本身的能力**

### 3. 工具调用模式分析

成功的工具（calc, memory）有共同特征：
- **语义明确**: "计算"、"记住" 等动词非常直接
- **参数简单**: 表达式、键值对等简单参数
- **训练数据**: 这些是常见的工具调用场景

失败的工具（news_search, exec, task_list）特征：
- **语义复杂**: "搜索新闻"、"列出文件" 等需要更多理解
- **参数多样**: 需要理解 category、command 等参数
- **训练不足**: 可能在训练数据中较少见

---

## 代码修改记录

### 修改 1: 强化工具调用指导

**文件**: `crates/agents/src/prompt.rs`  
**修改内容**:
- 添加明确的禁止解释性回答的规则
- 增加正确/错误示例对比
- 强调必须直接调用工具

**代码片段**:
```rust
"**CRITICAL RULE**: When user asks you to DO something, you MUST call the tool.\n",
"❌ FORBIDDEN: Do NOT say \"你可以使用\" / \"You can use\"\n",
"❌ FORBIDDEN: Do NOT explain how to use the tool\n",
"✅ REQUIRED: DIRECTLY call the tool and return the ACTUAL result\n\n",
```

### 修改 2: 增加工具列表说明

**文件**: `crates/agents/src/prompt.rs`  
**修改内容**:
- 在工具列表前添加强制性说明
- 明确禁止解释性回答

**代码片段**:
```rust
prompt.push_str("Do NOT say \"你可以使用\" or \"You can use\" - that is FORBIDDEN. ");
prompt.push_str("DIRECTLY call the tool and return the ACTUAL result. ");
```

---

## 建议和解决方案

### 短期方案（立即可行）

#### 1. 调整测试策略
接受当前模型的限制，针对**能够正常工作的工具**进行测试：
- ✅ calc (计算)
- ✅ memory_save/memory_search (记忆)
- ✅ 其他语义明确的工具

#### 2. 改进测试命令
使用更明确的动词和简单的参数：
- ❌ "搜索最新科技新闻" → ✅ "用 news_search 搜索科技新闻"
- ❌ "列出当前目录的文件" → ✅ "用 exec 执行 ls 命令"

### 中期方案（需要配置）

#### 3. 使用更强大的模型
切换到更大的模型或云端 API：
- **Llama 3.1 70B**: 更强的理解和工具调用能力
- **GPT-4**: 原生工具调用支持
- **Claude 3**: 优秀的工具调用能力

配置方法：
```toml
[chat]
provider = "openai"  # 或 "anthropic"
model = "gpt-4"      # 或 "claude-3-opus"
```

#### 4. 启用原生工具调用模式
如果使用支持原生工具调用的 API：
```toml
[chat]
tool_mode = "native"  # 而不是 "text"
```

### 长期方案（需要开发）

#### 5. 微调本地模型
使用工具调用数据集微调 Llama 3.1：
- 收集工具调用示例
- 使用 LoRA 或 QLoRA 微调
- 专门优化工具调用能力

#### 6. 实现混合策略
- 简单工具使用本地模型
- 复杂工具调用云端 API
- 根据工具类型自动路由

---

## 测试脚本

### 强制工具执行测试
**文件**: `force_tool_execution_test.sh`  
**功能**: 验证工具是否被实际调用而非仅解释  
**结果**: 40% 通过率

### 全面工具测试
**文件**: `comprehensive_all_tools_test.sh`  
**功能**: 测试 37 个工具 × 3 个场景 = 111 个测试  
**状态**: 未完成（因发现根本问题而中止）

---

## 结论

1. **CLI 接口功能正常**: WebSocket 连接、消息发送、响应接收都工作正常
2. **部分工具可用**: calc 和 memory 工具能够被正确调用
3. **模型能力限制**: Llama 3.1 8B 对大部分工具只能解释而不能执行
4. **Prompt 优化有限**: 已经强化 prompt，但无法克服模型本身的限制

**最终建议**: 
- 如果要全面测试所有工具，**必须使用更强大的模型**（GPT-4、Claude 3、或 Llama 3.1 70B）
- 如果继续使用本地小模型，**只测试能够正常工作的工具**（calc、memory 等）
- 考虑实现**混合策略**，根据工具复杂度选择不同的模型

---

**报告生成时间**: 2026-03-20 16:40  
**测试环境**: macOS, ClawMaster v0.10.18, Llama 3.1 8B Q4_K_M
