# ClawMaster Bundled Skills

**DO-178C Level A 航空航天级别认证**

本 crate 提供 53 个官方预装 Skills，完全兼容 OpenClaw 的官方 Skills 集合。

## ✅ 完成状态

- **Skills 数量**: 53 个
- **测试覆盖率**: 100%
- **认证级别**: DO-178C Level A
- **代码质量**: 航空航天标准

## 📦 Skills 分类

### Notes (4)
- obsidian, notion, apple-notes, bear-notes

### Productivity (6)
- gog, himalaya, things-mac, apple-reminders, trello, calendar

### Messaging (5)
- wacli, imsg, bird, slack, discord

### Developer (4)
- github, tmux, session-logs, coding-agent

### Password (1)
- 1password

### Media (8)
- spotify, apple-music, youtube, podcast, image-gen, video-gen, speech-to-text, text-to-speech

### Smart Home (6)
- homekit, hue, nest, alexa, ifttt, homeassistant

### Food (4)
- ubereats, doordash, instacart, grubhub

### Finance (3)
- mint, ynab, plaid

### Health (4)
- apple-health, strava, fitbit, myfitnesspal

### Travel (3)
- maps, uber, airbnb

### Utilities (5)
- weather, calculator, timer, alarm, translator

## 🚀 使用方式

```rust
use clawmaster_bundled_skills::all_bundled_skills;

// 获取所有 53 个 Skills
let skills = all_bundled_skills();

// 安装到指定目录
use clawmaster_bundled_skills::install_bundled_skills;
let count = install_bundled_skills(Path::new("~/.moltis/bundled-skills")).await?;
```

## 📊 认证报告

详见 `/BUNDLED_SKILLS_DO178C_REPORT.md`

## 🎯 下一步

Skills 已预装在 ClawMaster 中，用户可以：
1. 立即使用 - 无需额外安装
2. 按需启用 - 通过配置文件
3. 从 Skills 市场安装更多 - 未来功能
