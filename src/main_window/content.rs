use gpui_kit::{Context, FocusHandle, Focusable, IntoElement, ParentElement, Render, Window, div};

pub struct MainView {
    focus_handle: FocusHandle,
}

impl MainView {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for MainView {
    fn focus_handle(&self, _: &gpui_kit::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for MainView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child("Test")
    }
}
