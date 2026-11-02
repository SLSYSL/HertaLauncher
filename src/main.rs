mod config;
mod game_shell;
mod games;
mod main_window;

use crate::main_window::MainWindow;
use gpui_kit::{
    AppContext, application,
    assets::AllAssets,
    component::{Theme, ThemeMode, TitleBar},
    open_window,
};

fn register_shell_modules() {
    // 宿主能力：必须在加载任何 shell 应用之前注册。
    // 原则：只暴露脚本需要的粒度，不交出 AppConfig 整体。
    // 同步函数内用 with_current_app 拿 App（读 config / 启动进程）。
    use gpui_shell::{HostModule, HostValue};

    gpui_shell::export_module(
        HostModule::new("launcher")
            .function("game_list", |_| {
                gpui_shell::with_current_app(|cx| {
                    let list = crate::games::all(cx)
                        .iter()
                        .map(|g| {
                            HostValue::Object(vec![
                                ("id".into(), HostValue::from(g.id.clone())),
                                ("name".into(), HostValue::from(g.name.clone())),
                            ])
                        })
                        .collect::<Vec<_>>();
                    Ok(HostValue::from(list))
                })
                .unwrap_or_else(|| Ok(HostValue::from(Vec::<HostValue>::new())))
            })
            .function("launch_game", |args| {
                gpui_shell::with_current_app(|cx| {
                    let id = args.string(0)?;
                    let entry = crate::games::all(cx)
                        .into_iter()
                        .find(|g| g.id == id)
                        .ok_or_else(|| "unknown game id")?;
                    if entry.exe.is_empty() {
                        return Err("该游戏未配置可执行文件".into());
                    }
                    let exe = entry.exe.clone();
                    std::process::Command::new(&exe)
                        .current_dir(
                            std::path::Path::new(&exe)
                                .parent()
                                .map(|p| p.to_path_buf())
                                .unwrap_or_default(),
                        )
                        .spawn()
                        .map_err(|e| format!("启动失败：{e}"))?;
                    Ok(HostValue::from(true))
                })
                .unwrap_or_else(|| Err("no app scope".into()))
            }),
    )
    .expect("register launcher host module");
}

fn main() {
    // shell 运行时的诊断（点击分发失败/脚本错误等）都走 tracing，
    // 无订阅者则全部静默。RUST_LOG=gpui_shell=debug 可看点击分发细节。
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .compact()
        .init();

    application().with_assets(AllAssets).run(move |cx| {
        gpui_kit::init(cx);
        gpui_shell::init(cx);
        register_shell_modules();
        crate::config::init(cx);

        // 应用持久化主题
        if crate::config::current(cx).dark_mode {
            Theme::change(ThemeMode::Dark, None, cx);
        }

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
