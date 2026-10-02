use crate::main_window::{MainView, StatusBar, TitleBar};
use gpui_kit::{AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window, component::v_flex, div};

pub struct MainWindow {
    title_bar: Entity<TitleBar>,
    content: Entity<MainView>,
    status_bar: Entity<StatusBar>,
}

impl MainWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            title_bar: cx.new(|cx| TitleBar::new(window, cx)),
            content: cx.new(|cx| MainView::new(window, cx)),
            status_bar: cx.new(|_| StatusBar),
        }
    }
}

impl Render for MainWindow {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("main-window")
            .size_full()
            .child(self.title_bar.clone())
            .child(div().flex_1().min_h_0().child(self.content.clone()))
            .child(self.status_bar.clone())
    }
}
