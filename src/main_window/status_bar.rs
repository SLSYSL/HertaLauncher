use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    base::h_flex,
    component::{
        ActiveTheme, Side, sidebar::SidebarToggleButton, status_bar::StatusBar as KitStatusBar,
    },
    div,
};

use crate::{config, games};

pub struct StatusBar;
impl StatusBar {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // 配置变化（其他视图切换游戏/折叠侧栏）→ 重绘
        // 回调里的 cx.notify() 不可省（逐视图渲染缓存，不标脏就不重绘）
        cx.observe_global::<config::ConfigStore>(|_, cx| cx.notify())
            .detach();
        Self
    }
}

impl Render for StatusBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cfg = config::current(cx);
        let game_name = games::resolve(cx)
            .map(|g| g.name)
            .unwrap_or_else(|| "未导入游戏".into());

        KitStatusBar::new()
            .left(
                h_flex()
                    .items_center()
                    .gap_2()
                    .child(
                        SidebarToggleButton::new()
                            .side(Side::Left)
                            .collapsed(cfg.sidebar_collapsed)
                            .accessibility_label(if cfg.sidebar_collapsed {
                                "展开侧边栏"
                            } else {
                                "收起侧边栏"
                            })
                            .on_click(|_, _, cx| {
                                config::update(cx, |c| c.sidebar_collapsed = !c.sidebar_collapsed)
                            }),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(cx.theme().foreground)
                            .child(game_name),
                    ),
            )
            .right(format!("v{}", env!("CARGO_PKG_VERSION")))
    }
}
