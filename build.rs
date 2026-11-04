fn main() {
    if std::env::var_os("CARGO_FEATURE_DEV_TOOLS").is_some() {
        unsafe {
            std::env::set_var("SLINT_LIVE_PREVIEW", "1");
            std::env::set_var("SLINT_EMIT_DEBUG_INFO", "1");
        }
        println!("cargo:rerun-if-env-changed=CARGO_FEATURE_DEV_TOOLS");
    }

    let mut config = slint_kit_build::config();
    if std::env::var("PROFILE").as_deref() == Ok("debug") {
        config = config.with_debug_info(true);
    }

    slint_build::compile_with_config("ui/app.slint", config).unwrap();
}
