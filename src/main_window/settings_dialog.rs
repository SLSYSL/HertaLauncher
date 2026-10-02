use gpui_kit::{
    App, ParentElement, Styled, Window,
    component::{
        ActiveTheme, Theme, ThemeMode,
        dialog::Dialog,
        group_box::GroupBoxVariant::Fill,
        setting::{SettingField, SettingGroup, SettingItem, SettingPage, Settings},
    },
    px,
};

pub fn settings_dialog(dialog: Dialog, _: &mut App, _: &mut Window) -> Dialog {
    dialog.w(px(720.)).h(px(600.)).p_0().child(
        Settings::new("app-settings")
            .with_group_variant(Fill)
            .page(
                SettingPage::new("General").group(
                    SettingGroup::new().title("Appearance").item(
                        SettingItem::new(
                            "Dark Mode",
                            SettingField::switch(
                                |cx| cx.theme().is_dark(),
                                |val, cx| {
                                    let mode = if val {
                                        ThemeMode::Dark
                                    } else {
                                        ThemeMode::Light
                                    };
                                    Theme::change(mode, None, cx);
                                },
                            ),
                        )
                        .description("Switch between light and dark themes."),
                    ),
                ),
            ),
    )
}
