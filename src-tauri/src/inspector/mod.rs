mod process;

pub use process::InspectorHandle;

/// Inspector 进程相关错误
#[derive(thiserror::Error, Debug)]
pub enum InspectorError {
    #[error("Node.js runtime not found in PATH")]
    NodeNotFound,

    #[error("No available port in range {0}-{1}")]
    NoAvailablePort(u16, u16),

    #[error("Process spawn failed: {0}")]
    SpawnError(#[from] std::io::Error),

    #[error("Inspector exited prematurely with code {0:?}")]
    PrematureExit(Option<i32>),
}

/// Inspector 进程结果类型
pub type Result<T> = std::result::Result<T, InspectorError>;
