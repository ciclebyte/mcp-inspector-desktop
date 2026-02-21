use crate::config::AppConfig;
use crate::inspector::InspectorHandle;
use std::sync::Mutex;

/// 应用全局状态
pub struct AppState {
    /// 当前运行的 Inspector 进程句柄
    pub inspector: Mutex<Option<InspectorHandle>>,
    /// 应用配置
    pub config: Mutex<AppConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inspector: Mutex::new(None),
            config: Mutex::new(AppConfig::load()),
        }
    }
}
