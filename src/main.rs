slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = App::new()?;
    slint_kit::init_form_validation!(self, app);
    app.run()?;
    Ok(())
}
