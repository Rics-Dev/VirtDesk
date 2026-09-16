// use tracing_appender::non_blocking::WorkerGuard;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

// pub fn init_tracing() -> WorkerGuard {
#[allow(clippy::missing_panics_doc)]
pub fn init_tracing() {
    // Log to ~/.local/share/virtdesk/virtdesk.log
    let log_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("virtdesk");
    std::fs::create_dir_all(&log_dir).ok();

    // let file_appender = tracing_appender::rolling::daily(log_dir, "virtdesk.log");
    // let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    // let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

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
}
