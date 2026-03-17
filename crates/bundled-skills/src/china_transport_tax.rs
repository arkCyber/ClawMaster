//! China Transport & Tax Skills
//! 
//! This module provides Skills for Chinese transportation and tax services.
//! 中国交通和税务服务专用 Skills

use crate::create_bundled_skill;
use clawmaster_skills::types::SkillContent;

/// Get all China transport and tax skills (5 skills)
pub fn china_transport_tax_skills() -> Vec<SkillContent> {
    vec![
        // Aviation (1)
        china_airlines_skill(),
        
        // Railway (1)
        china_railway_skill(),
        
        // Highway (1)
        china_highway_skill(),
        
        // Shanghai Tax (2)
        shanghai_tax_skill(),
        shanghai_etax_skill(),
    ]
}

// ============================================================================
// Aviation Skills
// ============================================================================

/// China Airlines - 中国航空公司
/// Comprehensive Chinese airline services
fn china_airlines_skill() -> SkillContent {
    create_bundled_skill(
        "china-airlines",
        "中国航空 (China Airlines)",
        r#"---
name: china-airlines
description: 中国航空公司综合服务
homepage: https://www.airchina.com.cn
---

# China Airlines Skill

中国航空公司集成，包括国航、东航、南航等主要航空公司服务。

## 功能

### 主要航空公司

- **中国国际航空 (Air China)** - CA
  - 航班查询和预订
  - 值机服务
  - 里程累积
  - 凤凰知音会员

- **中国东方航空 (China Eastern)** - MU
  - 航班查询和预订
  - 在线值机
  - 东方万里行会员
  - 行李追踪

- **中国南方航空 (China Southern)** - CZ
  - 航班查询和预订
  - 自助值机
  - 南航明珠会员
  - 航班动态

- **海南航空 (Hainan Airlines)** - HU
  - 航班预订
  - 金鹏俱乐部
  - 特价机票

- **厦门航空 (Xiamen Airlines)** - MF
  - 航班服务
  - 白鹭会员
  - 行李服务

## 核心功能

- 航班实时查询
- 机票预订和改签
- 在线值机
- 登机牌打印
- 航班动态追踪
- 里程查询和兑换
- 会员权益管理
- 行李额度查询
- 退改签服务
- 特价机票推送

## 使用示例

```
查询北京到上海的航班
预订国航航班
在线值机
查询航班动态
累积里程
兑换免费机票
查询行李额度
```

## 技术实现

使用各航空公司开放 API 和航旅纵横 API 进行集成。

## 机场服务

- 首都国际机场 (PEK)
- 浦东国际机场 (PVG)
- 虹桥国际机场 (SHA)
- 白云国际机场 (CAN)
- 双流国际机场 (CTU)
- 天河国际机场 (WUH)

## 会员体系

- 凤凰知音 (国航)
- 东方万里行 (东航)
- 南航明珠 (南航)
- 金鹏俱乐部 (海航)
- 白鹭会员 (厦航)

## 安全保障

- 实名制购票
- 安全检查
- 航班保险
- 退改保障
"#,
        &["curl", "python3"],
        &["web_fetch", "exec"],
    )
}

// ============================================================================
// Railway Skills
// ============================================================================

/// China Railway - 中国铁路
/// China Railway 12306 ticketing system
fn china_railway_skill() -> SkillContent {
    create_bundled_skill(
        "china-railway",
        "中国铁路 12306 (China Railway)",
        r#"---
name: china-railway
description: 中国铁路 12306 购票系统
homepage: https://www.12306.cn
---

# China Railway 12306 Skill

中国铁路 12306 购票系统集成，高铁、动车、普通列车票务服务。

## 功能

### 列车类型

- **高速铁路 (G)** - 时速 300-350 公里
  - 京沪高铁
  - 京广高铁
  - 沪昆高铁
  - 京津城际

- **动车组 (D)** - 时速 200-250 公里
  - 城际动车
  - 跨省动车

- **城际列车 (C)** - 城际快速
  - 珠三角城际
  - 长三角城际

- **直达特快 (Z)** - 夕发朝至
- **特快列车 (T)** - 长途快速
- **快速列车 (K)** - 普通快速

## 核心功能

- 车票查询
- 在线购票
- 退票改签
- 候补购票
- 学生票优惠
- 儿童票购买
- 铁路畅行会员
- 常旅客服务
- 行程规划
- 列车时刻表
- 余票监控
- 自动抢票

## 使用示例

```
查询北京到上海的高铁
购买高铁票
改签火车票
退火车票
候补购票
查询列车时刻表
监控余票
自动抢票
```

## 技术实现

使用 12306 官方 API 和第三方抢票工具集成。

## 主要线路

### 高铁干线

- **京沪高铁** - 北京 ↔ 上海 (1318 公里)
- **京广高铁** - 北京 ↔ 广州 (2298 公里)
- **沪昆高铁** - 上海 ↔ 昆明 (2252 公里)
- **京哈高铁** - 北京 ↔ 哈尔滨 (1249 公里)

### 城际铁路

- 京津城际
- 沪宁城际
- 广深城际
- 成渝城际

## 车站服务

- 自助取票机
- 人工窗口
- 候车室
- VIP 候车室
- 行李寄存
- 餐饮服务

## 优惠政策

- 学生票 75 折
- 儿童票半价
- 残疾军人优惠
- 铁路畅行积分

## 安全提示

- 实名制购票
- 身份证验证
- 安检流程
- 禁止携带物品清单
"#,
        &["curl", "python3"],
        &["web_fetch", "exec"],
    )
}

// ============================================================================
// Highway Skills
// ============================================================================

/// China Highway - 中国公路
/// China highway and ETC services
fn china_highway_skill() -> SkillContent {
    create_bundled_skill(
        "china-highway",
        "中国高速公路 (China Highway)",
        r#"---
name: china-highway
description: 中国高速公路和 ETC 服务
homepage: https://www.txffp.com
---

# China Highway Skill

中国高速公路系统集成，ETC 服务、路况查询、收费查询。

## 功能

### ETC 服务

- **ETC 办理**
  - 线上申请
  - 设备安装
  - 账户充值
  - 发票开具

- **ETC 使用**
  - 不停车收费
  - 95 折优惠
  - 通行记录查询
  - 消费明细

### 路况服务

- 实时路况查询
- 拥堵预警
- 事故提醒
- 施工信息
- 天气预报
- 服务区信息

### 收费查询

- 过路费计算
- 收费标准查询
- 优惠政策
- 免费时段

## 使用示例

```
办理 ETC
查询高速路况
计算过路费
查询服务区
ETC 充值
查询通行记录
```

## 技术实现

使用全国高速公路联网收费系统 API 和各省交通厅 API。

## 主要高速公路

### 国家高速公路网

- **G1 京哈高速** - 北京 ↔ 哈尔滨
- **G2 京沪高速** - 北京 ↔ 上海
- **G3 京台高速** - 北京 ↔ 台北
- **G4 京港澳高速** - 北京 ↔ 香港/澳门
- **G5 京昆高速** - 北京 ↔ 昆明
- **G6 京藏高速** - 北京 ↔ 拉萨
- **G15 沈海高速** - 沈阳 ↔ 海口
- **G20 青银高速** - 青岛 ↔ 银川

### 区域高速

- 沪宁高速
- 沪杭高速
- 广深高速
- 成渝高速

## 服务区设施

- 加油站
- 充电桩
- 餐饮服务
- 便利店
- 卫生间
- 休息区
- 汽车维修

## ETC 优惠

- 通行费 95 折
- 部分路段更多优惠
- 节假日免费通行
- 绿色通道免费

## 免费通行时段

- 春节
- 清明节
- 劳动节
- 国庆节

## 安全提示

- 保持车距
- 限速行驶
- 疲劳驾驶提醒
- 恶劣天气预警
"#,
        &["curl"],
        &["web_fetch"],
    )
}

// ============================================================================
// Shanghai Tax Skills
// ============================================================================

/// Shanghai Tax - 上海税务
/// Shanghai tax filing and management
fn shanghai_tax_skill() -> SkillContent {
    create_bundled_skill(
        "shanghai-tax",
        "上海税务 (Shanghai Tax)",
        r#"---
name: shanghai-tax
description: 上海市税务局报税服务
homepage: https://shanghai.chinatax.gov.cn
---

# Shanghai Tax Skill

上海市税务局集成，个人所得税、企业税务申报和管理。

## 功能

### 个人所得税

- **综合所得申报**
  - 工资薪金
  - 劳务报酬
  - 稿酬所得
  - 特许权使用费

- **专项附加扣除**
  - 子女教育
  - 继续教育
  - 大病医疗
  - 住房贷款利息
  - 住房租金
  - 赡养老人
  - 3 岁以下婴幼儿照护

- **年度汇算清缴**
  - 收入核对
  - 扣除确认
  - 退税申请
  - 补税缴纳

### 企业税务

- **增值税申报**
  - 一般纳税人
  - 小规模纳税人
  - 进项税抵扣
  - 销项税计算

- **企业所得税**
  - 季度预缴
  - 年度汇算
  - 税前扣除
  - 优惠政策

- **其他税种**
  - 城建税
  - 教育费附加
  - 印花税
  - 房产税
  - 土地使用税

## 使用示例

```
个人所得税申报
专项附加扣除
年度汇算清缴
企业增值税申报
企业所得税申报
查询纳税记录
下载完税证明
```

## 技术实现

使用上海市电子税务局 API 和国家税务总局 API。

## 办税渠道

- **线上办税**
  - 电子税务局网站
  - 个人所得税 APP
  - 支付宝/微信小程序
  - 企业电子税务局

- **线下办税**
  - 办税服务厅
  - 自助办税终端
  - 银行代征点

## 主要税种

### 个人税种

- 个人所得税 (3%-45%)
- 财产转让所得税
- 利息股息红利所得税

### 企业税种

- 增值税 (13%/9%/6%)
- 企业所得税 (25%)
- 城建税 (7%)
- 教育费附加 (3%)
- 地方教育附加 (2%)

## 优惠政策

- 小微企业税收优惠
- 高新技术企业优惠
- 研发费用加计扣除
- 残疾人就业优惠
- 创业投资优惠

## 申报期限

- 个人所得税: 次月 15 日前
- 增值税: 次月 15 日前
- 企业所得税: 季后 15 日内
- 年度汇算: 次年 3-6 月

## 安全保障

- 实名认证
- 数字证书
- 电子签名
- 数据加密
- 隐私保护
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Shanghai E-Tax - 上海电子税务局
/// Shanghai electronic tax bureau
fn shanghai_etax_skill() -> SkillContent {
    create_bundled_skill(
        "shanghai-etax",
        "上海电子税务局 (Shanghai E-Tax)",
        r#"---
name: shanghai-etax
description: 上海电子税务局在线服务
homepage: https://etax.shanghai.chinatax.gov.cn
---

# Shanghai E-Tax Skill

上海电子税务局在线服务，全流程电子化办税。

## 功能

### 在线办税

- **税务登记**
  - 新办企业登记
  - 变更登记
  - 注销登记
  - 税种认定

- **申报缴税**
  - 在线申报
  - 电子缴税
  - 申报表下载
  - 缴款凭证

- **发票管理**
  - 发票领用
  - 发票开具
  - 发票查验
  - 发票作废
  - 红字发票

- **证明开具**
  - 完税证明
  - 纳税证明
  - 税收居民证明
  - 出口退税证明

### 查询服务

- 纳税信用查询
- 欠税查询
- 发票查询
- 申报记录查询
- 缴款记录查询

### 互动服务

- 在线咨询
- 预约办税
- 投诉建议
- 政策查询
- 表单下载

## 使用示例

```
在线申报增值税
电子缴税
领用发票
开具发票
查验发票
下载完税证明
查询纳税信用
预约办税
```

## 技术实现

使用上海电子税务局 API 和 CA 数字证书认证。

## 电子发票

### 增值税电子发票

- 电子普通发票
- 电子专用发票
- 全电发票 (数电票)

### 发票功能

- 在线开具
- 电子签章
- 实时验证
- 自动归档
- 便捷报销

## 实名认证

- **个人实名**
  - 身份证认证
  - 人脸识别
  - 手机验证

- **企业实名**
  - 营业执照
  - 法人身份
  - 数字证书
  - 电子签章

## 支付方式

- 银行卡支付
- 第三方支付
- 银行端查询缴税
- 批量扣款

## 办税时间

- 7×24 小时在线服务
- 申报期延长服务
- 节假日不打烊

## 技术支持

- 在线客服
- 电话咨询: 12366
- 远程协助
- 操作指南
- 视频教程

## 安全措施

- CA 数字证书
- 双因素认证
- 数据加密传输
- 操作日志记录
- 异常登录提醒

## 优势特点

- 全程网上办
- 一次不用跑
- 即时办结
- 电子存档
- 智能提醒
"#,
        &["curl"],
        &["web_fetch"],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_china_transport_tax_skills_count() {
        let skills = china_transport_tax_skills();
        assert_eq!(skills.len(), 5, "Should have 5 transport and tax skills");
    }

    #[test]
    fn test_all_transport_tax_skills_have_metadata() {
        let skills = china_transport_tax_skills();
        
        for skill in &skills {
            assert!(!skill.metadata.name.is_empty());
            assert!(!skill.metadata.description.is_empty());
            assert!(!skill.body.is_empty());
            assert!(skill.metadata.description.contains("(") || 
                    skill.metadata.description.contains("（"));
        }
    }

    #[test]
    fn test_transport_tax_skill_names() {
        let skills = china_transport_tax_skills();
        let names: Vec<&str> = skills.iter()
            .map(|s| s.metadata.name.as_str())
            .collect();
        
        assert!(names.contains(&"china-airlines"));
        assert!(names.contains(&"china-railway"));
        assert!(names.contains(&"china-highway"));
        assert!(names.contains(&"shanghai-tax"));
        assert!(names.contains(&"shanghai-etax"));
    }

    #[test]
    fn test_transport_tax_categories() {
        let skills = china_transport_tax_skills();
        
        // Aviation: 1
        let aviation = vec!["china-airlines"];
        let aviation_count = skills.iter()
            .filter(|s| aviation.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(aviation_count, 1);
        
        // Railway: 1
        let railway = vec!["china-railway"];
        let railway_count = skills.iter()
            .filter(|s| railway.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(railway_count, 1);
        
        // Highway: 1
        let highway = vec!["china-highway"];
        let highway_count = skills.iter()
            .filter(|s| highway.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(highway_count, 1);
        
        // Tax: 2
        let tax = vec!["shanghai-tax", "shanghai-etax"];
        let tax_count = skills.iter()
            .filter(|s| tax.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(tax_count, 2);
    }
}
