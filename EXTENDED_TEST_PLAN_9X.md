# ClawMaster 扩展测试计划（9倍场景）

**创建时间**: 2026-03-19 23:12  
**工具总数**: 32 个（除 news_search）  
**每工具场景**: 9 个（从 3 个扩展到 9 个）  
**总测试场景**: 288 个

---

## 📊 测试规模

**原计划**: 32 工具 × 3 场景 = 96 个测试  
**新计划**: 32 工具 × 9 场景 = **288 个测试**  
**扩展倍数**: 3 倍  
**预计时间**: 3-4 小时

---

## 🎯 扩展测试场景设计原则

每个工具的 9 个测试场景分为：

1. **基础功能（3个）** - 核心功能验证
2. **边界情况（3个）** - 边界值和异常处理
3. **高级功能（3个）** - 复杂场景和组合功能

---

## 📋 完整测试场景（288个）

### 1. calc 工具（9个场景）

#### 基础功能
1. 简单加法: `计算 123 + 456`
2. 简单减法: `Calculate 1000 - 234`
3. 简单乘法: `What is 25 times 4?`

#### 边界情况
4. 除法运算: `Divide 100 by 5`
5. 负数运算: `Calculate -50 + 30`
6. 小数运算: `What is 3.14 * 2?`

#### 高级功能
7. 复杂表达式: `Calculate (15 + 25) * 3`
8. 幂运算: `What is 2 to the power of 10?`
9. 混合运算: `Compute (100 - 20) / 4 + 5`

---

### 2. web_search 工具（9个场景）

#### 基础功能
1. 技术搜索: `Search for Rust programming tutorials`
2. 中文搜索: `搜索 Rust 编程教程`
3. 简单查询: `Find information about Python`

#### 边界情况
4. 长查询: `Search for comprehensive guide to async programming in Rust with tokio`
5. 特殊字符: `Search for "C++" programming`
6. 多语言: `Search for 日本語 programming tutorials`

#### 高级功能
7. 问题搜索: `How to fix async/await in Rust?`
8. 比较搜索: `Compare Rust vs Go performance`
9. 最新资讯: `Find latest news about Rust 2024`

---

### 3. web_fetch 工具（9个场景）

#### 基础功能
1. 获取网页: `Fetch content from https://www.rust-lang.org`
2. 获取API: `Get data from https://api.github.com/repos/rust-lang/rust`
3. 获取JSON: `Fetch JSON from https://jsonplaceholder.typicode.com/posts/1`

#### 边界情况
4. 大文件: `Fetch https://www.rust-lang.org/learn`
5. 重定向: `Get content from http://rust-lang.org` (会重定向到 https)
6. 超时处理: `Fetch from a slow server`

#### 高级功能
7. 带参数: `Fetch https://api.github.com/search/repositories?q=rust`
8. 多个URL: `Get data from multiple GitHub repos`
9. 解析内容: `Fetch and parse JSON from API`

---

### 4. browser 工具（9个场景）

#### 基础功能
1. 打开网页: `Open https://www.rust-lang.org in browser`
2. 截图: `Take a screenshot of https://github.com`
3. 提取文本: `Extract text from https://www.rust-lang.org`

#### 边界情况
4. 动态内容: `Open a page with JavaScript`
5. 大页面: `Screenshot a long webpage`
6. 多标签: `Open multiple tabs`

#### 高级功能
7. 交互操作: `Click button on webpage`
8. 表单填写: `Fill out a form`
9. 等待加载: `Wait for page to fully load`

---

### 5. exec 工具（9个场景）

#### 基础功能
1. 简单命令: `Run 'echo Hello World' command`
2. 列出文件: `Execute 'ls -la' command`
3. 查看文件: `Show me the contents of README.md`

#### 边界情况
4. 长输出: `Run 'find . -name "*.rs"'`
5. 错误处理: `Execute invalid command`
6. 超时命令: `Run a long-running process`

#### 高级功能
7. 系统信息: `What's my system information?`
8. 管道命令: `Run 'ls | grep test'`
9. 环境变量: `Show environment variables`

---

### 6. process 工具（9个场景）

#### 基础功能
1. 启动进程: `Start a new process for monitoring`
2. 列出进程: `List all running processes`
3. 查看进程: `Show process with ID 1234`

#### 边界情况
4. 停止进程: `Stop process with ID 1234`
5. 重启进程: `Restart the monitoring process`
6. 进程状态: `Check status of process 1234`

#### 高级功能
7. 进程树: `Show process tree`
8. 资源使用: `Show CPU and memory usage`
9. 进程过滤: `List processes using port 8080`

---

### 7. task_list 工具（9个场景）

#### 基础功能
1. 添加任务: `Add a task: Review code changes`
2. 列出任务: `Show me my tasks`
3. 完成任务: `Mark task 1 as complete`

#### 边界情况
4. 删除任务: `Delete task 2`
5. 更新任务: `Update task 1 description`
6. 空列表: `Show tasks when list is empty`

#### 高级功能
7. 优先级: `Add high priority task: Fix critical bug`
8. 截止日期: `Set deadline for task 1`
9. 任务搜索: `Find tasks containing 'code'`

---

### 8. sessions_list 工具（9个场景）

#### 基础功能
1. 列出所有: `List all sessions`
2. 活跃会话: `Show me active sessions`
3. 查找会话: `Find session named 'main'`

#### 边界情况
4. 空会话: `List sessions when none exist`
5. 单个会话: `Show details of session 'main'`
6. 会话计数: `How many sessions are there?`

#### 高级功能
7. 按时间排序: `List sessions by creation time`
8. 按活动排序: `Show most active sessions`
9. 会话过滤: `Find sessions created today`

---

### 9. sessions_history 工具（9个场景）

#### 基础功能
1. 查看历史: `Show history of session main`
2. 最近消息: `Show me the last 10 messages in session main`
3. 搜索历史: `Search for 'news' in session main history`

#### 边界情况
4. 空历史: `Show history of new session`
5. 单条消息: `Show first message in session`
6. 全部历史: `Show all messages in session main`

#### 高级功能
7. 时间范围: `Show messages from last hour`
8. 用户过滤: `Show my messages in session`
9. 导出历史: `Export session history`

---

### 10. sessions_send 工具（9个场景）

#### 基础功能
1. 发送消息: `Send 'Hello' to session test`
2. 发送通知: `Notify session main about the update`
3. 广播消息: `Broadcast 'System maintenance' to all sessions`

#### 边界情况
4. 空消息: `Send empty message to session`
5. 长消息: `Send a very long message`
6. 特殊字符: `Send message with emojis 😊`

#### 高级功能
7. 带附件: `Send file to session`
8. 定时发送: `Schedule message for later`
9. 批量发送: `Send to multiple sessions`

---

### 11. sessions_create 工具（9个场景）

#### 基础功能
1. 创建会话: `Create a new session named 'test'`
2. 临时会话: `Create a temporary session for debugging`
3. 配置会话: `Create session 'dev' with debug mode enabled`

#### 边界情况
4. 重名会话: `Create session with existing name`
5. 空名称: `Create session without name`
6. 特殊字符: `Create session with special chars`

#### 高级功能
7. 带描述: `Create session with description`
8. 带标签: `Create session with tags`
9. 带权限: `Create private session`

---

### 12. sessions_delete 工具（9个场景）

#### 基础功能
1. 删除会话: `Delete session named 'test'`
2. 清理旧会话: `Delete all inactive sessions`
3. 删除临时: `Remove temporary sessions`

#### 边界情况
4. 删除不存在: `Delete non-existent session`
5. 删除活跃: `Delete currently active session`
6. 批量删除: `Delete multiple sessions`

#### 高级功能
7. 条件删除: `Delete sessions older than 7 days`
8. 确认删除: `Delete with confirmation`
9. 软删除: `Archive session instead of delete`

---

### 13. spawn_agent 工具（9个场景）

#### 基础功能
1. 生成代理: `Spawn a new agent for monitoring`
2. 专用代理: `Create an agent specialized in news gathering`
3. 临时代理: `Spawn a temporary agent for this task`

#### 边界情况
4. 多个代理: `Spawn 3 agents`
5. 代理限制: `Check agent spawn limit`
6. 代理冲突: `Spawn agent with existing name`

#### 高级功能
7. 带配置: `Spawn agent with custom config`
8. 带权限: `Create agent with limited permissions`
9. 代理通信: `Spawn agent and establish communication`

---

### 14. show_map 工具（9个场景）

#### 基础功能
1. 城市地图: `Show me a map of Beijing`
2. 地区地图: `Display a map of Silicon Valley`
3. 路线地图: `Show route from Beijing to Shanghai`

#### 边界情况
4. 缩放级别: `Show map with zoom level 15`
5. 地图类型: `Show satellite map of Beijing`
6. 多个位置: `Show map with multiple markers`

#### 高级功能
7. 实时交通: `Show traffic on map`
8. 3D地图: `Display 3D map of city`
9. 地图导出: `Export map as image`

---

### 15. get_user_location 工具（9个场景）

#### 基础功能
1. 当前位置: `Where am I?`
2. 位置详情: `What's my current location with details?`
3. GPS坐标: `Give me my GPS coordinates`

#### 边界情况
4. 精度要求: `Get high-precision location`
5. 位置历史: `Show my location history`
6. 位置权限: `Check location permissions`

#### 高级功能
7. 地址解析: `Convert coordinates to address`
8. 附近地点: `Find nearby places`
9. 位置分享: `Share my location`

---

### 16. send_image 工具（9个场景）

#### 基础功能
1. 发送图片: `Send image.png to the chat`
2. 发送截图: `Send a screenshot of the current screen`
3. 发送图表: `Send the chart as an image`

#### 边界情况
4. 大图片: `Send high-resolution image`
5. 多张图片: `Send multiple images`
6. 图片格式: `Send image in different format`

#### 高级功能
7. 压缩图片: `Send compressed image`
8. 带描述: `Send image with caption`
9. 图片编辑: `Edit and send image`

---

### 17. image 工具（9个场景）

#### 基础功能
1. 生成图片: `Generate an image of a sunset`
2. 创建图表: `Create a bar chart showing sales data`
3. 生成图标: `Generate an icon for the app`

#### 边界情况
4. 图片尺寸: `Generate 1920x1080 image`
5. 图片风格: `Create image in watercolor style`
6. 多个图片: `Generate 3 variations`

#### 高级功能
7. AI生成: `Use AI to generate realistic image`
8. 图片合成: `Combine multiple images`
9. 图片优化: `Generate and optimize image`

---

### 18. sandbox_packages 工具（9个场景）

#### 基础功能
1. 列出包: `List all sandbox packages`
2. 搜索包: `Find packages related to Python`
3. 包详情: `Show details of package 'rust'`

#### 边界情况
4. 空包列表: `List when no packages installed`
5. 包版本: `Show all versions of package`
6. 包依赖: `Show dependencies of package`

#### 高级功能
7. 安装包: `Install package 'nodejs'`
8. 更新包: `Update all packages`
9. 卸载包: `Remove package 'python'`

---

### 19. nodes_list 工具（9个场景）

#### 基础功能
1. 列出所有: `List all nodes`
2. 活跃节点: `Show active nodes`
3. 按类型列出: `List compute nodes`

#### 边界情况
4. 空节点: `List when no nodes available`
5. 节点状态: `Show node status`
6. 节点计数: `How many nodes are there?`

#### 高级功能
7. 节点过滤: `List nodes with high CPU`
8. 节点排序: `Sort nodes by performance`
9. 节点分组: `Group nodes by region`

---

### 20. nodes_describe 工具（9个场景）

#### 基础功能
1. 描述节点: `Describe node 'node1'`
2. 节点状态: `What's the status of node 'node1'?`
3. 节点配置: `Show configuration of node 'node1'`

#### 边界情况
4. 不存在节点: `Describe non-existent node`
5. 节点详情: `Show all details of node`
6. 节点历史: `Show node history`

#### 高级功能
7. 节点性能: `Show performance metrics`
8. 节点日志: `Show node logs`
9. 节点诊断: `Run diagnostics on node`

---

### 21. nodes_select 工具（9个场景）

#### 基础功能
1. 选择节点: `Select node 'node1'`
2. 最佳节点: `Select the best node for computation`
3. 空闲节点: `Select an idle node`

#### 边界情况
4. 多个节点: `Select multiple nodes`
5. 节点不可用: `Select when all nodes busy`
6. 节点优先级: `Select by priority`

#### 高级功能
7. 负载均衡: `Select node with load balancing`
8. 地理位置: `Select nearest node`
9. 自动选择: `Auto-select optimal node`

---

### 22. loop_detection 工具（9个场景）

#### 基础功能
1. 检测循环: `Check for loops in the current process`
2. 分析循环: `Analyze loop patterns`
3. 报告循环: `Report any detected loops`

#### 边界情况
4. 无循环: `Check when no loops exist`
5. 嵌套循环: `Detect nested loops`
6. 循环深度: `Show loop depth`

#### 高级功能
7. 循环优化: `Suggest loop optimizations`
8. 循环性能: `Analyze loop performance`
9. 循环重构: `Refactor detected loops`

---

### 23. create_skill 工具（9个场景）

#### 基础功能
1. 创建技能: `Create a new skill called 'weather_checker'`
2. 数据技能: `Create a skill for data analysis`
3. 自动化技能: `Create an automation skill`

#### 边界情况
4. 重名技能: `Create skill with existing name`
5. 空技能: `Create minimal skill`
6. 复杂技能: `Create skill with dependencies`

#### 高级功能
7. 带模板: `Create skill from template`
8. 带测试: `Create skill with tests`
9. 技能发布: `Create and publish skill`

---

### 24. update_skill 工具（9个场景）

#### 基础功能
1. 更新技能: `Update skill 'weather_checker'`
2. 修改配置: `Modify configuration of skill 'data_analyzer'`
3. 升级技能: `Upgrade skill to latest version`

#### 边界情况
4. 不存在技能: `Update non-existent skill`
5. 版本冲突: `Update with version conflict`
6. 回滚更新: `Rollback skill update`

#### 高级功能
7. 批量更新: `Update all skills`
8. 选择性更新: `Update specific components`
9. 测试更新: `Test skill update before applying`

---

### 25. delete_skill 工具（9个场景）

#### 基础功能
1. 删除技能: `Delete skill 'old_skill'`
2. 移除未使用: `Remove unused skills`
3. 清理技能: `Clean up old skills`

#### 边界情况
4. 删除依赖: `Delete skill with dependencies`
5. 强制删除: `Force delete skill`
6. 删除确认: `Delete with confirmation`

#### 高级功能
7. 批量删除: `Delete multiple skills`
8. 条件删除: `Delete skills not used in 30 days`
9. 备份删除: `Backup before delete`

---

### 26. cron 工具（9个场景）

#### 基础功能
1. 创建定时: `Schedule a task to run every day at 9am`
2. 列出定时: `Show all scheduled tasks`
3. 删除定时: `Remove the daily backup task`

#### 边界情况
4. 无效时间: `Schedule with invalid time`
5. 冲突任务: `Schedule conflicting tasks`
6. 过期任务: `Handle expired tasks`

#### 高级功能
7. 复杂定时: `Schedule task with complex cron expression`
8. 任务依赖: `Schedule dependent tasks`
9. 任务监控: `Monitor scheduled tasks`

---

### 27. apply_patch 工具（9个场景）

#### 基础功能
1. 应用补丁: `Apply patch file 'fix.patch'`
2. 代码更新: `Apply the code update patch`
3. 安全补丁: `Apply security patch`

#### 边界情况
4. 冲突补丁: `Apply patch with conflicts`
5. 大补丁: `Apply large patch file`
6. 多个补丁: `Apply multiple patches`

#### 高级功能
7. 测试补丁: `Test patch before applying`
8. 回滚补丁: `Rollback applied patch`
9. 补丁验证: `Verify patch integrity`

---

### 28. branch_session 工具（9个场景）

#### 基础功能
1. 创建分支: `Branch current session`
2. 实验分支: `Create an experimental branch of this session`
3. 新上下文: `Branch session to new context`

#### 边界情况
4. 空会话分支: `Branch empty session`
5. 多层分支: `Create nested branches`
6. 分支限制: `Check branch limit`

#### 高级功能
7. 分支合并: `Merge session branches`
8. 分支比较: `Compare session branches`
9. 分支管理: `Manage session branches`

---

### 29. session_state 工具（9个场景）

#### 基础功能
1. 查看状态: `Show session state`
2. 检查健康: `Check session health`
3. 会话信息: `Get current session information`

#### 边界情况
4. 状态历史: `Show session state history`
5. 状态变化: `Track state changes`
6. 状态恢复: `Restore previous state`

#### 高级功能
7. 状态导出: `Export session state`
8. 状态比较: `Compare session states`
9. 状态监控: `Monitor session state`

---

### 30. gateway 工具（9个场景）

#### 基础功能
1. 查看配置: `Show gateway configuration`
2. 更新设置: `Update gateway settings`
3. 检查状态: `Check gateway status`

#### 边界情况
4. 配置验证: `Validate gateway config`
5. 配置备份: `Backup gateway config`
6. 配置恢复: `Restore gateway config`

#### 高级功能
7. 性能调优: `Optimize gateway performance`
8. 安全设置: `Configure gateway security`
9. 监控指标: `Show gateway metrics`

---

### 31. agents_list 工具（9个场景）

#### 基础功能
1. 列出所有: `List all agents`
2. 活跃代理: `Show active agents`
3. 查找代理: `Find agent named 'monitor'`

#### 边界情况
4. 空代理列表: `List when no agents exist`
5. 代理状态: `Show agent status`
6. 代理计数: `How many agents are running?`

#### 高级功能
7. 代理过滤: `List agents by type`
8. 代理排序: `Sort agents by activity`
9. 代理分组: `Group agents by function`

---

### 32. pdf 工具（9个场景）

#### 基础功能
1. 读取PDF: `Read content from document.pdf`
2. 提取文本: `Extract text from report.pdf`
3. 分析PDF: `Analyze the PDF document`

#### 边界情况
4. 大PDF: `Read large PDF file`
5. 加密PDF: `Read password-protected PDF`
6. 损坏PDF: `Handle corrupted PDF`

#### 高级功能
7. PDF转换: `Convert PDF to text`
8. PDF搜索: `Search for keyword in PDF`
9. PDF合并: `Merge multiple PDFs`

---

## 📊 测试统计

**工具总数**: 32  
**每工具场景**: 9  
**总测试场景**: 288  
**预计时间**: 3-4 小时  
**覆盖类型**: 基础(96) + 边界(96) + 高级(96)

---

**文档状态**: ✅ 完整  
**测试准备**: 就绪  
**下一步**: 创建扩展测试脚本
