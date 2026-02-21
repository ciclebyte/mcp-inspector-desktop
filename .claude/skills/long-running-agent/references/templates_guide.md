# 模板使用指南

本指南说明如何使用长时间运行代理 skill 中的模板文件。

## 文件结构

```
long-running-agent/
├── SKILL.md                           # Skill 主文档
├── scripts/
│   └── init_template.sh              # 初始化脚本模板
└── references/
    ├── feature_list.json             # 功能列表模板
    ├── progress_template.txt         # 进度文件模板
    ├── session_workflow.md           # 会话工作流详细指南
    └── templates_guide.md            # 本文件
```

## 快速开始

### 1. 初始化新项目

创建以下文件在项目根目录：

```bash
# 1. 复制功能列表模板
cp references/feature_list.json ./features.json

# 2. 复制进度文件模板
cp references/progress_template.txt ./claude-progress.txt

# 3. 复制并修改初始化脚本
cp scripts/init_template.sh ./init.sh
chmod +x init.sh

# 4. 初始化 Git 仓库
git init
git add .
git commit -m "初始化项目结构"
```

### 2. 自定义功能列表

编辑 `features.json`，添加你的项目功能：

```json
{
  "project_name": "我的待办事项应用",
  "created_date": "2024-01-15",
  "features": [
    {
      "id": 1,
      "category": "functional",
      "priority": "high",
      "description": "用户可以创建新的待办事项",
      "steps": [
        "打开应用",
        "点击'添加'按钮",
        "输入待办事项内容",
        "按回车或点击确认",
        "验证待办事项出现在列表中"
      ],
      "passes": false,
      "notes": ""
    }
    // 添加更多功能...
  ]
}
```

**功能类别建议**：
- `functional` - 核心功能
- `ui` - 用户界面
- `performance` - 性能指标
- `security` - 安全相关
- `accessibility` - 可访问性

### 3. 自定义初始化脚本

编辑 `init.sh`，修改配置区域：

```bash
# 配置区域
PROJECT_TYPE="nodejs"    # 或 python, go, rust
DEV_PORT=3000            # 开发服务器端口
INSTALL_DEPS=true        # 是否安装依赖
START_CMD="npm run dev"  # 启动命令
HEALTH_CHECK_URL="http://localhost:${DEV_PORT}"
```

### 4. 开始第一次会话

按照以下顺序执行：

```bash
# 1. 确认目录
pwd

# 2. 阅读 init.sh
cat init.sh

# 3. 阅读功能列表
cat features.json

# 4. 启动服务器
./init.sh

# 5. 开始实现第一个功能
```

## 功能列表最佳实践

### 细粒度分解

**太粗略**（不好）：
```json
{
  "description": "实现用户管理"
}
```

**太细碎**（也不好）：
```json
{
  "description": "添加用户名字段的第一个输入框的样式"
}
```

**刚刚好**（推荐）：
```json
{
  "description": "用户可以注册新账户",
  "steps": [
    "导航到注册页面",
    "输入用户名、邮箱和密码",
    "点击注册按钮",
    "验证账户创建成功",
    "验证自动登录并跳转到主页"
  ]
}
```

### 功能描述模板

```json
{
  "id": 1,
  "category": "functional",  // 或 ui, performance, security
  "priority": "high",        // high, medium, low
  "description": "用户可以[动作]，以实现[目标]",
  "steps": [
    "前置条件或初始状态",
    "具体操作步骤 1",
    "具体操作步骤 2",
    "验证预期结果"
  ],
  "passes": false,
  "notes": "任何额外的说明或注意事项"
}
```

### 功能依赖关系

如果功能有依赖，在 notes 中说明：

```json
{
  "id": 5,
  "description": "用户可以删除待办事项",
  "steps": [...],
  "passes": false,
  "notes": "依赖于功能 #1 (创建待办事项)"
}
```

## 进度文件最佳实践

### 每次会话更新模板

```bash
cat >> claude-progress.txt << 'EOF'

### $(date +%Y-%m-%d) 会话 #N

**工作内容**：
- 简洁描述本次会话的核心工作

**已完成功能**：
- features.json #5 - 功能名称
- features.json #6 - 功能名称

**Git 提交**：
- `abc1234` - 简短的提交描述

**遇到的问题**：
- 描述遇到的问题
- 说明如何解决的

**下一步计划**：
- 下次会话应该做什么

EOF
```

### 统计信息维护

定期更新统计表格：

```
## 统计信息

| 指标 | 数值 |
|------|------|
| 总功能数 | 25 |
| 已完成 | 8 |
| 进行中 | 2 |
| 待开始 | 15 |
| 完成率 | 32% |
```

## 初始化脚本自定义

### Node.js 项目

```bash
PROJECT_TYPE="nodejs"
START_CMD="npm run dev"
```

### Python 项目

```bash
PROJECT_TYPE="python"
# 如果是 Flask
START_CMD="flask run --port=5000"
# 如果是 FastAPI
START_CMD="uvicorn main:app --reload"
```

### 多进程项目

如果需要同时启动前端和后端：

```bash
# 在 package.json 中
{
  "scripts": {
    "dev:frontend": "cd frontend && npm run dev",
    "dev:backend": "cd backend && npm run dev",
    "dev:all": "concurrently \"npm run dev:frontend\" \"npm run dev:backend\""
  }
}

# 在 init.sh 中
START_CMD="npm run dev:all"
```

## 常见问题

### Q: 功能列表应该有多少项？

A: 取决于项目规模：
- 小型项目（1-2 天）：10-30 项
- 中型项目（1 周）：50-100 项
- 大型项目（1 月+）：100+ 项

### Q: 如何处理不确定的功能？

A: 有两种方式：
1. 先不加入列表，等项目更清晰时再添加
2. 加入列表但在 notes 中标记"待明确"

### Q: 进度文件会变得很大吗？

A: 会，但这是正常的。每次会话只添加新的部分，不修改历史。文件大小通常不会成为问题。

### Q: init.sh 执行失败怎么办？

A: 检查：
1. 脚本有执行权限 (`chmod +x init.sh`)
2. 配置区域的设置正确
3. 依赖已安装（如 node, python 等）
4. 查看错误信息并相应调试

## 示例项目

### 示例 1：简单 Web 应用

```
my-todo-app/
├── features.json        # 15 个功能
├── claude-progress.txt  # 进度跟踪
├── init.sh              # 启动脚本
├── package.json
└── src/
```

### 示例 2：全栈应用

```
my-fullstack-app/
├── features.json           # 50 个功能（前端 + 后端）
├── claude-progress.txt     # 进度跟踪
├── init.sh                 # 启动前后端
├── frontend/
│   └── package.json
└── backend/
    └── package.json
```

## 下一步

1. 复制模板到你的项目
2. 自定义功能列表
3. 开始第一次会话
4. 按照 session_workflow.md 的流程工作

祝你的项目开发顺利！
