use super::{InspectorError, Result};
use portpicker::pick_unused_port;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

/// Inspector 进程句柄
pub struct InspectorHandle {
    child: Child,
    client_port: u16,
    server_port: u16,
    session_id: String,
}

impl InspectorHandle {
    /// 启动一个新的 Inspector 进程
    pub fn spawn(
        server_cmd: String,
        working_dir: PathBuf,
        env_vars: HashMap<String, String>,
    ) -> Result<Self> {
        // 1. 分配端口 (Client: 5174-5274, Server: 6277-6377)
        let client_port = pick_unused_port_in_range(5174, 5274)
            .ok_or(InspectorError::NoAvailablePort(5174, 5274))?;
        let server_port = pick_unused_port_in_range(6277, 6377)
            .ok_or(InspectorError::NoAvailablePort(6277, 6377))?;

        // 2. 构建 Node 命令
        let mut cmd = Command::new("node");
        cmd.current_dir(&working_dir)
            .arg("./node_modules/@modelcontextprotocol/inspector/bin/cli.js")
            .arg(&server_cmd)
            .env("CLIENT_PORT", client_port.to_string())
            .env("SERVER_PORT", server_port.to_string())
            .env("MCP_INSPECTER_OPEN", "false") // 禁止自动打开浏览器
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // 3. 注入用户环境变量
        for (k, v) in &env_vars {
            cmd.env(k, v);
        }

        // 4. 启动进程
        let child = cmd.spawn()?;

        let session_id = uuid::Uuid::new_v4().to_string();

        Ok(Self {
            child,
            client_port,
            server_port,
            session_id,
        })
    }

    /// 终止 Inspector 进程
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
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
        match self.child.try_wait() {
            Ok(Some(_status)) => false, // 进程已退出
            Ok(None) => true,           // 进程还在运行
            Err(_) => false,            // 出错，认为已退出
        }
    }
}

impl Drop for InspectorHandle {
    fn drop(&mut self) {
        // 当 InspectorHandle 被丢弃时，自动终止进程
        if let Ok(_) = self.child.try_wait() {
            // 进程还在运行，需要终止
            let _ = self.child.kill();
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
