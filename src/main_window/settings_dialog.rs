use crate::config;

use gpui_kit::{
    App, ParentElement, Styled, Window,
    component::{
        ActiveTheme, Theme, ThemeMode,
        dialog::Dialog,
        group_box::GroupBoxVariant::Outline,
        setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    },
    px,
};

pub fn settings_dialog(dialog: Dialog, _: &mut App, _: &mut Window) -> Dialog {
    dialog.w(px(720.)).h(px(600.)).p_0().child(
        Settings::new("app-settings")
            .with_group_variant(Outline)
            .page(
                SettingPage::new("启动器").group(
                    SettingGroup::new().title("视觉").item(
                        SettingItem::new(
                            "深色主题",
                            SettingField::switch(
                                |cx| cx.theme().is_dark(),
                                |val, cx| {
                                    Theme::change(
                                        if val {
                                            ThemeMode::Dark
                                        } else {
                                            ThemeMode::Light
                                        },
                                        None,
                                        cx,
                                    );
                                    let _ = config::update(|c| c.dark_mode = val);
                                },
                            ),
                        )
                        .description("在浅色与深色主题之间切换"),
                    ),
                ),
            ),
    )
}
