# ClawMaster 所有工具测试计划

**测试时间**: 2026-03-21 07:47  
**测试方式**: CLI 自然语言接口  
**模型**: Qwen 3.5 9B (Ollama)

---

## 📋 ClawMaster 工具列表

根据代码分析，ClawMaster 包含以下工具模块：

### 核心工具类别

#### 1. 计算和时间 (2个)
- **calc** - 数学计算
- **time** - 时间查询

#### 2. 文件和系统 (6个)
- **read_file** - 读取文件
- **write_file** - 写入文件
- **glob** - 文件查找
- **grep** - 文本搜索
- **list_dir** - 目录列表
- **exec** - 命令执行

#### 3. Web 和搜索 (4个)
- **web_fetch** - 网页获取
- **web_search** - Web 搜索
- **search_news** - 新闻搜索
- **browser** - 浏览器操作

#### 4. 内存和知识 (3个)
- **memory_save** - 保存记忆
- **memory_search** - 搜索记忆
- **memory_list** - 列出记忆

#### 5. 任务和会话 (5个)
- **task_list** - 任务列表
- **task_create** - 创建任务
- **sessions_list** - 会话列表
- **sessions_communicate** - 会话通信
- **branch_session** - 分支会话

#### 6. 图像和媒体 (3个)
- **image_gen** - 图像生成
- **send_image** - 发送图像
- **pdf_tool** - PDF 处理

#### 7. 定时和自动化 (2个)
- **cron_create** - 创建定时任务
- **cron_list** - 列出定时任务

#### 8. 位置和地图 (2个)
- **location** - 位置服务
- **map** - 地图服务

#### 9. 高级功能 (10个)
- **spawn_agent** - 生成代理
- **agents_list** - 代理列表
- **nodes** - 节点管理
- **process** - 进程管理
- **sandbox** - 沙箱执行
- **apply_patch** - 应用补丁
- **gateway_config** - 网关配置
- **skill_tools** - 技能工具
- **loop_detection** - 循环检测
- **session_state** - 会话状态

**总计**: 约 37+ 个工具

---

## 🧪 测试策略

### 第一阶段：核心工具 (10个)
1. calc - 数学计算
2. time - 时间查询
3. read_file - 文件读取
4. glob - 文件查找
5. grep - 文本搜索
6. list_dir - 目录列表
7. web_search - Web 搜索
8. search_news - 新闻搜索
9. memory_save - 保存记忆
10. task_list - 任务列表

### 第二阶段：文件操作 (5个)
11. write_file - 写入文件
12. exec - 命令执行
13. browser - 浏览器操作
14. web_fetch - 网页获取
15. memory_search - 搜索记忆

### 第三阶段：会话和任务 (5个)
16. sessions_list - 会话列表
17. task_create - 创建任务
18. cron_create - 创建定时任务
19. cron_list - 列出定时任务
20. sessions_communicate - 会话通信

### 第四阶段：高级功能 (测试可用的)
21. image_gen - 图像生成
22. location - 位置服务
23. map - 地图服务
24. agents_list - 代理列表
25. 其他可用工具...

---

## 📊 测试记录

### 已完成测试
- ✅ calc (计算 1234 + 5678 = 6912)

### 待测试
- ⏳ 其他 36+ 个工具

---

**测试状态**: 🔄 准备开始
