# 🚀 快速开始：下载 Llama 3.1 8B 并自动测试

**一键完成所有步骤！**

---

## 🎯 方案 A：完全自动化（推荐）

### 运行自动测试脚本

```bash
cd /Users/arksong/ClawMaster
./auto_test_with_new_model.sh
```

**这个脚本会自动**:
1. ✅ 检查/下载 Llama 3.1 8B 模型（4.9GB）
2. ✅ 查找/创建配置文件
3. ✅ 更新模型配置
4. ✅ 重新编译 ClawMaster
5. ✅ 重启服务
6. ✅ 等待您测试（30秒）
7. ✅ 自动分析日志
8. ✅ 显示测试结果

**预计时间**: 10-15 分钟（主要是下载）

---

## 🎯 方案 B：分步执行

### 步骤 1: 下载模型

```bash
cd /Users/arksong/ClawMaster
./download_llama31_8b.sh
```

下载 Llama 3.1 8B Instruct Q4_K_M（约 4.9GB）

### 步骤 2: 查找配置文件

```bash
find ~ -name "clawmaster.toml" 2>/dev/null
```

### 步骤 3: 修改配置

编辑找到的配置文件：
```toml
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
gpu_layers = 33
temperature = 0.7
context_size = 8192
```

### 步骤 4: 重启服务

```bash
pkill -9 -f clawmaster
./target/debug/clawmaster > /tmp/clawmaster_llama31.log 2>&1 &
sleep 10
```

### 步骤 5: 测试

访问: https://localhost:59233  
输入: `美国新闻`

### 步骤 6: 查看日志

```bash
tail -100 /tmp/clawmaster_llama31.log | grep -E "(tool_mode|native_tools|tool_calls_count)"
```

---

## 📊 预期结果

### ✅ 成功标志

**日志应该显示**:
```
resolved effective tool mode
  effective_mode = Text
  native_tools = false

tool mode configuration for this request
  tool_mode = Text
  native_tools = false

streaming LLM response complete
  tool_calls_count = 1  ← 关键！
```

**WebUI 应该显示**:
```
```tool_call
{"tool": "news_search", "arguments": {"query": "news", "location": "USA"}}
```
```

然后返回实际的新闻列表。

### ❌ 失败标志

如果仍然 `tool_calls_count = 0`:
- 检查 `native_tools` 是否为 `false`
- 检查 `effective_mode` 是否为 `Text`
- 可能需要切换到 API 模型

---

## 🔍 当前状态检查

### 检查新日志是否生效

```bash
# 测试一次后运行
tail -50 /tmp/clawmaster_with_logs.log | grep "tool mode configuration"
```

应该看到新添加的日志输出。

---

## 💡 为什么选择 Llama 3.1 8B？

| 特性 | Llama 3.2 1B | Llama 3.1 8B | Qwen 2.5 Coder 14B |
|------|--------------|--------------|-------------------|
| 工具调用 | ✅ 支持 | ✅✅✅ **最强** | ❌ 不支持 |
| 理解能力 | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| 速度 | 很快 | 快 | 中等 |
| 大小 | 0.9GB | 4.9GB | 8.4GB |
| 推荐度 | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐ |

**Llama 3.1 8B 的优势**:
- ✅ 在 function calling 数据上充分训练
- ✅ 理解复杂指令
- ✅ 多语言支持优秀
- ✅ 社区广泛验证
- ✅ 速度和性能平衡好

---

## 🆘 故障排除

### 问题 1: 下载失败
```bash
# 检查网络
curl -I https://huggingface.co

# 使用代理（如果需要）
export https_proxy=http://your-proxy:port
./download_llama31_8b.sh
```

### 问题 2: 模型加载失败
```bash
# 检查文件完整性
ls -lh ~/.clawmaster/models/llama-3.1-8b-instruct-q4_k_m.gguf

# 应该约 4.9GB
```

### 问题 3: 配置文件位置
```bash
# 创建默认配置
mkdir -p ~/.clawmaster
cat > ~/.clawmaster/clawmaster.toml << 'EOF'
[providers.local-llm]
enabled = true
model_id = "llama-3.1-8b-instruct-q4_k_m"
gpu_layers = 33
temperature = 0.7
context_size = 8192
EOF
```

### 问题 4: 仍然不调用工具
考虑使用 API 模型：
```toml
[providers.anthropic]
enabled = true
api_key = "sk-ant-..."
model = "claude-3-5-sonnet-20241022"

[providers.local-llm]
enabled = false
```

---

## 📝 总结

### 推荐执行方式

**最简单**: 运行 `./auto_test_with_new_model.sh`

这个脚本会：
- 自动下载模型（如果需要）
- 自动配置
- 自动测试
- 自动分析结果

**预计时间**: 10-15 分钟

---

**现在就开始吧！** 🚀

```bash
cd /Users/arksong/ClawMaster
./auto_test_with_new_model.sh
```
