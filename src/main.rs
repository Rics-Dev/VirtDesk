use color_eyre::eyre::Result;

mod app;
mod theme;
pub mod tracing;
pub mod virtual_machine;

fn main() -> Result<()> {
    color_eyre::install()?;

    tracing::init_tracing();

    app::run();

    Ok(())
}
