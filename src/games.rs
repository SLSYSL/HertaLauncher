use gpui_kit::assets::IconName;

pub struct Game {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: IconName,
}

pub const ALL: &[Game] = &[
    Game {
        id: "hsr",
        name: "崩坏：星穹铁道",
        icon: IconName::Star,
    },
    Game {
        id: "zzz",
        name: "绝区零",
        icon: IconName::Blocks,
    },
    Game {
        id: "gi",
        name: "原神",
        icon: IconName::Gamepad,
    },
];

/// config 里存的 id 可能过期（游戏被移除），这里统一回退到第一个
pub fn resolve(id: &str) -> &'static Game {
    ALL.iter()
        .find(|g| g.id == id)
        .unwrap_or_else(|| &ALL[0])
}
