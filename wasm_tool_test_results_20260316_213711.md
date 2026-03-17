# ClawMaster WASM 工具测试报告

**测试时间**: Mon Mar 16 21:37:11 CST 2026
**测试会话**: main
**WASM 工具数量**: 3

---

## 测试结果

### ✅ calc (WASM) - 测试 W1.1
- **问题**: 请用 WASM 计算器计算：(100 + 200) * 3
- **结果**: 成功

### ✅ calc (WASM) - 测试 W1.2
- **问题**: 请计算：sqrt(144) + pow(2, 8)
- **结果**: 成功

### ✅ calc (WASM) - 测试 W1.3
- **问题**: 请计算：sin(pi/2) + cos(0)
- **结果**: 成功

### ✅ web_fetch (WASM) - 测试 W2.1
- **问题**: 请使用 WASM 获取 https://example.com 的内容
- **结果**: 成功

### ✅ web_fetch (WASM) - 测试 W2.2
- **问题**: 请用 WASM 工具获取 https://www.rust-lang.org 的内容
- **结果**: 成功

### ✅ web_search (WASM) - 测试 W3.1
- **问题**: 请使用 WASM 搜索：Rust WebAssembly
- **结果**: 成功

### ✅ web_search (WASM) - 测试 W3.2
- **问题**: 请用 WASM 搜索今天的新闻
- **结果**: 成功

### ✅ 对比测试 - 测试 C1.1
- **问题**: 请分别用 WASM 和原生工具计算 123 * 456，比较结果
- **结果**: 成功

### ✅ 对比测试 - 测试 C2.1
- **问题**: 请用两种方式获取 https://example.com，比较性能
- **结果**: 成功


---

## 测试统计

- **总测试数**: 9
- **通过**: 9
- **失败**: 0
- **跳过**: 0
- **通过率**: 100.0%

## WASM 工具信息

### 已测试的 WASM 工具

1. **calc** - WASM 计算器 (Pure Tool)
2. **web_fetch** - WASM 网页获取 (HTTP Tool)
3. **web_search** - WASM 网络搜索 (HTTP Tool)

