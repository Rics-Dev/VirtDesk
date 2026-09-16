use color_eyre::eyre::Result;

mod app;
mod theme;
pub mod tracing;
pub mod home;
mod qemu_backend;

fn main() -> Result<()> {
    color_eyre::install()?;

    tracing::init_tracing();

    app::run();

    Ok(())
}
