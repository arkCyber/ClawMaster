# OpenClaw 工具调用策略分析与应用

**分析时间**: 2026-03-20  
**目的**: 学习 OpenClaw 如何处理工具调用，改进 ClawMaster

---

## OpenClaw 的核心策略

### 1. **"默认不解释" (Do Not Narrate) 策略**

从搜索结果中发现的关键 prompt 指令：

```
## Tool Call Style

Default: do not narrate routine, low-risk tool calls (just call the tool).

Narrate only when it helps:
- multi-step work
- complex/challenging problems
- sensitive actions (e.g., deletions)
- or when the user explicitly asks

Keep narration brief and value-dense; avoid repeating obvious information.
```

**核心思想**：
- ✅ **默认直接调用工具**，不解释
- ❌ **不要解释如何使用工具**
- ✅ 只在复杂/敏感操作时简短说明
- ✅ 保持简洁，避免重复明显信息

### 2. **System Prompt 结构**

OpenClaw 的 system prompt 包含以下关键部分：

1. **Tooling**: 当前工具列表 + 简短描述
2. **Safety**: 安全防护提醒
3. **Skills**: 技能加载指导
4. **Workspace**: 工作目录
5. **Workspace Files (injected)**: 注入的引导文件
   - `AGENTS.md`
   - `SOUL.md`
   - **`TOOLS.md`** ← 用户自定义工具使用规则
   - `IDENTITY.md`
   - `USER.md`
   - `HEARTBEAT.md`
   - `MEMORY.md`

### 3. **TOOLS.md 的作用**

OpenClaw 使用 **`TOOLS.md`** 文件让用户定义：
- 特定工具的使用规则
- 环境特定的配置（SSH 主机、相机名称等）
- 工具调用偏好

**关键点**：
- `TOOLS.md` 在 system prompt 中注入
- 用户可以自定义工具行为
- 与工具 schema 分离

### 4. **工具呈现方式**

OpenClaw 使用两种方式呈现工具：

1. **System prompt text**: 人类可读的列表 + 指导
2. **Tool schema**: 结构化的函数定义发送给模型 API

---

## ClawMaster 当前实现对比

### 相同点 ✅

1. **工具注册表**: ClawMaster 有完整的工具注册系统
2. **Tool schema**: 支持原生和文本模式的工具调用
3. **Workspace files**: 支持 `AGENTS.md`、`TOOLS.md` 等文件注入

### 差异点 ⚠️

| 特性 | OpenClaw | ClawMaster 当前 |
|------|----------|----------------|
| **默认行为** | "不解释，直接调用" | "可以使用工具" 提示 |
| **Tool Call Style** | 明确的 "Do Not Narrate" 规则 | 强调 "必须调用" 但没有禁止解释 |
| **TOOLS.md 位置** | 在工具列表**之后**注入 | 在工具列表**之后**（已优化） |
| **Prompt 语气** | 简洁、命令式 | 详细、强调式 |

### 问题根源分析

ClawMaster 的 prompt 虽然强调"必须调用工具"，但**没有明确禁止解释行为**：

```rust
// ClawMaster 当前 (过于强调，但没有禁止解释)
"❌ FORBIDDEN: Do NOT say \"你可以使用\" / \"You can use\""
"✅ REQUIRED: DIRECTLY call the tool and return the ACTUAL result"
```

vs.

```
// OpenClaw (简洁、直接)
"Default: do not narrate routine, low-risk tool calls (just call the tool)."
```

---

## 改进方案

### 方案 1: 采用 OpenClaw 的 "Do Not Narrate" 策略

修改 `crates/agents/src/prompt.rs` 中的 `tool_call_guidance`：

```rust
// 新的简洁风格
concat!(
    "\n## Tool Call Style\n\n",
    "Default: do not narrate routine tool calls. Just call the tool.\n\n",
    "When to narrate (briefly):\n",
    "- Multi-step work\n",
    "- Complex problems\n",
    "- Sensitive actions (deletions, system changes)\n",
    "- User explicitly asks\n\n",
    "Keep narration brief and value-dense.\n\n",
)
```

### 方案 2: 强化 TOOLS.md 的作用

在 `~/.clawmaster/TOOLS.md` 中添加强制性规则：

```markdown
# Tool Usage Rules

## CRITICAL: Direct Tool Execution

When user asks you to DO something:
1. Call the tool IMMEDIATELY
2. Do NOT explain how to use the tool
3. Do NOT say "你可以使用" or "You can use"
4. Return the ACTUAL result

## Examples

User: "搜索科技新闻"
❌ WRONG: "你可以使用 news_search..."
✅ CORRECT: [Call news_search tool directly]

User: "列出文件"
❌ WRONG: "你可以使用 exec..."
✅ CORRECT: [Call exec tool directly]
```

### 方案 3: 简化 Prompt，减少噪音

当前 ClawMaster 的 prompt 过于冗长，可能导致：
- 模型注意力分散
- 关键指令被淹没
- 过度强调反而适得其反

**改进**：
- 删除重复的强调
- 使用简洁的命令式语气
- 减少示例数量，只保留最关键的

---

## 实施计划

### 立即可行

1. **修改 tool_call_guidance**
   - 采用 OpenClaw 的简洁风格
   - 添加 "Do Not Narrate" 规则
   - 删除冗余的强调

2. **更新 TOOLS.md**
   - 添加明确的"直接执行"规则
   - 提供正确/错误示例对比

3. **简化 prompt**
   - 删除重复的禁止语句
   - 保持简洁、直接的语气

### 需要验证

4. **测试不同模型**
   - Llama 3.1 8B (当前)
   - Llama 3.2 1B Instruct (已知支持工具调用)
   - 更大的模型 (70B)

5. **A/B 测试**
   - 对比新旧 prompt 的效果
   - 测量工具实际调用率

---

## 关键洞察

### OpenClaw 的成功之处

1. **简洁优于冗长**: 简短、直接的指令比长篇强调更有效
2. **默认行为清晰**: "默认不解释" 比 "禁止解释" 更自然
3. **用户可定制**: TOOLS.md 让用户控制工具行为
4. **分离关注点**: 工具定义 vs. 工具使用规则

### ClawMaster 的改进方向

1. **减少 prompt 噪音**: 删除重复强调，保持简洁
2. **采用自然语气**: "默认直接调用" 而非 "禁止解释"
3. **强化 TOOLS.md**: 让用户自定义工具行为
4. **模型选择**: 确保使用支持工具调用的模型

---

## 下一步行动

1. ✅ 分析 OpenClaw 策略（已完成）
2. ⏭️ 修改 `prompt.rs` 采用简洁风格
3. ⏭️ 更新 `TOOLS.md` 模板
4. ⏭️ 重新测试工具调用
5. ⏭️ 对比新旧实现效果

---

**结论**: OpenClaw 的成功在于**简洁、直接的指令**，而不是冗长的强调。ClawMaster 应该采用类似的策略，同时保持 Rust 的类型安全和企业级质量。
