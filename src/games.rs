//! 已导入游戏的读取辅助（数据本体在 `config::AppConfig::games`）。

use gpui_kit::App;

use crate::config::{self, GameEntry};

/// 全部已导入游戏（快照）
pub fn all(cx: &App) -> Vec<GameEntry> {
    config::current(cx).games.clone()
}

/// 当前选中：game_id 匹配项，否则第一个；未导入任何游戏时为 None
pub fn resolve(cx: &App) -> Option<GameEntry> {
    let cfg = config::current(cx);
    cfg.games
        .iter()
        .find(|g| g.id == cfg.game_id)
        .cloned()
        .or_else(|| cfg.games.first().cloned())
}

/// 导入时生成稳定 id —— 同时是插件目录名（shell/<id>/），保证文件系统安全且可读
pub fn new_id(name: &str) -> String {
    let slug: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    let slug = slug.trim_matches('_');
    let stem = if slug.is_empty() { "game" } else { slug };
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{stem}-{stamp}")
}
