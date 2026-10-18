//! 应用配置：Global 为运行时事实源，`update` 写时自动落盘并刷新界面。
//!
//! 扩展：只把需要持久化的状态放进 `AppConfig`；新增领域字段时建对应子节 struct。

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    sync::Arc,
};

use gpui_kit::{App, Global};
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)] // 缺失字段用默认值
pub struct AppConfig {
    pub dark_mode: bool,
    pub game_id: String, // 当前选中的游戏 id，空 = games::ALL[0]
    pub sidebar_collapsed: bool,
}

pub struct ConfigStore(Arc<AppConfig>);
impl Global for ConfigStore {}

/// 启动时调用：从磁盘加载（失败回退默认值）
pub fn init(cx: &mut App) {
    let cfg = fs::read_to_string(config_path())
        .ok()
        .and_then(|s| serde_json::from_str::<AppConfig>(&s).ok())
        .unwrap_or_default();
    cx.set_global(ConfigStore(Arc::new(cfg)));
}

/// 读取配置快照（可持有或传入后台任务）。
pub fn current(cx: &App) -> Arc<AppConfig> {
    cx.global::<ConfigStore>().0.clone()
}

/// 修改配置：自动落盘并刷新相关视图。
pub fn update<F: FnOnce(&mut AppConfig)>(cx: &mut App, f: F) {
    // 副本上应用修改后整体替换；global_mut 写入自动通知 observe_global 的视图
    let store = cx.global_mut::<ConfigStore>();
    let mut next = (*store.0).clone();
    f(&mut next);
    if let Err(err) = save(&next) {
        eprintln!("[herta_launcher] failed to save config: {err}");
    }
    store.0 = Arc::new(next);
}

fn save(cfg: &AppConfig) -> io::Result<()> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(cfg)?;
    write_atomic(&path, json.as_bytes())
}

/// 写 config.json.tmp 后 rename 防崩溃
fn write_atomic(path: &Path, data: &[u8]) -> io::Result<()> {
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, data)?;
    fs::rename(&tmp, path)
}
