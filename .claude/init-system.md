# MCP Inspector Desktop - 长时间运行代理系统

## 双代理架构

### 1. Initializer Agent（初始化代理）

**职责**：
- 初始化 Tauri + React 项目
- 创建完整的目录结构
- 生成 `feature_list.json`（基于 PRD）
- 生成 `init.sh` 开发脚本
- 创建初始 git commit

**输出文件**：
```
mcp-inspector-desktop/
├── feature_list.json      # 功能检查清单
├── claude-progress.txt    # 进度日志
├── init.sh                # 开发环境启动
└── .claude/
    ├── initializer-agent.md    # 初始化代理提示词
    └── coding-agent.md         # 编码代理提示词
```

### 2. Coding Agent（编码代理）

**每次会话流程**：
```
1. pwd - 确认工作目录
2. cat claude-progress.txt - 了解上次进度
3. git log --oneline -10 - 查看最近提交
4. cat feature_list.json - 找到下一个未完成功能
5. npm run dev - 启动开发服务器
6. 测试基础功能 - 确保环境正常
7. 实现单个功能
8. 测试功能
9. git commit -m "feat: xxx"
10. 更新 claude-progress.txt
11. 更新 feature_list.json 中对应功能的 passes: true
```

## 功能列表结构

基于 PRD 的 Milestone 规划：

### MVP (可运行版本)
- [ ] F-001: Tauri 项目初始化
- [ ] F-002: 基础目录结构搭建
- [ ] F-003: React + TypeScript 配置
- [ ] F-004: TailwindCSS + shadcn/ui 安装
- [ ] F-005: Inspector npm 包安装
- [ ] F-006: Launcher UI 基础布局
- [ ] F-007: 服务器路径选择功能
- [ ] F-008: 环境变量编辑器
- [ ] F-009: Inspector 进程启动/停止
- [ ] F-010: Webview 嵌入 Inspector UI

### 稳定性 (可用版本)
- [ ] F-101: 配置持久化 (JSON)
- [ ] F-102: 最近使用 Profile 列表
- [ ] F-103: 实时日志面板
- [ ] F-104: 端口冲突处理
- [ ] F-105: 错误处理与 Toast 提示
- [ ] F-106: 进程状态监控

### 生产就绪
- [ ] F-201: 系统托盘集成
- [ ] F-202: 自动更新机制
- [ ] F-203: CI/CD 配置
- [ ] F-204: 代码签名
- [ ] F-205: 配置导入导出
