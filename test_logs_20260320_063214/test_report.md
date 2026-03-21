# ClawMaster 所有工具 CLI 测试报告

**执行时间**: Fri Mar 20 06:32:14 CST 2026  
**工具总数**: 32  
**测试场景**: 96  
**日志目录**: ./test_logs_20260320_063214

---

## 测试结果

### ❌ 测试 1: calc - 简单算术

**输入**: 计算 123 + 456  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/1_calc_简单算术.log

---

### ❌ 测试 2: calc - 复杂表达式

**输入**: Calculate (15 + 25) * 3  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/2_calc_复杂表达式.log

---

### ❌ 测试 3: calc - 幂运算

**输入**: What is 2 to the power of 10?  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/3_calc_幂运算.log

---

### ❌ 测试 4: web_search - 技术搜索

**输入**: Search for Rust programming tutorials  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/4_web_search_技术搜索.log

---

### ❌ 测试 5: web_search - 中文搜索

**输入**: 搜索 Rust 编程教程  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/5_web_search_中文搜索.log

---

### ❌ 测试 6: web_search - 问题搜索

**输入**: How to fix async/await in Rust?  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/6_web_search_问题搜索.log

---

### ❌ 测试 7: web_fetch - 获取网页

**输入**: Fetch content from https://www.rust-lang.org  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/7_web_fetch_获取网页.log

---

### ❌ 测试 8: web_fetch - 获取API

**输入**: Get data from https://api.github.com/repos/rust-lang/rust  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/8_web_fetch_获取API.log

---

### ❌ 测试 9: web_fetch - 获取JSON

**输入**: Fetch JSON from https://jsonplaceholder.typicode.com/posts/1  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/9_web_fetch_获取JSON.log

---

### ❌ 测试 10: browser - 打开网页

**输入**: Open https://www.rust-lang.org in browser  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/10_browser_打开网页.log

---

### ❌ 测试 11: browser - 截图

**输入**: Take a screenshot of https://github.com  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/11_browser_截图.log

---

### ❌ 测试 12: browser - 提取内容

**输入**: Extract text from https://www.rust-lang.org  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/12_browser_提取内容.log

---

### ❌ 测试 13: exec - 执行命令

**输入**: Run 'echo Hello World' command  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/13_exec_执行命令.log

---

### ❌ 测试 14: exec - 系统信息

**输入**: What's my system information?  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/14_exec_系统信息.log

---

### ❌ 测试 15: exec - 查看文件

**输入**: Show me the contents of README.md  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/15_exec_查看文件.log

---

### ❌ 测试 16: process - 启动进程

**输入**: Start a new process for monitoring  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/16_process_启动进程.log

---

### ❌ 测试 17: process - 列出进程

**输入**: List all running processes  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/17_process_列出进程.log

---

### ❌ 测试 18: process - 停止进程

**输入**: Stop process with ID 1234  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/18_process_停止进程.log

---

### ❌ 测试 19: task_list - 添加任务

**输入**: Add a task: Review code changes  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/19_task_list_添加任务.log

---

### ❌ 测试 20: task_list - 列出任务

**输入**: Show me my tasks  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/20_task_list_列出任务.log

---

### ❌ 测试 21: task_list - 完成任务

**输入**: Mark task 1 as complete  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/21_task_list_完成任务.log

---

### ❌ 测试 22: sessions_list - 列出会话

**输入**: List all sessions  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/22_sessions_list_列出会话.log

---

### ❌ 测试 23: sessions_list - 活跃会话

**输入**: Show me active sessions  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/23_sessions_list_活跃会话.log

---

### ❌ 测试 24: sessions_list - 查找会话

**输入**: Find session named 'main'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/24_sessions_list_查找会话.log

---

### ❌ 测试 25: sessions_history - 查看历史

**输入**: Show history of session main  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/25_sessions_history_查看历史.log

---

### ❌ 测试 26: sessions_history - 最近消息

**输入**: Show me the last 10 messages in session main  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/26_sessions_history_最近消息.log

---

### ❌ 测试 27: sessions_history - 搜索历史

**输入**: Search for 'news' in session main history  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/27_sessions_history_搜索历史.log

---

### ❌ 测试 28: sessions_send - 发送消息

**输入**: Send 'Hello' to session test  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/28_sessions_send_发送消息.log

---

### ❌ 测试 29: sessions_send - 发送通知

**输入**: Notify session main about the update  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/29_sessions_send_发送通知.log

---

### ❌ 测试 30: sessions_send - 广播消息

**输入**: Broadcast 'System maintenance' to all sessions  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/30_sessions_send_广播消息.log

---

### ❌ 测试 31: sessions_create - 创建会话

**输入**: Create a new session named 'test'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/31_sessions_create_创建会话.log

---

### ❌ 测试 32: sessions_create - 临时会话

**输入**: Create a temporary session for debugging  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/32_sessions_create_临时会话.log

---

### ❌ 测试 33: sessions_create - 配置会话

**输入**: Create session 'dev' with debug mode enabled  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/33_sessions_create_配置会话.log

---

### ❌ 测试 34: sessions_delete - 删除会话

**输入**: Delete session named 'test'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/34_sessions_delete_删除会话.log

---

### ❌ 测试 35: sessions_delete - 清理会话

**输入**: Delete all inactive sessions  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/35_sessions_delete_清理会话.log

---

### ❌ 测试 36: sessions_delete - 删除临时

**输入**: Remove temporary sessions  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/36_sessions_delete_删除临时.log

---

### ❌ 测试 37: spawn_agent - 生成代理

**输入**: Spawn a new agent for monitoring  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/37_spawn_agent_生成代理.log

---

### ❌ 测试 38: spawn_agent - 专用代理

**输入**: Create an agent specialized in news gathering  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/38_spawn_agent_专用代理.log

---

### ❌ 测试 39: spawn_agent - 临时代理

**输入**: Spawn a temporary agent for this task  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/39_spawn_agent_临时代理.log

---

### ❌ 测试 40: show_map - 城市地图

**输入**: Show me a map of Beijing  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/40_show_map_城市地图.log

---

### ❌ 测试 41: show_map - 地区地图

**输入**: Display a map of Silicon Valley  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/41_show_map_地区地图.log

---

### ❌ 测试 42: show_map - 路线地图

**输入**: Show route from Beijing to Shanghai  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/42_show_map_路线地图.log

---

### ❌ 测试 43: get_user_location - 当前位置

**输入**: Where am I?  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/43_get_user_location_当前位置.log

---

### ❌ 测试 44: get_user_location - 位置详情

**输入**: What's my current location with details?  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/44_get_user_location_位置详情.log

---

### ❌ 测试 45: get_user_location - GPS坐标

**输入**: Give me my GPS coordinates  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/45_get_user_location_GPS坐标.log

---

### ❌ 测试 46: send_image - 发送图片

**输入**: Send image.png to the chat  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/46_send_image_发送图片.log

---

### ❌ 测试 47: send_image - 发送截图

**输入**: Send a screenshot of the current screen  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/47_send_image_发送截图.log

---

### ❌ 测试 48: send_image - 发送图表

**输入**: Send the chart as an image  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/48_send_image_发送图表.log

---

### ❌ 测试 49: image - 生成图片

**输入**: Generate an image of a sunset  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/49_image_生成图片.log

---

### ❌ 测试 50: image - 创建图表

**输入**: Create a bar chart showing sales data  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/50_image_创建图表.log

---

### ❌ 测试 51: image - 生成图标

**输入**: Generate an icon for the app  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/51_image_生成图标.log

---

### ❌ 测试 52: sandbox_packages - 列出包

**输入**: List all sandbox packages  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/52_sandbox_packages_列出包.log

---

### ❌ 测试 53: sandbox_packages - 搜索包

**输入**: Find packages related to Python  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/53_sandbox_packages_搜索包.log

---

### ❌ 测试 54: sandbox_packages - 包详情

**输入**: Show details of package 'rust'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/54_sandbox_packages_包详情.log

---

### ❌ 测试 55: nodes_list - 列出节点

**输入**: List all nodes  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/55_nodes_list_列出节点.log

---

### ❌ 测试 56: nodes_list - 活跃节点

**输入**: Show active nodes  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/56_nodes_list_活跃节点.log

---

### ❌ 测试 57: nodes_list - 按类型列出

**输入**: List compute nodes  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/57_nodes_list_按类型列出.log

---

### ❌ 测试 58: nodes_describe - 描述节点

**输入**: Describe node 'node1'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/58_nodes_describe_描述节点.log

---

### ❌ 测试 59: nodes_describe - 节点状态

**输入**: What's the status of node 'node1'?  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/59_nodes_describe_节点状态.log

---

### ❌ 测试 60: nodes_describe - 节点配置

**输入**: Show configuration of node 'node1'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/60_nodes_describe_节点配置.log

---

### ❌ 测试 61: nodes_select - 选择节点

**输入**: Select node 'node1'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/61_nodes_select_选择节点.log

---

### ❌ 测试 62: nodes_select - 最佳节点

**输入**: Select the best node for computation  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/62_nodes_select_最佳节点.log

---

### ❌ 测试 63: nodes_select - 空闲节点

**输入**: Select an idle node  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/63_nodes_select_空闲节点.log

---

### ❌ 测试 64: loop_detection - 检测循环

**输入**: Check for loops in the current process  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/64_loop_detection_检测循环.log

---

### ❌ 测试 65: loop_detection - 分析循环

**输入**: Analyze loop patterns  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/65_loop_detection_分析循环.log

---

### ❌ 测试 66: loop_detection - 报告循环

**输入**: Report any detected loops  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/66_loop_detection_报告循环.log

---

### ❌ 测试 67: create_skill - 创建技能

**输入**: Create a new skill called 'weather_checker'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/67_create_skill_创建技能.log

---

### ❌ 测试 68: create_skill - 数据技能

**输入**: Create a skill for data analysis  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/68_create_skill_数据技能.log

---

### ❌ 测试 69: create_skill - 自动化技能

**输入**: Create an automation skill  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/69_create_skill_自动化技能.log

---

### ❌ 测试 70: update_skill - 更新技能

**输入**: Update skill 'weather_checker'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/70_update_skill_更新技能.log

---

### ❌ 测试 71: update_skill - 修改配置

**输入**: Modify configuration of skill 'data_analyzer'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/71_update_skill_修改配置.log

---

### ❌ 测试 72: update_skill - 升级技能

**输入**: Upgrade skill to latest version  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/72_update_skill_升级技能.log

---

### ❌ 测试 73: delete_skill - 删除技能

**输入**: Delete skill 'old_skill'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/73_delete_skill_删除技能.log

---

### ❌ 测试 74: delete_skill - 移除未使用

**输入**: Remove unused skills  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/74_delete_skill_移除未使用.log

---

### ❌ 测试 75: delete_skill - 清理技能

**输入**: Clean up old skills  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/75_delete_skill_清理技能.log

---

### ❌ 测试 76: cron - 创建定时

**输入**: Schedule a task to run every day at 9am  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/76_cron_创建定时.log

---

### ❌ 测试 77: cron - 列出定时

**输入**: Show all scheduled tasks  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/77_cron_列出定时.log

---

### ❌ 测试 78: cron - 删除定时

**输入**: Remove the daily backup task  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/78_cron_删除定时.log

---

### ❌ 测试 79: apply_patch - 应用补丁

**输入**: Apply patch file 'fix.patch'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/79_apply_patch_应用补丁.log

---

### ❌ 测试 80: apply_patch - 代码更新

**输入**: Apply the code update patch  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/80_apply_patch_代码更新.log

---

### ❌ 测试 81: apply_patch - 安全补丁

**输入**: Apply security patch  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/81_apply_patch_安全补丁.log

---

### ❌ 测试 82: branch_session - 创建分支

**输入**: Branch current session  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/82_branch_session_创建分支.log

---

### ❌ 测试 83: branch_session - 实验分支

**输入**: Create an experimental branch of this session  
**状态**: 失败  
**退出码**: 127  
**耗时**: 1s  
**日志**: ./test_logs_20260320_063214/83_branch_session_实验分支.log

---

### ❌ 测试 84: branch_session - 新上下文

**输入**: Branch session to new context  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/84_branch_session_新上下文.log

---

### ❌ 测试 85: session_state - 查看状态

**输入**: Show session state  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/85_session_state_查看状态.log

---

### ❌ 测试 86: session_state - 检查健康

**输入**: Check session health  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/86_session_state_检查健康.log

---

### ❌ 测试 87: session_state - 会话信息

**输入**: Get current session information  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/87_session_state_会话信息.log

---

### ❌ 测试 88: gateway - 查看配置

**输入**: Show gateway configuration  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/88_gateway_查看配置.log

---

### ❌ 测试 89: gateway - 更新设置

**输入**: Update gateway settings  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/89_gateway_更新设置.log

---

### ❌ 测试 90: gateway - 检查状态

**输入**: Check gateway status  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/90_gateway_检查状态.log

---

### ❌ 测试 91: agents_list - 列出代理

**输入**: List all agents  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/91_agents_list_列出代理.log

---

### ❌ 测试 92: agents_list - 活跃代理

**输入**: Show active agents  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/92_agents_list_活跃代理.log

---

### ❌ 测试 93: agents_list - 查找代理

**输入**: Find agent named 'monitor'  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/93_agents_list_查找代理.log

---

### ❌ 测试 94: pdf - 读取PDF

**输入**: Read content from document.pdf  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/94_pdf_读取PDF.log

---

### ❌ 测试 95: pdf - 提取文本

**输入**: Extract text from report.pdf  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/95_pdf_提取文本.log

---

### ❌ 测试 96: pdf - 分析PDF

**输入**: Analyze the PDF document  
**状态**: 失败  
**退出码**: 127  
**耗时**: 0s  
**日志**: ./test_logs_20260320_063214/96_pdf_分析PDF.log

---


## 测试摘要

**总测试数**: 96  
**通过**: 0 ✅  
**失败**: 96 ❌  
**超时**: 0 ⏱️  
**通过率**: 0%

---

## 日志文件

- **主日志**: ./test_logs_20260320_063214/master_test.log
- **详细日志**: ./test_logs_20260320_063214/*.log

---

**报告生成时间**: Fri Mar 20 06:33:53 CST 2026

