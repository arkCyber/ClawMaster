# 🚀 开始测试 ClawMaster 工具

## 立即开始测试的三种方式

### 方式 1: 在 WebUI 中手动测试（推荐）

1. **打开 WebUI**
   ```
   浏览器访问: https://localhost:3000
   ```

2. **复制第一个测试问题**
   ```
   请执行命令：echo "Hello from ClawMaster"
   ```

3. **粘贴到对话窗口，发送**

4. **观察结果**
   - 查看 AI 是否调用了 `exec` 工具
   - 查看命令执行结果
   - 确认响应正确

5. **继续下一个测试**
   - 参考 `TOOL_TEST_QUESTIONS.md` 中的问题
   - 逐个测试

---

### 方式 2: 使用交互式脚本

```bash
cd /Users/arksong/ClawMaster
./scripts/test_tools_interactive.sh
```

脚本会逐个显示测试问题，你在 WebUI 中复制粘贴即可。

---

### 方式 3: 完全自动化测试

```bash
cd /Users/arksong/ClawMaster
./scripts/auto_test_tools.sh
```

脚本会自动通过 API 测试所有工具，并生成报告。

---

## 📝 前 10 个测试问题（复制即用）

### 1. exec 工具
```
请执行命令：echo "Hello from ClawMaster"
```

### 2. exec 工具
```
请列出当前目录的文件
```

### 3. calc 工具
```
请计算：(123 + 456) * 789
```

### 4. calc 工具
```
请计算 2 的 10 次方
```

### 5. web_search 工具
```
请搜索：Rust 编程语言最新特性
```

### 6. web_fetch 工具
```
请获取 https://example.com 的内容
```

### 7. browser 工具
```
请打开浏览器访问 https://www.wikipedia.org
```

### 8. memory_save 工具
```
请记住：今天完成了 ClawMaster 工具测试
```

### 9. memory_search 工具
```
请搜索关于'测试'的记忆
```

### 10. sessions_list 工具
```
请列出所有活跃的会话
```

---

## ✅ 测试检查清单

复制以下内容到你的笔记中，测试完成后打勾：

```
核心工具:
□ exec - 命令执行
□ calc - 计算器
□ process - 进程管理
□ sandbox_packages - 沙箱包
□ cron - 定时任务

网络工具:
□ web_search - 网络搜索
□ web_fetch - 网页获取
□ browser - 浏览器自动化

内存工具:
□ memory_search - 搜索记忆
□ memory_save - 保存记忆
□ memory_get - 获取记忆

会话工具:
□ sessions_list - 列出会话
□ sessions_create - 创建会话
□ sessions_history - 会话历史
□ sessions_send - 发送消息
□ branch_session - 分支会话

辅助工具:
□ task_list - 任务列表
□ location - 位置服务
□ show_map - 地图显示
□ speak - 文本转语音
□ spawn_agent - 生成智能体
□ session_state - 会话状态
```

---

## 🎯 现在就开始！

**第一步**: 打开 https://localhost:3000

**第二步**: 复制这个问题到对话窗口：
```
请执行命令：echo "Hello from ClawMaster"
```

**第三步**: 观察 AI 的响应和工具调用

**第四步**: 继续测试下一个工具

---

## 📚 完整文档

- **测试指南**: `TOOL_TESTING_GUIDE.md`
- **测试问题**: `TOOL_TEST_QUESTIONS.md`
- **测试报告**: `AUTO_TOOL_TEST_REPORT.md`
- **自动化脚本**: `scripts/auto_test_tools.sh`
- **交互式脚本**: `scripts/test_tools_interactive.sh`

---

**祝测试顺利！** 🎉
