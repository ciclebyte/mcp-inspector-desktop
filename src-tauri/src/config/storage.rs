use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// 服务器配置文件
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerProfile {
    pub id: String,
    pub name: String,
    pub command: String,
    pub working_directory: PathBuf,
    pub env_vars: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

/// 应用设置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub theme: String,
    pub auto_start: bool,
}

/// 应用配置
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub version: String,
    pub recent_profiles: Vec<ServerProfile>,
    pub default_env_vars: HashMap<String, String>,
    pub settings: AppSettings,
}

impl AppConfig {
    /// 获取配置目录
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .expect("Failed to get config dir")
            .join("mcp-inspector-desktop")
    }

    /// 获取配置文件路径
    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.json")
    }

    /// 加载配置
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = std::fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// 保存配置
    pub fn save(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(Self::config_dir())?;
        let content = serde_json::to_string_pretty(self)?;

        // 原子写入：先写临时文件，再重命名
        let temp_path = Self::config_path().with_extension("tmp");
        std::fs::write(&temp_path, content)?;
        std::fs::rename(temp_path, Self::config_path())?;

        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            recent_profiles: Vec::new(),
            default_env_vars: HashMap::new(),
            settings: AppSettings::default(),
        }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            auto_start: false,
        }
    }
}
