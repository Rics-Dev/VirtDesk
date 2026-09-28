use color_eyre::eyre::Result;

mod app;
mod qemu_backend;
mod theme;
pub mod tracing;
pub mod virtual_machines;

fn main() -> Result<()> {
    color_eyre::install()?;

    let _guard = tracing::init_tracing()?;

    app::run();

    Ok(())
}
