//! 德国新闻测试脚本
//! 专门测试德国城市新闻获取

use clawmaster_tools::news_tool::{NewsQuery, query_news};

#[tokio::main]
async fn main() {
    println!("\n🇩🇪 德国新闻测试开始");
    println!("{}", "=".repeat(80));

    // 测试德国不同城市
    let test_cases = vec![
        ("柏林", "Berlin"),
        ("慕尼黑", "Munich"),
        ("法兰克福", "Frankfurt"),
        ("汉堡", "Hamburg"),
        ("科隆", "Cologne"),
    ];

    for (city_cn, city_en) in test_cases {
        println!("\n📰 测试城市: {} ({})", city_cn, city_en);
        println!("{}", "-".repeat(60));

        // 中文查询
        let start = std::time::Instant::now();
        let query_cn = NewsQuery {
            query: format!("{} 新闻", city_cn),
            country: Some("de".to_string()),
            category: None,
            language: Some("zh".to_string()),
            max_results: Some(5),
        };

        match query_news(query_cn).await {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!(
                    "✅ 中文查询成功 | 结果: {} 条 | 耗时: {:.2}秒",
                    result.total,
                    elapsed.as_secs_f64()
                );

                for (i, article) in result.articles.iter().enumerate() {
                    println!("  {}. {}", i + 1, article.title);
                    if let Some(pub_time) = &article.published_at {
                        println!("     发布时间: {}", pub_time);
                    }
                    if let Some(desc) = &article.description {
                        let short: String = desc.chars().take(100).collect();
                        println!("     {}", short);
                    }
                    println!();
                }
            },
            Err(e) => {
                println!("❌ 中文查询失败: {}", e);
            },
        }

        // 英文查询
        let start = std::time::Instant::now();
        let query_en = NewsQuery {
            query: format!("{} news", city_en),
            country: Some("de".to_string()),
            category: None,
            language: Some("en".to_string()),
            max_results: Some(5),
        };

        match query_news(query_en).await {
            Ok(result) => {
                let elapsed = start.elapsed();
                println!(
                    "✅ 英文查询成功 | 结果: {} 条 | 耗时: {:.2}秒",
                    result.total,
                    elapsed.as_secs_f64()
                );

                for (i, article) in result.articles.iter().enumerate() {
                    println!("  {}. {}", i + 1, article.title);
                    if let Some(pub_time) = &article.published_at {
                        println!("     发布时间: {}", pub_time);
                    }
                    if let Some(desc) = &article.description {
                        let short: String = desc.chars().take(100).collect();
                        println!("     {}", short);
                    }
                    println!();
                }
            },
            Err(e) => {
                println!("❌ 英文查询失败: {}", e);
            },
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("🎯 德国新闻测试完成");
    println!("{}", "=".repeat(80));
}
