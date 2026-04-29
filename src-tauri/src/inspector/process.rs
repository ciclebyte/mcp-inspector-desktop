use super::{InspectorError, Result};
use portpicker::pick_unused_port;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use tauri::{Emitter, Window};

// Windows 平台特定配置：隐藏子进程控制台窗口
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Inspector 进程句柄
pub struct InspectorHandle {
    child: Option<Child>,
    client_port: u16,
    _server_port: u16,
    _session_id: String,
    _log_thread: Option<thread::JoinHandle<()>>,
}

impl InspectorHandle {
    /// 启动一个新的 Inspector 进程
    pub fn spawn(
        window: Window,
        working_dir: PathBuf,
        env_vars: HashMap<String, String>,
        inspector_path: String,
    ) -> Result<Self> {
        let _ = window.emit("inspector-log", serde_json::json!({
            "type": "system",
            "text": "正在分配端口...",
            "sessionId": ""
        }));

        // 1. 分配端口 - 直接使用 portpicker，不限制范围
        let client_port = pick_unused_port()
            .ok_or(InspectorError::NoAvailablePort(5174, 5274))?;
        let server_port = pick_unused_port()
            .ok_or(InspectorError::NoAvailablePort(6277, 6377))?;

        let _ = window.emit("inspector-log", serde_json::json!({
            "type": "system",
            "text": format!("分配端口: 客户端={}, 服务端={}", client_port, server_port),
            "sessionId": ""
        }));

        // 2. 使用 mcp-inspector CLI（使用解析到的完整路径）
        let mut cmd = Command::new(&inspector_path);

        // 将 mcp-inspector 所在目录注入 PATH，确保 shebang (#!/usr/bin/env node) 能找到 node
        // nvm/fnm/volta 等工具将 node 和全局 CLI 安装在同一 bin 目录下
        if let Some(parent) = std::path::Path::new(&inspector_path).parent() {
            if let Some(dir) = parent.to_str() {
                let existing_path = std::env::var("PATH").unwrap_or_default();
                let new_path = if existing_path.is_empty() {
                    dir.to_string()
                } else {
                    format!("{}:{}", dir, existing_path)
                };
                cmd.env("PATH", new_path);
            }
        }

        cmd.current_dir(&working_dir)
            .env("CLIENT_PORT", client_port.to_string())
            .env("SERVER_PORT", server_port.to_string())
            .env("MCP_AUTO_OPEN_ENABLED", "false") // 阻止自动打开浏览器
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Windows: 隐藏子进程的控制台窗口
        #[cfg(target_os = "windows")]
        cmd.creation_flags(CREATE_NO_WINDOW);

        let _ = window.emit("inspector-log", serde_json::json!({
            "type": "system",
            "text": "正在启动 mcp-inspector...",
            "sessionId": ""
        }));

        // 3. 注入用户环境变量
        for (k, v) in &env_vars {
            cmd.env(k, v);
        }

        // 4. 启动进程
        let mut child = cmd.spawn()?;

        let session_id = uuid::Uuid::new_v4().to_string();

        // 5. 获取 stdout 和 stderr 的所有权
        let stdout = child.stdout.take().ok_or(InspectorError::SpawnError(
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stdout")
        ))?;
        let stderr = child.stderr.take().ok_or(InspectorError::SpawnError(
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stderr")
        ))?;

        // 6. 启动日志读取线程，同时捕获认证令牌
        let window_clone_for_token = window.clone();
        let window_clone_for_log = window.clone();
        let session_id_clone = session_id.clone();
        let client_port_for_url = client_port;
        let server_port_for_url = server_port;

        let log_thread = thread::spawn(move || {
            // 读取 stdout，捕获认证令牌
            let mut pending_url: Option<String> = None;
            let stdout_reader = BufReader::new(stdout);
            for line in stdout_reader.lines() {
                if let Ok(text) = line {
                    // 检查是否是认证令牌行，构造 URL 但暂不发送
                    if text.contains("Session token:") {
                        if let Some(token_part) = text.split("Session token:").nth(1) {
                            let auth_token = token_part.trim();
                            let full_url = format!(
                                "http://localhost:{}?MCP_PROXY_PORT={}&MCP_PROXY_AUTH_TOKEN={}",
                                client_port_for_url, server_port_for_url, auth_token
                            );
                            pending_url = Some(full_url);
                            let _ = window_clone_for_token.emit("inspector-log", serde_json::json!({
                                "type": "system",
                                "text": "捕获到 Session Token，等待 Inspector 就绪...",
                                "sessionId": session_id_clone
                            }));
                        }
                    }

                    // Inspector 确认就绪后才发送 URL，避免 iframe 加载未就绪的服务
                    if pending_url.is_some() && text.contains("up and running") {
                        if let Some(url) = pending_url.take() {
                            let _ = window_clone_for_token.emit("inspector-log", serde_json::json!({
                                "type": "system",
                                "text": format!("Inspector 已就绪: {}", url),
                                "sessionId": session_id_clone
                            }));
                            let _ = window_clone_for_token.emit("inspector-url-ready", url);
                        }
                    }

                    let _ = window_clone_for_log.emit("inspector-log", serde_json::json!({
                        "type": "stdout",
                        "text": text,
                        "sessionId": session_id_clone
                    }));
                }
            }

            // 兜底：stdout 结束但 URL 未发送（Inspector 版本输出格式变化时）
            if let Some(url) = pending_url.take() {
                let _ = window_clone_for_log.emit("inspector-url-ready", url);
            }

            // 读取 stderr
            let stderr_reader = BufReader::new(stderr);
            for line in stderr_reader.lines() {
                if let Ok(text) = line {
                    let _ = window_clone_for_log.emit("inspector-log", serde_json::json!({
                        "type": "stderr",
                        "text": text,
                        "sessionId": session_id_clone
                    }));
                }
            }

            // 进程结束时发送事件
            let _ = window_clone_for_log.emit("inspector-exited", session_id_clone);
        });

        Ok(Self {
            child: Some(child),
            client_port,
            _server_port: server_port,
            _session_id: session_id,
            _log_thread: Some(log_thread),
        })
    }

    /// 终止 Inspector 进程
    pub fn kill(&mut self) -> std::io::Result<()> {
        if let Some(ref mut child) = self.child {
            child.kill()
        } else {
            Ok(())
        }
    }

    /// 获取客户端 URL
    pub fn client_url(&self) -> String {
        format!("http://localhost:{}", self.client_port)
    }

    /// 获取客户端端口
    pub fn client_port(&self) -> u16 {
        self.client_port
    }

    /// 检查进程是否还在运行
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(Some(_status)) => false, // 进程已退出
                Ok(None) => true,           // 进程还在运行
                Err(_) => false,            // 出错，认为已退出
            }
        } else {
            false
        }
    }
}

impl Drop for InspectorHandle {
    fn drop(&mut self) {
        // 当 InspectorHandle 被丢弃时，自动终止进程
        if let Some(ref mut child) = self.child {
            if let Ok(_) = child.try_wait() {
                // 进程还在运行，需要终止
                let _ = child.kill();
            }
        }
    }
}
