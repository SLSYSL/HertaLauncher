//! 闭包访问 原子写入

use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock, PoisonError};

use serde::{Deserialize, Serialize};

/// |Platform|Path|
/// |-|-|
/// |Win|%APPDATA%|
/// |Mac|~/Library/Application Support|
/// |Linux|~/.config|
fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| env::current_dir().unwrap_or_default())
        .join("herta_launcher")
        .join("config.json")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)] // 缺失字段用默认值
pub struct AppConfig {
    pub dark_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { dark_mode: false }
    }
}

static CONFIG: OnceLock<Mutex<AppConfig>> = OnceLock::new();

/// 读文件 失败回退默认值
fn init() -> Mutex<AppConfig> {
    let cfg: AppConfig = std::fs::read_to_string(config_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    Mutex::new(cfg)
}

/// 闭包式只读访问
pub fn with<R>(f: impl FnOnce(&AppConfig) -> R) -> R {
    f(&CONFIG
        .get_or_init(init)
        .lock()
        .unwrap_or_else(PoisonError::into_inner))
}

/// 闭包式修改
pub fn update(f: impl FnOnce(&mut AppConfig)) -> io::Result<()> {
    let mut m = CONFIG
        .get_or_init(init)
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    f(&mut m);
    let path = config_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(&*m)?;
    atomic_write(&path, json.as_bytes())
}

/// 写 config.json.tmp 后 rename 防崩溃
fn atomic_write(path: &Path, data: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)
}
