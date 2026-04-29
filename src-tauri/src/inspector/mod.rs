mod process;

pub use process::InspectorHandle;

/// 返回当前平台上 mcp-inspector CLI 的命令名
pub fn inspector_command() -> &'static str {
    if cfg!(target_os = "windows") {
        "mcp-inspector.cmd"
    } else {
        "mcp-inspector"
    }
}

/// 通过 login shell 解析命令的完整路径。
/// macOS GUI 应用不继承终端 PATH，需要通过 shell 解析。
/// nvm/fnm/volta 等工具的 PATH 配置在 .zshrc/.bashrc 中（interactive shell 才加载），
/// 因此依次尝试 login shell 和 login+interactive shell 两种模式。
pub fn resolve_command_path(cmd: &str) -> Option<String> {
    #[cfg(unix)]
    {
        let mut shells: Vec<String> = vec![
            "/bin/zsh".into(),
            "/bin/bash".into(),
            "/bin/sh".into(),
        ];
        // 将 $SHELL 插入到最前面，优先使用用户默认 shell
        if let Ok(shell) = std::env::var("SHELL") {
            shells.insert(0, shell);
        }

        // 依次尝试不同的 shell flag 组合
        // "-l -c" : login shell（加载 .zprofile/.bash_profile）
        // "-l -i -c" : login + interactive shell（额外加载 .zshrc/.bashrc）
        let flag_modes: Vec<Vec<&str>> = vec![
            vec!["-l", "-c"],
            vec!["-l", "-i", "-c"],
        ];

        let which_cmd = format!("which {}", cmd);

        for shell in &shells {
            for flags in &flag_modes {
                let mut args: Vec<&str> = flags.clone();
                args.push(&which_cmd);

                if let Ok(output) = std::process::Command::new(shell)
                    .args(&args)
                    .output()
                {
                    if output.status.success() {
                        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !path.is_empty() && !path.contains("not found") {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    // Windows 或 Unix fallback：直接尝试执行
    if std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success())
    {
        Some(cmd.to_string())
    } else {
        None
    }
}

/// Inspector 进程相关错误
#[derive(thiserror::Error, Debug)]
pub enum InspectorError {
    #[error("No available port in range {0}-{1}")]
    NoAvailablePort(u16, u16),

    #[error("Process spawn failed: {0}")]
    SpawnError(#[from] std::io::Error),
}

/// Inspector 进程结果类型
pub type Result<T> = std::result::Result<T, InspectorError>;
