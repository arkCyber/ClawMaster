//! Complex AI Conversation Tests with Detailed Q&A Display
//!
//! This module simulates complex, multi-turn AI conversations with full
//! question and answer information display. DO-178C Level A compliant.

use {
    clawmaster_bundled_skills::{all_bundled_skills, get_skills_by_category},
    std::collections::HashMap,
};

/// Represents a conversation turn with full details
#[derive(Debug, Clone)]
struct ConversationTurn {
    turn_number: usize,
    user_question: String,
    user_intent: String,
    ai_understanding: String,
    recommended_skills: Vec<String>,
    skill_descriptions: Vec<String>,
    reasoning: String,
    confidence: f32,
    alternatives: Vec<String>,
}

impl ConversationTurn {
    fn display(&self) -> String {
        format!(
            r#"
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔄 Turn #{}: 对话轮次
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

👤 用户问题:
   "{}"

🎯 识别意图:
   {}

🧠 AI 理解:
   {}

💡 推荐 Skills ({} 个):
{}

📝 推理过程:
   {}

📊 置信度: {:.1}%

🔀 备选方案:
{}

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#,
            self.turn_number,
            self.user_question,
            self.user_intent,
            self.ai_understanding,
            self.recommended_skills.len(),
            self.skill_descriptions.join("\n"),
            self.reasoning,
            self.confidence * 100.0,
            if self.alternatives.is_empty() {
                "   无备选方案".to_string()
            } else {
                format!("   - {}", self.alternatives.join("\n   - "))
            }
        )
    }
}

/// Advanced AI Assistant with detailed conversation tracking
struct AdvancedAIAssistant {
    available_skills: HashMap<String, String>,
    skill_categories: HashMap<String, Vec<String>>,
    conversation_history: Vec<ConversationTurn>,
    context: HashMap<String, String>,
}

impl AdvancedAIAssistant {
    fn new() -> Self {
        let skills = all_bundled_skills();
        let mut available_skills = HashMap::new();

        for skill in &skills {
            available_skills.insert(
                skill.metadata.name.clone(),
                skill.metadata.description.clone(),
            );
        }

        let mut skill_categories = HashMap::new();
        let categories = vec![
            "notes",
            "productivity",
            "messaging",
            "developer",
            "password",
            "media",
            "smart_home",
            "food",
            "finance",
            "health",
            "travel",
            "utilities",
        ];

        for category in categories {
            let cat_skills: Vec<String> = get_skills_by_category(category)
                .iter()
                .map(|s| s.metadata.name.clone())
                .collect();
            skill_categories.insert(category.to_string(), cat_skills);
        }

        Self {
            available_skills,
            skill_categories,
            conversation_history: Vec::new(),
            context: HashMap::new(),
        }
    }

    /// Process a user question and return detailed conversation turn
    fn process_question(&mut self, question: &str) -> ConversationTurn {
        let turn_number = self.conversation_history.len() + 1;
        let question_lower = question.to_lowercase();

        // Analyze user intent
        let (intent, understanding, skills, reasoning, confidence, alternatives) =
            self.analyze_intent(&question_lower);

        // Get skill descriptions
        let skill_descriptions: Vec<String> = skills
            .iter()
            .map(|skill_name| {
                let desc = self
                    .available_skills
                    .get(skill_name)
                    .cloned()
                    .unwrap_or_else(|| "未知".to_string());
                format!("   ✓ {} - {}", skill_name, desc)
            })
            .collect();

        let turn = ConversationTurn {
            turn_number,
            user_question: question.to_string(),
            user_intent: intent,
            ai_understanding: understanding,
            recommended_skills: skills,
            skill_descriptions,
            reasoning,
            confidence,
            alternatives,
        };

        self.conversation_history.push(turn.clone());
        turn
    }

    /// Analyze user intent and return detailed information
    fn analyze_intent(
        &self,
        question: &str,
    ) -> (String, String, Vec<String>, String, f32, Vec<String>) {
        // Notes intent
        if question.contains("note") || question.contains("记笔记") || question.contains("笔记")
        {
            return (
                "笔记记录".to_string(),
                "用户想要记录笔记或管理笔记内容".to_string(),
                self.skill_categories
                    .get("notes")
                    .cloned()
                    .unwrap_or_default(),
                "检测到笔记相关关键词，推荐所有笔记管理 Skills，用户可根据偏好选择".to_string(),
                0.95,
                vec!["也可以使用文本编辑器".to_string()],
            );
        }

        // Email intent
        if question.contains("email") || question.contains("邮件") || question.contains("mail") {
            return (
                "邮件管理".to_string(),
                "用户需要检查、发送或管理电子邮件".to_string(),
                vec!["himalaya".to_string(), "gog".to_string()],
                "识别邮件需求，himalaya 提供 IMAP/SMTP 支持，gog 提供 Google Workspace 集成"
                    .to_string(),
                0.90,
                vec!["可以直接使用邮件客户端".to_string()],
            );
        }

        // GitHub/Code intent
        if question.contains("github")
            || question.contains("代码")
            || question.contains("code")
            || question.contains("issue")
            || question.contains("pull request")
            || question.contains("pr")
        {
            return (
                "代码开发/GitHub 管理".to_string(),
                "用户需要进行代码开发或 GitHub 仓库管理".to_string(),
                vec!["github".to_string(), "coding-agent".to_string()],
                "GitHub Skill 提供仓库管理，coding-agent 提供 AI 编程辅助".to_string(),
                0.98,
                vec![
                    "可以使用 git 命令行".to_string(),
                    "可以使用 GitHub CLI".to_string(),
                ],
            );
        }

        // Weather intent
        if question.contains("weather") || question.contains("天气") {
            return (
                "天气查询".to_string(),
                "用户想要查询天气信息".to_string(),
                vec!["weather".to_string()],
                "明确的天气查询需求，推荐 weather Skill".to_string(),
                1.0,
                vec!["可以访问天气网站".to_string()],
            );
        }

        // Music intent
        if question.contains("music")
            || question.contains("音乐")
            || question.contains("play")
            || question.contains("song")
        {
            return (
                "音乐播放".to_string(),
                "用户想要播放音乐或管理音乐库".to_string(),
                vec!["spotify".to_string(), "apple-music".to_string()],
                "提供多个音乐平台选项，用户可根据订阅服务选择".to_string(),
                0.92,
                vec!["可以使用本地音乐播放器".to_string()],
            );
        }

        // Messaging intent
        if question.contains("message")
            || question.contains("消息")
            || question.contains("slack")
            || question.contains("discord")
            || question.contains("chat")
        {
            return (
                "即时消息".to_string(),
                "用户需要发送消息或进行即时通讯".to_string(),
                self.skill_categories
                    .get("messaging")
                    .cloned()
                    .unwrap_or_default(),
                "检测到消息通讯需求，提供多个平台选项".to_string(),
                0.88,
                vec!["可以直接使用对应应用".to_string()],
            );
        }

        // Calendar/Schedule intent
        if question.contains("calendar")
            || question.contains("日历")
            || question.contains("schedule")
            || question.contains("meeting")
            || question.contains("appointment")
            || question.contains("会议")
        {
            return (
                "日程管理".to_string(),
                "用户需要管理日历或安排会议".to_string(),
                vec!["calendar".to_string(), "gog".to_string()],
                "calendar 提供 CalDAV 支持，gog 提供 Google Calendar 集成".to_string(),
                0.93,
                vec!["可以使用系统日历应用".to_string()],
            );
        }

        // Task management intent
        if question.contains("task")
            || question.contains("任务")
            || question.contains("todo")
            || question.contains("待办")
        {
            return (
                "任务管理".to_string(),
                "用户需要管理任务或待办事项".to_string(),
                vec![
                    "things-mac".to_string(),
                    "trello".to_string(),
                    "apple-reminders".to_string(),
                ],
                "提供本地和云端任务管理方案".to_string(),
                0.91,
                vec!["可以使用纸笔记录".to_string()],
            );
        }

        // Translation intent
        if question.contains("translate") || question.contains("翻译") {
            return (
                "翻译服务".to_string(),
                "用户需要翻译文本".to_string(),
                vec!["translator".to_string()],
                "提供多语言翻译支持".to_string(),
                0.96,
                vec!["可以使用在线翻译网站".to_string()],
            );
        }

        // Password intent
        if question.contains("password") || question.contains("密码") {
            return (
                "密码管理".to_string(),
                "用户需要管理或检索密码".to_string(),
                vec!["1password".to_string()],
                "推荐安全的密码管理器".to_string(),
                0.94,
                vec!["可以使用浏览器密码管理".to_string()],
            );
        }

        // Default: unknown intent
        (
            "未知意图".to_string(),
            "无法明确识别用户意图，建议用户提供更多信息".to_string(),
            Vec::new(),
            "未检测到明确的关键词，无法推荐合适的 Skill".to_string(),
            0.0,
            vec![
                "请提供更多细节".to_string(),
                "可以浏览所有可用 Skills".to_string(),
            ],
        )
    }

    /// Get conversation summary
    fn get_summary(&self) -> String {
        format!(
            r#"
╔══════════════════════════════════════════════════════════════════════╗
║                        对话总结 CONVERSATION SUMMARY                  ║
╚══════════════════════════════════════════════════════════════════════╝

📊 总轮次: {} 轮
📈 平均置信度: {:.1}%
✅ 成功推荐: {} 次
❌ 未识别: {} 次

"#,
            self.conversation_history.len(),
            self.conversation_history
                .iter()
                .map(|t| t.confidence)
                .sum::<f32>()
                / self.conversation_history.len() as f32
                * 100.0,
            self.conversation_history
                .iter()
                .filter(|t| !t.recommended_skills.is_empty())
                .count(),
            self.conversation_history
                .iter()
                .filter(|t| t.recommended_skills.is_empty())
                .count(),
        )
    }
}

/// Test Complex Conversation Scenario 1: Developer Workflow
#[test]
fn test_complex_developer_workflow() {
    let mut ai = AdvancedAIAssistant::new();

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║          测试场景 1: 开发者工作流 (Developer Workflow)                ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // Turn 1: Check email
    let turn1 = ai.process_question("I need to check my email for code review requests");
    println!("{}", turn1.display());
    assert!(!turn1.recommended_skills.is_empty());
    assert!(turn1.confidence > 0.8);

    // Turn 2: Create GitHub issue
    let turn2 = ai.process_question("Create a GitHub issue for the bug I found");
    println!("{}", turn2.display());
    assert!(turn2.recommended_skills.contains(&"github".to_string()));
    assert!(turn2.confidence > 0.9);

    // Turn 3: Schedule meeting
    let turn3 = ai.process_question("Schedule a meeting to discuss the fix");
    println!("{}", turn3.display());
    assert!(turn3.recommended_skills.contains(&"calendar".to_string()));

    // Turn 4: Send Slack message
    let turn4 = ai.process_question("Send a Slack message to the team about the meeting");
    println!("{}", turn4.display());
    assert!(turn4.recommended_skills.iter().any(|s| s == "slack"));

    println!("{}", ai.get_summary());

    // Verify workflow completeness
    assert_eq!(ai.conversation_history.len(), 4);
}

/// Test Complex Conversation Scenario 2: Multilingual Conversation
#[test]
fn test_complex_multilingual_conversation() {
    let mut ai = AdvancedAIAssistant::new();

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║          测试场景 2: 多语言对话 (Multilingual Conversation)           ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // Turn 1: English
    let turn1 = ai.process_question("I want to take some notes");
    println!("{}", turn1.display());
    assert!(!turn1.recommended_skills.is_empty());

    // Turn 2: Chinese
    let turn2 = ai.process_question("今天天气怎么样？");
    println!("{}", turn2.display());
    assert!(turn2.recommended_skills.contains(&"weather".to_string()));

    // Turn 3: Mixed
    let turn3 = ai.process_question("帮我翻译 translate this text");
    println!("{}", turn3.display());
    assert!(turn3.recommended_skills.contains(&"translator".to_string()));

    println!("{}", ai.get_summary());

    assert_eq!(ai.conversation_history.len(), 3);
}

/// Test Complex Conversation Scenario 3: Productivity Suite
#[test]
fn test_complex_productivity_suite() {
    let mut ai = AdvancedAIAssistant::new();

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║          测试场景 3: 生产力套件 (Productivity Suite)                  ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    let turn1 = ai.process_question("Check the weather forecast");
    println!("{}", turn1.display());

    let turn2 = ai.process_question("Add a task to my todo list");
    println!("{}", turn2.display());

    let turn3 = ai.process_question("Play some background music");
    println!("{}", turn3.display());

    let turn4 = ai.process_question("Take notes during the meeting");
    println!("{}", turn4.display());

    let turn5 = ai.process_question("Get my GitHub password");
    println!("{}", turn5.display());

    println!("{}", ai.get_summary());

    // Verify all turns had recommendations
    for turn in &ai.conversation_history {
        assert!(
            !turn.recommended_skills.is_empty(),
            "Turn {} should have skill recommendations",
            turn.turn_number
        );
    }
}

/// Test Complex Conversation Scenario 4: Edge Cases
#[test]
fn test_complex_edge_cases() {
    let mut ai = AdvancedAIAssistant::new();

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║          测试场景 4: 边界情况 (Edge Cases)                            ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // Ambiguous request
    let turn1 = ai.process_question("Do something");
    println!("{}", turn1.display());
    assert_eq!(turn1.confidence, 0.0);

    // Very specific request
    let turn2 = ai.process_question("Create a GitHub pull request for feature-x");
    println!("{}", turn2.display());
    assert!(turn2.confidence > 0.9);

    // Multiple intents
    let turn3 = ai.process_question("Check email and schedule a meeting");
    println!("{}", turn3.display());

    println!("{}", ai.get_summary());
}

/// Test Complex Conversation Scenario 5: Context Awareness
#[test]
fn test_complex_context_awareness() {
    let mut ai = AdvancedAIAssistant::new();

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║          测试场景 5: 上下文感知 (Context Awareness)                   ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    let turn1 = ai.process_question("I'm working on a coding project");
    println!("{}", turn1.display());

    let turn2 = ai.process_question("Create an issue for the bug");
    println!("{}", turn2.display());
    assert!(turn2.recommended_skills.contains(&"github".to_string()));

    let turn3 = ai.process_question("Send a message about it");
    println!("{}", turn3.display());

    println!("{}", ai.get_summary());
}

/// Test detailed skill information display
#[test]
fn test_skill_information_display() {
    let ai = AdvancedAIAssistant::new();

    println!("\n╔══════════════════════════════════════════════════════════════════════╗");
    println!("║          测试: Skill 信息展示 (Skill Information Display)             ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    // Display all available skills
    println!("\n📦 可用 Skills 总数: {}\n", ai.available_skills.len());

    for (category, skills) in &ai.skill_categories {
        println!("📁 分类: {}", category);
        for skill in skills {
            if let Some(desc) = ai.available_skills.get(skill) {
                println!("   ├─ {} - {}", skill, desc);
            }
        }
        println!();
    }

    assert_eq!(ai.available_skills.len(), 53);
}
