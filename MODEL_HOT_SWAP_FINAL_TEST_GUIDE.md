# 🎯 模型热切换最终测试指南

**测试时间**: 2026年3月17日 23:35  
**测试目标**: 验证模型热切换功能完整性  
**测试标准**: DO-178C Level A  

---

## ✅ 已完成工作

### 1. 代码实现 ✅
- ✅ LocalGgufProvider::reload() - Provider 层
- ✅ models.reload RPC 端点 - Gateway 层
- ✅ selectModel() 热切换逻辑 - 前端层

### 2. 配置修复 ✅
- ✅ 创建 `~/.clawmaster/local_llm.json`
- ✅ 配置两个模型：
  - Llama 3.2 1B (小模型)
  - Qwen 2.5 Coder 14B (大模型)

### 3. 编译验证 ✅
- ✅ 所有核心组件编译成功
- ✅ 无编译警告
- ✅ 无编译错误

---

## 🧪 测试步骤

### 步骤 1: 验证 WebUI 启动 ⏳
```bash
# 检查进程
ps aux | grep clawmaster

# 检查端口
lsof -i :59233

# 访问 WebUI
open https://localhost:59233
```

**预期结果**:
- ✅ 进程正在运行
- ✅ 端口 59233 监听
- ✅ WebUI 可访问

---

### 步骤 2: 验证模型列表 ⏳
```
操作:
1. 打开浏览器访问 https://localhost:59233
2. 点击顶部的模型选择器
3. 查看下拉列表

预期结果:
✅ 显示两个模型:
   - local-llm::llama-3.2-1b-q4_k_m (Llama 3.2 1B)
   - local-llm::qwen2.5-coder-14b-q4_k_m (Qwen 2.5 Coder 14B)

❌ 如果仍显示 "没有匹配的模型":
   - 检查终端日志
   - 查找 "local-llm" 相关错误
   - 验证配置文件格式
```

---

### 步骤 3: 测试基本热切换 ⏳
```
测试场景: 小模型 → 大模型

操作:
1. 确认当前模型: Llama 3.2 1B
2. 点击模型选择器
3. 选择 "Qwen 2.5 Coder 14B"
4. 观察终端日志

预期日志输出:
INFO Received model reload request model_id=local-llm::qwen2.5-coder-14b-q4_k_m
INFO Starting model hot-swap old_model=llama-3.2-1b-q4_k_m new_model=qwen2.5-coder-14b-q4_k_m
INFO Dropping old model model=llama-3.2-1b-q4_k_m
INFO Unloading GGUF model and releasing backend model=llama-3.2-1b-q4_k_m
INFO Old model resources released
INFO Loading new model model=qwen2.5-coder-14b-q4_k_m
INFO initializing llama backend model=qwen2.5-coder-14b-q4_k_m
INFO Model hot-swap completed successfully model=qwen2.5-coder-14b-q4_k_m

预期时间:
- Drop 旧模型: < 1秒
- 等待清理: 300ms
- 加载新模型: 5-10秒
- 总时间: < 15秒

验证:
✅ 日志显示完整流程
✅ 无错误信息
✅ 新模型可用
✅ 可以发送查询
```

---

### 步骤 4: 测试反向切换 ⏳
```
测试场景: 大模型 → 小模型

操作:
1. 确认当前模型: Qwen 2.5 Coder 14B
2. 点击模型选择器
3. 选择 "Llama 3.2 1B"
4. 观察终端日志

预期结果:
✅ 切换成功
✅ 日志完整
✅ 时间更短 (大→小 比 小→大 快)
✅ 无错误
```

---

### 步骤 5: 测试快速连续切换 ⏳
```
测试场景: 压力测试

操作:
1. Llama → Qwen (等待 2秒)
2. Qwen → Llama (等待 2秒)
3. Llama → Qwen (等待 2秒)
4. Qwen → Llama

预期结果:
✅ 所有切换成功
✅ 无崩溃
✅ 无 "backend already initialized" 错误
✅ 资源正确释放
✅ 内存使用稳定
```

---

### 步骤 6: 测试功能验证 ⏳
```
测试场景: 验证新模型可用

操作:
1. 切换到 Qwen 2.5 Coder 14B
2. 发送查询: "用 Rust 写一个 Hello World"
3. 验证响应

预期结果:
✅ 模型正确响应
✅ 响应质量符合 Qwen 特点
✅ 无错误
```

---

## 📊 验收标准

### 功能性
- [ ] 模型列表正确显示
- [ ] 基本切换成功
- [ ] 反向切换成功
- [ ] 快速连续切换成功
- [ ] 新模型功能正常

### 可靠性
- [ ] 无崩溃
- [ ] 无内存泄漏
- [ ] 无资源泄漏
- [ ] 错误处理正确

### 性能
- [ ] 切换时间 < 15秒
- [ ] 日志输出完整
- [ ] 用户体验流畅

### DO-178C Level A
- [ ] 所有需求已实现
- [ ] 所有测试已执行
- [ ] 所有问题已修复
- [ ] 文档完整

---

## 🐛 故障排查

### 问题 1: 模型列表仍为空
```bash
# 检查配置文件
cat ~/.clawmaster/local_llm.json

# 检查日志
grep -i "local-llm\|model" /tmp/clawmaster.log

# 验证文件权限
ls -la ~/.clawmaster/local_llm.json
```

### 问题 2: 切换失败
```bash
# 查看完整日志
tail -f /tmp/clawmaster.log | grep -i "reload\|swap\|model"

# 检查模型文件
ls -lh ~/.clawmaster/models/

# 验证内存
top -pid $(pgrep clawmaster)
```

### 问题 3: Backend 错误
```
错误: "initializing llama backend" 失败

原因: 旧 backend 未完全释放

解决:
1. 增加等待时间 (300ms → 500ms)
2. 验证 Drop trait 被调用
3. 检查 Arc 引用计数
```

---

## 📝 测试记录模板

```
测试日期: 2026-03-17
测试人员: [Your Name]
测试环境: macOS

步骤 1: 验证 WebUI 启动
结果: [ ] PASS  [ ] FAIL
备注: _______________

步骤 2: 验证模型列表
结果: [ ] PASS  [ ] FAIL
模型数量: ___
备注: _______________

步骤 3: 测试基本热切换
结果: [ ] PASS  [ ] FAIL
切换时间: ___ 秒
备注: _______________

步骤 4: 测试反向切换
结果: [ ] PASS  [ ] FAIL
切换时间: ___ 秒
备注: _______________

步骤 5: 测试快速连续切换
结果: [ ] PASS  [ ] FAIL
成功次数: ___ / 4
备注: _______________

步骤 6: 测试功能验证
结果: [ ] PASS  [ ] FAIL
响应质量: [ ] 优秀  [ ] 良好  [ ] 一般
备注: _______________

总体评价:
[ ] 完全通过 - 所有测试 PASS
[ ] 部分通过 - 大部分测试 PASS
[ ] 未通过 - 多个测试 FAIL

发现问题:
1. _______________
2. _______________

建议改进:
1. _______________
2. _______________
```

---

## 🚀 开始测试！

### 当前状态
- ✅ 代码实现完成
- ✅ 配置文件创建
- ✅ WebUI 启动中
- ⏳ 等待模型注册

### 下一步
1. 等待 WebUI 完全启动 (约 10-15 秒)
2. 访问 https://localhost:59233
3. 按照测试步骤执行
4. 记录测试结果
5. 生成最终报告

---

**准备好了吗？让我们开始测试！** 🎯
