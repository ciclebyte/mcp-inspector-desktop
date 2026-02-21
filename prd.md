**mcp-inspector-desktop**  
**需求规格说明书 (PRD) - Tauri/Rust 实现版**

**版本**: v1.0  
**日期**: 2026-02-21  
**技术栈**: Tauri (Rust) + TypeScript + Sidecar (Node)  
**仓库**: `https://github.com/ciclebyte/mcp-inspector-desktop`
---

## 1. 项目概述

### 1.1 项目定位
**mcp-inspector-desktop** 是 MCP (Model Context Protocol) Inspector 的桌面端封装应用。通过 Tauri (Rust) 构建，将命令行工具 `@modelcontextprotocol/inspector` 转化为零配置的图形化调试环境，解决开发者记忆命令参数困难、进程管理繁琐、浏览器标签混乱等痛点。

### 1.2 核心架构
```text
mcp-inspector-desktop/
├── src-tauri/                     # Rust 后端核心
│   ├── src/
│   │   ├── main.rs               # 应用入口与 State 管理
│   │   ├── inspector/            # Inspector 进程管理模块
│   │   │   ├── mod.rs           # InspectorHandle 结构体
│   │   │   └── process.rs       # 子进程生命周期管理
│   │   ├── config/              # 配置持久化
│   │   │   ├── mod.rs           # AppConfig 与 Profile 管理
│   │   │   └── storage.rs       # 文件系统操作 (JSON/SQLite)
│   │   └── commands.rs          # Tauri Command 暴露接口
│   ├── Cargo.toml              # Rust 依赖 (tokio, serde, portpicker)
│   └── tauri.conf.json         # 打包配置与权限声明
├── src/                          # 前端 (TypeScript)
│   ├── components/              # UI 组件 (启动页、日志面板)
│   ├── api/                     # Rust 调用封装 (invoke)
│   └── styles/                  # TailwindCSS / 原生样式
├── node_modules/                # @modelcontextprotocol/inspector 安装位置
└── sidecar/                     # [可选] 捆绑的 Node 二进制文件
```

---

## 2. 技术架构详设 (Rust 专项)

### 2.1 后端架构 (src-tauri)

#### 核心状态管理
使用 Tauri 的 State 管理系统实现跨请求状态共享：

```rust
// src-tauri/src/state.rs
use std::sync::Mutex;
use crate::inspector::InspectorHandle;

pub struct AppState {
    /// 当前运行的 Inspector 进程句柄
    pub inspector: Mutex<Option<InspectorHandle>>,
    /// 应用配置（最近使用、默认环境变量）
    pub config: Mutex<AppConfig>,
    /// 日志通道发送端
    pub log_tx: Mutex<Option<mpsc::Sender<String>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inspector: Mutex::new(None),
            config: Mutex::new(AppConfig::load()),
            log_tx: Mutex::new(None),
        }
    }
}
```

#### Inspector 进程管理 (src-tauri/src/inspector/mod.rs)
```rust
use std::process::{Child, Command, Stdio};
use std::collections::HashMap;
use std::path::PathBuf;
use portpicker::pick_unused_port;
use thiserror::Error;

pub struct InspectorHandle {
    child: Child,
    client_port: u16,
    server_port: u16,
    session_id: String,
}

#[derive(Error, Debug)]
pub enum InspectorError {
    #[error("Node.js runtime not found in PATH")]
    NodeNotFound,
    #[error("No available port in range {0}-{1}")]
    NoAvailablePort(u16, u16),
    #[error("Process spawn failed: {0}")]
    SpawnError(#[from] std::io::Error),
    #[error("Inspector exited prematurely with code {0}")]
    PrematureExit(Option<i32>),
}

impl InspectorHandle {
    pub fn spawn(
        server_cmd: String,
        working_dir: PathBuf,
        env_vars: HashMap<String, String>,
    ) -> Result<Self, InspectorError> {
        // 1. 端口分配 (Client: 5174+, Server: 6277+)
        let client_port = pick_unused_port_in_range(5174..=5274)
            .ok_or(InspectorError::NoAvailablePort(5174, 5274))?;
        let server_port = pick_unused_port_in_range(6277..=6377)
            .ok_or(InspectorError::NoAvailablePort(6277, 6377))?;

        // 2. 构建 Node 命令
        let mut cmd = Command::new("node");
        cmd.current_dir(working_dir)
           .arg("./node_modules/@modelcontextprotocol/inspector/bin/cli.js")
           .arg(&server_cmd)
           .env("CLIENT_PORT", client_port.to_string())
           .env("SERVER_PORT", server_port.to_string())
           .env("MCP_INSPECTER_OPEN", "false")  // 禁止自动打开浏览器
           .stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .kill_on_drop(true);  // Rust 1.68+ 特性：所有者 drop 时自动 kill

        // 3. 注入用户环境变量
        for (k, v) in env_vars {
            cmd.env(k, v);
        }

        // 4. 启动进程
        let child = cmd.spawn()?;
        
        Ok(Self {
            child,
            client_port,
            server_port,
            session_id: uuid::Uuid::new_v4().to_string(),
        })
    }

    pub fn kill(&mut self) -> Result<(), std::io::Error> {
        self.child.kill()
    }

    pub fn client_url(&self) -> String {
        format!("http://localhost:{}", self.client_port)
    }
}
```

#### 配置持久化 (src-tauri/src/config/mod.rs)
使用 `dirs` crate 实现跨平台配置目录管理：

```rust
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerProfile {
    pub id: String,
    pub name: String,
    pub command: String,           // 例如: "node ./dist/index.js"
    pub working_directory: PathBuf,
    pub env_vars: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    pub version: String,
    pub recent_profiles: Vec<ServerProfile>,
    pub default_env_vars: HashMap<String, String>,
    pub settings: AppSettings,
}

impl AppConfig {
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .expect("Failed to get config dir")
            .join("mcp-inspector-desktop")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(Self::config_dir())?;
        let content = serde_json::to_string_pretty(self)?;
        // 原子写入防止配置损坏
        let temp_path = Self::config_path().with_extension("tmp");
        std::fs::write(&temp_path, content)?;
        std::fs::rename(temp_path, Self::config_path())?;
        Ok(())
    }
}
```

### 2.2 Tauri Command 接口 (src-tauri/src/commands.rs)
```rust
use tauri::{State, Window, command};
use crate::state::AppState;
use crate::inspector::{InspectorHandle, InspectorError};

#[command]
pub async fn start_inspector(
    window: Window,
    state: State<'_, AppState>,
    command: String,
    working_dir: String,
    env_vars: HashMap<String, String>,
) -> Result<u16, String> {
    // 检查是否已有运行实例
    let mut inspector_guard = state.inspector.lock().map_err(|e| e.to_string())?;
    if inspector_guard.is_some() {
        return Err("Inspector already running. Please stop current instance first.".to_string());
    }

    let handle = InspectorHandle::spawn(
        command,
        PathBuf::from(working_dir),
        env_vars,
    ).map_err(|e| e.to_string())?;

    let port = handle.client_port;
    
    // 存储句柄
    *inspector_guard = Some(handle);
    drop(inspector_guard);  // 显式释放锁

    // 启动日志转发线程
    let window_clone = window.clone();
    tokio::spawn(async move {
        // 日志转发逻辑...
        window_clone.emit("inspector-ready", port).unwrap();
    });

    Ok(port)
}

#[command]
pub fn stop_inspector(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state.inspector.lock().map_err(|e| e.to_string())?;
    if let Some(mut handle) = guard.take() {
        handle.kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
pub fn get_recent_profiles(state: State<'_, AppState>) -> Vec<ServerProfile> {
    let config = state.config.lock().unwrap();
    config.recent_profiles.clone()
}
```

### 2.3 前端架构 (src/)

**技术栈**:
- **构建工具**: Vite (与 Tauri 集成)
- **语言**: TypeScript (严格模式)
- **UI 框架**: 轻量级方案二选一:
  - **方案 A**: 原生 Web Components (零依赖，最小体积)
  - **方案 B**: Svelte (编译时优化，与 Rust 哲学契合)
- **通信**: `@tauri-apps/api` (invoke/listen)

**核心 API 封装 (src/lib/api.ts)**:
```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface Profile {
  id: string;
  name: string;
  command: string;
  workingDirectory: string;
  envVars: Record<string, string>;
}

export const startInspector = async (config: {
  command: string;
  workingDir: string;
  envVars: Record<string, string>;
}): Promise<number> => {
  return await invoke('start_inspector', {
    command: config.command,
    workingDir: config.workingDir,
    envVars: config.envVars,
  });
};

export const onInspectorReady = (callback: (port: number) => void) => {
  return listen<number>('inspector-ready', (event) => {
    callback(event.payload);
  });
};

export const onLogOutput = (callback: (line: string) => void) => {
  return listen<string>('log-output', (event) => {
    callback(event.payload);
  });
};
```

---

## 3. 功能需求

### 3.1 启动配置界面 (Launcher View)

| ID | 功能 | 技术实现 | 优先级 |
|----|------|----------|--------|
| F-001 | **Server 路径选择** | `<input type="file">` + Tauri `open` dialog API，过滤 `.js/.py/.ts` | P0 |
| F-002 | **运行时检测** | Rust 端 `which node/python`，前端显示检测状态图标 | P1 |
| F-003 | **环境变量编辑器** | 动态表单 (Key-Value 对)，支持导入 `.env` 文件 (Rust 端解析) | P0 |
| F-004 | **最近使用列表** | 从 `AppConfig::recent_profiles` 加载，侧边栏展示 | P1 |
| F-005 | **一键启动** | 调用 `start_inspector` Command，显示加载动画 (Rust 返回前阻塞) | P0 |
| F-006 | **端口状态检测** | 启动前 Rust 端 `TcpListener::bind` 测试 5174/6277 占用情况 | P1 |

### 3.2 主工作区 (Inspector Container)

| ID | 功能 | 技术实现 | 优先级 |
|----|------|----------|--------|
| F-101 | **Webview 嵌入** | Tauri `WebviewWindow` 或 `<iframe>` (需处理 localhost CORS) | P0 |
| F-102 | **进程状态栏** | Rust 每 5s 检测 `Child` 状态，前端显示 ● 运行中 / ○ 已停止 | P1 |
| F-103 | **实时日志面板** | 底部可折叠面板，Rust `BufReader` 读取 stdout 通过 Event 推送 | P1 |
| F-104 | **停止/重启按钮** | 调用 `stop_inspector` 后回到 Launcher，或保持配置直接重启 | P0 |
| F-105 | **开发者工具** | 菜单栏 `Ctrl+Shift+I` 打开 Tauri DevTools (调试 Webview) | P2 |

### 3.3 系统级功能

| ID | 功能 | Rust 实现细节 | 优先级 |
|----|------|---------------|--------|
| F-201 | **托盘驻留** | `tauri::SystemTray` 支持，最小化到托盘而非关闭 | P2 |
| F-202 | **进程保活** | 应用崩溃时 `Drop` trait 确保 `kill -9` 子进程 | P0 |
| F-203 | **自动更新** | Tauri Updater Plugin (检查 GitHub Releases 新版本) | P2 |
| F-204 | **配置导入导出** | Rust `fs` 模块操作 JSON，支持分享 Profile 配置 | P2 |

---

## 4. 非功能需求

### 4.1 性能与资源
- **启动时间**: 冷启动 < 2 秒 (Rust 端)，Webview 渲染 < 1 秒
- **内存占用**: 运行时 < 150MB (含 Node 子进程)，纯 Rust 部分 < 50MB
- **磁盘空间**: 安装包 < 10MB (不含捆绑 Node)；完整版 < 40MB (含 Node)
- **并发**: 单实例设计，禁止同时启动多个 Inspector 进程 (避免端口冲突)

### 4.2 可靠性 (Rust 特有优势)
- **崩溃安全**: 使用 `std::panic::catch_unwind` 捕获 panic，确保子进程清理
- **端口泄漏防护**: 应用启动时扫描并 kill 残留的 `node inspector` 进程 (通过进程名匹配)
- **原子配置写入**: 使用 `tempfile + rename` 模式防止配置损坏
- **日志持久化**: 写入 `%APPDATA%/mcp-inspector-desktop/logs/YYYY-MM-DD-HH-mm-ss.log`

### 4.3 跨平台兼容

| 平台 | 构建目标 | 特殊处理 |
|------|----------|----------|
| Windows | `x86_64-pc-windows-msvc` | 使用 `taskkill /F /T /IM node.exe` 清理僵尸进程 |
| macOS | `x86_64-apple-darwin`, `aarch64-apple-darwin` | 签名与 Notarization，配置文件在 `~/Library/Application Support` |
| Linux | `x86_64-unknown-linux-gnu` | AppImage 格式，依赖系统 Node 或捆绑 musl 静态链接 Node |

---

## 5. 构建与分发

### 5.1 开发工作流

```bash
# 1. 克隆仓库
git clone https://github.com/[org]/mcp-inspector-desktop.git
cd mcp-inspector-desktop

# 2. 安装依赖
cargo install tauri-cli
npm install  # 安装 @modelcontextprotocol/inspector

# 3. 开发模式 (热重载)
cargo tauri dev

# 4. 测试
cargo test --lib  # Rust 单元测试
cargo test --test integration  # 集成测试 (模拟启动 Inspector)

# 5. 生产构建
cargo tauri build --target x86_64-pc-windows-msvc
```

### 5.2 CI/CD 配置 (GitHub Actions)

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags: ['v*']

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-pc-windows-msvc
            os: windows-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: 20
      
      - name: Install dependencies
        run: npm ci
      
      - name: Build Tauri
        run: cargo tauri build --target ${{ matrix.target }}
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: mcp-inspector-desktop-${{ matrix.target }}
          path: |
            src-tauri/target/${{ matrix.target }}/release/bundle/**/*.msi
            src-tauri/target/${{ matrix.target }}/release/bundle/**/*.dmg
            src-tauri/target/${{ matrix.target }}/release/bundle/**/*.AppImage
```

### 5.3 发布策略

**双版本策略**:
1. **轻量版 (Lite)**: 依赖系统 Node.js (>= 18)，体积 ~8MB
   - 首次启动检测 `node --version`，未安装引导至官网下载
   
2. **完整版 (Full)**: 捆绑 Node 二进制 (`sidecar/node`)，体积 ~35MB
   - 使用 Tauri `externalBin` 配置，自动选择对应平台 Node

---

## 6. 里程碑规划

### Milestone 1: Core (Week 1-2)
- [x] Rust 项目结构搭建 (`cargo tauri init`)
- [x] `InspectorHandle` 进程管理实现
- [x] 基础 Launcher UI (HTML + 原生 JS)
- [x] Webview 嵌入 Inspector (localhost:5174)

**交付物**: 可本地运行的 MVP，支持启动/停止单个 Server

### Milestone 2: Stability (Week 3-4)
- [ ] `AppConfig` 持久化 (JSON + `dirs`)
- [ ] 最近使用 Profile 列表
- [ ] 日志实时展示面板 (Rust stdout 捕获)
- [ ] 端口冲突自动处理 (`portpicker`)
- [ ] 错误处理统一化 (Toast 提示)

**交付物**: 可用的 Alpha 版本，支持配置记忆

### Milestone 3: Production (Week 5-6)
- [ ] 系统托盘集成
- [ ] CI/CD 自动化构建 (GitHub Actions)
- [ ] 代码签名 (Windows EV / macOS Notarization)
- [ ] 自动更新机制
- [ ] 文档与示例

**交付物**: v1.0 正式版，GitHub Release 分发

---

## 7. 风险与缓解

| 风险 | 可能性 | 影响 | 缓解措施 (Rust 方案) |
|------|--------|------|---------------------|
| **Node 运行时缺失** | 高 | 高 | 启动时检测 `which node`，提供下载链接；提供 Full 版捆绑 Node |
| **Inspector 版本不兼容** | 中 | 中 | 锁定 `package.json` 版本；定期测试最新版 Inspector；支持自动更新 |
| **进程残留** | 中 | 高 | Rust `Drop` trait + `kill_on_drop` + 应用启动时扫描残留 Node 进程并清理 |
| **Webview 兼容性问题** | 低 | 中 | 使用 Tauri 的 WebView2 (Win) 和 WKWebView (Mac)，避免边缘 CSS 特性 |
| **配置损坏** | 低 | 中 | 原子写入 (write to temp then rename)；启动时验证 JSON Schema，损坏时自动重置 |

---

## 8. 附录

### 8.1 关键依赖版本 (Cargo.toml)
```toml
[dependencies]
tauri = { version = "1.6", features = ["shell-open", "process-command-api", "system-tray"] }
tokio = { version = "1.35", features = ["process", "sync", "time"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
dirs = "5.0"
portpicker = "0.1"
uuid = { version = "1.6", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"
```

### 8.2 目录规范
- **配置**: `%APPDATA%/mcp-inspector-desktop/config.json`
- **日志**: `%APPDATA%/mcp-inspector-desktop/logs/`
- **缓存**: `%LOCALAPPDATA%/mcp-inspector-desktop/cache/`

### 8.3 相关链接
- MCP Inspector: https://github.com/modelcontextprotocol/inspector
- Tauri Docs: https://tauri.app/v1/guides/
- 本仓库: https://github.com/[org]/mcp-inspector-desktop

---

**文档维护**: 本 PRD 随 `mcp-inspector-desktop` 仓库迭代更新，版本号与 Git Tag 同步。
