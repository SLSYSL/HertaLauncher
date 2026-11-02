use crate::main_window::{GameSidebar, MainView, StatusBar, TitleBar};
use gpui_kit::{
    App, AppContext, Context, Entity, FocusHandle, Focusable, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window,
    base::{h_flex, v_flex},
    div,
};

pub struct MainWindow {
    focus_handle: FocusHandle,
    title_bar: Entity<TitleBar>,
    sidebar: Entity<GameSidebar>,
    content: Entity<MainView>,
    status_bar: Entity<StatusBar>,
}

impl MainWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let content = cx.new(|cx| MainView::new(window, cx));
        // 预加载当前游戏的 shell（浅栈异步任务）
        content.update(cx, |view, cx| view.ensure_current_shell(cx));
        Self {
            focus_handle: cx.focus_handle(),
            title_bar: cx.new(|cx| TitleBar::new(window, cx)),
            sidebar: cx.new(|cx| GameSidebar::new(window, cx)),
            content,
            status_bar: cx.new(|cx| StatusBar::new(cx)),
        }
    }
}

impl Focusable for MainWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("main-window")
            .size_full()
            .track_focus(&self.focus_handle)
            .child(self.title_bar.clone())
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .child(self.sidebar.clone()) // 左：导航（折叠后宽度自动归 0）
                    .child(
                        div()
                            .flex_1()
                            .min_w_0() // 关键：允许收缩，否则内容会把侧栏挤走
                            .child(self.content.clone()),
                    ),
            )
            .child(self.status_bar.clone())
    }
}
