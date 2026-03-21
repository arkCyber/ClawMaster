# 🎯 新闻工具完整诊断与修复方案

**时间**: 2026年3月18日 07:45  
**状态**: 深度调试中  

---

## 📊 已完成的工作

### 1. 代码审计 ✅
- news_search 工具已注册
- TOOLS.md 文件正确加载
- HTML 注释正确去除
- System prompt 包含 TOOLS.md

### 2. 自动化测试 ✅
- 创建了完整的测试脚本
- 5 个真实测试案例
- 自动捕获日志和响应

### 3. 第一轮修复 ✅
- 调整 TOOLS.md 位置（移到工具列表后）
- 添加工具权限前缀
- 强化标题显示

---

## 🚨 当前问题

### LLM 仍然拒绝使用工具
```
响应: "抱歉，我无法直接使用 `news_search` 工具..."
日志: tool_calls_count=0
```

**这说明**:
- ❌ 所有修复都未生效
- ❌ LLM 仍然认为它"不能"使用工具
- ❌ 问题比预期更深

---

## 🔍 深度诊断

### 需要检查的关键点

1. **native_tools 配置**
   - 当前值是什么？
   - 模型是否支持该模式？

2. **完整 System Prompt**
   - 是否有冲突的指令？
   - 工具权限前缀是否真的在 prompt 中？
   - TOOLS.md 是否真的在工具列表后？

3. **模型能力**
   - Qwen 2.5 Coder 14B 是否支持工具调用？
   - 需要什么格式？

4. **其他工具测试**
   - calc 工具能否被调用？
   - 如果不能 → 通用问题
   - 如果能 → news_search 特定问题

---

## 🔧 下一步修复方案

### 方案 1: 添加调试日志 ✅ 正在执行
```rust
// 在 build_system_prompt 后添加日志
debug!("System prompt tools section: {}", tools_excerpt);
debug!("System prompt CRITICAL section: {}", critical_excerpt);
```

### 方案 2: 测试其他工具
```
用户查询: "请使用 calc 工具计算 2+2"
```

如果 calc 能调用:
- → 问题在 news_search 的描述或配置
- → 需要修改 news_tool.rs

如果 calc 也不能调用:
- → 问题在工具调用机制本身
- → 需要检查 native_tools 配置
- → 可能需要切换到文本模式

### 方案 3: 强制文本模式工具调用
如果 native_tools=true 但模型不支持:
```rust
// 在 run_streaming 中强制设置
let native_tools = false; // 强制使用文本模式
```

### 方案 4: 在 SOUL.md 中添加指令
```markdown
# SOUL.md

## Tool Usage Philosophy

YOU HAVE FULL PERMISSION TO USE ALL AVAILABLE TOOLS.
When a tool matches the user's request, call it immediately.
NEVER say "I cannot use this tool" - that is FALSE.
```

### 方案 5: 修改基础 System Prompt
```rust
let mut prompt = String::from(if include_tools {
    "You are a helpful assistant with access to powerful tools. \
     YOU MUST USE TOOLS when they match the user's request. \
     You have full permission to call any tool listed below.\n\n"
} else {
    "You are a helpful assistant. Answer questions clearly and concisely.\n\n"
});
```

---

## 📝 调试检查清单

- [ ] 获取 native_tools 的实际值
- [ ] 查看完整 system prompt 的工具部分
- [ ] 确认权限前缀在 prompt 中
- [ ] 确认 TOOLS.md 在工具列表后
- [ ] 测试 calc 工具
- [ ] 检查模型是否支持 function calling
- [ ] 尝试文本模式工具调用
- [ ] 检查是否有其他限制性指令

---

## 🎯 最终目标

让 LLM 在看到新闻查询时:
1. 识别这是新闻请求
2. 看到 news_search 工具
3. 理解它有权限使用
4. 实际调用工具
5. 返回新闻结果

**而不是**: "抱歉，我无法使用工具"

---

**当前状态**: 已添加调试日志，等待查看 system prompt 实际内容
