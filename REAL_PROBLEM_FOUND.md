# 🎯 真正的问题找到了！

**时间**: 2026年3月18日 11:17  
**问题**: 旧会话数据使用了错误的模型ID  

---

## 🔍 问题根源

### 错误的模型ID
旧的 `main` 会话保存的模型ID是：
```
local-llm::custom-llama-3.1-8b-instruct-q4_k_m.gguf
```

这个模型ID **不存在**，因为：
1. 使用了文件名而不是注册表ID
2. 包含了 `custom-` 前缀
3. 包含了 `.gguf` 后缀

### 正确的模型ID
应该是：
```
local-llm::llama-3.1-8b-q4_k_m
```

---

## 💡 为什么会出现这个问题

1. **配置已修复**: `clawmaster.toml` 和 `local-llm.json` 都已经使用正确的模型ID
2. **新会话正常**: 新创建的会话会使用正确的模型ID
3. **旧会话有问题**: `main` 会话在数据库中保存了旧的错误模型ID
4. **WebUI使用旧数据**: 当用户在 `main` 会话中发送消息时，系统尝试加载旧的模型ID，失败后返回"服务暂时不可用"

---

## 🔧 解决方案

### 执行的修复
```bash
sqlite3 ~/.clawmaster/sessions.db \
  "UPDATE sessions SET model='local-llm::llama-3.1-8b-q4_k_m' WHERE key='main';"
```

### 修复内容
- 更新 `main` 会话的模型ID
- 从错误的 `custom-llama-3.1-8b-instruct-q4_k_m.gguf`
- 改为正确的 `llama-3.1-8b-q4_k_m`

---

## ✅ 验证步骤

### 1. 检查数据库
```bash
sqlite3 ~/.clawmaster/sessions.db "SELECT key, model FROM sessions WHERE key='main';"
```

**期望输出**:
```
main|local-llm::llama-3.1-8b-q4_k_m
```

### 2. 刷新WebUI
用户需要刷新浏览器页面，重新加载会话数据。

### 3. 测试新闻工具
在 `main` 会话中输入：`美国新闻`

---

## 📊 完整的问题链

1. ❌ 最初使用文件名作为模型ID
2. ✅ 修复配置文件使用注册表ID
3. ✅ 重启服务，模型加载成功
4. ❌ 但旧会话数据仍然使用错误的模型ID
5. ✅ 更新会话数据库
6. ⏳ 等待用户刷新WebUI并测试

---

## 🎯 关键经验

### 1. 配置 vs 会话数据
- 配置文件：定义可用的模型
- 会话数据：保存每个会话使用的模型
- **两者必须一致！**

### 2. 数据持久化
会话数据保存在SQLite数据库中，修改配置文件不会自动更新旧会话。

### 3. 完整的修复流程
1. 修复配置文件
2. 重启服务
3. **更新旧会话数据**
4. 刷新客户端

---

**现在问题已修复！用户需要刷新浏览器并重新测试。**
