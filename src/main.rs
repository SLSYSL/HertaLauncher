mod main_window;

use crate::main_window::MainWindow;
use gpui_kit::{
    AppContext, application,
    assets::AllAssets,
    component::{TitleBar, init},
    open_window,
};

fn main() {
    application().with_assets(AllAssets).run(move |cx| {
        init(cx);

        cx.spawn(async move |cx| {
            let (window, _) = cx.update(|cx| {
                open_window(TitleBar::window_options(), cx, |window, cx| {
                    cx.new(|cx| MainWindow::new(window, cx))
                })
                .expect("Failed to open window")
            });

            let _ = window.update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title("HertaLauncher");
            });
        })
        .detach();
    });
}
