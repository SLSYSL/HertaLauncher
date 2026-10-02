use gpui_kit::{Context, IntoElement, Render, Window, component::status_bar::StatusBar as KitStatusBar};

pub struct StatusBar;
impl Render for StatusBar {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        KitStatusBar::new().right(format!("v{}", env!("CARGO_PKG_VERSION")))
    }
}
