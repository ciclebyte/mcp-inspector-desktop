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
    server_port: u16,
    session_id: String,
    auth_token: String,
    _log_thread: Option<thread::JoinHandle<()>>,
}

impl InspectorHandle {
    /// 启动一个新的 Inspector 进程
    pub fn spawn(
        window: Window,
        working_dir: PathBuf,
        env_vars: HashMap<String, String>,
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

        // 2. 使用 mcp-inspector.cmd（Windows）
        let mut cmd = Command::new("mcp-inspector.cmd");
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
            let mut auth_token = String::new();

            // 读取 stdout，捕获认证令牌
            let stdout_reader = BufReader::new(stdout);
            for line in stdout_reader.lines() {
                if let Ok(text) = line {
                    // 检查是否是认证令牌行
                    if text.contains("Session token:") {
                        if let Some(token_part) = text.split("Session token:").nth(1) {
                            auth_token = token_part.trim().to_string();
                            // 发送完整 URL 到前端
                            let full_url = format!(
                                "http://localhost:{}?MCP_PROXY_PORT={}&MCP_PROXY_AUTH_TOKEN={}",
                                client_port_for_url, server_port_for_url, auth_token
                            );
                            let _ = window_clone_for_token.emit("inspector-log", serde_json::json!({
                                "type": "system",
                                "text": format!("捕获到完整 URL: {}", full_url),
                                "sessionId": session_id_clone
                            }));
                            let _ = window_clone_for_token.emit("inspector-url-ready", full_url);
                        }
                    }

                    let _ = window_clone_for_log.emit("inspector-log", serde_json::json!({
                        "type": "stdout",
                        "text": text,
                        "sessionId": session_id_clone
                    }));
                }
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
            server_port,
            session_id,
            auth_token: String::new(),
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

/// 在指定范围内选择未使用的端口
fn pick_unused_port_in_range(min: u16, max: u16) -> Option<u16> {
    let mut port = pick_unused_port();
    while let Some(p) = port {
        if p >= min && p <= max {
            return Some(p);
        }
        port = pick_unused_port();
    }
    None
}
