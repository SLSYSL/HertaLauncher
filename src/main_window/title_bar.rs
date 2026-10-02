use gpui_kit::{
    Context, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Render,
    Styled, Window,
    component::{
        IconName, Sizable, TitleBar as KitTitleBar, WindowExt,
        button::{Button, ButtonVariants},
        h_flex,
    },
    div,
};

use crate::main_window::settings_dialog;

pub struct TitleBar {
    focus_handle: FocusHandle,
}

impl TitleBar {
    pub fn new(_: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for TitleBar {
    fn focus_handle(&self, _: &gpui_kit::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TitleBar {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        KitTitleBar::new()
            .child(
                div()
                    .text_sm()
                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                    .child("HertaLauncher"),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .track_focus(&self.focus_handle)
                    .on_mouse_down(gpui_kit::MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation()
                    })
                    .child(
                        Button::new("settings")
                            .small()
                            .ghost()
                            .icon(IconName::Settings)
                            .on_click(|_, window, cx| {
                                window.open_dialog(cx, |dialog, window, cx| {
                                    settings_dialog(dialog, cx, window)
                                });
                            }),
                    )
                    .child(
                        Button::new("Github")
                            .small()
                            .ghost()
                            .icon(IconName::Github)
                            .on_click(|_, _, cx| {
                                cx.open_url("https://github.com/SLSYSL/HertaLauncher");
                            }),
                    ),
            )
    }
}
