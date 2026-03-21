//! 新闻工具自动化测试脚本
//! 在 CLI 上测试新闻工具的各种场景

use clawmaster_tools::news_tool::{query_news, NewsQuery};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    println!("\n🚀 新闻工具自动化测试开始\n");
    println!("=" .repeat(80));

    let mut total_tests = 0;
    let mut passed_tests = 0;
    let mut failed_tests = 0;

    // 测试场景列表
    let test_scenarios = vec![
        // 中国城市
        ("场景 1: 成都美食", NewsQuery {
            query: "美食".to_string(),
            country: Some("cn".to_string()),
            category: None,
            language: Some("zh".to_string()),
            max_results: Some(5),
        }),
        ("场景 2: 北京科技", NewsQuery {
            query: "科技".to_string(),
            country: Some("cn".to_string()),
            category: Some("technology".to_string()),
            language: Some("zh".to_string()),
            max_results: Some(5),
        }),
        ("场景 3: 上海金融", NewsQuery {
            query: "金融".to_string(),
            country: Some("cn".to_string()),
            category: Some("business".to_string()),
            language: Some("zh".to_string()),
            max_results: Some(5),
        }),
        // 国际城市
        ("场景 4: 东京科技", NewsQuery {
            query: "technology".to_string(),
            country: Some("jp".to_string()),
            category: Some("technology".to_string()),
            language: Some("ja".to_string()),
            max_results: Some(5),
        }),
        ("场景 5: 纽约金融", NewsQuery {
            query: "finance".to_string(),
            country: Some("us".to_string()),
            category: Some("business".to_string()),
            language: Some("en".to_string()),
            max_results: Some(5),
        }),
        ("场景 6: 伦敦科技", NewsQuery {
            query: "tech".to_string(),
            country: Some("gb".to_string()),
            category: Some("technology".to_string()),
            language: Some("en".to_string()),
            max_results: Some(5),
        }),
        // 社交媒体
        ("场景 7: Reddit 科技", NewsQuery {
            query: "technology".to_string(),
            country: Some("world".to_string()),
            category: Some("technology".to_string()),
            language: Some("en".to_string()),
            max_results: Some(5),
        }),
        ("场景 8: 世界新闻", NewsQuery {
            query: "news".to_string(),
            country: Some("world".to_string()),
            category: None,
            language: Some("en".to_string()),
            max_results: Some(5),
        }),
    ];

    // 执行测试
    for (name, query) in test_scenarios {
        total_tests += 1;
        println!("\n📰 {}", name);
        println!("-".repeat(80));
        
        let start = std::time::Instant::now();
        
        match query_news(query.clone()).await {
            Ok(result) => {
                let elapsed = start.elapsed();
                
                println!("✅ 成功");
                println!("   查询: {}", query.query);
                println!("   国家: {:?}", query.country);
                println!("   类别: {:?}", query.category);
                println!("   结果数量: {}", result.total);
                println!("   响应时间: {:.2}秒", elapsed.as_secs_f64());
                
                if result.total > 0 {
                    println!("\n   📄 前 3 条新闻:");
                    for (i, article) in result.articles.iter().take(3).enumerate() {
                        println!("   {}. {}", i + 1, article.title);
                        println!("      来源: {}", article.source);
                        if let Some(desc) = &article.description {
                            let short_desc = if desc.len() > 100 {
                                format!("{}...", &desc[..100])
                            } else {
                                desc.clone()
                            };
                            println!("      描述: {}", short_desc);
                        }
                        println!("      链接: {}", article.url);
                        println!();
                    }
                    passed_tests += 1;
                } else {
                    println!("   ⚠️  警告: 没有返回新闻");
                    failed_tests += 1;
                }
            }
            Err(e) => {
                let elapsed = start.elapsed();
                println!("❌ 失败");
                println!("   错误: {}", e);
                println!("   响应时间: {:.2}秒", elapsed.as_secs_f64());
                failed_tests += 1;
            }
        }
    }

    // 测试总结
    println!("\n");
    println!("=" .repeat(80));
    println!("🎯 测试总结");
    println!("=" .repeat(80));
    println!("总测试数: {}", total_tests);
    println!("✅ 通过: {}", passed_tests);
    println!("❌ 失败: {}", failed_tests);
    println!("通过率: {:.1}%", (passed_tests as f64 / total_tests as f64) * 100.0);
    
    if failed_tests == 0 {
        println!("\n🎉 所有测试通过！");
    } else {
        println!("\n⚠️  有 {} 个测试失败", failed_tests);
    }
    
    println!("=" .repeat(80));
}
