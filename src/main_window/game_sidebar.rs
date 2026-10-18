use gpui_kit::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window,
    assets::IconName,
    base::v_flex,
    component::{
        Icon, Sizable,
        input::{Input, InputEvent, InputState},
        sidebar::{Sidebar, SidebarCollapsible, SidebarMenuItem},
    },
};

use crate::{config, games};

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

        // 配置变化（其他视图切换游戏/折叠侧栏）→ 重绘
        // 注意：回调里的 cx.notify() 不可省（逐视图渲染缓存，不标脏就不重绘）
        cx.observe_global::<config::ConfigStore>(|_, cx| cx.notify())
            .detach();

        Self { search }
    }
}

impl Render for GameSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let cfg = config::current(cx);
        let collapsed = cfg.sidebar_collapsed;
        let selected = cfg.game_id.clone();
        let query = self.search.read(cx).value().to_lowercase();

        // 整个侧栏折叠：宽度 255 → 0，动画结束后内容卸载
        // 注意不要写 .w_full()，相对宽度会让 Offcanvas 变成"瞬间消失"（无动画）
        Sidebar::new("game-sidebar")
            .collapsible(SidebarCollapsible::Offcanvas)
            .collapsed(collapsed)
            .header(
                v_flex()
                    .w_full()
                    .pt_1()
                    .child(Input::new(&self.search).prefix(Icon::new(IconName::Search).small())),
            )
            .children(
                games::ALL
                    .iter()
                    .filter(|g| query.is_empty() || g.name.to_lowercase().contains(&query))
                    .map(|g| {
                        let id = g.id;
                        SidebarMenuItem::new(g.name)
                            .icon(g.icon)
                            .active(selected == id)
                            .on_click(move |_, _, cx| {
                                config::update(cx, |c| {
                                    c.game_id = games::resolve(id).id.to_string()
                                })
                            })
                    }),
            )
    }
}
