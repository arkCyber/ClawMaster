//! 新闻工具自动化测试
//! 运行: cargo run --example test_news

use clawmaster_tools::news_tool::{NewsQuery, query_news};

#[tokio::main]
async fn main() {
    println!("\n🚀 新闻工具自动化测试");
    println!("{}", "=".repeat(80));

    let mut total = 0;
    let mut passed = 0;

    // 测试场景
    let tests = vec![
        ("成都美食", "美食", Some("cn"), None),
        ("北京科技", "科技", Some("cn"), Some("technology")),
        ("东京科技", "technology", Some("jp"), Some("technology")),
        ("纽约金融", "finance", Some("us"), Some("business")),
        ("伦敦科技", "tech", Some("gb"), Some("technology")),
        ("世界新闻", "news", Some("world"), None),
    ];

    for (name, query, country, category) in tests {
        total += 1;
        println!("\n📰 测试: {}", name);
        println!("{}", "-".repeat(80));

        let start = std::time::Instant::now();

        let news_query = NewsQuery {
            query: query.to_string(),
            country: country.map(|s| s.to_string()),
            category: category.map(|s| s.to_string()),
            language: if country == Some("cn") {
                Some("zh".to_string())
            } else {
                Some("en".to_string())
            },
            max_results: Some(3),
        };

        match query_news(news_query).await {
            Ok(result) => {
                let elapsed = start.elapsed();

                println!(
                    "✅ 成功 | 结果: {} 条 | 耗时: {:.2}秒",
                    result.total,
                    elapsed.as_secs_f64()
                );

                if result.total > 0 {
                    for (i, article) in result.articles.iter().enumerate() {
                        println!("\n  {}. {}", i + 1, article.title);
                        println!("     来源: {}", article.source);
                        if let Some(desc) = &article.description {
                            let short: String = desc.chars().take(80).collect();
                            if desc.len() > 80 {
                                println!("     {}...", short);
                            } else {
                                println!("     {}", short);
                            }
                        }
                    }
                    passed += 1;
                } else {
                    println!("⚠️  警告: 无结果");
                }
            },
            Err(e) => {
                println!("❌ 失败: {}", e);
            },
        }
    }

    // 总结
    println!("\n{}", "=".repeat(80));
    println!(
        "📊 测试总结: {}/{} 通过 ({:.0}%)",
        passed,
        total,
        (passed as f64 / total as f64) * 100.0
    );
    println!("{}", "=".repeat(80));
}
