# 🔍 调试"服务暂时不可用"问题

**时间**: 2026年3月18日 11:16  
**问题**: WebUI显示"服务暂时不可用。请重试。"  

---

## 📊 当前状态

### 服务状态
```
✅ ClawMaster 运行中 (PID: 16726)
✅ 监听端口: https://localhost:59233
✅ 模型加载: 2 个模型
```

### 日志分析
```
INFO clawmaster_chat: models.list response model_count=2
WARN send: clawmaster_chat: client seq out of order (duplicate or reorder) session=main seq=53 last_seq=53
```

### 关键发现
1. **模型已加载**: `model_count=2`
2. **工具已注册**: `news_search` 在工具列表中
3. **序列号警告**: `client seq out of order` - 这可能是问题所在！

---

## 🔍 可能的原因

### 1. 会话状态问题
日志显示 `seq=53 last_seq=53`，说明可能有重复的消息序列号。

### 2. 会话ID问题
用户使用的是 `session=main`，可能这个会话有问题。

### 3. 模型选择问题
用户可能选择了错误的模型ID。

---

## 🎯 诊断步骤

### 1. 检查会话状态
```bash
tail -1000 /tmp/clawmaster_test.log | grep "session=main" | tail -20
```

### 2. 检查模型ID
从gon数据看，最近的会话使用的模型是：
```
local-llm::custom-llama-3.1-8b-instruct-q4_k_m.gguf
```

**这是错误的模型ID！** 应该是：
```
local-llm::llama-3.1-8b-q4_k_m
```

---

## 💡 问题根源

**旧的会话数据仍然使用错误的模型ID！**

用户的 `main` 会话保存了旧的模型ID `custom-llama-3.1-8b-instruct-q4_k_m.gguf`，这个模型不存在，导致"服务暂时不可用"。

---

## 🔧 解决方案

### 方案1: 创建新会话
让用户创建一个新的会话，不使用 `main`。

### 方案2: 清除会话数据
```bash
# 备份
cp ~/.clawmaster/sessions.db ~/.clawmaster/sessions.db.backup

# 清除main会话
sqlite3 ~/.clawmaster/sessions.db "DELETE FROM sessions WHERE key='main';"
```

### 方案3: 更新会话模型
```bash
sqlite3 ~/.clawmaster/sessions.db "UPDATE sessions SET model='local-llm::llama-3.1-8b-q4_k_m' WHERE key='main';"
```

---

## 🚀 立即执行

我将执行方案3，更新会话模型ID。
