//! Extended China-specific Skills (10 additional skills)
//! 
//! This module provides additional Skills for popular Chinese services.
//! 扩展的中国大陆服务专用 Skills

use crate::create_bundled_skill;
use clawmaster_skills::types::SkillContent;

/// Get all extended China-specific skills (10 additional skills)
pub fn china_extended_skills() -> Vec<SkillContent> {
    vec![
        // Social & Content (4)
        xiaohongshu_skill(),
        zhihu_skill(),
        kuaishou_skill(),
        xigua_skill(),
        
        // E-commerce & Delivery (3)
        eleme_skill(),
        pinduoduo_skill(),
        suning_skill(),
        
        // Transportation & Travel (2)
        didi_skill(),
        ctrip_skill(),
        
        // Finance & Payment (1)
        wepay_skill(),
    ]
}

// ============================================================================
// Social & Content Skills
// ============================================================================

/// Xiaohongshu - 小红书
/// China's lifestyle and shopping platform
fn xiaohongshu_skill() -> SkillContent {
    create_bundled_skill(
        "xiaohongshu",
        "小红书 (Xiaohongshu/RED)",
        r#"---
name: xiaohongshu
description: 小红书生活方式分享平台
homepage: https://www.xiaohongshu.com
---

# Xiaohongshu Skill

小红书集成，生活方式分享和购物推荐。

## 功能

- 浏览笔记和视频
- 搜索内容和商品
- 发布笔记
- 收藏和点赞
- 购物车管理
- 达人推荐

## 使用示例

```
搜索小红书美妆笔记
发布小红书笔记
查看小红书购物车
```

## 技术实现

使用小红书开放平台 API 进行集成。

## 特色功能

- UGC 内容生态
- 种草和拔草
- 社区电商
- 生活方式分享

## 安全说明

- 内容审核机制
- 用户隐私保护
- 交易安全保障
"#,
        &["curl", "python3"],
        &["web_fetch", "exec"],
    )
}

/// Zhihu - 知乎
/// China's Q&A and knowledge sharing platform
fn zhihu_skill() -> SkillContent {
    create_bundled_skill(
        "zhihu",
        "知乎 (Zhihu)",
        r#"---
name: zhihu
description: 知乎问答和知识分享平台
homepage: https://www.zhihu.com
---

# Zhihu Skill

知乎集成，问答和知识分享。

## 功能

- 浏览问题和回答
- 搜索知识内容
- 发布问题和回答
- 关注话题和用户
- 收藏和点赞
- 专栏文章

## 使用示例

```
搜索知乎问题
查看知乎热榜
发布知乎回答
关注知乎话题
```

## 技术实现

使用知乎 API 和爬虫技术。

## 特色功能

- 高质量问答社区
- 专业知识分享
- 话题讨论
- 盐选会员内容

## 内容质量

- 专业领域深度
- 用户认证体系
- 内容推荐算法
"#,
        &["curl", "python3"],
        &["web_fetch", "exec", "read"],
    )
}

/// Kuaishou - 快手
/// Short video platform
fn kuaishou_skill() -> SkillContent {
    create_bundled_skill(
        "kuaishou",
        "快手 (Kuaishou)",
        r#"---
name: kuaishou
description: 快手短视频平台
homepage: https://www.kuaishou.com
---

# Kuaishou Skill

快手集成，短视频创作和分享。

## 功能

- 浏览短视频
- 搜索视频内容
- 上传视频
- 直播互动
- 关注创作者
- 视频下载

## 使用示例

```
搜索快手视频
查看快手直播
下载快手视频
关注快手创作者
```

## 技术实现

使用快手开放平台 API。

## 特色功能

- 老铁文化
- 直播带货
- 短视频创作
- 社交互动

## 内容类型

- 生活记录
- 才艺展示
- 知识分享
- 娱乐搞笑
"#,
        &["curl", "ffmpeg"],
        &["web_fetch", "exec"],
    )
}

/// Xigua Video - 西瓜视频
/// ByteDance's video platform
fn xigua_skill() -> SkillContent {
    create_bundled_skill(
        "xigua",
        "西瓜视频 (Xigua Video)",
        r#"---
name: xigua
description: 字节跳动西瓜视频平台
homepage: https://www.ixigua.com
---

# Xigua Video Skill

西瓜视频集成，中长视频内容平台。

## 功能

- 浏览视频内容
- 搜索视频
- 订阅频道
- 视频下载
- 评论互动
- 个性化推荐

## 使用示例

```
搜索西瓜视频
订阅西瓜频道
下载西瓜视频
查看推荐内容
```

## 技术实现

使用西瓜视频 API 和字节跳动技术栈。

## 特色功能

- 中长视频内容
- 创作者分成
- 智能推荐
- 多元化内容

## 内容分类

- 知识科普
- 娱乐综艺
- 影视解说
- 生活 vlog
"#,
        &["curl", "ffmpeg"],
        &["web_fetch", "exec"],
    )
}

// ============================================================================
// E-commerce & Delivery Skills
// ============================================================================

/// Ele.me - 饿了么
/// Food delivery platform
fn eleme_skill() -> SkillContent {
    create_bundled_skill(
        "eleme",
        "饿了么 (Ele.me)",
        r#"---
name: eleme
description: 饿了么外卖配送平台
homepage: https://www.ele.me
---

# Ele.me Skill

饿了么集成，外卖和配送服务。

## 功能

- 搜索餐厅和商品
- 下单点餐
- 订单追踪
- 优惠券管理
- 会员服务
- 配送时间预估

## 使用示例

```
点饿了么外卖
搜索附近餐厅
查看饿了么订单
使用饿了么优惠券
```

## 技术实现

使用饿了么开放平台 API。

## 特色功能

- 30 分钟送达
- 品质外卖
- 超市便利
- 医药健康

## 配送服务

- 即时配送
- 预约配送
- 无接触配送
- 配送保险

## 安全保障

- 食品安全
- 配送安全
- 支付安全
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Pinduoduo - 拼多多
/// Social e-commerce platform
fn pinduoduo_skill() -> SkillContent {
    create_bundled_skill(
        "pinduoduo",
        "拼多多 (Pinduoduo)",
        r#"---
name: pinduoduo
description: 拼多多社交电商平台
homepage: https://www.pinduoduo.com
---

# Pinduoduo Skill

拼多多集成，社交电商和拼团购物。

## 功能

- 搜索商品
- 发起拼团
- 参与拼团
- 订单管理
- 优惠券领取
- 砍价助力

## 使用示例

```
搜索拼多多商品
发起拼团
查看拼多多订单
领取优惠券
```

## 技术实现

使用拼多多开放平台 API。

## 特色功能

- 拼团购物
- 砍价免费拿
- 多多果园
- 现金签到

## 商品类型

- 农产品直供
- 品牌特卖
- 日用百货
- 数码家电

## 社交电商

- 分享赚钱
- 好友助力
- 团长招募
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Suning - 苏宁易购
/// Major e-commerce platform
fn suning_skill() -> SkillContent {
    create_bundled_skill(
        "suning",
        "苏宁易购 (Suning)",
        r#"---
name: suning
description: 苏宁易购电商平台
homepage: https://www.suning.com
---

# Suning Skill

苏宁易购集成，综合电商和零售服务。

## 功能

- 搜索商品
- 在线购物
- 订单查询
- 物流追踪
- 售后服务
- 门店查询

## 使用示例

```
搜索苏宁商品
查看苏宁订单
追踪物流
查询附近门店
```

## 技术实现

使用苏宁开放平台 API。

## 特色功能

- 线上线下融合
- 家电 3C 专业
- 急速配送
- 以旧换新

## 服务优势

- 正品保障
- 专业安装
- 延保服务
- 金融分期
"#,
        &["curl"],
        &["web_fetch"],
    )
}

// ============================================================================
// Transportation & Travel Skills
// ============================================================================

/// DiDi - 滴滴出行
/// Ride-hailing platform
fn didi_skill() -> SkillContent {
    create_bundled_skill(
        "didi",
        "滴滴出行 (DiDi)",
        r#"---
name: didi
description: 滴滴出行网约车平台
homepage: https://www.didiglobal.com
---

# DiDi Skill

滴滴出行集成，网约车和出行服务。

## 功能

- 呼叫快车/专车
- 预约用车
- 行程追踪
- 费用估算
- 发票开具
- 安全中心

## 使用示例

```
叫滴滴快车
预约滴滴专车
查看行程
估算费用
```

## 技术实现

使用滴滴开放平台 API。

## 特色功能

- 多种车型选择
- 实时定位
- 行程分享
- 安全保障

## 服务类型

- 滴滴快车
- 滴滴专车
- 滴滴顺风车
- 滴滴代驾

## 安全措施

- 行程录音
- 紧急联系人
- 一键报警
- 行程分享

## 支付方式

- 微信支付
- 支付宝
- 滴滴钱包
- 企业支付
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Ctrip - 携程
/// Travel booking platform
fn ctrip_skill() -> SkillContent {
    create_bundled_skill(
        "ctrip",
        "携程 (Ctrip/Trip.com)",
        r#"---
name: ctrip
description: 携程旅行预订平台
homepage: https://www.ctrip.com
---

# Ctrip Skill

携程集成，旅行预订和服务。

## 功能

- 机票预订
- 酒店预订
- 火车票预订
- 旅游度假
- 门票预订
- 用车服务

## 使用示例

```
预订携程机票
搜索携程酒店
预订火车票
查看旅游线路
```

## 技术实现

使用携程开放平台 API。

## 特色功能

- 一站式旅行服务
- 智能推荐
- 价格保障
- 24小时客服

## 服务类型

- 国内旅游
- 出境旅游
- 商务出行
- 自由行

## 会员权益

- 积分累积
- 专属优惠
- 优先服务
- 贵宾通道
"#,
        &["curl"],
        &["web_fetch"],
    )
}

// ============================================================================
// Finance & Payment Skills
// ============================================================================

/// WePay - 财付通
/// Tencent's payment platform
fn wepay_skill() -> SkillContent {
    create_bundled_skill(
        "wepay",
        "财付通 (Tenpay/WePay)",
        r#"---
name: wepay
description: 腾讯财付通支付平台
homepage: https://www.tenpay.com
---

# WePay (Tenpay) Skill

财付通集成，腾讯支付和金融服务。

## 功能

- 在线支付
- 转账汇款
- 账户管理
- 理财服务
- 信用卡还款
- 生活缴费

## 使用示例

```
使用财付通支付
查看财付通余额
转账到银行卡
购买理财产品
```

## 技术实现

使用财付通 API 和微信支付体系。

## 特色功能

- 快捷支付
- 扫码支付
- 声波支付
- NFC 支付

## 金融服务

- 理财通
- 基金投资
- 保险服务
- 信用服务

## 安全保障

- 实名认证
- 支付密码
- 指纹/面容识别
- 交易监控

## 安全说明

所有支付操作需要用户确认，资金安全有保障。
"#,
        &["curl"],
        &["web_fetch"],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_china_extended_skills_count() {
        let skills = china_extended_skills();
        assert_eq!(skills.len(), 10, "Should have 10 extended China skills");
    }

    #[test]
    fn test_all_extended_skills_have_metadata() {
        let skills = china_extended_skills();
        
        for skill in &skills {
            assert!(!skill.metadata.name.is_empty());
            assert!(!skill.metadata.description.is_empty());
            assert!(!skill.body.is_empty());
            assert!(skill.metadata.description.contains("(") || 
                    skill.metadata.description.contains("（"));
        }
    }

    #[test]
    fn test_extended_skill_names() {
        let skills = china_extended_skills();
        let names: Vec<&str> = skills.iter()
            .map(|s| s.metadata.name.as_str())
            .collect();
        
        assert!(names.contains(&"xiaohongshu"));
        assert!(names.contains(&"zhihu"));
        assert!(names.contains(&"eleme"));
        assert!(names.contains(&"didi"));
        assert!(names.contains(&"pinduoduo"));
    }

    #[test]
    fn test_extended_skills_categories() {
        let skills = china_extended_skills();
        
        // Social & Content: 4
        let social_content = vec!["xiaohongshu", "zhihu", "kuaishou", "xigua"];
        let social_count = skills.iter()
            .filter(|s| social_content.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(social_count, 4);
        
        // E-commerce & Delivery: 3
        let ecommerce = vec!["eleme", "pinduoduo", "suning"];
        let ecommerce_count = skills.iter()
            .filter(|s| ecommerce.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(ecommerce_count, 3);
        
        // Transportation & Travel: 2
        let transport = vec!["didi", "ctrip"];
        let transport_count = skills.iter()
            .filter(|s| transport.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(transport_count, 2);
        
        // Finance & Payment: 1
        let finance = vec!["wepay"];
        let finance_count = skills.iter()
            .filter(|s| finance.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(finance_count, 1);
    }
}
