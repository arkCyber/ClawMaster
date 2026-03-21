//! China Health & Social Skills
//!
//! This module provides Skills for Chinese healthcare and social platform services.
//! 中国医疗健康和社交平台服务专用 Skills

use {crate::create_bundled_skill, clawmaster_skills::types::SkillContent};

/// Get all China health and social skills (8 skills)
pub fn china_health_social_skills() -> Vec<SkillContent> {
    vec![
        // Healthcare (5)
        china_hospital_skill(),
        wechat_doctor_skill(),
        alipay_health_skill(),
        jd_health_skill(),
        meituan_doctor_skill(),
        // Social Platforms (3)
        douban_skill(),
        tieba_skill(),
        momo_skill(),
    ]
}

// ============================================================================
// Healthcare Skills
// ============================================================================

/// China Hospital - 中国医院挂号
/// Hospital appointment and healthcare services
fn china_hospital_skill() -> SkillContent {
    create_bundled_skill(
        "china-hospital",
        "中国医院挂号 (China Hospital)",
        r#"---
name: china-hospital
description: 中国医院在线挂号和医疗服务
homepage: https://www.guahao.com
---

# China Hospital Skill

中国医院在线挂号系统集成，提供预约挂号、在线问诊等医疗服务。

## 核心功能

### 在线挂号

- **医院查询**
  - 按地区查询医院
  - 按科室查询医院
  - 按医生查询医院
  - 医院等级筛选 (三甲、二甲等)

- **预约挂号**
  - 在线预约挂号
  - 选择就诊时间
  - 选择医生
  - 预约确认

- **挂号管理**
  - 查看挂号记录
  - 取消挂号
  - 改约时间
  - 就诊提醒

### 在线问诊

- **图文问诊**
  - 文字描述病情
  - 上传检查报告
  - 医生在线回复
  - 开具处方

- **视频问诊**
  - 视频面诊
  - 实时沟通
  - 远程诊断
  - 电子处方

### 医疗服务

- **体检预约**
  - 体检套餐选择
  - 体检预约
  - 体检报告查询
  - 健康评估

- **疫苗接种**
  - 疫苗预约
  - 接种记录
  - 接种提醒
  - 疫苗查询

- **住院服务**
  - 住院预约
  - 床位查询
  - 住院费用查询
  - 出院结算

## 主要平台

### 挂号平台

- **微医 (挂号网)**
  - 全国 7400+ 医院
  - 27万+ 医生
  - 在线预约挂号
  - 在线问诊

- **好大夫在线**
  - 全国医院覆盖
  - 专家在线咨询
  - 图文/电话/视频问诊
  - 疾病知识库

- **春雨医生**
  - 在线问诊
  - 快速响应
  - 专业医生
  - 健康管理

- **平安好医生**
  - 24小时在线问诊
  - 1小时送药上门
  - 健康管理
  - 保险服务

## 使用示例

```
预约北京协和医院挂号
查询上海三甲医院
预约心内科专家
在线问诊
查看挂号记录
取消挂号
预约体检
查询疫苗接种
```

## 技术实现

使用各大挂号平台开放 API 进行集成。

## 医保支持

- **医保卡绑定**
  - 电子医保卡
  - 医保余额查询
  - 医保支付
  - 医保报销

- **费用结算**
  - 在线支付
  - 医保结算
  - 自费部分支付
  - 电子发票

## 健康档案

- 就诊记录
- 检查报告
- 用药记录
- 过敏史
- 家族病史

## 安全保障

- 实名认证
- 隐私保护
- 数据加密
- 合规认证
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// WeChat Doctor - 微信医疗
/// WeChat healthcare services
fn wechat_doctor_skill() -> SkillContent {
    create_bundled_skill(
        "wechat-doctor",
        "微信医疗 (WeChat Doctor)",
        r#"---
name: wechat-doctor
description: 微信医疗健康服务
homepage: https://weixin.qq.com
---

# WeChat Doctor Skill

微信医疗健康服务集成，提供在线问诊、挂号、购药等服务。

## 核心功能

### 微信医保

- **电子医保卡**
  - 医保卡激活
  - 医保余额查询
  - 医保支付
  - 医保报销记录

- **医保服务**
  - 定点医院查询
  - 医保政策查询
  - 异地就医备案
  - 医保转移

### 在线问诊

- **微信问诊**
  - 图文问诊
  - 语音问诊
  - 视频问诊
  - 快速响应

- **AI 智能问诊**
  - 症状描述
  - AI 初步诊断
  - 就医建议
  - 科室推荐

### 预约挂号

- **医院挂号**
  - 全国医院覆盖
  - 在线预约
  - 挂号缴费
  - 就诊提醒

- **专家号源**
  - 专家推荐
  - 号源实时更新
  - 预约确认
  - 取号指引

### 在线购药

- **处方药购买**
  - 上传处方
  - 药师审核
  - 在线购药
  - 送药上门

- **OTC 药品**
  - 常用药品
  - 在线下单
  - 快速配送
  - 用药指导

## 健康管理

- **健康档案**
  - 个人健康档案
  - 就诊记录
  - 检查报告
  - 用药记录

- **健康监测**
  - 血压记录
  - 血糖记录
  - 体重管理
  - 运动记录

- **健康提醒**
  - 用药提醒
  - 复诊提醒
  - 体检提醒
  - 健康建议

## 使用示例

```
激活微信医保卡
查询医保余额
微信在线问诊
预约医院挂号
购买处方药
查看健康档案
记录血压
设置用药提醒
```

## 技术实现

使用微信开放平台医疗健康 API。

## 特色服务

- 24小时在线服务
- 三甲医院医生
- 1小时送药上门
- 医保在线支付

## 安全保障

- 实名认证
- 处方审核
- 隐私保护
- 数据加密
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Alipay Health - 支付宝医疗
/// Alipay healthcare services
fn alipay_health_skill() -> SkillContent {
    create_bundled_skill(
        "alipay-health",
        "支付宝医疗 (Alipay Health)",
        r#"---
name: alipay-health
description: 支付宝医疗健康服务
homepage: https://www.alipay.com
---

# Alipay Health Skill

支付宝医疗健康服务集成，提供挂号、问诊、购药、体检等全方位服务。

## 核心功能

### 医疗服务

- **在线挂号**
  - 全国医院覆盖
  - 实时号源
  - 在线支付
  - 就诊提醒

- **在线问诊**
  - 图文问诊
  - 视频问诊
  - 专家咨询
  - 快速响应

- **体检服务**
  - 体检预约
  - 体检套餐
  - 体检报告
  - 健康评估

### 医药服务

- **在线购药**
  - 处方药购买
  - OTC 药品
  - 送药上门
  - 用药指导

- **慢病管理**
  - 慢病用药
  - 定期配送
  - 用药提醒
  - 健康监测

### 医保服务

- **电子医保卡**
  - 医保卡激活
  - 医保支付
  - 医保余额
  - 报销记录

- **医保查询**
  - 缴费记录
  - 报销明细
  - 定点医院
  - 医保政策

## 健康管理

- **健康档案**
  - 个人档案
  - 家庭档案
  - 就诊记录
  - 检查报告

- **健康监测**
  - 步数统计
  - 体重管理
  - 血压血糖
  - 睡眠监测

- **健康保险**
  - 医疗保险
  - 重疾保险
  - 意外保险
  - 在线理赔

## 使用示例

```
支付宝预约挂号
支付宝在线问诊
购买处方药
预约体检
激活医保卡
查询医保余额
查看健康档案
购买医疗保险
```

## 技术实现

使用支付宝开放平台医疗健康 API。

## 特色服务

- **蚂蚁保险**
  - 医疗保险
  - 在线投保
  - 快速理赔

- **送药上门**
  - 1小时送达
  - 专业配送
  - 药师指导

- **健康金**
  - 步行赚金币
  - 兑换权益
  - 健康激励

## 安全保障

- 实名认证
- 处方审核
- 隐私保护
- 资金安全
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// JD Health - 京东健康
/// JD.com's healthcare platform
fn jd_health_skill() -> SkillContent {
    create_bundled_skill(
        "jd-health",
        "京东健康 (JD Health)",
        r#"---
name: jd-health
description: 京东健康医疗服务平台
homepage: https://www.jd.com/health
---

# JD Health Skill

京东健康集成，京东旗下的一站式医疗健康服务平台。

## 核心功能

### 在线问诊

- **24小时问诊**
  - 全科医生
  - 专科医生
  - 图文问诊
  - 视频问诊

- **专家咨询**
  - 三甲医院专家
  - 预约专家
  - 深度咨询
  - 治疗方案

### 医药服务

- **在线购药**
  - 处方药
  - OTC 药品
  - 医疗器械
  - 保健品

- **送药服务**
  - 28分钟送达 (部分城市)
  - 1小时达
  - 次日达
  - 冷链配送

- **慢病管理**
  - 慢病用药
  - 定期配送
  - 用药提醒
  - 健康监测

### 互联网医院

- **在线诊疗**
  - 复诊续方
  - 开具处方
  - 电子病历
  - 检查预约

- **住院服务**
  - 住院预约
  - 住院押金
  - 费用查询
  - 出院结算

## 健康管理

- **健康档案**
  - 个人档案
  - 家庭成员
  - 就诊记录
  - 用药记录

- **健康监测**
  - 血压血糖
  - 体重管理
  - 运动记录
  - 睡眠分析

- **健康商城**
  - 医疗器械
  - 保健品
  - 健康食品
  - 家庭护理

## 使用示例

```
京东健康在线问诊
购买处方药
预约专家咨询
28分钟送药
查看健康档案
慢病用药管理
购买医疗器械
```

## 技术实现

使用京东健康开放平台 API。

## 特色服务

- **28分钟送药** (部分城市)
- **24小时问诊**
- **三甲医院专家**
- **互联网医院**

## 服务优势

- 药品正品保障
- 专业药师审核
- 快速配送
- 医保支付

## 安全保障

- 药品资质认证
- 处方审核
- 隐私保护
- 冷链运输
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Meituan Doctor - 美团医疗
/// Meituan's healthcare services
fn meituan_doctor_skill() -> SkillContent {
    create_bundled_skill(
        "meituan-doctor",
        "美团医疗 (Meituan Doctor)",
        r#"---
name: meituan-doctor
description: 美团医疗健康服务
homepage: https://www.meituan.com
---

# Meituan Doctor Skill

美团医疗健康服务集成，提供在线问诊、购药、体检等服务。

## 核心功能

### 在线问诊

- **快速问诊**
  - 5分钟响应
  - 图文问诊
  - 语音问诊
  - 视频问诊

- **专家问诊**
  - 三甲医院专家
  - 预约咨询
  - 深度诊疗
  - 治疗方案

### 购药服务

- **美团买药**
  - 处方药
  - OTC 药品
  - 医疗器械
  - 保健品

- **送药上门**
  - 30分钟送达 (部分城市)
  - 1小时达
  - 24小时配送
  - 夜间送药

### 体检服务

- **体检预约**
  - 体检机构
  - 体检套餐
  - 在线预约
  - 优惠价格

- **体检报告**
  - 在线查看
  - 专家解读
  - 健康建议
  - 复检提醒

## 健康管理

- **健康档案**
  - 个人信息
  - 就诊记录
  - 用药记录
  - 过敏史

- **用药管理**
  - 用药提醒
  - 药品说明
  - 相互作用
  - 不良反应

- **健康商城**
  - 医疗器械
  - 保健品
  - 健康食品
  - 家庭护理

## 使用示例

```
美团在线问诊
美团买药
30分钟送药上门
预约体检
查看体检报告
设置用药提醒
购买医疗器械
```

## 技术实现

使用美团开放平台医疗健康 API。

## 特色服务

- **30分钟送药** (部分城市)
- **24小时配送**
- **夜间送药**
- **优惠价格**

## 服务优势

- 快速响应
- 配送便捷
- 价格优惠
- 品类丰富

## 安全保障

- 药品正品
- 处方审核
- 隐私保护
- 配送安全
"#,
        &["curl"],
        &["web_fetch"],
    )
}

// ============================================================================
// Social Platform Skills
// ============================================================================

/// Douban - 豆瓣
/// Chinese social networking and review platform
fn douban_skill() -> SkillContent {
    create_bundled_skill(
        "douban",
        "豆瓣 (Douban)",
        r#"---
name: douban
description: 豆瓣社交网络和评论平台
homepage: https://www.douban.com
---

# Douban Skill

豆瓣集成，中国知名的社交网络和评论平台。

## 核心功能

### 豆瓣读书

- **图书信息**
  - 图书搜索
  - 图书详情
  - 作者信息
  - 出版信息

- **图书评论**
  - 书评阅读
  - 发表书评
  - 评分打分
  - 书评点赞

- **阅读管理**
  - 想读
  - 在读
  - 读过
  - 阅读进度

### 豆瓣电影

- **电影信息**
  - 电影搜索
  - 电影详情
  - 演员信息
  - 上映信息

- **影评**
  - 影评阅读
  - 发表影评
  - 评分打分
  - 影评点赞

- **观影管理**
  - 想看
  - 在看
  - 看过
  - 观影记录

### 豆瓣音乐

- **音乐信息**
  - 音乐搜索
  - 专辑详情
  - 艺人信息
  - 发行信息

- **乐评**
  - 乐评阅读
  - 发表乐评
  - 评分打分
  - 乐评点赞

### 豆瓣小组

- **小组讨论**
  - 加入小组
  - 发帖讨论
  - 回复帖子
  - 关注话题

- **兴趣小组**
  - 读书小组
  - 电影小组
  - 音乐小组
  - 生活小组

### 豆瓣广播

- **动态分享**
  - 发布动态
  - 分享内容
  - 评论互动
  - 关注好友

## 使用示例

```
搜索豆瓣图书
查看豆瓣电影评分
发表影评
加入豆瓣小组
发布豆瓣广播
标记想读的书
查看观影记录
```

## 技术实现

使用豆瓣 API 和爬虫技术。

## 社区特色

- **文艺社区**
  - 高质量内容
  - 深度评论
  - 文艺青年聚集地

- **评分系统**
  - 公正客观
  - 参考价值高
  - 影响力大

## 内容分类

- 图书
- 电影
- 音乐
- 同城活动
- 小组讨论

## 用户特点

- 文艺青年
- 知识分子
- 深度用户
- 高质量内容
"#,
        &["curl", "python3"],
        &["web_fetch", "exec"],
    )
}

/// Tieba - 百度贴吧
/// Baidu's online community platform
fn tieba_skill() -> SkillContent {
    create_bundled_skill(
        "tieba",
        "百度贴吧 (Tieba)",
        r#"---
name: tieba
description: 百度贴吧社区平台
homepage: https://tieba.baidu.com
---

# Tieba Skill

百度贴吧集成，中国最大的中文社区平台。

## 核心功能

### 贴吧浏览

- **贴吧搜索**
  - 搜索贴吧
  - 热门贴吧
  - 推荐贴吧
  - 分类浏览

- **帖子浏览**
  - 浏览帖子
  - 热门帖子
  - 精品帖子
  - 最新帖子

### 发帖互动

- **发帖**
  - 发布主题帖
  - 图文混排
  - 视频发布
  - 投票发起

- **回帖**
  - 回复帖子
  - 楼中楼
  - 表情包
  - @用户

### 贴吧管理

- **关注贴吧**
  - 关注感兴趣的贴吧
  - 贴吧推荐
  - 取消关注

- **签到**
  - 每日签到
  - 签到奖励
  - 连续签到
  - 签到排行

### 吧务管理

- **吧主权限**
  - 帖子管理
  - 用户管理
  - 贴吧设置
  - 活动组织

## 使用示例

```
搜索百度贴吧
浏览热门帖子
发布帖子
回复帖子
关注贴吧
每日签到
查看精品帖
```

## 技术实现

使用百度贴吧 API。

## 社区特色

- **兴趣聚集**
  - 各类兴趣贴吧
  - 明星粉丝团
  - 游戏讨论
  - 地域贴吧

- **活跃度高**
  - 用户活跃
  - 内容丰富
  - 互动频繁

## 贴吧分类

- 游戏贴吧
- 明星贴吧
- 地域贴吧
- 兴趣贴吧
- 品牌贴吧

## 用户等级

- 等级体系
- 签到积分
- 会员特权
- 勋章系统
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Momo - 陌陌
/// Chinese social networking app
fn momo_skill() -> SkillContent {
    create_bundled_skill(
        "momo",
        "陌陌 (Momo)",
        r#"---
name: momo
description: 陌陌社交平台
homepage: https://www.immomo.com
---

# Momo Skill

陌陌集成，基于地理位置的移动社交平台。

## 核心功能

### 附近的人

- **发现**
  - 附近的人
  - 距离显示
  - 在线状态
  - 个人资料

- **筛选**
  - 性别筛选
  - 年龄筛选
  - 距离筛选
  - 在线筛选

### 聊天功能

- **即时通讯**
  - 文字聊天
  - 语音聊天
  - 视频聊天
  - 表情包

- **群组聊天**
  - 创建群组
  - 加入群组
  - 群聊天
  - 群管理

### 动态广场

- **发布动态**
  - 图文动态
  - 视频动态
  - 位置分享
  - 话题标签

- **浏览动态**
  - 附近动态
  - 关注动态
  - 热门动态
  - 点赞评论

### 直播功能

- **观看直播**
  - 直播列表
  - 热门主播
  - 直播互动
  - 送礼物

- **开播**
  - 开启直播
  - 直播设置
  - 收益管理

## 使用示例

```
查看附近的人
发起聊天
发布动态
加入群组
观看直播
送礼物
```

## 技术实现

使用陌陌开放平台 API。

## 社交特色

- **LBS 社交**
  - 基于地理位置
  - 附近的人
  - 同城交友

- **兴趣社交**
  - 兴趣群组
  - 话题讨论
  - 活动组织

## 功能模块

- 附近的人
- 即时通讯
- 动态广场
- 直播
- 群组

## 安全保障

- 实名认证
- 隐私保护
- 举报机制
- 内容审核
"#,
        &["curl"],
        &["web_fetch"],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_china_health_social_skills_count() {
        let skills = china_health_social_skills();
        assert_eq!(skills.len(), 8, "Should have 8 health and social skills");
    }

    #[test]
    fn test_all_health_social_skills_have_metadata() {
        let skills = china_health_social_skills();

        for skill in &skills {
            assert!(!skill.metadata.name.is_empty());
            assert!(!skill.metadata.description.is_empty());
            assert!(!skill.body.is_empty());
            assert!(
                skill.metadata.description.contains("(")
                    || skill.metadata.description.contains("（")
            );
        }
    }

    #[test]
    fn test_health_social_skill_names() {
        let skills = china_health_social_skills();
        let names: Vec<&str> = skills.iter().map(|s| s.metadata.name.as_str()).collect();

        assert!(names.contains(&"china-hospital"));
        assert!(names.contains(&"wechat-doctor"));
        assert!(names.contains(&"alipay-health"));
        assert!(names.contains(&"jd-health"));
        assert!(names.contains(&"meituan-doctor"));
        assert!(names.contains(&"douban"));
        assert!(names.contains(&"tieba"));
        assert!(names.contains(&"momo"));
    }

    #[test]
    fn test_health_social_categories() {
        let skills = china_health_social_skills();

        // Healthcare: 5
        let healthcare = vec![
            "china-hospital",
            "wechat-doctor",
            "alipay-health",
            "jd-health",
            "meituan-doctor",
        ];
        let healthcare_count = skills
            .iter()
            .filter(|s| healthcare.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(healthcare_count, 5);

        // Social: 3
        let social = vec!["douban", "tieba", "momo"];
        let social_count = skills
            .iter()
            .filter(|s| social.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(social_count, 3);
    }
}
