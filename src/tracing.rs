use color_eyre::eyre::{Result, eyre};
use directories::ProjectDirs;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[allow(clippy::missing_panics_doc)]
pub fn init_tracing() -> Result<WorkerGuard> {
    // Log to ~/.local/share/virtdesk/virtdesk.log
    let proj_dirs = ProjectDirs::from("com", "VirtDesk", "VirtDesk")
        .ok_or_else(|| eyre!("Failed to determine application directories"))?;
    let log_dir = proj_dirs.data_local_dir();
    std::fs::create_dir_all(log_dir)?;
    

    let file_appender = tracing_appender::rolling::daily(log_dir, "virtdesk.log");
    let (_, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        // .with(fmt::layer().with_writer(non_blocking).with_ansi(false)) // file only, no stdout
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
            .add_directive("gpui_linux=off".parse().unwrap())
            .add_directive("gpui_wgpu=off".parse().unwrap())
            .add_directive("wgpu_hal=off".parse().unwrap())
        )
        .with(ErrorLayer::default())
        .init();

    // guard // must be held alive for the duration of main, dropped at end
    Ok(guard)
}
