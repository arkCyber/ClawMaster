# News Search Query 参数修复完成

**修复时间**: 2026-03-19 17:56  
**状态**: ✅ 已完成并编译成功

---

## 🔍 问题诊断

### 原始问题
模型输出：
```json
{"tool": "news_search", "arguments": {"location": "USA"}}
```

错误信息：
```
tool execution failed: missing field `query`
```

### 根本原因
1. `NewsQuery` 结构体要求 `query` 为必需字段
2. 模型输出 `location` 而不是 `country`
3. 模型没有提供 `query` 参数

---

## ✅ 实施的修复

### 1. Query 参数改为可选

**修改前**:
```rust
pub struct NewsQuery {
    pub query: String,  // 必需
    pub country: Option<String>,
    ...
}
```

**修改后**:
```rust
pub struct NewsQuery {
    #[serde(default)]
    pub query: Option<String>,  // 可选
    #[serde(alias = "location")]
    pub country: Option<String>,  // 支持 location 别名
    ...
}
```

---

### 2. 智能默认值实现

新增 `effective_query()` 方法：

```rust
pub fn effective_query(&self) -> String {
    if let Some(q) = &self.query {
        if !q.trim().is_empty() {
            return q.clone();
        }
    }
    
    // 基于 category 生成默认查询
    if let Some(cat) = &self.category {
        return format!("{} news", cat);
    }
    
    // 基于 country 生成默认查询
    if let Some(country) = &self.country {
        return format!("{} news", country);
    }
    
    // 最终回退
    "news".to_string()
}
```

---

### 3. 更新所有使用点

修改了 5 处代码，将 `params.query` 改为 `params.effective_query()`：

1. `query_newsapi()` - NewsAPI 查询
2. `query_rss_feeds()` - RSS 订阅查询
3. `query_google_news_fallback()` - Google News 回退
4. `detect_country_from_query()` - 国家检测
5. `contains_chinese()` - 中文检测

---

## 📊 修复效果

### 支持的输入格式

#### 格式 1: 只有 location
```json
{"location": "USA"}
```
**生成查询**: `"USA news"`

#### 格式 2: 只有 category
```json
{"category": "tech"}
```
**生成查询**: `"tech news"`

#### 格式 3: 完整参数
```json
{"query": "technology", "country": "us"}
```
**使用查询**: `"technology"`

#### 格式 4: location 别名
```json
{"location": "Shanghai"}
```
**映射为**: `country: "Shanghai"`  
**生成查询**: `"Shanghai news"`

---

## 🧪 测试场景

### 场景 1: 美国新闻
**输入**: `美国新闻？`

**模型可能输出**:
- `{"location": "USA"}` ✅ 支持
- `{"country": "us"}` ✅ 支持
- `{"query": "news", "country": "us"}` ✅ 支持

**生成查询**: `"USA news"` 或 `"us news"` 或 `"news"`

---

### 场景 2: 科技新闻
**输入**: `科技新闻`

**模型可能输出**:
- `{"category": "tech"}` ✅ 支持
- `{"query": "technology news"}` ✅ 支持

**生成查询**: `"tech news"` 或 `"technology news"`

---

### 场景 3: 上海新闻
**输入**: `上海新闻`

**模型可能输出**:
- `{"location": "Shanghai"}` ✅ 支持
- `{"country": "cn", "query": "Shanghai"}` ✅ 支持

**生成查询**: `"Shanghai news"` 或 `"Shanghai"`

---

## 📝 代码变更摘要

**文件**: `crates/tools/src/news_tool.rs`

**修改行数**: ~50 行

**关键变更**:
1. ✅ `query: String` → `query: Option<String>`
2. ✅ 添加 `#[serde(alias = "location")]`
3. ✅ 新增 `effective_query()` 方法
4. ✅ 更新 `validate()` 方法
5. ✅ 更新 5 处函数调用

---

## 🚀 部署状态

**编译**: ✅ 成功  
**WebUI**: ✅ 已重启  
**地址**: https://localhost:59233  
**模型**: Llama 3.1 8B (Q4_K_M)

---

## ✅ 验证测试

请在 WebUI 中测试以下场景：

### 测试 1: 美国新闻
```
美国新闻？
```
**预期**: 成功调用工具，返回美国新闻

### 测试 2: 科技新闻
```
科技新闻
```
**预期**: 成功调用工具，返回科技新闻

### 测试 3: 上海新闻
```
上海新闻
```
**预期**: 成功调用工具，返回上海新闻

### 测试 4: 英文新闻
```
US news?
```
**预期**: 成功调用工具，英文回答

---

## 🎯 预期改进

### 修复前
- ❌ `missing field 'query'` 错误
- ❌ 工具调用失败
- ❌ 模型重试多次

### 修复后
- ✅ 接受任何参数组合
- ✅ 智能生成查询字符串
- ✅ 支持 location 别名
- ✅ 工具调用成功

---

**修复完成！请开始测试验证。**
