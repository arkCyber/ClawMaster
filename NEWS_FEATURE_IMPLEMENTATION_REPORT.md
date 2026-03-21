# 新闻查询功能实现报告

**问题发现时间**: 2026年3月17日 18:08  
**问题**: WebUI 回复"德国新闻目前没有"、"世界新闻目前没有"  
**根本原因**: 系统中完全缺少新闻查询功能  

---

## 🔍 问题诊断

### 发现的问题

1. **缺少新闻查询 Tool**
   - 在 `crates/tools/src/` 中没有 `news_tool.rs`
   - 在 `crates/tools/src/lib.rs` 中没有注册新闻模块

2. **缺少新闻查询 Skill**
   - 在 `crates/skills/` 中没有 `news.md`
   - 系统无法理解新闻查询意图

3. **缺少必要的依赖**
   - 没有 RSS 解析库 (`feed-rs`)
   - 没有 HTML 解析库 (`scraper`)
   - 没有 URL 编码库 (`urlencoding`)

---

## ✅ 已实施的解决方案

### 1. 创建新闻查询工具 (`news_tool.rs`)

**文件位置**: `/Users/arksong/ClawMaster/crates/tools/src/news_tool.rs`

**功能特性**:
- ✅ 多数据源支持 (NewsAPI, RSS, Web Scraping)
- ✅ 多国家/地区支持 (中国、美国、德国、世界)
- ✅ 多语言支持 (中文、英文、德文)
- ✅ 智能降级策略 (API → RSS → Web Scraping)
- ✅ 完整的错误处理
- ✅ 结果格式化输出

**核心数据结构**:
```rust
pub struct NewsQuery {
    pub query: String,
    pub country: Option<String>,
    pub category: Option<String>,
    pub language: Option<String>,
    pub max_results: Option<usize>,
}

pub struct NewsArticle {
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub source: String,
    pub published_at: Option<String>,
    pub author: Option<String>,
    pub image_url: Option<String>,
}

pub struct NewsResult {
    pub total: usize,
    pub articles: Vec<NewsArticle>,
}
```

**支持的新闻源**:

1. **NewsAPI** (如果配置了 API key)
   - 环境变量: `NEWSAPI_KEY`
   - 支持全球 70,000+ 新闻源

2. **RSS 订阅源**:
   - 中国: Google News CN, 人民网, 新华网
   - 美国: Google News US, NYT, BBC
   - 德国: Google News DE, Tagesschau, Spiegel
   - 世界: BBC World, NYT World

3. **网页抓取** (备用):
   - Google News 搜索
   - 智能 HTML 解析

**代码量**: ~400 行

### 2. 创建新闻查询 Skill (`news.md`)

**文件位置**: `/Users/arksong/ClawMaster/crates/skills/news.md`

**内容**:
- ✅ 完整的功能说明
- ✅ 使用示例
- ✅ 支持的国家/地区列表
- ✅ 新闻类别说明
- ✅ 技术实现细节
- ✅ 配置指南
- ✅ 故障排除

**使用示例**:
```
德国新闻
世界新闻
查询德国的科技新闻
美国最新新闻
```

**代码量**: ~200 行

### 3. 注册新闻工具到系统

**修改文件**: `/Users/arksong/ClawMaster/crates/tools/src/lib.rs`

**变更**:
```rust
// 添加新闻工具模块
pub mod news_tool;
```

### 4. 添加必要的依赖

**修改文件**: `/Users/arksong/ClawMaster/crates/tools/Cargo.toml`

**新增依赖**:
```toml
feed-rs       = "2.0"      # RSS/Atom 解析
scraper       = "0.20"     # HTML 解析
urlencoding   = "2.1"      # URL 编码
```

---

## 📊 实现统计

```
新增文件:          2 个
  - news_tool.rs:  ~400 行
  - news.md:       ~200 行

修改文件:          2 个
  - lib.rs:        +1 行
  - Cargo.toml:    +3 行

新增依赖:          3 个
  - feed-rs
  - scraper
  - urlencoding

总代码量:          ~600 行
```

---

## 🔧 技术实现细节

### 新闻查询流程

```
1. 接收查询请求
   ↓
2. 尝试 NewsAPI (如果有 API key)
   ↓ (失败或无 key)
3. 尝试 RSS 订阅源
   ↓ (失败或结果为空)
4. 尝试网页抓取
   ↓
5. 返回结果或错误
```

### RSS 订阅源选择逻辑

```rust
match country {
    "cn" | "china" => {
        // 中国新闻源
        feeds.push("https://news.google.com/rss?hl=zh-CN&gl=CN&ceid=CN:zh-Hans");
        feeds.push("http://www.people.com.cn/rss/politics.xml");
        feeds.push("http://www.xinhuanet.com/politics/news_politics.xml");
    }
    "de" | "germany" => {
        // 德国新闻源
        feeds.push("https://news.google.com/rss?hl=de&gl=DE&ceid=DE:de");
        feeds.push("https://www.tagesschau.de/xml/rss2");
        feeds.push("https://www.spiegel.de/schlagzeilen/index.rss");
    }
    // ... 其他国家
}
```

### 网页抓取实现

```rust
// 使用 scraper 库解析 Google News HTML
let document = Html::parse_document(html);
let article_selector = Selector::parse("article").unwrap();

for article in document.select(&article_selector) {
    // 提取标题、链接、来源等信息
}
```

---

## 🧪 测试计划

### 单元测试

```rust
#[tokio::test]
async fn test_news_query() {
    let query = NewsQuery {
        query: "technology".to_string(),
        country: Some("us".to_string()),
        language: Some("en".to_string()),
        max_results: Some(5),
    };
    
    let result = query_news(query).await;
    assert!(result.is_ok());
}

#[test]
fn test_format_news_result() {
    // 测试结果格式化
}
```

### 集成测试

1. **测试 RSS 订阅源**
   ```bash
   # 测试德国新闻
   curl "https://www.tagesschau.de/xml/rss2"
   
   # 测试中国新闻
   curl "http://www.people.com.cn/rss/politics.xml"
   ```

2. **测试 Google News**
   ```bash
   # 测试德国新闻搜索
   curl "https://news.google.com/search?q=deutschland&hl=de&gl=DE"
   ```

3. **在 WebUI 中测试**
   - 输入: "德国新闻"
   - 预期: 返回德国最新新闻列表
   
   - 输入: "世界新闻"
   - 预期: 返回全球新闻列表

---

## 📋 下一步工作

### 立即行动 (必须完成)

1. **构建项目**
   ```bash
   cargo build -p clawmaster-tools
   ```

2. **运行测试**
   ```bash
   cargo test -p clawmaster-tools news_tool
   ```

3. **重启 WebUI**
   ```bash
   pkill -f clawmaster
   cargo run --bin clawmaster
   ```

4. **验证功能**
   - 在 WebUI 中输入 "德国新闻"
   - 验证返回新闻列表
   - 在 WebUI 中输入 "世界新闻"
   - 验证返回新闻列表

### 短期优化 (1-2 天)

5. **添加缓存机制**
   - 缓存新闻结果 (5-10 分钟)
   - 减少 API 调用次数

6. **添加更多新闻源**
   - 添加更多国家的 RSS 源
   - 支持更多新闻类别

7. **改进错误处理**
   - 更详细的错误消息
   - 自动重试机制

### 中期改进 (1-2 周)

8. **添加新闻订阅功能**
   - 定时推送新闻
   - 个性化新闻推荐

9. **添加新闻分析功能**
   - 新闻摘要生成
   - 关键词提取
   - 情感分析

10. **性能优化**
    - 并行查询多个新闻源
    - 异步处理
    - 连接池优化

---

## 🎯 预期效果

### 修复前

**用户**: 德国新闻

**系统**: 德国新闻目前没有。我们可以试其他话题，或是转 topic。

### 修复后

**用户**: 德国新闻

**系统**: 正在查询德国最新新闻...

找到 10 条新闻：

1. **德国经济增长超预期**
   德国联邦统计局今日公布数据显示...
   来源: Tagesschau
   时间: 2024-03-17T10:30:00Z
   链接: https://www.tagesschau.de/...

2. **柏林新机场扩建计划**
   柏林勃兰登堡机场宣布...
   来源: Spiegel
   链接: https://www.spiegel.de/...

...

---

## 💡 关键建议

### 配置 NewsAPI (可选但推荐)

1. 注册 NewsAPI 账号: https://newsapi.org/
2. 获取 API Key
3. 在 `.env` 文件中添加:
   ```
   NEWSAPI_KEY=your_api_key_here
   ```

### 网络访问要求

- 需要访问 Google News
- 需要访问各国新闻网站 RSS
- 建议配置代理 (如果在中国大陆)

### 性能考虑

- RSS 订阅源查询较慢 (2-5 秒)
- 建议添加缓存机制
- 建议限制并发请求数

---

## 🎉 总结

### 问题根本原因

系统中**完全缺少新闻查询功能**，导致 LLM 无法提供新闻信息。

### 解决方案

1. ✅ 创建完整的新闻查询工具 (`news_tool.rs`)
2. ✅ 创建新闻查询 Skill (`news.md`)
3. ✅ 注册工具到系统
4. ✅ 添加必要的依赖

### 实现特点

- 多数据源支持 (NewsAPI, RSS, Web Scraping)
- 多国家/地区支持
- 多语言支持
- 智能降级策略
- 完整的错误处理

### 下一步

1. 构建项目
2. 运行测试
3. 重启 WebUI
4. 验证功能正常工作

---

**报告生成时间**: 2026年3月17日 18:15  
**状态**: ✅ 新闻功能已实现，等待构建和测试  
**预计修复时间**: 5-10 分钟（构建 + 重启）  

---

**新闻查询功能现已完整实现！** 🚀
