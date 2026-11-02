use gpui_kit::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    PathPromptOptions,
    assets::IconName,
    base::h_flex,
    component::{
        Icon, Sizable, WindowExt,
        button::{Button, ButtonVariants},
        input::{Input, InputEvent, InputState},
        notification::Notification,
        sidebar::{Sidebar, SidebarCollapsible, SidebarMenuItem},
    },
};

use crate::{config, game_shell, games};

pub struct GameSidebar {
    search: Entity<InputState>,
}

impl GameSidebar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search = cx.new(|cx| InputState::new(window, cx).placeholder("搜索游戏名"));

        // 搜索词变化 → 重绘（过滤结果在 render 里算）
        cx.subscribe_in(&search, window, |_, _, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        })
        .detach();

        // 配置变化（导入/删除/切换/折叠）→ 重绘
        cx.observe_global::<config::ConfigStore>(|_, cx| cx.notify())
            .detach();

        Self { search }
    }

    /// 原生文件对话框选插件包（.js / .zip）→ 安装到 shell/<id>/ → 写入 config 并选中。
    fn import(&mut self, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("选择插件包（main.js 或 .zip）".into()),
        });

        cx.spawn(async move |this, cx| {
            let picked = rx.await.ok().and_then(|r| r.ok()).flatten();
            let Some(path) = picked.and_then(|mut v| v.pop()) else {
                return; // 用户取消
            };
            let name = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "未命名插件".into());
            let id = games::new_id(&name);

            // 安装（update_in 是为了拿 window 发通知；文件 IO 阻塞主线程，量大时会顿一下）
            let installed = this
                .update_in(cx, |_, _, _| game_shell::install_from_path(&path, &id))
                .and_then(|r| r.map_err(gpui_shell::anyhow::Error::from));

            match installed {
                Ok(()) => {
                    let ok_msg = format!("已导入「{name}」");
                    this.update(cx, |_, cx| {
                        config::update(cx, |c| {
                            c.games.push(config::GameEntry {
                                id: id.clone(),
                                name,
                                exe: String::new(), // v1：导入不收集 exe，launch_game 需要时由后续功能补充
                            });
                            c.game_id = id.clone();
                        });
                    })
                    .ok();
                    notify(&this, cx, ok_msg).await;
                }
                Err(err) => {
                    let _ = std::fs::remove_dir_all(game_shell::app_dir(&id));
                    notify(&this, cx, format!("导入失败：{err}")).await;
                }
            }
        })
        .detach();
    }
}

/// 删除游戏（侧栏条目的 X 按钮调用）：卸载插件目录与 storage，从 config 移除；
/// 若删的是当前选中，回落到第一个。自由函数——suffix 构建器里没有 self 可用。
fn remove_game(id: &str, cx: &mut App) {
    game_shell::uninstall(id);
    config::update(cx, |c| {
        c.games.retain(|g| g.id != id);
        if c.game_id == id {
            c.game_id = c.games.first().map(|g| g.id.clone()).unwrap_or_default();
        }
    });
}

/// 通知横幅（需要 window，经 WeakEntity::update_in 在异步任务里调用）
async fn notify(
    this: &gpui_kit::WeakEntity<GameSidebar>,
    cx: &mut gpui_kit::AsyncApp,
    message: String,
) {
    this.update_in(cx, |_, window, cx| {
        window.push_notification(Notification::new().message(message), cx)
    })
    .ok();
}

impl Render for GameSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cfg = config::current(cx);
        let collapsed = cfg.sidebar_collapsed;
        let selected = cfg.game_id.clone();
        let query = self.search.read(cx).value().to_lowercase();

        Sidebar::new("game-sidebar")
            .collapsible(SidebarCollapsible::Offcanvas)
            .collapsed(collapsed)
            .header(
                h_flex()
                    .w_full()
                    .pt_1()
                    .gap_1()
                    .child(
                        Input::new(&self.search)
                            .small()
                            .prefix(Icon::new(IconName::Search).small())
                            .flex_1(),
                    )
                    .child(
                        Button::new("import-game")
                            .xsmall()
                            .ghost()
                            .icon(IconName::Plus)
                            .tooltip("导入插件（.js / .zip）")
                            .on_click(cx.listener(|this, _, _, cx| this.import(cx))),
                    ),
            )
            .children(
                cfg.games
                    .iter()
                    .enumerate()
                    .filter(|(_, g)| {
                        query.is_empty() || g.name.to_lowercase().contains(&query)
                    })
                    .map(|(ix, g)| {
                        let id = g.id.clone();
                        let name = g.name.clone();
                        let del_id = id.clone(); // 删除按钮与选中回调各自持有一份
                        SidebarMenuItem::new(name)
                            .icon(IconName::Gamepad)
                            .active(selected == g.id)
                            .suffix(move |_, _| {
                                let del_id = del_id.clone(); // suffix 是 Fn（可多次渲染），每次克隆一份给点击处理器
                                Button::new(("delete-game", ix))
                                    .xsmall()
                                    .ghost()
                                    .icon(IconName::X)
                                    .tooltip("删除")
                                    .on_click(move |_, _, cx| {
                                        cx.stop_propagation(); // 阻止冒泡成“选中”
                                        remove_game(&del_id, cx);
                                    })
                            })
                            .on_click(move |_, _, cx| config::update(cx, |c| c.game_id = id.clone()))
                    }),
            )
    }
}
