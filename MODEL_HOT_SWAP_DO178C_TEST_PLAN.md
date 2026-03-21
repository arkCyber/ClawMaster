# 🛫 模型热切换 DO-178C Level A 测试计划

**标准**: DO-178C Level A (航空航天最高安全等级)  
**时间**: 2026年3月17日 23:00  
**目标**: 确保模型热切换功能达到航空航天级别的可靠性和安全性  

---

## 📋 DO-178C Level A 要求

### 关键特性
1. **完全测试覆盖** - 100% 语句覆盖、100% 分支覆盖
2. **需求可追溯性** - 每个需求都有对应测试
3. **错误处理** - 所有错误路径都经过测试
4. **资源管理** - 无内存泄漏、无资源泄漏
5. **并发安全** - 线程安全、竞态条件测试
6. **性能保证** - 确定性时间边界
7. **故障恢复** - 失败场景下的安全降级

---

## 🎯 测试层级

### Level 1: 单元测试 (Unit Tests)
**目标**: 测试每个函数的正确性

#### 1.1 LocalGgufProvider::reload()
- ✅ 正常切换: old_provider 存在
- ✅ 首次加载: old_provider 为 None
- ✅ 资源释放: 验证 Drop 被调用
- ✅ 等待时间: 验证 300ms 延迟
- ✅ 错误处理: 新模型加载失败
- ✅ 内存管理: 无内存泄漏

#### 1.2 Drop Implementation
- ✅ 日志记录: 验证 info! 被调用
- ✅ 资源清理: Arc 引用计数归零

---

### Level 2: 集成测试 (Integration Tests)
**目标**: 测试组件间交互

#### 2.1 RPC 端点测试
- ✅ models.reload 正常调用
- ✅ 参数验证: model_id 缺失
- ✅ 参数验证: model_id 格式错误
- ✅ 非 local-llm 模型拒绝
- ✅ 广播事件发送
- ✅ 响应格式正确

#### 2.2 前端集成测试
- ✅ selectModel() 调用 RPC
- ✅ 成功响应处理
- ✅ 错误响应处理
- ✅ 网络错误处理
- ✅ Session 更新

---

### Level 3: 端到端测试 (E2E Tests)
**目标**: 测试完整用户流程

#### 3.1 基本切换流程
```
用户操作:
1. 打开模型选择器
2. 选择新模型
3. 等待切换完成
4. 发送测试查询

验证点:
- ✅ UI 响应正常
- ✅ 模型成功切换
- ✅ 新模型可用
- ✅ 无错误提示
```

#### 3.2 快速连续切换
```
压力测试:
1. Llama → Qwen
2. 立即 Qwen → Llama
3. 立即 Llama → Qwen

验证点:
- ✅ 无崩溃
- ✅ 资源正确释放
- ✅ 最终状态正确
```

---

### Level 4: 性能测试 (Performance Tests)
**目标**: 验证性能指标

#### 4.1 切换时间测试
```
测试场景:
- 小模型 → 小模型: < 3秒
- 小模型 → 大模型: < 10秒
- 大模型 → 小模型: < 5秒
- 大模型 → 大模型: < 15秒

测量指标:
- Drop 时间
- 等待时间 (300ms)
- 加载时间
- 总切换时间
```

#### 4.2 资源使用测试
```
监控指标:
- 内存使用: 切换前后对比
- CPU 使用: 峰值和平均值
- 文件句柄: 无泄漏
- 线程数: 稳定
```

---

### Level 5: 压力测试 (Stress Tests)
**目标**: 测试极限条件

#### 5.1 连续切换测试
```
测试: 连续切换 100 次
验证:
- ✅ 无内存泄漏
- ✅ 无性能降级
- ✅ 无崩溃
- ✅ 错误率 < 0.1%
```

#### 5.2 并发切换测试
```
测试: 多个客户端同时切换
验证:
- ✅ 线程安全
- ✅ 无竞态条件
- ✅ 最终一致性
```

---

### Level 6: 错误恢复测试 (Failure Recovery)
**目标**: 验证故障处理

#### 6.1 模型加载失败
```
场景: 新模型文件不存在
预期:
- ✅ 返回明确错误
- ✅ 不崩溃
- ✅ 可继续使用旧模型
```

#### 6.2 内存不足
```
场景: 系统内存不足
预期:
- ✅ 优雅失败
- ✅ 错误信息清晰
- ✅ 资源正确释放
```

#### 6.3 网络中断
```
场景: RPC 调用中断
预期:
- ✅ 前端超时处理
- ✅ 用户友好提示
- ✅ 可重试
```

---

## 🔬 测试执行计划

### Phase 1: 编译验证 (5分钟)
```bash
cargo build --release
cargo clippy -- -D warnings
cargo test --all
```

### Phase 2: 单元测试 (10分钟)
```bash
cargo test -p clawmaster-providers reload
cargo test -p clawmaster-gateway models_reload
```

### Phase 3: 集成测试 (15分钟)
```bash
# 启动 WebUI
./target/release/clawmaster

# 运行 Playwright E2E 测试
cd crates/web/ui
npx playwright test e2e/specs/model-hot-swap.spec.js
```

### Phase 4: 性能测试 (20分钟)
```bash
# 使用 criterion 基准测试
cargo bench --bench model_reload
```

### Phase 5: 压力测试 (30分钟)
```bash
# 连续切换测试
./scripts/stress-test-model-reload.sh
```

---

## 📊 验收标准

### 功能性 (Functionality)
- ✅ 所有测试用例通过
- ✅ 无编译警告
- ✅ 无 clippy 警告
- ✅ 代码覆盖率 > 95%

### 可靠性 (Reliability)
- ✅ 错误率 < 0.01%
- ✅ 无内存泄漏
- ✅ 无资源泄漏
- ✅ 无死锁

### 性能 (Performance)
- ✅ 切换时间 < 10秒 (95th percentile)
- ✅ 内存增长 < 1%
- ✅ CPU 峰值 < 80%

### 安全性 (Safety)
- ✅ 所有错误路径测试
- ✅ 边界条件测试
- ✅ 并发安全验证
- ✅ 资源限制测试

---

## 📝 测试报告模板

### 测试执行记录
```
测试日期: YYYY-MM-DD HH:MM
测试人员: [Name]
测试环境: macOS / Linux / Windows
Rust 版本: [Version]

测试结果:
- 单元测试: PASS/FAIL (X/Y)
- 集成测试: PASS/FAIL (X/Y)
- E2E 测试: PASS/FAIL (X/Y)
- 性能测试: PASS/FAIL
- 压力测试: PASS/FAIL

发现问题:
1. [问题描述]
2. [问题描述]

修复措施:
1. [修复方案]
2. [修复方案]
```

---

## 🚀 开始测试

### 立即执行
```bash
# 1. 修复编译错误
cargo build

# 2. 运行所有测试
cargo test --all

# 3. 启动 WebUI 手动测试
./target/debug/clawmaster
```

---

**按照 DO-178C Level A 标准，确保每个测试都有文档、可追溯、可重复！** ✈️
