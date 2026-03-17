# ClawMaster 市场使用指南

**版本**: 1.0.0  
**标准**: DO-178C Level A  
**最后更新**: 2026年3月17日  

---

## 📋 目录

1. [简介](#简介)
2. [浏览 Skills](#浏览-skills)
3. [搜索 Skills](#搜索-skills)
4. [安装 Skills](#安装-skills)
5. [管理 Skills](#管理-skills)
6. [发布 Skills](#发布-skills)
7. [最佳实践](#最佳实践)

---

## 简介

ClawMaster 市场提供了丰富的 Skills 和 Tools，帮助你扩展 AI 助手的能力。

### 市场特性

- ✅ **105+ 内置 Skills**: DO-178C Level A 认证
- ✅ **社区 Skills**: 来自全球开发者
- ✅ **安全审查**: 所有 Skills 经过安全检查
- ✅ **一键安装**: 简单快速的安装流程
- ✅ **版本管理**: 支持多版本和自动更新

---

## 浏览 Skills

### CLI 浏览

```bash
# 列出所有可用 Skills
clawmaster skills list

# 列出已安装 Skills
clawmaster skills list --installed

# 列出内置 Skills
clawmaster skills list --bundled

# 按分类浏览
clawmaster skills list --category productivity
clawmaster skills list --category china
```

### 输出示例

```
Available Skills (105):

International Skills (53):
  ✓ obsidian          - Obsidian note-taking app integration
  ✓ notion            - Notion workspace management
  ✓ apple-notes       - Apple Notes integration
  ✓ github            - GitHub repository management
  ...

China Skills (52):
  ✓ wechat            - 微信 (WeChat) integration
  ✓ alipay            - 支付宝 (Alipay) payment and services
  ✓ taobao            - 淘宝 (Taobao) shopping
  ✓ jd                - 京东 (JD.com) e-commerce
  ...
```

---

## 搜索 Skills

### 基本搜索

```bash
# 搜索关键词
clawmaster skills search "weather"

# 搜索描述
clawmaster skills search "file management"

# 搜索中文
clawmaster skills search "微信"
```

### 高级搜索

```bash
# 按分类搜索
clawmaster skills search --category productivity

# 按格式搜索
clawmaster skills search --format skill-md

# 按安全状态搜索
clawmaster skills search --security verified

# 组合搜索
clawmaster skills search "git" --category developer --security verified
```

### 搜索结果示例

```
Search Results for "weather" (3 found):

1. weather-api
   Description: Fetch weather data from multiple APIs
   Category: utilities
   Downloads: 1,234
   Stars: 56
   Security: ✓ Verified

2. openweather
   Description: OpenWeatherMap API integration
   Category: utilities
   Downloads: 890
   Stars: 42
   Security: ✓ Verified

3. weather-forecast
   Description: 7-day weather forecast
   Category: utilities
   Downloads: 567
   Stars: 28
   Security: ⚠ Unverified
```

---

## 安装 Skills

### 从 GitHub 安装

```bash
# 基本安装
clawmaster skills install owner/repo

# 示例
clawmaster skills install vercel-labs/agent-skills
clawmaster skills install remotion-dev/skills
```

### 从本地安装

```bash
# 安装本地目录
clawmaster skills install /path/to/my-skill

# 安装当前目录
clawmaster skills install .
```

### 安装流程

```
1. 下载 Skill 仓库
   ├─ 从 GitHub 下载 tarball
   ├─ 解压到 ~/.clawmaster/installed-skills/
   └─ 验证文件完整性

2. 检测 Skill 格式
   ├─ SKILL.md 格式
   ├─ Claude Code Plugin 格式
   └─ 自动选择适配器

3. 扫描 Skills
   ├─ 解析元数据
   ├─ 验证依赖
   └─ 检查安全性

4. 更新清单
   ├─ 记录安装信息
   ├─ 保存 commit SHA
   └─ 设置初始状态 (disabled, untrusted)

5. 完成安装
   └─ 显示安装结果
```

### 安装后配置

```bash
# 启用 Skill
clawmaster skills enable skill-name

# 信任 Skill (重要!)
clawmaster skills trust skill-name

# 验证安装
clawmaster skills list --installed
```

---

## 管理 Skills

### 启用/禁用 Skills

```bash
# 启用 Skill
clawmaster skills enable obsidian

# 禁用 Skill
clawmaster skills disable obsidian

# 批量启用
clawmaster skills enable obsidian notion github

# 启用所有
clawmaster skills enable --all
```

### 信任管理

```bash
# 信任 Skill (允许执行)
clawmaster skills trust obsidian

# 取消信任
clawmaster skills untrust obsidian

# 查看信任状态
clawmaster skills list --trusted
```

### 更新 Skills

```bash
# 检查更新
clawmaster skills check-updates

# 更新单个 Skill
clawmaster skills update owner/repo

# 更新所有 Skills
clawmaster skills update --all
```

### 卸载 Skills

```bash
# 卸载 Skill
clawmaster skills remove owner/repo

# 强制卸载
clawmaster skills remove owner/repo --force

# 卸载并清理数据
clawmaster skills remove owner/repo --purge
```

---

## 发布 Skills

### 准备发布

#### 1. 创建 Skill 仓库

```bash
mkdir my-awesome-skill
cd my-awesome-skill

# 创建 SKILL.md
cat > SKILL.md << 'EOF'
---
name: my-awesome-skill
description: An awesome skill that does amazing things
homepage: https://github.com/yourusername/my-awesome-skill
license: MIT
allowed_tools:
  - bash
  - read_file
requires:
  bins:
    - git
---

# My Awesome Skill

Detailed description...

## Usage

Examples...

EOF
```

#### 2. 添加文档

```bash
# README.md
cat > README.md << 'EOF'
# My Awesome Skill

## Installation

```bash
clawmaster skills install yourusername/my-awesome-skill
```

## Usage

...

EOF

# LICENSE
cat > LICENSE << 'EOF'
MIT License
...
EOF

# CHANGELOG.md
cat > CHANGELOG.md << 'EOF'
# Changelog

## [1.0.0] - 2026-03-17
- Initial release
EOF
```

#### 3. 创建 Git 仓库

```bash
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/yourusername/my-awesome-skill.git
git push -u origin main
```

### 发布到 GitHub

```bash
# 创建标签
git tag v1.0.0
git push origin v1.0.0

# 创建 Release (在 GitHub 网页上)
# 1. 访问 https://github.com/yourusername/my-awesome-skill/releases/new
# 2. 选择标签 v1.0.0
# 3. 填写发布说明
# 4. 点击 "Publish release"
```

### 发布到 ClawHub (可选)

```bash
# 发布到 ClawHub
clawmaster skills publish yourusername/my-awesome-skill

# 验证发布
clawmaster skills search my-awesome-skill
```

### 发布清单

- [ ] SKILL.md 完整且格式正确
- [ ] README.md 包含安装和使用说明
- [ ] LICENSE 文件存在
- [ ] CHANGELOG.md 记录变更
- [ ] 所有示例可以运行
- [ ] 依赖明确列出
- [ ] 安全性说明完整
- [ ] 测试通过
- [ ] Git 标签创建
- [ ] GitHub Release 发布

---

## 最佳实践

### 1. 选择 Skills

#### 评估标准

- ✅ **安全状态**: 优先选择 Verified Skills
- ✅ **下载量**: 高下载量表示受欢迎
- ✅ **星标数**: 高星标表示质量好
- ✅ **更新频率**: 定期更新表示维护良好
- ✅ **文档质量**: 完整的文档更易使用

#### 安全检查

```bash
# 查看 Skill 详情
clawmaster skills info owner/repo

# 检查安全状态
clawmaster skills info owner/repo | grep Security

# 查看允许的工具
clawmaster skills info owner/repo | grep allowed_tools

# 查看依赖
clawmaster skills info owner/repo | grep requires
```

### 2. 安装 Skills

#### 安装前

- [ ] 阅读 Skill 描述
- [ ] 查看安全状态
- [ ] 检查依赖要求
- [ ] 阅读使用示例
- [ ] 查看许可证

#### 安装后

- [ ] 验证安装成功
- [ ] 检查依赖是否满足
- [ ] 启用 Skill
- [ ] 审查后信任 Skill
- [ ] 测试基本功能

### 3. 管理 Skills

#### 定期维护

```bash
# 每周检查更新
clawmaster skills check-updates

# 更新所有 Skills
clawmaster skills update --all

# 清理未使用的 Skills
clawmaster skills list --disabled
clawmaster skills remove <unused-skill>
```

#### 安全审查

```bash
# 定期审查已安装 Skills
clawmaster skills list --installed

# 检查信任状态
clawmaster skills list --trusted

# 取消不再使用的 Skills 的信任
clawmaster skills untrust <skill-name>
```

### 4. 发布 Skills

#### 质量检查

- [ ] 代码质量高
- [ ] 测试覆盖完整
- [ ] 文档清晰完整
- [ ] 示例可运行
- [ ] 无安全漏洞
- [ ] 性能良好

#### 发布流程

1. **开发阶段**: 本地开发和测试
2. **测试阶段**: 完整的测试覆盖
3. **文档阶段**: 编写完整文档
4. **审查阶段**: 代码审查和安全审查
5. **发布阶段**: 创建 Release
6. **推广阶段**: 社区推广

---

## 常见问题

### Q: 如何找到适合的 Skill？

**A**: 
1. 使用搜索功能: `clawmaster skills search <keyword>`
2. 浏览分类: `clawmaster skills list --category <category>`
3. 查看热门 Skills: `clawmaster skills list --sort downloads`
4. 参考官方推荐: [https://hub.clawmaster.ai/recommended](https://hub.clawmaster.ai/recommended)

### Q: Skill 安装失败怎么办？

**A**:
```bash
# 检查错误信息
clawmaster skills install owner/repo --verbose

# 常见原因:
# 1. 网络问题 - 检查网络连接
# 2. 仓库不存在 - 验证仓库名称
# 3. 格式不支持 - 检查 Skill 格式
# 4. 依赖缺失 - 安装所需依赖
```

### Q: 如何更新 Skill？

**A**:
```bash
# 检查可用更新
clawmaster skills check-updates

# 更新单个 Skill
clawmaster skills update owner/repo

# 更新所有 Skills
clawmaster skills update --all
```

### Q: 如何贡献 Skill？

**A**:
1. 开发 Skill (参考 [SKILL_DEVELOPMENT.md](SKILL_DEVELOPMENT.md))
2. 发布到 GitHub
3. 提交到 ClawHub (可选)
4. 分享到社区

### Q: Skill 和 Tool 有什么区别？

**A**:
- **Skill**: 高级能力模块，描述如何完成任务
- **Tool**: 底层执行单元，实际执行操作

Skill 使用 Tools 来完成任务。

---

## 示例场景

### 场景 1: 安装生产力 Skills

```bash
# 搜索生产力 Skills
clawmaster skills search --category productivity

# 安装常用 Skills
clawmaster skills install obsidian
clawmaster skills install notion
clawmaster skills install github

# 启用和信任
clawmaster skills enable obsidian notion github
clawmaster skills trust obsidian notion github

# 验证
clawmaster skills list --installed --enabled
```

### 场景 2: 安装中国本地化 Skills

```bash
# 搜索中国 Skills
clawmaster skills search --category china

# 安装微信和支付宝
clawmaster skills install wechat
clawmaster skills install alipay

# 启用
clawmaster skills enable wechat alipay
clawmaster skills trust wechat alipay
```

### 场景 3: 开发和发布自定义 Skill

```bash
# 1. 创建 Skill
mkdir my-skill
cd my-skill
# ... 创建 SKILL.md

# 2. 本地测试
clawmaster skills install .
clawmaster skills enable my-skill
clawmaster skills trust my-skill

# 3. 发布到 GitHub
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/yourusername/my-skill.git
git push -u origin main
git tag v1.0.0
git push origin v1.0.0

# 4. 让其他人安装
# 其他用户: clawmaster skills install yourusername/my-skill
```

---

## 资源链接

- [ClawMaster 官方文档](https://docs.clawmaster.ai)
- [Skills 市场](https://hub.clawmaster.ai)
- [Skill 开发指南](SKILL_DEVELOPMENT.md)
- [Tool 开发指南](TOOL_DEVELOPMENT.md)
- [社区论坛](https://community.clawmaster.ai)

---

## 支持

如有问题，请：
1. 查看[常见问题](#常见问题)
2. 搜索[社区论坛](https://community.clawmaster.ai)
3. 提交 [GitHub Issue](https://github.com/clawmaster/clawmaster/issues)

---

**版本**: 1.0.0  
**标准**: DO-178C Level A  
**最后更新**: 2026年3月17日  
**维护者**: ClawMaster Team
