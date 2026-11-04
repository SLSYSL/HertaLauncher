slint::include_modules!();

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "dev-tools")]
    unsafe {
        if std::env::var_os("SLINT_MCP_PORT").is_none() {
            std::env::set_var("SLINT_MCP_PORT", "50302");
        }
    }

    let app = App::new()?;
    slint_kit::init_form_validation!(self, app);
    app.run()?;
    Ok(())
}
