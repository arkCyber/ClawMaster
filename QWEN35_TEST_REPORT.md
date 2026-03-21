# Qwen 3.5 9B 模型测试报告

**测试时间**: 2026-03-20 21:00  
**模型**: Qwen 3.5 9B (unsloth/Qwen3.5-9B-GGUF)  
**环境**: ClawMaster v0.10.18

---

## 模型信息

| 属性 | 值 |
|------|------|
| 模型名称 | Qwen 3.5 9B |
| 大小 | 6.6 GB |
| 上下文窗口 | 32,768 tokens |
| 量化格式 | GGUF |
| Chat Template | ChatML/Qwen |

---

## 配置变更

### 1. 模型注册
在 `crates/providers/src/local_gguf/models.rs` 中添加了 Qwen 3.5 9B 定义：

```rust
GgufModelDef {
    id: "qwen3.5:9b",
    display_name: "Qwen 3.5 9B",
    hf_repo: "",
    hf_filename: "",
    min_ram_gb: 16,
    context_window: 32_768,
    chat_template: ChatTemplateHint::ChatML,
    backend: ModelBackend::Gguf,
},
```

### 2. 工具模式配置
修改了 `crates/providers/src/local_gguf/mod.rs`：

```rust
// LocalGgufProvider
fn supports_tools(&self) -> bool {
    false  // 禁用原生工具调用，强制使用文本模式
}

fn tool_mode(&self) -> Option<clawmaster_config::ToolMode> {
    Some(clawmaster_config::ToolMode::Text)
}

// LazyLocalGgufProvider
fn supports_tools(&self) -> bool {
    false  // 禁用原生工具调用，强制使用文本模式
}

fn tool_mode(&self) -> Option<clawmaster_config::ToolMode> {
    Some(clawmaster_config::ToolMode::Text)
}
```

---

## 初步测试结果

### 测试 1: 数学计算
**输入**: "计算 50 * 4"  
**预期**: 调用 `calc` 工具，返回 200  
**实际**: ✅ 成功 - "结果是 200"

### 测试 2: 新闻搜索
**输入**: "搜索人工智能新闻"  
**预期**: 调用 `news_search` 工具  
**实际**: ⏳ 待测试

### 测试 3: 文件操作
**输入**: "列出当前目录的文件"  
**预期**: 调用 `exec` 工具  
**实际**: ⏳ 待测试

### 测试 4: 记忆管理
**输入**: "记住我喜欢 Rust 和 AI"  
**预期**: 调用 `memory_save` 工具  
**实际**: ⚠️ 部分成功 - LLM 解释了如何使用工具，但未直接调用

---

## 与 Llama 3.1 8B 对比

| 指标 | Llama 3.1 8B | Qwen 3.5 9B |
|------|--------------|-------------|
| 模型大小 | 4.9 GB | 6.6 GB |
| 上下文窗口 | 128K | 32K |
| 工具调用成功率 | 10-40% | ⏳ 测试中 |
| 响应速度 | 快 | ⏳ 测试中 |
| 工具调用质量 | 低 | ⏳ 测试中 |

---

## 优势分析

### Qwen 3.5 9B 的优势
1. **更大的参数量**: 9B vs 8B，理论上理解能力更强
2. **专门优化**: Qwen 系列对中文和工具调用有特殊优化
3. **最新架构**: Qwen 3.5 是最新一代模型
4. **工具调用支持**: 官方文档显示对工具调用有原生支持

### 预期改进
1. **工具调用成功率**: 从 40% 提升到 60-80%
2. **中文理解**: 更好的中文自然语言理解
3. **推理能力**: 更强的多步推理能力

---

## 下一步测试计划

### 1. 完整工具测试
运行 `force_tool_execution_test.sh` 进行 10 个工具的全面测试

### 2. 真实场景测试
运行 `real_world_tool_test.sh` 进行 18 个真实场景测试

### 3. 性能对比
- 响应时间
- 工具调用准确率
- 多步推理能力

### 4. 中文场景测试
- 中文自然语言理解
- 中文工具调用
- 中文响应质量

---

## 技术细节

### 日志检查点
需要验证的关键日志：
```
native_tools=false  ← 必须是 false
tool_mode=Text      ← 必须是 Text
tool_calls_count>0  ← 工具被调用
```

### 已知问题
1. ❌ `native_tools` 仍然显示为 `true`（需要进一步调查）
2. ⚠️ 部分工具调用仍然是解释而非执行

### 待解决
1. 确保 `effective_tool_mode` 返回 `Text` 而非 `Native`
2. 验证 prompt 中包含 ````tool_call` 格式指导
3. 测试 Qwen 3.5 的实际工具调用能力

---

## 结论

Qwen 3.5 9B 是一个很好的选择，比 Llama 3.1 8B 有以下优势：
- ✅ 更大的模型容量
- ✅ 更好的中文支持
- ✅ 官方工具调用优化
- ⏳ 实际效果待全面测试验证

**推荐**: 继续使用 Qwen 3.5 9B 并进行全面测试，预期工具调用成功率将显著提升。

---

**报告生成时间**: 2026-03-20 21:00  
**下次更新**: 完成全面测试后
