//! China Express & Aviation Skills
//! 
//! This module provides Skills for Chinese express delivery and enhanced aviation services.
//! 中国快递物流和航空服务专用 Skills

use crate::create_bundled_skill;
use clawmaster_skills::types::SkillContent;

/// Get all China express and aviation skills (6 skills)
pub fn china_express_aviation_skills() -> Vec<SkillContent> {
    vec![
        // Express Delivery (6)
        sf_express_skill(),
        jd_logistics_skill(),
        cainiao_skill(),
        yto_express_skill(),
        zto_express_skill(),
        yunda_express_skill(),
    ]
}

// ============================================================================
// Express Delivery Skills
// ============================================================================

/// SF Express - 顺丰速运
/// China's leading express delivery service
fn sf_express_skill() -> SkillContent {
    create_bundled_skill(
        "sf-express",
        "顺丰速运 (SF Express)",
        r#"---
name: sf-express
description: 顺丰速运快递物流服务
homepage: https://www.sf-express.com
---

# SF Express Skill

顺丰速运集成，中国领先的快递物流服务商。

## 核心功能

### 寄件服务

- **上门取件**
  - 预约上门取件
  - 实时定位快递员
  - 取件时间选择
  - 包装材料提供

- **寄件方式**
  - 标准快递
  - 顺丰特快
  - 顺丰即日
  - 顺丰次晨
  - 顺丰隔日

- **增值服务**
  - 保价服务
  - 代收货款
  - 签收回单
  - 包装服务

### 查件服务

- **物流追踪**
  - 实时位置查询
  - 物流轨迹追踪
  - 预计送达时间
  - 签收状态查询

- **智能提醒**
  - 揽件提醒
  - 派件提醒
  - 签收通知
  - 异常提醒

### 收件服务

- **签收方式**
  - 本人签收
  - 代收签收
  - 丰巢自提
  - 驿站自提

- **预约配送**
  - 指定时间送达
  - 指定地点送达
  - 延迟配送
  - 改派地址

## 特色服务

### 顺丰丰巢

- **智能快递柜**
  - 24小时自助取件
  - 扫码开箱
  - 超时提醒
  - 免费保管时长

- **丰巢寄件**
  - 自助寄件
  - 在线下单
  - 扫码投递

### 顺丰会员

- **会员等级**
  - 普通会员
  - 银卡会员
  - 金卡会员
  - 钻石会员

- **会员权益**
  - 积分累积
  - 积分兑换
  - 专属优惠
  - 优先派送

## 使用示例

```
预约顺丰上门取件
查询顺丰快递
顺丰寄件到北京
查看顺丰运费
顺丰会员积分查询
```

## 技术实现

使用顺丰开放平台 API 进行集成。

## 服务网络

- **覆盖范围**: 全国 2000+ 城市
- **网点数量**: 20000+ 网点
- **快递员**: 500000+ 快递员
- **日均件量**: 8000万+ 件

## 时效承诺

- 顺丰特快: 省内当日达
- 顺丰即日: 市内 4 小时
- 顺丰次晨: 次日 12:00 前
- 顺丰隔日: 隔日 18:00 前

## 安全保障

- 实名寄递
- 全程监控
- 保价理赔
- 隐私保护
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// JD Logistics - 京东物流
/// JD.com's logistics service
fn jd_logistics_skill() -> SkillContent {
    create_bundled_skill(
        "jd-logistics",
        "京东物流 (JD Logistics)",
        r#"---
name: jd-logistics
description: 京东物流快递配送服务
homepage: https://www.jdl.com
---

# JD Logistics Skill

京东物流集成，京东集团旗下的综合物流服务商。

## 核心功能

### 快递服务

- **京东快递**
  - 特快送 (211 限时达)
  - 京准达 (预约配送)
  - 京尊达 (高端配送)
  - 夜间配送

- **仓配一体**
  - 京东仓储
  - 智能分拣
  - 快速配送
  - 库存管理

### 物流追踪

- **实时追踪**
  - GPS 定位
  - 配送员位置
  - 预计送达时间
  - 配送进度

- **智能通知**
  - 出库通知
  - 配送通知
  - 签收通知
  - 异常提醒

### 增值服务

- **京东快递柜**
  - 自助取件
  - 24 小时服务
  - 免费保管

- **上门服务**
  - 上门取件
  - 送装一体
  - 以旧换新
  - 上门维修

## 特色服务

### 211 限时达

- **服务承诺**
  - 上午 11 点前下单，当日送达
  - 晚上 11 点前下单，次日 15 点前送达

### 京准达

- **预约配送**
  - 2 小时精准送达
  - 自选配送时段
  - 配送员提前联系

### 京尊达

- **高端配送**
  - 专人配送
  - 白手套服务
  - 专业安装
  - 包装回收

## 使用示例

```
查询京东物流
京东快递寄件
预约京准达配送
查看配送员位置
京东快递柜取件
```

## 技术实现

使用京东物流开放平台 API。

## 服务优势

- **配送速度**: 211 限时达
- **服务质量**: 白手套服务
- **覆盖范围**: 全国 99% 区县
- **智能化**: 无人仓、无人车、无人机

## 物流网络

- 仓库数量: 1400+ 个
- 配送站点: 12000+ 个
- 配送员: 300000+ 人
- 日均订单: 2000万+ 单
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Cainiao - 菜鸟网络
/// Alibaba's logistics platform
fn cainiao_skill() -> SkillContent {
    create_bundled_skill(
        "cainiao",
        "菜鸟网络 (Cainiao)",
        r#"---
name: cainiao
description: 菜鸟网络智能物流平台
homepage: https://www.cainiao.com
---

# Cainiao Skill

菜鸟网络集成，阿里巴巴旗下的智能物流平台。

## 核心功能

### 物流追踪

- **全网物流查询**
  - 支持 100+ 快递公司
  - 统一查询入口
  - 实时物流信息
  - 智能预测送达

- **物流详情**
  - 揽件信息
  - 运输轨迹
  - 派送信息
  - 签收状态

### 菜鸟驿站

- **包裹代收**
  - 免费代收
  - 安全存放
  - 短信通知
  - 扫码取件

- **自助服务**
  - 自助寄件
  - 包装材料
  - 打印面单
  - 称重计费

### 菜鸟裹裹

- **寄件服务**
  - 在线下单
  - 上门取件
  - 价格对比
  - 多家快递选择

- **优惠活动**
  - 新人优惠
  - 会员折扣
  - 满减活动
  - 积分兑换

## 特色服务

### 智能物流

- **大数据分析**
  - 路径优化
  - 智能分单
  - 需求预测
  - 库存优化

- **物联网技术**
  - 电子面单
  - 智能分拣
  - 无人仓
  - 无人车配送

### 菜鸟云仓

- **仓储服务**
  - 智能仓储
  - 库存管理
  - 订单处理
  - 快速发货

## 使用示例

```
查询菜鸟物流
菜鸟驿站取件
菜鸟裹裹寄件
查看驿站位置
菜鸟会员权益
```

## 技术实现

使用菜鸟网络开放平台 API。

## 服务网络

- **驿站数量**: 100000+ 个
- **合作快递**: 100+ 家
- **覆盖范围**: 全国 2800+ 区县
- **日均包裹**: 3亿+ 件

## 合作快递

- 中通快递
- 圆通速递
- 申通快递
- 韵达快递
- 百世快递
- 天天快递
- 德邦快递

## 智能技术

- 电子面单
- 智能分拣机器人
- 无人仓
- 无人车
- 无人机
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// YTO Express - 圆通速递
/// One of China's major express delivery companies
fn yto_express_skill() -> SkillContent {
    create_bundled_skill(
        "yto-express",
        "圆通速递 (YTO Express)",
        r#"---
name: yto-express
description: 圆通速递快递物流服务
homepage: https://www.yto.net.cn
---

# YTO Express Skill

圆通速递集成，中国领先的快递物流企业。

## 核心功能

### 快递服务

- **标准快递**
  - 省内 1-2 天
  - 跨省 2-3 天
  - 偏远地区 3-5 天

- **特色服务**
  - 圆通承诺达
  - 圆通次日达
  - 圆通即日达
  - 圆通航空件

### 物流查询

- **运单查询**
  - 单号查询
  - 手机号查询
  - 批量查询
  - 订阅推送

- **物流信息**
  - 收寄信息
  - 运输信息
  - 派送信息
  - 签收信息

### 寄件服务

- **在线下单**
  - 网页下单
  - APP 下单
  - 微信下单
  - 支付宝下单

- **上门取件**
  - 预约取件
  - 快速响应
  - 免费上门
  - 包装建议

## 增值服务

- **保价服务**
  - 货物保价
  - 理赔服务
  - 快速理赔

- **代收货款**
  - COD 服务
  - 安全可靠
  - 快速回款

- **签收回单**
  - 签收凭证
  - 回单返还
  - 电子回单

## 使用示例

```
查询圆通快递
圆通寄件
预约圆通上门取件
圆通运费查询
圆通网点查询
```

## 技术实现

使用圆通速递开放平台 API。

## 服务网络

- 网点数量: 30000+ 个
- 服务范围: 全国 2800+ 区县
- 快递员: 200000+ 人
- 日均件量: 5000万+ 件

## 时效标准

- 同城: 1 天
- 省内: 1-2 天
- 跨省: 2-3 天
- 偏远: 3-5 天
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// ZTO Express - 中通快递
/// China's largest express delivery company by volume
fn zto_express_skill() -> SkillContent {
    create_bundled_skill(
        "zto-express",
        "中通快递 (ZTO Express)",
        r#"---
name: zto-express
description: 中通快递物流服务
homepage: https://www.zto.com
---

# ZTO Express Skill

中通快递集成，中国快递业务量第一的快递企业。

## 核心功能

### 快递服务

- **标准快递**
  - 经济实惠
  - 覆盖全国
  - 时效稳定

- **特快服务**
  - 中通特快
  - 次日达
  - 隔日达

### 物流追踪

- **实时查询**
  - 运单查询
  - 物流轨迹
  - 签收状态
  - 异常提醒

- **智能推送**
  - 短信通知
  - 微信推送
  - APP 推送
  - 邮件通知

### 寄件服务

- **多种方式**
  - 在线下单
  - 电话下单
  - 网点寄件
  - 上门取件

- **便捷支付**
  - 微信支付
  - 支付宝
  - 现金支付
  - 月结账户

## 增值服务

- **保价服务**
  - 货物保险
  - 丢失赔付
  - 破损赔付

- **代收货款**
  - COD 服务
  - 货款代收
  - 快速回款

- **签收回单**
  - 签收证明
  - 回单返还

## 使用示例

```
查询中通快递
中通寄件
中通上门取件
中通运费计算
中通网点查询
```

## 技术实现

使用中通快递开放平台 API。

## 服务规模

- 网点数量: 30000+ 个
- 转运中心: 90+ 个
- 快递员: 300000+ 人
- 日均件量: 7000万+ 件
- 市场份额: 第一

## 服务承诺

- 全国覆盖
- 时效稳定
- 价格优惠
- 服务专业
"#,
        &["curl"],
        &["web_fetch"],
    )
}

/// Yunda Express - 韵达快递
/// Major Chinese express delivery company
fn yunda_express_skill() -> SkillContent {
    create_bundled_skill(
        "yunda-express",
        "韵达快递 (Yunda Express)",
        r#"---
name: yunda-express
description: 韵达快递物流服务
homepage: https://www.yundaex.com
---

# Yunda Express Skill

韵达快递集成，中国主要的快递物流企业。

## 核心功能

### 快递服务

- **标准快递**
  - 全国配送
  - 价格实惠
  - 服务稳定

- **特色服务**
  - 韵达特快
  - 次日达
  - 经济件

### 物流查询

- **查询方式**
  - 运单号查询
  - 手机号查询
  - 批量查询
  - API 查询

- **物流详情**
  - 收件信息
  - 中转信息
  - 派件信息
  - 签收信息

### 寄件服务

- **下单方式**
  - 官网下单
  - APP 下单
  - 微信下单
  - 电话下单

- **取件服务**
  - 预约上门
  - 快速响应
  - 免费取件
  - 包装指导

## 增值服务

- **保价服务**
  - 物品保价
  - 理赔保障
  - 快速处理

- **代收货款**
  - 货款代收
  - 安全可靠
  - 及时回款

- **签收回单**
  - 签收证明
  - 回单服务

## 使用示例

```
查询韵达快递
韵达寄件
韵达上门取件
韵达运费查询
韵达网点查询
```

## 技术实现

使用韵达快递开放平台 API。

## 服务网络

- 网点数量: 40000+ 个
- 服务范围: 全国 31 省市
- 快递员: 180000+ 人
- 日均件量: 4000万+ 件

## 服务特色

- 全国网络
- 快速时效
- 优质服务
- 合理价格
"#,
        &["curl"],
        &["web_fetch"],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_china_express_aviation_skills_count() {
        let skills = china_express_aviation_skills();
        assert_eq!(skills.len(), 6, "Should have 6 express and aviation skills");
    }

    #[test]
    fn test_all_express_aviation_skills_have_metadata() {
        let skills = china_express_aviation_skills();
        
        for skill in &skills {
            assert!(!skill.metadata.name.is_empty());
            assert!(!skill.metadata.description.is_empty());
            assert!(!skill.body.is_empty());
            assert!(skill.metadata.description.contains("(") || 
                    skill.metadata.description.contains("（"));
        }
    }

    #[test]
    fn test_express_aviation_skill_names() {
        let skills = china_express_aviation_skills();
        let names: Vec<&str> = skills.iter()
            .map(|s| s.metadata.name.as_str())
            .collect();
        
        assert!(names.contains(&"sf-express"));
        assert!(names.contains(&"jd-logistics"));
        assert!(names.contains(&"cainiao"));
        assert!(names.contains(&"yto-express"));
        assert!(names.contains(&"zto-express"));
        assert!(names.contains(&"yunda-express"));
    }

    #[test]
    fn test_express_categories() {
        let skills = china_express_aviation_skills();
        
        // Express: 6
        let express = vec!["sf-express", "jd-logistics", "cainiao", 
                          "yto-express", "zto-express", "yunda-express"];
        let express_count = skills.iter()
            .filter(|s| express.contains(&s.metadata.name.as_str()))
            .count();
        assert_eq!(express_count, 6);
    }
}
