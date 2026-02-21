# MCP Inspector Desktop - 长时间运行代理系统

基于 Anthropic 的"双代理架构"设计，实现可跨越多个上下文窗口的全自动编程系统。

## 📁 项目结构

```
mcp-inspector-desktop/
├── .claude/
│   ├── README.md                    # 本文件
│   ├── init-system.md               # 系统设计文档
│   ├── initializer-agent.md         # 初始化代理提示词
│   └── coding-agent.md              # 编码代理提示词
├── feature_list.json                # 功能检查清单
├── claude-progress.txt              # 进度日志（初始化后创建）
├── init.sh                          # Unix/Linux/macOS 启动脚本
├── init.bat                         # Windows 启动脚本
├── prd.md                           # 产品需求文档
└── src-tauri/                       # Tauri 项目（初始化后创建）
```

## 🚀 快速开始

### 方式一：使用 Long-Running Agent Skill

这是最推荐的方式，使用 Claude Code 内置的 `long-running-agent` skill：

```
/long-running-agent
```

该 skill 会自动：
1. 运行初始化代理设置项目
2. 自动启动编码代理实现功能
3. 跨会话保持进度

### 方式二：手动运行代理

#### 步骤 1: 运行初始化代理

复制 `.claude/initializer-agent.md` 的内容发送给 Claude：

```
请读取并执行 .claude/initializer-agent.md 中的初始化代理任务
```

初始化代理会：
- 创建 Tauri + React 项目
- 安装所有依赖
- 创建目录结构
- 生成初始提交

#### 步骤 2: 运行编码代理

每次需要继续开发时，复制 `.claude/coding-agent.md` 的内容发送给 Claude：

```
请作为编码代理，按照 .claude/coding-agent.md 的协议工作
```

## 📋 功能清单

所有功能都在 `feature_list.json` 中定义，分为三个里程碑：

### MVP (F-001 ~ F-010)
核心功能，实现可运行的应用

### 稳定性 (F-101 ~ F-107)
配置持久化、日志、错误处理

### 生产就绪 (F-201 ~ F-205)
托盘、自动更新、CI/CD

## 🔄 开发循环

每个编码代理会话遵循以下流程：

```
1. pwd → 确认工作目录
2. cat claude-progress.txt → 了解上次进度
3. git log --oneline -10 → 查看最近提交
4. cat feature_list.json → 找到下一个未完成功能
5. npm run tauri dev → 启动开发服务器
6. 测试基础功能 → 确保环境正常
7. 实现单个功能
8. 测试功能
9. git commit → 提交代码
10. 更新 claude-progress.txt
11. 更新 feature_list.json (passes: true)
```

## 🛠️ 开发环境

### 启动开发服务器

**Unix/Linux/macOS:**
```bash
chmod +x init.sh
./init.sh
```

**Windows:**
```bash
init.bat
```

### 手动启动

```bash
npm install
npm run tauri dev
```

## 📊 进度追踪

- **feature_list.json**: 功能检查清单，每个功能有明确的验收步骤
- **claude-progress.txt**: 会话日志，记录每次会话完成的工作
- **Git commits**: 每个功能一次提交，可回滚

## 🎯 关键原则

1. **单功能原则**: 每次会话只实现一个功能
2. **测试先行**: 实现前验证基准，实现后验证功能
3. **清晰提交**: 每个功能一个 git commit
4. **文档更新**: 每次会话更新进度文件
5. **环境清洁**: 离开时代码库必须是可工作状态

## 📚 参考

- [Anthropic 长时间运行代理文章](https://www.anthropic.com/engineering/effective-harnesses-for-long-running-agents)
- [Tauri 文档](https://tauri.app/v1/guides/)
- [MCP Inspector](https://github.com/modelcontextprotocol/inspector)

## 🐛 故障排除

### 项目初始化失败
- 检查 Node.js 版本 (需要 >= 18)
- 检查 Rust 工具链是否安装
- 尝试手动运行 `npm create tauri-app@latest`

### 功能实现错误
- 查看 claude-progress.txt 了解最近变更
- 使用 `git log` 查看提交历史
- 使用 `git revert` 回滚有问题的提交

### 端口冲突
- 检查 5174 和 6277 端口是否被占用
- 应用会自动选择可用端口

---

**设计理念**: 这个系统让多个 AI 代理像工程师轮班一样协作，每个代理接手时都能快速了解项目状态，完成一个功能后留下清晰的记录，供下一个代理继续工作。
