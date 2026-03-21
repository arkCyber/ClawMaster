# ClawMaster 所有工具和 Skills 全面测试计划

**创建时间**: 2026-03-19 22:08  
**测试范围**: 所有工具 + 所有 Skills

---

## 🛠️ 发现的工具列表

### 核心工具（20+）

1. **news_search** - 新闻搜索
2. **calc** - 计算器
3. **web_search** - 网页搜索
4. **web_fetch** - 网页获取
5. **browser** - 浏览器控制
6. **exec** - 命令执行
7. **process** - 进程管理
8. **task_list** - 任务列表
9. **sessions_list** - 会话列表
10. **sessions_history** - 会话历史
11. **sessions_send** - 会话发送
12. **spawn_agent** - 生成代理
13. **show_map** - 显示地图
14. **location** - 位置获取
15. **send_image** - 发送图片
16. **sandbox_packages** - 沙箱包管理
17. **nodes_list** - 节点列表
18. **nodes_describe** - 节点描述
19. **nodes_select** - 节点选择
20. **loop_detection** - 循环检测
21. **create_skill** - 创建技能
22. **cron** - 定时任务

### Skills 功能

- 技能发现
- 技能安装
- 技能更新
- 技能审查
- 技能注册

---

## 📋 自然语言测试场景

### 场景 1: news_search 工具 ✅

**测试 1.1**: 中文新闻查询
```
输入: "今天有什么中国新闻？"
预期: 调用 news_search，location=China
```

**测试 1.2**: 英文新闻查询
```
输入: "What's the latest news in USA?"
预期: 调用 news_search，location=USA
```

**测试 1.3**: 特定类别新闻
```
输入: "给我看看美国的科技新闻"
预期: 调用 news_search，location=USA, category=technology
```

**测试 1.4**: 体育新闻
```
输入: "Show me sports news from UK"
预期: 调用 news_search，location=UK, category=sports
```

---

### 场景 2: calc 工具

**测试 2.1**: 简单计算
```
输入: "计算 123 + 456"
预期: 调用 calc 工具，返回 579
```

**测试 2.2**: 复杂表达式
```
输入: "Calculate (15 + 25) * 3"
预期: 调用 calc 工具，返回 120
```

**测试 2.3**: 幂运算
```
输入: "What is 2 to the power of 10?"
预期: 调用 calc 工具，返回 1024
```

**测试 2.4**: 除法和模运算
```
输入: "100 divided by 7, what's the remainder?"
预期: 调用 calc 工具，计算 100 % 7
```

---

### 场景 3: web_search 工具

**测试 3.1**: 一般搜索
```
输入: "Search for Rust programming tutorials"
预期: 调用 web_search
```

**测试 3.2**: 中文搜索
```
输入: "搜索 Rust 编程教程"
预期: 调用 web_search
```

**测试 3.3**: 技术问题搜索
```
输入: "How to fix async/await in Rust?"
预期: 调用 web_search
```

---

### 场景 4: task_list 工具

**测试 4.1**: 添加任务
```
输入: "Add a task: Review code changes"
预期: 调用 task_list，action=add
```

**测试 4.2**: 列出任务
```
输入: "Show me my tasks"
预期: 调用 task_list，action=list
```

**测试 4.3**: 完成任务
```
输入: "Mark task 1 as complete"
预期: 调用 task_list，action=complete
```

---

### 场景 5: sessions 工具

**测试 5.1**: 列出会话
```
输入: "List all sessions"
预期: 调用 sessions_list
```

**测试 5.2**: 查看会话历史
```
输入: "Show history of session main"
预期: 调用 sessions_history
```

**测试 5.3**: 发送消息到会话
```
输入: "Send 'Hello' to session test"
预期: 调用 sessions_send
```

---

### 场景 6: 地图和位置工具

**测试 6.1**: 显示地图
```
输入: "Show me a map of Beijing"
预期: 调用 show_map，location=Beijing
```

**测试 6.2**: 获取位置
```
输入: "Where am I?"
预期: 调用 location 工具
```

---

### 场景 7: 身份问答（不应调用工具）

**测试 7.1**: 中文身份问答
```
输入: "你是谁？"
预期: 直接回答，不调用工具
```

**测试 7.2**: 英文身份问答
```
输入: "What can you do?"
预期: 直接回答，不调用工具
```

**测试 7.3**: 能力查询
```
输入: "你能帮我做什么？"
预期: 直接回答，不调用工具
```

---

### 场景 8: Skills 功能

**测试 8.1**: 创建技能
```
输入: "Create a new skill called 'weather_checker'"
预期: 调用 create_skill
```

**测试 8.2**: 列出技能
```
输入: "List all available skills"
预期: 列出已安装的 skills
```

---

## 🎯 测试执行计划

### 阶段 1: 核心工具测试（优先）

1. ✅ news_search - 已测试，功能正常
2. ⏳ calc - 待测试
3. ⏳ web_search - 待测试
4. ⏳ task_list - 待测试

### 阶段 2: 会话和通信工具

5. ⏳ sessions_list
6. ⏳ sessions_history
7. ⏳ sessions_send

### 阶段 3: 地图和位置工具

8. ⏳ show_map
9. ⏳ location

### 阶段 4: Skills 功能

10. ⏳ create_skill
11. ⏳ skills 列表

---

## 📊 预期结果分析

### 成功标准

对于每个测试：

1. **工具调用**: 是否正确调用了预期的工具
2. **参数提取**: 参数是否正确提取
3. **结果返回**: 是否返回了有效结果
4. **错误处理**: 错误是否被正确处理
5. **日志完整**: 日志是否记录了关键信息

### 需要补全的代码类型

1. **缺失的日志**: 添加关键路径日志
2. **错误处理**: 改进错误处理逻辑
3. **参数验证**: 加强参数验证
4. **结果格式**: 优化结果展示
5. **性能优化**: 提高响应速度

---

## 🔍 测试方法

### 方法 1: WebUI 测试（推荐）

由于 CLI `agent` 命令需要完整配置，使用 WebUI 进行测试：

1. 在 WebUI 中输入测试消息
2. 观察后端日志输出
3. 分析工具调用和结果
4. 识别需要改进的地方

### 方法 2: 日志分析

从后端日志中提取：
- 工具调用次数
- 参数提取情况
- 错误信息
- 性能指标

---

## 📝 测试记录模板

```markdown
### 测试: [工具名称] - [场景描述]

**输入**: [自然语言输入]

**观察到的行为**:
- 工具调用: [是/否]
- 工具名称: [实际调用的工具]
- 参数: [提取的参数]
- 结果: [返回的结果]
- 错误: [如有错误]

**日志摘要**:
```
[关键日志行]
```

**分析**:
- ✅ 成功 / ❌ 失败 / ⚠️ 部分成功
- 发现的问题: [描述]
- 需要补全: [代码位置和内容]

**状态**: ✅ 通过 / ❌ 失败 / ⏳ 待测试
```

---

## 🎉 测试总结（待完成）

测试完成后将生成：

1. **测试覆盖率报告**
   - 每个工具的测试状态
   - 成功/失败统计
   - 覆盖率百分比

2. **代码补全清单**
   - 需要添加的日志
   - 需要修复的bug
   - 需要优化的性能

3. **最终测试报告**
   - 详细的测试结果
   - 代码改进建议
   - 部署建议

---

**文档状态**: 测试计划已创建  
**下一步**: 开始执行测试
