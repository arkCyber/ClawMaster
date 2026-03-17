# ClawMaster Skill 开发指南

**版本**: 1.0.0  
**标准**: DO-178C Level A  
**最后更新**: 2026年3月17日  

---

## 📋 目录

1. [简介](#简介)
2. [快速开始](#快速开始)
3. [Skill 结构](#skill-结构)
4. [开发流程](#开发流程)
5. [质量标准](#质量标准)
6. [测试要求](#测试要求)
7. [发布流程](#发布流程)
8. [最佳实践](#最佳实践)
9. [常见问题](#常见问题)

---

## 简介

ClawMaster Skills 是可复用的 AI 能力模块，遵循 DO-178C Level A 航空航天软件标准。本指南将帮助你开发高质量的 Skills。

### 什么是 Skill？

Skill 是一个包含以下内容的模块：
- **元数据**: 名称、描述、依赖等
- **功能描述**: Markdown 格式的详细说明
- **使用示例**: 实际应用场景
- **工具权限**: 允许使用的工具列表

---

## 快速开始

### 1. 创建你的第一个 Skill

```bash
# 创建 Skill 目录
mkdir -p my-skills/hello-world
cd my-skills/hello-world

# 创建 SKILL.md
cat > SKILL.md << 'EOF'
---
name: hello-world
description: A simple hello world skill
homepage: https://github.com/yourusername/my-skills
license: MIT
allowed_tools:
  - bash
  - read_file
requires:
  bins:
    - echo
---

# Hello World Skill

这是一个简单的示例 Skill，演示如何创建和使用 Skills。

## 功能

- 打印 "Hello, World!" 消息
- 读取文件内容
- 执行简单的 bash 命令

## 使用示例

### 示例 1: 打印消息

用户: "Say hello to the world"

助手: 我将使用 hello-world skill 来打印消息。

```bash
echo "Hello, World!"
```

输出: Hello, World!

### 示例 2: 读取文件

用户: "Read the README file"

助手: 我将读取 README.md 文件。

```bash
cat README.md
```

## 依赖

- `echo` 命令 (通常预装)
- `cat` 命令 (通常预装)

## 安全性

此 Skill 仅使用安全的只读命令，不会修改系统文件。

EOF
```

### 2. 测试你的 Skill

```bash
# 安装到本地
clawmaster skills install /path/to/my-skills

# 列出已安装的 Skills
clawmaster skills list

# 启用 Skill
clawmaster skills enable hello-world

# 信任 Skill
clawmaster skills trust hello-world
```

### 3. 发布到 GitHub

```bash
cd my-skills
git init
git add .
git commit -m "Add hello-world skill"
git remote add origin https://github.com/yourusername/my-skills.git
git push -u origin main
```

### 4. 让其他人安装

```bash
clawmaster skills install yourusername/my-skills
```

---

## Skill 结构

### SKILL.md 格式

```markdown
---
# 必需字段
name: skill-name                    # Skill 唯一标识符
description: Short description      # 简短描述 (< 120 字符)

# 可选字段
homepage: https://example.com       # 项目主页
license: MIT                        # 许可证 (MIT, Apache-2.0, GPL-3.0 等)
compatibility: ["macos", "linux"]   # 兼容平台
allowed_tools:                      # 允许使用的工具
  - bash
  - read_file
  - write_file
requires:                           # 依赖
  bins:                             # 必需的二进制程序
    - git
    - node
  any_bins:                         # 可选的二进制程序 (至少一个)
    - npm
    - yarn
  install:                          # 安装说明
    - "npm install -g typescript"
dockerfile: Dockerfile              # Docker 镜像 (可选)
---

# Skill 标题

详细的 Skill 描述和使用说明。

## 功能

- 功能 1
- 功能 2
- 功能 3

## 使用示例

### 示例 1: 基本用法

用户: "Do something"

助手: [助手响应]

### 示例 2: 高级用法

用户: "Do something advanced"

助手: [助手响应]

## 依赖

列出所有外部依赖。

## 安全性

说明 Skill 的安全特性和限制。
```

### 元数据字段详解

| 字段 | 类型 | 必需 | 说明 |
|------|------|------|------|
| `name` | string | ✅ | Skill 唯一标识符 (小写、连字符) |
| `description` | string | ✅ | 简短描述 (< 120 字符) |
| `homepage` | string | ❌ | 项目主页 URL |
| `license` | string | ❌ | 许可证类型 |
| `compatibility` | array | ❌ | 兼容平台列表 |
| `allowed_tools` | array | ❌ | 允许使用的工具列表 |
| `requires.bins` | array | ❌ | 必需的二进制程序 |
| `requires.any_bins` | array | ❌ | 可选的二进制程序 (至少一个) |
| `requires.install` | array | ❌ | 安装说明 |
| `dockerfile` | string | ❌ | Docker 镜像路径 |

---

## 开发流程

### 1. 规划阶段

- [ ] 确定 Skill 的功能和用途
- [ ] 检查是否已有类似 Skill
- [ ] 确定依赖和工具需求
- [ ] 设计 API 和使用方式

### 2. 开发阶段

- [ ] 创建 SKILL.md 文件
- [ ] 编写元数据 (frontmatter)
- [ ] 编写功能描述
- [ ] 添加使用示例 (至少 3 个)
- [ ] 说明依赖和安装步骤
- [ ] 添加安全性说明

### 3. 测试阶段

- [ ] 本地测试 Skill 安装
- [ ] 测试所有使用示例
- [ ] 验证依赖检查
- [ ] 测试工具权限
- [ ] 进行安全审查

### 4. 文档阶段

- [ ] 编写 README.md
- [ ] 添加 LICENSE 文件
- [ ] 创建 CHANGELOG.md
- [ ] 添加贡献指南

### 5. 发布阶段

- [ ] 创建 Git 仓库
- [ ] 推送到 GitHub
- [ ] 创建 Release
- [ ] 发布到 ClawHub (可选)

---

## 质量标准

### DO-178C Level A 要求

所有 ClawMaster Skills 必须满足以下质量标准：

#### 1. 代码质量

- ✅ **元数据完整性**: 所有必需字段必须填写
- ✅ **描述清晰性**: 描述必须准确、简洁
- ✅ **示例完整性**: 至少包含 3 个实际使用示例
- ✅ **依赖明确性**: 所有依赖必须明确列出

#### 2. 安全性

- ✅ **工具权限**: 仅请求必需的工具权限
- ✅ **路径安全**: 不使用绝对路径或路径遍历
- ✅ **输入验证**: 验证所有用户输入
- ✅ **错误处理**: 优雅处理所有错误情况

#### 3. 可维护性

- ✅ **版本控制**: 使用语义化版本 (SemVer)
- ✅ **变更日志**: 记录所有重要变更
- ✅ **向后兼容**: 避免破坏性变更

#### 4. 文档质量

- ✅ **README**: 包含完整的使用说明
- ✅ **LICENSE**: 明确的许可证
- ✅ **示例**: 实际可运行的示例
- ✅ **故障排除**: 常见问题解答

---

## 测试要求

### 单元测试

每个 Skill 应包含以下测试：

```bash
# 测试目录结构
my-skill/
├── SKILL.md
├── README.md
├── tests/
│   ├── test_metadata.sh      # 元数据测试
│   ├── test_examples.sh      # 示例测试
│   └── test_dependencies.sh  # 依赖测试
└── .github/
    └── workflows/
        └── test.yml          # CI/CD 测试
```

### 测试清单

- [ ] **元数据测试**: 验证 frontmatter 格式正确
- [ ] **示例测试**: 所有示例可以运行
- [ ] **依赖测试**: 依赖检查正确工作
- [ ] **工具权限测试**: 工具权限正确限制
- [ ] **安全测试**: 无安全漏洞
- [ ] **性能测试**: 响应时间 < 1 秒

### CI/CD 示例

```yaml
# .github/workflows/test.yml
name: Test Skill

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install ClawMaster
        run: |
          curl -sSL https://install.clawmaster.ai | sh
      
      - name: Test Skill Installation
        run: |
          clawmaster skills install .
      
      - name: Run Tests
        run: |
          bash tests/test_metadata.sh
          bash tests/test_examples.sh
          bash tests/test_dependencies.sh
```

---

## 发布流程

### 1. 准备发布

```bash
# 更新版本号
echo "1.0.0" > VERSION

# 更新 CHANGELOG.md
cat >> CHANGELOG.md << 'EOF'
## [1.0.0] - 2026-03-17

### Added
- Initial release
- Basic functionality
- Documentation

EOF

# 提交变更
git add .
git commit -m "Release v1.0.0"
git tag v1.0.0
git push origin main --tags
```

### 2. 创建 GitHub Release

1. 访问 `https://github.com/yourusername/my-skills/releases/new`
2. 选择标签 `v1.0.0`
3. 填写发布说明
4. 点击 "Publish release"

### 3. 发布到 ClawHub (可选)

```bash
# 发布到 ClawHub
clawmaster skills publish yourusername/my-skills

# 验证发布
clawmaster skills search my-skill
```

---

## 最佳实践

### 1. 命名规范

- ✅ 使用小写字母和连字符: `my-awesome-skill`
- ✅ 避免使用下划线: ~~`my_skill`~~
- ✅ 保持简短有意义: `git-helper` 而不是 `git-repository-management-helper`
- ✅ 避免通用名称: `weather-api` 而不是 `api`

### 2. 描述规范

- ✅ 简短精确 (< 120 字符)
- ✅ 使用动词开头: "Helps you manage..."
- ✅ 说明核心功能: "Fetch weather data from OpenWeatherMap API"
- ✅ 避免营销语言: ~~"The best weather skill ever!"~~

### 3. 示例规范

- ✅ 使用真实场景
- ✅ 包含用户输入和助手响应
- ✅ 展示不同用例
- ✅ 提供预期输出

### 4. 依赖管理

- ✅ 最小化依赖
- ✅ 使用广泛可用的工具
- ✅ 提供安装说明
- ✅ 考虑跨平台兼容性

### 5. 安全实践

- ✅ 最小权限原则
- ✅ 验证所有输入
- ✅ 避免执行任意代码
- ✅ 使用安全的 API

---

## 常见问题

### Q: Skill 和 Tool 有什么区别？

**A**: 
- **Skill**: 高级能力模块，描述如何完成任务
- **Tool**: 底层执行单元，实际执行操作

Skill 使用 Tools 来完成任务。

### Q: 如何测试 Skill？

**A**:
```bash
# 本地安装
clawmaster skills install /path/to/my-skill

# 启用和信任
clawmaster skills enable my-skill
clawmaster skills trust my-skill

# 在聊天中测试
clawmaster chat "Use my-skill to do something"
```

### Q: 如何更新已发布的 Skill？

**A**:
```bash
# 更新代码
git add .
git commit -m "Update skill"

# 创建新版本
git tag v1.1.0
git push origin main --tags

# 用户更新
clawmaster skills update yourusername/my-skill
```

### Q: Skill 可以调用其他 Skill 吗？

**A**: 可以。在 `allowed_tools` 中包含其他 Skill 的名称即可。

### Q: 如何处理平台差异？

**A**: 使用 `compatibility` 字段限制平台，或在 Skill 中提供平台特定的说明。

```yaml
compatibility: ["macos", "linux"]  # 不支持 Windows
```

### Q: 如何贡献到官方 Skills？

**A**: 
1. Fork `clawmaster/bundled-skills` 仓库
2. 添加你的 Skill
3. 提交 Pull Request
4. 等待审核

---

## 示例 Skills

### 示例 1: 简单 Skill

```markdown
---
name: hello
description: Print hello message
allowed_tools: [bash]
---

# Hello Skill

Print a hello message.

## Usage

User: "Say hello"
Assistant: Hello, World!
```

### 示例 2: 复杂 Skill

```markdown
---
name: git-helper
description: Git repository management helper
homepage: https://github.com/example/git-helper
license: MIT
allowed_tools:
  - bash
  - read_file
  - write_file
requires:
  bins:
    - git
---

# Git Helper Skill

Comprehensive Git repository management.

## Features

- Clone repositories
- Create branches
- Commit changes
- Push to remote

## Usage Examples

### Example 1: Clone Repository

User: "Clone the react repository"
Assistant: I'll clone the repository for you.

```bash
git clone https://github.com/facebook/react.git
```

### Example 2: Create Branch

User: "Create a new feature branch"
Assistant: I'll create a new branch.

```bash
git checkout -b feature/new-feature
```

## Dependencies

- Git 2.0+

## Security

This skill only uses standard Git commands and does not execute arbitrary code.
```

---

## 资源链接

- [ClawMaster 官方文档](https://docs.clawmaster.ai)
- [Skills 市场](https://hub.clawmaster.ai)
- [示例 Skills 仓库](https://github.com/clawmaster/example-skills)
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
