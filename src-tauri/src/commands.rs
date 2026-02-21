use crate::config::ServerProfile;
use crate::inspector::InspectorHandle;
use crate::state::AppState;
use chrono::Utc;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use tauri::{Emitter, State, Window};

/// 启动 Inspector 进程
#[tauri::command]
pub async fn start_inspector(
    window: Window,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 发送启动日志
    let _ = window.emit("inspector-log", serde_json::json!({
        "type": "system",
        "text": "开始启动 Inspector 进程...",
        "sessionId": ""
    }));

    // 检查 mcp-inspector 是否可用
    let check_result = Command::new("mcp-inspector.cmd")
        .arg("--help")
        .output();

    if !check_result.is_ok() {
        let error_msg = "未检测到 mcp-inspector。请运行以下命令安装：\nnpm install -g @modelcontextprotocol/inspector";
        let _ = window.emit("inspector-log", serde_json::json!({
            "type": "stderr",
            "text": error_msg,
            "sessionId": ""
        }));
        return Err("mcp-inspector not found".to_string());
    }

    let _ = window.emit("inspector-log", serde_json::json!({
        "type": "system",
        "text": "检测到 mcp-inspector",
        "sessionId": ""
    }));

    // 检查是否已有运行实例
    let mut inspector_guard = state
        .inspector
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    if inspector_guard.is_some() {
        let _ = window.emit("inspector-log", serde_json::json!({
            "type": "system",
            "text": "错误: Inspector 已在运行中",
            "sessionId": ""
        }));
        return Err("Inspector already running. Please stop current instance first.".to_string());
    }

    // 使用当前目录
    let work_path = std::env::current_dir()
        .map_err(|e| format!("Failed to get current dir: {}", e))?;

    // 启动进程
    let handle = InspectorHandle::spawn(
        window.clone(),
        work_path,
        HashMap::new(),
    )
    .map_err(|e| {
        let _ = window.emit("inspector-log", serde_json::json!({
            "type": "stderr",
            "text": format!("启动失败: {}", e),
            "sessionId": ""
        }));
        e.to_string()
    })?;

    let _ = window.emit("inspector-log", serde_json::json!({
        "type": "system",
        "text": format!("Inspector 进程已启动，端口: {}", handle.client_port()),
        "sessionId": ""
    }));

    // 存储句柄
    *inspector_guard = Some(handle);
    drop(inspector_guard);

    Ok(())
}

/// 停止 Inspector 进程
#[tauri::command]
pub fn stop_inspector(state: State<'_, AppState>) -> Result<(), String> {
    let mut guard = state
        .inspector
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    if let Some(mut handle) = guard.take() {
        handle.kill().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 获取 Inspector 状态
#[tauri::command]
pub fn get_inspector_status(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let mut guard = state
        .inspector
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    if let Some(ref mut handle) = *guard {
        if handle.is_running() {
            Ok(Some(handle.client_url()))
        } else {
            guard.take(); // 清理已退出的进程
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

/// 获取最近的 Profile 列表
#[tauri::command]
pub fn get_recent_profiles(state: State<'_, AppState>) -> Vec<ServerProfile> {
    let config = state.config.lock().unwrap();
    config.recent_profiles.clone()
}

/// 保存 Profile
#[tauri::command]
pub fn save_profile(
    state: State<'_, AppState>,
    name: String,
    working_dir: String,
    env_vars: HashMap<String, String>,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();

    let profile = ServerProfile {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        working_directory: PathBuf::from(working_dir),
        env_vars,
        created_at: Utc::now(),
        last_used_at: Utc::now(),
    };

    // 检查是否已存在同名配置
    let existing_index = config.recent_profiles.iter().position(|p| p.name == profile.name);
    if let Some(index) = existing_index {
        config.recent_profiles[index] = profile;
    } else {
        config.recent_profiles.push(profile);
    }

    // 保持最多 10 个配置
    config.recent_profiles.sort_by(|a, b| b.last_used_at.cmp(&a.last_used_at));
    config.recent_profiles.truncate(10);

    config
        .save()
        .map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(())
}

/// 删除 Profile
#[tauri::command]
pub fn delete_profile(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.recent_profiles.retain(|p| p.id != id);

    config
        .save()
        .map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(())
}
