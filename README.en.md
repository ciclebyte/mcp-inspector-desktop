# MCP Inspector Desktop

> A native desktop wrapper for [MCP Inspector](https://github.com/modelcontextprotocol/inspector) — debug MCP Servers without leaving your desktop.

[![Release](https://github.com/cicbyte/mcp-inspector-desktop/actions/workflows/release.yml/badge.svg)](https://github.com/cicbyte/mcp-inspector-desktop/actions/workflows/release.yml)
![Tauri v2](https://img.shields.io/badge/Tauri-2.0-24C8D8?logo=tauri)
![React 18](https://img.shields.io/badge/React-18.3-61DAFB?logo=react)
![Rust](https://img.shields.io/badge/Rust-1.70+-DEA584?logo=rust)
![MIT](https://img.shields.io/badge/license-MIT-4C8?logo=opensourceinitiative)

**中文** | [English](README.en.md)

![Demo](images/running.gif)

## Features

- **Native Desktop Experience** — Embeds MCP Inspector in a desktop window, no browser tabs needed
- **One-Click Start/Stop** — Clean UI to control the Inspector process lifecycle with clear status
- **Real-time Log Panel** — Collapsible bottom panel with color-coded output (system, stdout, stderr)
- **Automatic Port Allocation** — Smart port selection to avoid conflicts
- **Auto Token Capture** — Parses CLI output to extract Session Token and build the complete URL
- **Cross-Platform** — Windows, macOS (Intel & Apple Silicon), and Linux

## Screenshots

**Launcher**

![Launcher](images/001.png)

**Inspector Running**

![Inspector Running](images/002.png)

## Installation

### Download from Releases

Download the installer for your platform from the [Releases](https://github.com/cicbyte/mcp-inspector-desktop/releases) page:

| Platform | Formats |
|----------|---------|
| Windows | `.exe` / `.msi` |
| macOS | `.dmg` (Universal Binary) |
| Linux | `.AppImage` / `.deb` / `.rpm` |

### Prerequisites

- **Node.js** v18+
- **Rust** 1.70+
- **System dependencies**:
  - Windows: [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
  - Linux: `libwebkit2gtk-4.1-dev`
  - macOS: None

### MCP Inspector CLI

The app requires the MCP Inspector CLI installed globally:

```bash
npm install -g @modelcontextprotocol/inspector
```

The app auto-detects the installation and prompts with the install command if missing.

## Quick Start

```bash
git clone https://github.com/cicbyte/mcp-inspector-desktop.git
cd mcp-inspector-desktop
npm install
npm run tauri dev
```

1. Click the **"Start Inspector"** button
2. The bottom log panel shows startup progress
3. The Inspector UI is automatically embedded in the app window

## Usage

### Stopping Inspector

Click the **"Stop"** button to terminate the Inspector process. You can restart it afterward.

### Viewing Logs

- **Launcher page**: Log panel is expanded by default
- **Running page**: Click the **"Show Logs"** button to expand

Log color coding:

| Color | Meaning |
|-------|---------|
| Blue | System messages |
| Gray | Standard output |
| Red | Error output |

## How It Works

1. **Process Management** — Rust backend spawns the `mcp-inspector` CLI via `std::process::Command`
2. **Log Capture** — Dedicated thread reads stdout/stderr in real-time, forwards via Tauri Events
3. **Token Capture** — Parses `Session token:` lines from stdout to extract the auth token
4. **Port Allocation** — Uses `portpicker` to automatically select available ports
5. **Browser Prevention** — Sets `MCP_AUTO_OPEN_ENABLED=false` to prevent auto-opening in browser

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | React 18 + TypeScript |
| Build Tool | Vite 6 |
| Styling | Tailwind CSS 3.4 |
| Desktop Framework | Tauri v2 |
| Backend Language | Rust (Edition 2021) |
| IPC | Tauri Commands / Events |
| Port Selection | portpicker |

## Development

```bash
# Dev mode (frontend hot reload + Rust auto recompile)
npm run tauri dev

# Frontend only
npm run dev

# Production build
npm run tauri build
```

### Project Structure

```
mcp-inspector-desktop/
├── src/                      # React frontend
│   ├── components/
│   │   ├── Launcher.tsx      # Launcher page (start button + log panel)
│   │   └── InspectorView.tsx # Runtime view (iframe + collapsible logs)
│   ├── App.tsx               # Root component, view switching
│   ├── lib/utils.ts          # cn() utility
│   └── styles/globals.css    # Tailwind + CSS variables theme
├── src-tauri/                # Rust backend
│   ├── src/
│   │   ├── main.rs           # Tauri Builder config
│   │   ├── commands.rs       # Tauri Command definitions
│   │   ├── state.rs          # Global state (Mutex)
│   │   ├── inspector/
│   │   │   ├── mod.rs        # Error types + cross-platform command resolution
│   │   │   └── process.rs    # Child process management
│   │   └── config/
│   │       └── storage.rs    # Config persistence
│   └── tauri.conf.json       # Tauri app configuration
└── package.json
```

## FAQ

### macOS says "cannot be opened because the developer cannot be verified"

The app is not notarized by Apple. Gatekeeper blocks it on first launch.

**Option 1: Via System Settings (Recommended)**

1. Right-click the app and select "Open"
2. Click "Open" in the dialog
3. If that doesn't work: go to System Settings → Privacy & Security → click "Open Anyway"

**Option 2: Remove quarantine attribute via Terminal**

```bash
xattr -cr /Applications/MCP\ Inspector\ Desktop.app
```

### Inspector fails to start

**Problem**: Clicking the start button shows "mcp-inspector not detected"

**Solution**:

```bash
npm install -g @modelcontextprotocol/inspector
```

### Inspector opens in browser instead of embedding

**Problem**: Inspector opens in the browser rather than the app window

**Solution**: Make sure `MCP_AUTO_OPEN_ENABLED=false` is set (built-in by default)

### Port conflicts

The app automatically selects available ports. If issues persist, check your firewall settings.

## License

[MIT](LICENSE)

## Acknowledgements

- [MCP Inspector](https://github.com/modelcontextprotocol/inspector) — Original CLI tool
- [Tauri](https://tauri.app/) — Cross-platform desktop app framework
- [React](https://react.dev/) — UI framework
- [Tailwind CSS](https://tailwindcss.com/) — CSS framework
