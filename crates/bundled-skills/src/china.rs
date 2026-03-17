//! China-specific Skills
//! 
//! This module provides Skills for popular Chinese services and platforms.
//! 中国大陆服务专用 Skills

use crate::create_bundled_skill;
use clawmaster_skills::types::SkillContent;

/// Get all China-specific skills (15 skills)
pub fn china_skills() -> Vec<SkillContent> {
    vec![
        // Messaging & Social (5)
        wechat_skill(),
        wecom_skill(),
        dingtalk_skill(),
        feishu_skill(),
        qq_skill(),
        
        // Payment & Finance (3)
        alipay_skill(),
        wechat_pay_skill(),
        unionpay_skill(),
        
        // Media & Entertainment (4)
        douyin_skill(),
        bilibili_skill(),
        weibo_skill(),
        netease_music_skill(),
        
        // E-commerce & Delivery (3)
        taobao_skill(),
        jd_skill(),
        meituan_skill(),
    ]
}

// ============================================================================
// Messaging & Social Skills
// ============================================================================

/// WeChat - 微信
/// China's most popular messaging and social platform
fn wechat_skill() -> SkillContent {
    create_bundled_skill(
        "wechat",
        "微信 (WeChat)",
        r#"---
name: wechat
description: 微信消息和朋友圈管理
---

# WeChat Skill

与微信进行交互，发送消息、查看朋友圈、管理联系人。

## 功能

- 发送文本消息
- 发送图片和文件
- 查看聊天记录
- 朋友圈互动
- 小程序调用

## 使用示例

```
发送微信消息给张三
查看微信聊天记录
发朋友圈
```

## 技术实现

使用 WeChat API 或 itchat 库进行集成。
"#,
        &["python3", "curl"],
        &["exec", "web_fetch", "read", "write"],
    )
}

/// WeCom - 企业微信
/// Enterprise WeChat for business communication
fn wecom_skill() -> SkillContent {
    create_bundled_skill(
        "wecom",
        "企业微信 (WeCom)",
        r#"---
name: wecom
description: 企业微信团队协作
---

# WeCom Skill

企业微信集成，用于团队协作和企业通讯。

## 功能

- 发送企业消息
- 审批流程
- 日程管理
- 文档协作
- 会议管理

## 使用示例

```
发送企业微信通知
创建审批单
查看企业日程
```

## 技术实现

使用企业微信 API 进行集成。
"#,
        &["curl"],
        &["web_fetch", "exec"],
    )
}

/// DingTalk - 钉钉
/// Alibaba's enterprise communication platform
fn dingtalk_skill() -> SkillContent {
    create_bundled_skill(
        "dingtalk",
        "钉钉 (DingTalk)",
        r#"---
name: dingtalk
description: 钉钉企业协作平台
---

# DingTalk Skill

阿里巴巴钉钉集成，企业级通讯和协作。

## 功能

- 发送钉钉消息
- DING 功能
- 审批流程
- 考勤打卡
- 视频会议

## 使用示例

```
发送钉钉消息
创建审批
查看考勤记录
```

## 技术实现

使用钉钉开放平台 API。
"#,
        &["curl"],
        &["web_fetch", "exec"],
    )
}

/// Feishu - 飞书
/// ByteDance's enterprise collaboration platform
fn feishu_skill() -> SkillContent {
    create_bundled_skill(
        "feishu",
        "飞书 (Feishu/Lark)",
        r#"---
name: feishu
description: 字节跳动飞书协作平台
---

# Feishu Skill

字节跳动飞书集成，现代化企业协作。

## 功能

- 发送飞书消息
- 文档协作
- 日程管理
- 视频会议
- 机器人集成

## 使用示例

```
发送飞书消息
创建飞书文档
安排会议
```

## 技术实现

使用飞书开放平台 API。
"#,
        &["curl"],
        &["web_fetch", "exec"],
    )
}

/// QQ - 腾讯 QQ
/// Tencent's instant messaging platform
fn qq_skill() -> SkillContent {
    create_bundled_skill(
        "qq",
        "腾讯 QQ (Tencent QQ)",
        r#"---
name: qq
description: 腾讯 QQ 即时通讯
---

# QQ Skill

腾讯 QQ 集成，即时通讯和社交。

## 功能

- 发送 QQ 消息
- 群聊管理
- 文件传输
- QQ 空间互动

## 使用示例

```
发送 QQ 消息
查看 QQ 群消息
发送文件
```

## 技术实现

使用 QQ 机器人 API 或 go-cqhttp。
"#,
        &["curl"],
        &["web_fetch", "exec"],
    )
}

// ============================================================================
// Payment & Finance Skills
// ============================================================================

/// Alipay - 支付宝
/// China's leading mobile payment platform
fn alipay_skill() -> SkillContent {
    create_bundled_skill(
        "alipay",
        "支付宝 (Alipay)",
        r#"---
name: alipay
description: 支付宝支付和生活服务
---

# Alipay Skill

支付宝集成，支付、转账和生活服务。

## 功能

- 查看余额
- 转账付款
- 账单查询
- 生活缴费
- 蚂蚁森林

## 使用示例

```
查看支付宝余额
转账给朋友
查看账单
收取蚂蚁森林能量
```

## 技术实现

使用支付宝开放平台 API。

## 安全说明

所有支付操作需要用户确认，保证资金安全。
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// WeChat Pay - 微信支付
/// WeChat's integrated payment system
fn wechat_pay_skill() -> SkillContent {
    create_bundled_skill(
        "wechat-pay",
        "微信支付 (WeChat Pay)",
        r#"---
name: wechat-pay
description: 微信支付服务
---

# WeChat Pay Skill

微信支付集成，移动支付和转账。

## 功能

- 查看零钱余额
- 转账红包
- 账单查询
- 理财通

## 使用示例

```
查看微信零钱
发红包
查看微信账单
```

## 技术实现

使用微信支付 API。

## 安全说明

所有支付操作需要用户确认。
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// UnionPay - 银联
/// China UnionPay payment system
fn unionpay_skill() -> SkillContent {
    create_bundled_skill(
        "unionpay",
        "银联 (UnionPay)",
        r#"---
name: unionpay
description: 中国银联支付服务
---

# UnionPay Skill

中国银联集成，银行卡支付和管理。

## 功能

- 银行卡管理
- 交易查询
- 云闪付
- 优惠活动

## 使用示例

```
查看银联卡余额
查询交易记录
云闪付
```

## 技术实现

使用银联开放平台 API。
"#,
        &["curl"],
        &["web_fetch"],
    )
}

// ============================================================================
// Media & Entertainment Skills
// ============================================================================

/// Douyin - 抖音
/// China's leading short video platform
fn douyin_skill() -> SkillContent {
    create_bundled_skill(
        "douyin",
        "抖音 (Douyin/TikTok China)",
        r#"---
name: douyin
description: 抖音短视频平台
---

# Douyin Skill

抖音集成，短视频浏览和创作。

## 功能

- 浏览推荐视频
- 搜索视频内容
- 下载视频
- 查看热榜
- 直播互动

## 使用示例

```
搜索抖音视频
查看抖音热榜
下载抖音视频
```

## 技术实现

使用抖音开放平台 API。
"#,
        &["curl", "ffmpeg"],
        &["web_fetch", "exec"],
    )
}

/// Bilibili - 哔哩哔哩
/// China's leading video sharing platform
fn bilibili_skill() -> SkillContent {
    create_bundled_skill(
        "bilibili",
        "哔哩哔哩 (Bilibili)",
        r#"---
name: bilibili
description: B站视频平台
---

# Bilibili Skill

哔哩哔哩集成，视频、直播和社区。

## 功能

- 搜索视频
- 下载视频
- 查看动态
- 直播提醒
- 弹幕互动

## 使用示例

```
搜索 B 站视频
下载 B 站视频
查看 UP 主动态
```

## 技术实现

使用 Bilibili API 和 you-get 工具。
"#,
        &["curl", "you-get"],
        &["web_fetch", "exec"],
    )
}

/// Weibo - 微博
/// China's Twitter-like social media platform
fn weibo_skill() -> SkillContent {
    create_bundled_skill(
        "weibo",
        "微博 (Weibo)",
        r#"---
name: weibo
description: 新浪微博社交平台
---

# Weibo Skill

新浪微博集成，社交媒体和热点资讯。

## 功能

- 发布微博
- 查看热搜
- 评论互动
- 搜索用户
- 话题追踪

## 使用示例

```
发微博
查看微博热搜
搜索微博内容
```

## 技术实现

使用微博开放平台 API。
"#,
        &["curl"],
        &["web_fetch", "exec"],
    )
}

/// NetEase Cloud Music - 网易云音乐
/// Popular music streaming platform in China
fn netease_music_skill() -> SkillContent {
    create_bundled_skill(
        "netease-music",
        "网易云音乐 (NetEase Music)",
        r#"---
name: netease-music
description: 网易云音乐流媒体
---

# NetEase Music Skill

网易云音乐集成，音乐播放和推荐。

## 功能

- 搜索歌曲
- 播放音乐
- 歌单管理
- 每日推荐
- 歌词显示

## 使用示例

```
播放网易云音乐
搜索歌曲
查看每日推荐
```

## 技术实现

使用网易云音乐 API。
"#,
        &["curl", "mpv"],
        &["web_fetch", "exec"],
    )
}

// ============================================================================
// E-commerce & Delivery Skills
// ============================================================================

/// Taobao - 淘宝
/// China's largest online shopping platform
fn taobao_skill() -> SkillContent {
    create_bundled_skill(
        "taobao",
        "淘宝 (Taobao)",
        r#"---
name: taobao
description: 淘宝购物平台
---

# Taobao Skill

淘宝集成，在线购物和订单管理。

## 功能

- 搜索商品
- 查看订单
- 物流追踪
- 购物车管理
- 优惠券查询

## 使用示例

```
搜索淘宝商品
查看淘宝订单
追踪物流
```

## 技术实现

使用淘宝开放平台 API。
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// JD - 京东
/// Major Chinese e-commerce platform
fn jd_skill() -> SkillContent {
    create_bundled_skill(
        "jd",
        "京东 (JD.com)",
        r#"---
name: jd
description: 京东购物平台
---

# JD Skill

京东集成，在线购物和物流服务。

## 功能

- 搜索商品
- 查看订单
- 物流追踪
- 价格监控
- 秒杀提醒

## 使用示例

```
搜索京东商品
查看京东订单
追踪快递
```

## 技术实现

使用京东开放平台 API。
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Meituan - 美团
/// Food delivery and local services platform
fn meituan_skill() -> SkillContent {
    create_bundled_skill(
        "meituan",
        "美团 (Meituan)",
        r#"---
name: meituan
description: 美团外卖和生活服务
---

# Meituan Skill

美团集成，外卖、酒店、电影等生活服务。

## 功能

- 点外卖
- 订单查询
- 酒店预订
- 电影票购买
- 优惠券

## 使用示例

```
点美团外卖
查看美团订单
预订酒店
```

## 技术实现

使用美团开放平台 API。
"#,
        &["curl"],
        &["web_fetch"],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_china_skills_count() {
        let skills = china_skills();
        assert_eq!(skills.len(), 15, "Should have 15 China-specific skills");
    }

    #[test]
    fn test_all_china_skills_have_metadata() {
        let skills = china_skills();
        
        for skill in &skills {
            assert!(!skill.metadata.name.is_empty());
            assert!(!skill.metadata.description.is_empty());
            assert!(!skill.body.is_empty());
            assert!(skill.metadata.description.contains("(") || 
                    skill.metadata.description.contains("（"));
        }
    }

    #[test]
    fn test_china_skill_names() {
        let skills = china_skills();
        let names: Vec<&str> = skills.iter()
            .map(|s| s.metadata.name.as_str())
            .collect();
        
        assert!(names.contains(&"wechat"));
        assert!(names.contains(&"alipay"));
        assert!(names.contains(&"douyin"));
        assert!(names.contains(&"bilibili"));
        assert!(names.contains(&"taobao"));
    }
}
