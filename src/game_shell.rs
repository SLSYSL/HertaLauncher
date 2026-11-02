//! 每个游戏一个独立的 JS 应用（独立 shell 实例：独立 VM 与状态）。
//!
//! 目录约定：`{配置目录}/herta_launcher/shell/{game_id}/main.js`
//!
//! 脚本内可用能力：
//! - `gpui-kit` / `gpui-base` 绑定（渲染；base-first，控件无样式）
//! - 标准模块：buffer / console / crypto / fs-promises / net / os / path /
//!   process / url / websocket / zlib + 全局 fetch（`quickjs` feature 自带）
//! - `launcher` 宿主模块：`main.rs` 里 `export_module` 注册的启动器能力；
//!   插件可用动态 import 探测，独立 CLI 预览时自动降级

use std::{fs, io, path::Path, path::PathBuf, rc::Rc};

use gpui_kit::{App, Entity, Window};
use gpui_shell::{ScriptView, ShellRuntime};

use crate::config::GameEntry;

pub struct GameShell {
    #[allow(dead_code)] // 存活锚点：ScriptView 的回调经由它驱动
    runtime: Rc<ShellRuntime>,
    view: Entity<ScriptView>,
}

impl GameShell {
    pub fn load(game: &GameEntry, window: &mut Window, cx: &mut App) -> gpui_shell::anyhow::Result<Self> {
        // 脚本 storage 按游戏隔离。注意：该路径是进程级全局，只影响之后打开的 storage；
        // 多游戏同帧并发首次加载时不保证隔离（典型使用一次只玩一个游戏，够用）
        gpui_shell::set_storage_path(storage_dir(&game.id));
        // 每游戏一个**隔离**运行时（new 会安装全局默认运行时，第二个游戏会报错），
        // 并显式传入 gpui-component 组件目录，脚本才能 import 官方样式化组件
        let runtime = ShellRuntime::new_isolated_with_components(
            gpui_shell::components::components()?,
        )?;
        let app = runtime.load_application(&app_dir(&game.id), "main.js")?;
        let view = runtime.mount_application(&app, window, cx)?;
        Ok(Self { runtime, view })
    }

    pub fn view(&self) -> Entity<ScriptView> {
        self.view.clone()
    }
}

fn base_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        .join("herta_launcher")
}

pub(crate) fn app_dir(game_id: &str) -> PathBuf {
    base_dir().join("shell").join(game_id)
}

fn storage_dir(game_id: &str) -> PathBuf {
    base_dir().join("shell-storage").join(game_id)
}

// ---------- 插件安装 / 卸载 ----------

/// 安装插件包到 shell/<id>/：接受单文件 main 源码（.js）或 zip 包（根目录需含 main.js）。
/// 失败时调用方负责清理目标目录。
pub(crate) fn install_from_path(src: &Path, game_id: &str) -> io::Result<()> {
    let dest = app_dir(game_id);
    fs::create_dir_all(&dest)?;

    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase());
    match ext.as_deref() {
        Some("js") => {
            fs::copy(src, dest.join("main.js"))?;
        }
        Some("zip") => {
            let file = fs::File::open(src)?;
            let mut archive = zip::ZipArchive::new(file)?;
            for i in 0..archive.len() {
                let mut entry = archive.by_index(i)?;
                // enclosed_name 拒绝绝对路径与 ../，防 zip slip
                let Some(rel) = entry.enclosed_name() else { continue };
                let out = dest.join(rel);
                if entry.is_dir() {
                    fs::create_dir_all(&out)?;
                } else {
                    if let Some(p) = out.parent() {
                        fs::create_dir_all(p)?;
                    }
                    let mut out_file = fs::File::create(&out)?;
                    io::copy(&mut entry, &mut out_file)?;
                }
            }
        }
        _ => return Err(io::Error::other("仅支持 .js 或 .zip 插件包")),
    }

    if !dest.join("main.js").exists() {
        return Err(io::Error::other("插件包根目录缺少 main.js"));
    }
    Ok(())
}

/// 卸载：删除插件目录与该游戏的脚本 storage。
pub(crate) fn uninstall(game_id: &str) {
    let _ = fs::remove_dir_all(app_dir(game_id));
    let _ = fs::remove_dir_all(storage_dir(game_id));
}
