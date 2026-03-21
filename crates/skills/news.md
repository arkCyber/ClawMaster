---
name: news
description: 新闻查询和资讯获取
---

# News Query Skill

查询全球新闻资讯，支持多个国家和地区的新闻源。

## 功能

- 全球新闻查询
- 按国家/地区筛选
- 按类别筛选
- 多语言支持
- RSS 订阅源聚合
- 实时新闻更新

## 支持的国家/地区

- **中国** (cn, china) - 人民网、新华网、Google News 中文
- **美国** (us, usa) - New York Times, BBC, Google News
- **德国** (de, germany) - Tagesschau, Spiegel, Google News 德语
- **世界** (world) - BBC World, NYT World, Google News

## 使用示例

### 查询德国新闻
```
德国新闻
德国最新新闻
查询德国的科技新闻
```

### 查询世界新闻
```
世界新闻
全球最新新闻
国际新闻
```

### 查询中国新闻
```
中国新闻
国内新闻
查询中国的经济新闻
```

### 查询美国新闻
```
美国新闻
美国科技新闻
查询美国的政治新闻
```

## 新闻类别

- **政治** (politics)
- **经济** (business, economy)
- **科技** (technology, tech)
- **体育** (sports)
- **娱乐** (entertainment)
- **健康** (health)
- **科学** (science)

## 技术实现

### 新闻源

1. **NewsAPI** (如果配置了 API key)
   - 环境变量: `NEWSAPI_KEY`
   - 支持全球 70,000+ 新闻源
   - 实时更新

2. **RSS 订阅源**
   - Google News RSS
   - 人民网 RSS
   - 新华网 RSS
   - Tagesschau RSS
   - Spiegel RSS
   - BBC RSS
   - New York Times RSS

3. **网页抓取** (备用方案)
   - Google News 搜索
   - 智能 HTML 解析
   - 自动提取标题、链接、来源

### 查询参数

- `query`: 查询关键词
- `country`: 国家/地区代码 (cn, us, de, world)
- `category`: 新闻类别
- `language`: 语言代码 (zh, en, de)
- `max_results`: 最大结果数 (默认 10)

## 工具调用

使用 `news_tool` 工具查询新闻：

```json
{
  "tool": "news_query",
  "params": {
    "query": "科技",
    "country": "cn",
    "language": "zh",
    "max_results": 5
  }
}
```

## 配置

### 可选：配置 NewsAPI Key

在 `.env` 文件中添加：
```
NEWSAPI_KEY=your_api_key_here
```

获取 API Key: https://newsapi.org/

### 默认配置

如果没有配置 NewsAPI，系统会自动使用 RSS 订阅源和网页抓取作为备用方案。

## 返回格式

```json
{
  "total": 5,
  "articles": [
    {
      "title": "新闻标题",
      "description": "新闻摘要",
      "url": "https://example.com/news",
      "source": "新闻来源",
      "published_at": "2024-01-01T12:00:00Z",
      "author": "作者",
      "image_url": "https://example.com/image.jpg"
    }
  ]
}
```

## 示例对话

**用户**: 德国新闻

**助手**: 正在查询德国最新新闻...

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

## 注意事项

1. **速率限制**: NewsAPI 免费版有请求限制
2. **网络访问**: 需要稳定的网络连接
3. **内容过滤**: 自动过滤不相关内容
4. **时效性**: RSS 订阅源更新频率不同
5. **语言**: 自动根据国家/地区选择语言

## 故障排除

### 问题：无法获取新闻

**解决方案**:
1. 检查网络连接
2. 验证 NewsAPI Key (如果使用)
3. 尝试不同的国家/地区
4. 检查防火墙设置

### 问题：返回结果为空

**解决方案**:
1. 尝试更通用的查询词
2. 移除类别限制
3. 增加 max_results 参数
4. 检查 RSS 源是否可访问

## 相关 Skills

- `web_search` - 网页搜索
- `web_fetch` - 网页获取
- `rss_reader` - RSS 阅读器

## 更新日志

- 2024-03-17: 初始版本
  - 支持多国新闻查询
  - 集成 NewsAPI, RSS, 网页抓取
  - 支持中文、英文、德文
