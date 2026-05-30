use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPrefs {
    pub server_url: Option<String>,
    pub insecure: bool,
    pub enable_debug_logging: bool,
}

impl Default for AppPrefs {
    fn default() -> Self {
        Self {
            server_url: None,
            insecure: false,
            enable_debug_logging: false,
        }
    }
}

fn prefs_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("truehub")
        .join("prefs.json")
}

pub async fn load() -> AppPrefs {
    let path = prefs_path();
    match fs::read_to_string(&path).await {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => AppPrefs::default(),
    }
}

pub async fn save(prefs: &AppPrefs) -> Result<(), String> {
    let path = prefs_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(prefs).map_err(|e| e.to_string())?;
    fs::write(&path, json).await.map_err(|e| e.to_string())
}