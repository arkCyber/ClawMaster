# ClawMaster 所有工具完整测试计划

**创建时间**: 2026-03-19 22:21  
**工具总数**: 33 个  
**测试场景**: 每个工具 3 个场景 = 99 个测试

---

## 📊 工具统计

**发现的工具**: 33 个

### 核心工具列表

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
12. **sessions_create** - 创建会话
13. **sessions_delete** - 删除会话
14. **spawn_agent** - 生成代理
15. **show_map** - 显示地图
16. **get_user_location** - 获取位置
17. **send_image** - 发送图片
18. **image** - 图片生成
19. **sandbox_packages** - 沙箱包管理
20. **nodes_list** - 节点列表
21. **nodes_describe** - 节点描述
22. **nodes_select** - 节点选择
23. **loop_detection** - 循环检测
24. **create_skill** - 创建技能
25. **update_skill** - 更新技能
26. **delete_skill** - 删除技能
27. **cron** - 定时任务
28. **apply_patch** - 应用补丁
29. **branch_session** - 分支会话
30. **session_state** - 会话状态
31. **gateway** - 网关配置
32. **agents_list** - 代理列表
33. **pdf** - PDF 处理

---

## 🎯 完整测试场景（99个）

### 1. news_search 工具（3个场景）✅

#### 场景 1.1: 中文新闻查询
```
输入: 今天有什么中国新闻？
预期: 调用 news_search，location=China
状态: ✅ 已测试
```

#### 场景 1.2: 英文新闻查询
```
输入: What's the latest news in USA?
预期: 调用 news_search，location=USA
状态: ✅ 已测试
```

#### 场景 1.3: 特定类别新闻
```
输入: 给我看看美国的科技新闻
预期: 调用 news_search，location=USA, category=technology
状态: ⏳ 待测试
```

---

### 2. calc 工具（3个场景）

#### 场景 2.1: 简单算术
```
输入: 计算 123 + 456
预期: 调用 calc，返回 579
状态: ⏳ 待测试
```

#### 场景 2.2: 复杂表达式
```
输入: Calculate (15 + 25) * 3
预期: 调用 calc，返回 120
状态: ⏳ 待测试
```

#### 场景 2.3: 幂运算
```
输入: What is 2 to the power of 10?
预期: 调用 calc，返回 1024
状态: ⏳ 待测试
```

---

### 3. web_search 工具（3个场景）

#### 场景 3.1: 技术搜索
```
输入: Search for Rust programming tutorials
预期: 调用 web_search
状态: ⏳ 待测试
```

#### 场景 3.2: 中文搜索
```
输入: 搜索 Rust 编程教程
预期: 调用 web_search
状态: ⏳ 待测试
```

#### 场景 3.3: 问题搜索
```
输入: How to fix async/await in Rust?
预期: 调用 web_search
状态: ⏳ 待测试
```

---

### 4. web_fetch 工具（3个场景）

#### 场景 4.1: 获取网页内容
```
输入: Fetch the content from https://www.rust-lang.org
预期: 调用 web_fetch
状态: ⏳ 待测试
```

#### 场景 4.2: 获取 API 数据
```
输入: Get data from https://api.github.com/repos/rust-lang/rust
预期: 调用 web_fetch
状态: ⏳ 待测试
```

#### 场景 4.3: 获取 JSON 数据
```
输入: Fetch JSON from https://jsonplaceholder.typicode.com/posts/1
预期: 调用 web_fetch
状态: ⏳ 待测试
```

---

### 5. browser 工具（3个场景）

#### 场景 5.1: 打开网页
```
输入: Open https://www.rust-lang.org in browser
预期: 调用 browser
状态: ⏳ 待测试
```

#### 场景 5.2: 截图
```
输入: Take a screenshot of https://github.com
预期: 调用 browser
状态: ⏳ 待测试
```

#### 场景 5.3: 提取内容
```
输入: Extract text from https://www.rust-lang.org
预期: 调用 browser
状态: ⏳ 待测试
```

---

### 6. exec 工具（3个场景）

#### 场景 6.1: 执行简单命令
```
输入: Run 'ls -la' command
预期: 调用 exec
状态: ⏳ 待测试
```

#### 场景 6.2: 检查系统信息
```
输入: What's my system information?
预期: 调用 exec (uname -a)
状态: ⏳ 待测试
```

#### 场景 6.3: 查看文件
```
输入: Show me the contents of README.md
预期: 调用 exec (cat README.md)
状态: ⏳ 待测试
```

---

### 7. process 工具（3个场景）

#### 场景 7.1: 启动进程
```
输入: Start a new process for monitoring
预期: 调用 process
状态: ⏳ 待测试
```

#### 场景 7.2: 列出进程
```
输入: List all running processes
预期: 调用 process
状态: ⏳ 待测试
```

#### 场景 7.3: 停止进程
```
输入: Stop process with ID 1234
预期: 调用 process
状态: ⏳ 待测试
```

---

### 8. task_list 工具（3个场景）

#### 场景 8.1: 添加任务
```
输入: Add a task: Review code changes
预期: 调用 task_list，action=add
状态: ⏳ 待测试
```

#### 场景 8.2: 列出任务
```
输入: Show me my tasks
预期: 调用 task_list，action=list
状态: ⏳ 待测试
```

#### 场景 8.3: 完成任务
```
输入: Mark task 1 as complete
预期: 调用 task_list，action=complete
状态: ⏳ 待测试
```

---

### 9. sessions_list 工具（3个场景）

#### 场景 9.1: 列出所有会话
```
输入: List all sessions
预期: 调用 sessions_list
状态: ⏳ 待测试
```

#### 场景 9.2: 列出活跃会话
```
输入: Show me active sessions
预期: 调用 sessions_list
状态: ⏳ 待测试
```

#### 场景 9.3: 查找特定会话
```
输入: Find session named 'main'
预期: 调用 sessions_list
状态: ⏳ 待测试
```

---

### 10. sessions_history 工具（3个场景）

#### 场景 10.1: 查看会话历史
```
输入: Show history of session main
预期: 调用 sessions_history
状态: ⏳ 待测试
```

#### 场景 10.2: 查看最近消息
```
输入: Show me the last 10 messages in session main
预期: 调用 sessions_history
状态: ⏳ 待测试
```

#### 场景 10.3: 搜索历史
```
输入: Search for 'news' in session main history
预期: 调用 sessions_history
状态: ⏳ 待测试
```

---

### 11. sessions_send 工具（3个场景）

#### 场景 11.1: 发送消息
```
输入: Send 'Hello' to session test
预期: 调用 sessions_send
状态: ⏳ 待测试
```

#### 场景 11.2: 发送通知
```
输入: Notify session main about the update
预期: 调用 sessions_send
状态: ⏳ 待测试
```

#### 场景 11.3: 广播消息
```
输入: Broadcast 'System maintenance' to all sessions
预期: 调用 sessions_send
状态: ⏳ 待测试
```

---

### 12. sessions_create 工具（3个场景）

#### 场景 12.1: 创建新会话
```
输入: Create a new session named 'test'
预期: 调用 sessions_create
状态: ⏳ 待测试
```

#### 场景 12.2: 创建临时会话
```
输入: Create a temporary session for debugging
预期: 调用 sessions_create
状态: ⏳ 待测试
```

#### 场景 12.3: 创建带配置的会话
```
输入: Create session 'dev' with debug mode enabled
预期: 调用 sessions_create
状态: ⏳ 待测试
```

---

### 13. sessions_delete 工具（3个场景）

#### 场景 13.1: 删除会话
```
输入: Delete session named 'test'
预期: 调用 sessions_delete
状态: ⏳ 待测试
```

#### 场景 13.2: 清理旧会话
```
输入: Delete all inactive sessions
预期: 调用 sessions_delete
状态: ⏳ 待测试
```

#### 场景 13.3: 删除临时会话
```
输入: Remove temporary sessions
预期: 调用 sessions_delete
状态: ⏳ 待测试
```

---

### 14. spawn_agent 工具（3个场景）

#### 场景 14.1: 生成新代理
```
输入: Spawn a new agent for monitoring
预期: 调用 spawn_agent
状态: ⏳ 待测试
```

#### 场景 14.2: 生成专用代理
```
输入: Create an agent specialized in news gathering
预期: 调用 spawn_agent
状态: ⏳ 待测试
```

#### 场景 14.3: 生成临时代理
```
输入: Spawn a temporary agent for this task
预期: 调用 spawn_agent
状态: ⏳ 待测试
```

---

### 15. show_map 工具（3个场景）

#### 场景 15.1: 显示城市地图
```
输入: Show me a map of Beijing
预期: 调用 show_map
状态: ⏳ 待测试
```

#### 场景 15.2: 显示地区地图
```
输入: Display a map of Silicon Valley
预期: 调用 show_map
状态: ⏳ 待测试
```

#### 场景 15.3: 显示路线
```
输入: Show route from Beijing to Shanghai
预期: 调用 show_map
状态: ⏳ 待测试
```

---

### 16. get_user_location 工具（3个场景）

#### 场景 16.1: 获取当前位置
```
输入: Where am I?
预期: 调用 get_user_location
状态: ⏳ 待测试
```

#### 场景 16.2: 获取位置详情
```
输入: What's my current location with details?
预期: 调用 get_user_location
状态: ⏳ 待测试
```

#### 场景 16.3: 获取坐标
```
输入: Give me my GPS coordinates
预期: 调用 get_user_location
状态: ⏳ 待测试
```

---

### 17. send_image 工具（3个场景）

#### 场景 17.1: 发送图片
```
输入: Send image.png to the chat
预期: 调用 send_image
状态: ⏳ 待测试
```

#### 场景 17.2: 发送截图
```
输入: Send a screenshot of the current screen
预期: 调用 send_image
状态: ⏳ 待测试
```

#### 场景 17.3: 发送图表
```
输入: Send the chart as an image
预期: 调用 send_image
状态: ⏳ 待测试
```

---

### 18. image 工具（3个场景）

#### 场景 18.1: 生成图片
```
输入: Generate an image of a sunset
预期: 调用 image
状态: ⏳ 待测试
```

#### 场景 18.2: 创建图表
```
输入: Create a bar chart showing sales data
预期: 调用 image
状态: ⏳ 待测试
```

#### 场景 18.3: 生成图标
```
输入: Generate an icon for the app
预期: 调用 image
状态: ⏳ 待测试
```

---

### 19. sandbox_packages 工具（3个场景）

#### 场景 19.1: 列出包
```
输入: List all sandbox packages
预期: 调用 sandbox_packages
状态: ⏳ 待测试
```

#### 场景 19.2: 搜索包
```
输入: Find packages related to Python
预期: 调用 sandbox_packages
状态: ⏳ 待测试
```

#### 场景 19.3: 查看包详情
```
输入: Show details of package 'rust'
预期: 调用 sandbox_packages
状态: ⏳ 待测试
```

---

### 20. nodes_list 工具（3个场景）

#### 场景 20.1: 列出所有节点
```
输入: List all nodes
预期: 调用 nodes_list
状态: ⏳ 待测试
```

#### 场景 20.2: 列出活跃节点
```
输入: Show active nodes
预期: 调用 nodes_list
状态: ⏳ 待测试
```

#### 场景 20.3: 按类型列出
```
输入: List compute nodes
预期: 调用 nodes_list
状态: ⏳ 待测试
```

---

### 21. nodes_describe 工具（3个场景）

#### 场景 21.1: 描述节点
```
输入: Describe node 'node1'
预期: 调用 nodes_describe
状态: ⏳ 待测试
```

#### 场景 21.2: 查看节点状态
```
输入: What's the status of node 'node1'?
预期: 调用 nodes_describe
状态: ⏳ 待测试
```

#### 场景 21.3: 查看节点配置
```
输入: Show configuration of node 'node1'
预期: 调用 nodes_describe
状态: ⏳ 待测试
```

---

### 22. nodes_select 工具（3个场景）

#### 场景 22.1: 选择节点
```
输入: Select node 'node1'
预期: 调用 nodes_select
状态: ⏳ 待测试
```

#### 场景 22.2: 选择最佳节点
```
输入: Select the best node for computation
预期: 调用 nodes_select
状态: ⏳ 待测试
```

#### 场景 22.3: 选择空闲节点
```
输入: Select an idle node
预期: 调用 nodes_select
状态: ⏳ 待测试
```

---

### 23. loop_detection 工具（3个场景）

#### 场景 23.1: 检测循环
```
输入: Check for loops in the current process
预期: 调用 loop_detection
状态: ⏳ 待测试
```

#### 场景 23.2: 分析循环
```
输入: Analyze loop patterns
预期: 调用 loop_detection
状态: ⏳ 待测试
```

#### 场景 23.3: 报告循环
```
输入: Report any detected loops
预期: 调用 loop_detection
状态: ⏳ 待测试
```

---

### 24. create_skill 工具（3个场景）

#### 场景 24.1: 创建新技能
```
输入: Create a new skill called 'weather_checker'
预期: 调用 create_skill
状态: ⏳ 待测试
```

#### 场景 24.2: 创建数据技能
```
输入: Create a skill for data analysis
预期: 调用 create_skill
状态: ⏳ 待测试
```

#### 场景 24.3: 创建自动化技能
```
输入: Create an automation skill
预期: 调用 create_skill
状态: ⏳ 待测试
```

---

### 25. update_skill 工具（3个场景）

#### 场景 25.1: 更新技能
```
输入: Update skill 'weather_checker'
预期: 调用 update_skill
状态: ⏳ 待测试
```

#### 场景 25.2: 修改技能配置
```
输入: Modify configuration of skill 'data_analyzer'
预期: 调用 update_skill
状态: ⏳ 待测试
```

#### 场景 25.3: 升级技能
```
输入: Upgrade skill to latest version
预期: 调用 update_skill
状态: ⏳ 待测试
```

---

### 26. delete_skill 工具（3个场景）

#### 场景 26.1: 删除技能
```
输入: Delete skill 'old_skill'
预期: 调用 delete_skill
状态: ⏳ 待测试
```

#### 场景 26.2: 移除未使用技能
```
输入: Remove unused skills
预期: 调用 delete_skill
状态: ⏳ 待测试
```

#### 场景 26.3: 清理技能
```
输入: Clean up old skills
预期: 调用 delete_skill
状态: ⏳ 待测试
```

---

### 27. cron 工具（3个场景）

#### 场景 27.1: 创建定时任务
```
输入: Schedule a task to run every day at 9am
预期: 调用 cron
状态: ⏳ 待测试
```

#### 场景 27.2: 列出定时任务
```
输入: Show all scheduled tasks
预期: 调用 cron
状态: ⏳ 待测试
```

#### 场景 27.3: 删除定时任务
```
输入: Remove the daily backup task
预期: 调用 cron
状态: ⏳ 待测试
```

---

### 28. apply_patch 工具（3个场景）

#### 场景 28.1: 应用补丁
```
输入: Apply patch file 'fix.patch'
预期: 调用 apply_patch
状态: ⏳ 待测试
```

#### 场景 28.2: 应用代码更新
```
输入: Apply the code update patch
预期: 调用 apply_patch
状态: ⏳ 待测试
```

#### 场景 28.3: 应用安全补丁
```
输入: Apply security patch
预期: 调用 apply_patch
状态: ⏳ 待测试
```

---

### 29. branch_session 工具（3个场景）

#### 场景 29.1: 创建分支会话
```
输入: Branch current session
预期: 调用 branch_session
状态: ⏳ 待测试
```

#### 场景 29.2: 创建实验分支
```
输入: Create an experimental branch of this session
预期: 调用 branch_session
状态: ⏳ 待测试
```

#### 场景 29.3: 分支到新上下文
```
输入: Branch session to new context
预期: 调用 branch_session
状态: ⏳ 待测试
```

---

### 30. session_state 工具（3个场景）

#### 场景 30.1: 查看会话状态
```
输入: Show session state
预期: 调用 session_state
状态: ⏳ 待测试
```

#### 场景 30.2: 检查会话健康
```
输入: Check session health
预期: 调用 session_state
状态: ⏳ 待测试
```

#### 场景 30.3: 获取会话信息
```
输入: Get current session information
预期: 调用 session_state
状态: ⏳ 待测试
```

---

### 31. gateway 工具（3个场景）

#### 场景 31.1: 查看网关配置
```
输入: Show gateway configuration
预期: 调用 gateway
状态: ⏳ 待测试
```

#### 场景 31.2: 更新网关设置
```
输入: Update gateway settings
预期: 调用 gateway
状态: ⏳ 待测试
```

#### 场景 31.3: 检查网关状态
```
输入: Check gateway status
预期: 调用 gateway
状态: ⏳ 待测试
```

---

### 32. agents_list 工具（3个场景）

#### 场景 32.1: 列出所有代理
```
输入: List all agents
预期: 调用 agents_list
状态: ⏳ 待测试
```

#### 场景 32.2: 列出活跃代理
```
输入: Show active agents
预期: 调用 agents_list
状态: ⏳ 待测试
```

#### 场景 32.3: 查找特定代理
```
输入: Find agent named 'monitor'
预期: 调用 agents_list
状态: ⏳ 待测试
```

---

### 33. pdf 工具（3个场景）

#### 场景 33.1: 读取 PDF
```
输入: Read content from document.pdf
预期: 调用 pdf
状态: ⏳ 待测试
```

#### 场景 33.2: 提取 PDF 文本
```
输入: Extract text from report.pdf
预期: 调用 pdf
状态: ⏳ 待测试
```

#### 场景 33.3: 分析 PDF
```
输入: Analyze the PDF document
预期: 调用 pdf
状态: ⏳ 待测试
```

---

## 📊 测试统计

**总工具数**: 33  
**总测试场景**: 99  
**已完成**: 2  
**待测试**: 97  
**完成率**: 2%

---

## 🎯 测试优先级

### 高优先级（核心功能）

1. ✅ news_search（已测试 2/3）
2. ⏳ calc
3. ⏳ web_search
4. ⏳ web_fetch
5. ⏳ task_list
6. ⏳ sessions_*（会话相关）

### 中优先级（常用功能）

7. ⏳ exec
8. ⏳ process
9. ⏳ browser
10. ⏳ cron
11. ⏳ skill_*（技能相关）

### 低优先级（高级功能）

12. ⏳ spawn_agent
13. ⏳ nodes_*（节点相关）
14. ⏳ image
15. ⏳ pdf
16. ⏳ 其他工具

---

**文档状态**: ✅ 完整测试计划已创建  
**下一步**: 开始执行测试
