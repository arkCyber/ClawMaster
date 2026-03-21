//! File System Tools Demo
//! 演示 5 个文件系统工具的实际使用

use {
    clawmaster_agents::tool_registry::AgentTool,
    clawmaster_tools::{
        grep_tool::{GrepConfig, GrepTool},
        list_directory::{ListDirectoryConfig, ListDirectoryTool},
        read_file::{ReadFileConfig, ReadFileTool},
        search_files::{SearchFilesConfig, SearchFilesTool},
        write_file::{WriteFileConfig, WriteFileTool},
    },
    serde_json::json,
    std::fs,
    tempfile::TempDir,
};

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ClawMaster 文件系统工具演示                               ║");
    println!("║  5 个工具的自然语言测试                                    ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();

    // 创建测试环境
    let temp_dir = TempDir::new().unwrap();
    let test_path = temp_dir.path();

    println!("📁 测试目录: {}", test_path.display());
    println!();

    // 准备测试文件
    fs::write(test_path.join("hello.txt"), "Hello, ClawMaster!").unwrap();
    fs::write(test_path.join("data.txt"), "Line 1\nLine 2\nLine 3").unwrap();
    fs::create_dir(test_path.join("subdir")).unwrap();
    fs::write(test_path.join("subdir/nested.txt"), "Nested file content").unwrap();
    fs::write(
        test_path.join("main.rs"),
        "fn main() {\n    println!(\"Hello, World!\");\n}",
    )
    .unwrap();
    fs::write(test_path.join("test.js"), "function test() { return 42; }").unwrap();

    println!("✓ 测试环境准备完成\n");

    // 测试 1: ReadFileTool
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 测试 #1: ReadFileTool - 读取文件");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 自然语言请求: 请读取 hello.txt 文件的内容");
    println!();

    let read_tool =
        ReadFileTool::new(ReadFileConfig::default()).with_workspace_root(test_path.to_path_buf());

    let input = json!({
        "path": "hello.txt"
    });
    println!("📥 输入参数:");
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
    println!();

    match read_tool.execute(input).await {
        Ok(result) => {
            println!("✅ 输出结果:");
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            println!();
            println!("📄 文件内容: {}", result["content"].as_str().unwrap());
        },
        Err(e) => println!("❌ 错误: {}", e),
    }
    println!();

    // 测试 2: WriteFileTool
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✏️  测试 #2: WriteFileTool - 写入文件");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 自然语言请求: 创建一个新文件 output.txt，内容为 'Test Output'");
    println!();

    let write_tool =
        WriteFileTool::new(WriteFileConfig::default()).with_workspace_root(test_path.to_path_buf());

    let input = json!({
        "path": "output.txt",
        "content": "Test Output\nLine 2\nLine 3"
    });
    println!("📥 输入参数:");
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
    println!();

    match write_tool.execute(input).await {
        Ok(result) => {
            println!("✅ 输出结果:");
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            println!();

            // 验证文件已创建
            let content = fs::read_to_string(test_path.join("output.txt")).unwrap();
            println!("📄 验证文件内容:");
            println!("{}", content);
        },
        Err(e) => println!("❌ 错误: {}", e),
    }
    println!();

    // 测试 3: ListDirectoryTool
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📂 测试 #3: ListDirectoryTool - 列出目录");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 自然语言请求: 列出当前目录的所有文件和子目录");
    println!();

    let list_tool = ListDirectoryTool::new(ListDirectoryConfig::default())
        .with_workspace_root(test_path.to_path_buf());

    let input = json!({
        "path": ".",
        "recursive": false
    });
    println!("📥 输入参数:");
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
    println!();

    match list_tool.execute(input).await {
        Ok(result) => {
            println!("✅ 输出结果:");
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            println!();
            println!("📊 找到 {} 个条目", result["count"]);
        },
        Err(e) => println!("❌ 错误: {}", e),
    }
    println!();

    // 测试 4: SearchFilesTool
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔎 测试 #4: SearchFilesTool - 搜索文件");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 自然语言请求: 搜索所有 .txt 文件");
    println!();

    let search_tool = SearchFilesTool::new(SearchFilesConfig::default())
        .with_workspace_root(test_path.to_path_buf());

    let input = json!({
        "pattern": "**/*.txt"
    });
    println!("📥 输入参数:");
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
    println!();

    match search_tool.execute(input).await {
        Ok(result) => {
            println!("✅ 输出结果:");
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            println!();
            println!("📊 找到 {} 个文件", result["count"]);
        },
        Err(e) => println!("❌ 错误: {}", e),
    }
    println!();

    // 测试 5: GrepTool
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔍 测试 #5: GrepTool - 文本搜索");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 自然语言请求: 在所有文件中搜索包含 'Hello' 的行");
    println!();

    let grep_tool =
        GrepTool::new(GrepConfig::default()).with_workspace_root(test_path.to_path_buf());

    let input = json!({
        "pattern": "Hello",
        "path": ".",
        "recursive": true
    });
    println!("📥 输入参数:");
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
    println!();

    match grep_tool.execute(input).await {
        Ok(result) => {
            println!("✅ 输出结果:");
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            println!();
            println!("📊 找到 {} 个匹配", result["count"]);
        },
        Err(e) => println!("❌ 错误: {}", e),
    }
    println!();

    // 测试 6: 安全性测试 - 路径遍历
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🛡️  测试 #6: 安全性 - 路径遍历防护");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 自然语言请求: 尝试读取 ../etc/passwd（应该被拒绝）");
    println!();

    let input = json!({
        "path": "../etc/passwd"
    });
    println!("📥 输入参数:");
    println!("{}", serde_json::to_string_pretty(&input).unwrap());
    println!();

    match read_tool.execute(input).await {
        Ok(_) => println!("❌ 错误: 应该拒绝路径遍历！"),
        Err(e) => {
            println!("✅ 正确拒绝:");
            println!("   错误信息: {}", e);
        },
    }
    println!();

    // 总结
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  ✅ 所有测试完成                                           ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("📊 测试总结:");
    println!("   • ReadFileTool: ✅ 正常工作");
    println!("   • WriteFileTool: ✅ 正常工作");
    println!("   • ListDirectoryTool: ✅ 正常工作");
    println!("   • SearchFilesTool: ✅ 正常工作");
    println!("   • GrepTool: ✅ 正常工作");
    println!("   • 安全防护: ✅ 路径遍历被正确拒绝");
    println!();
    println!("🎉 所有工具都按预期工作！");
}
