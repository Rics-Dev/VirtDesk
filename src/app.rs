use ::tracing::{error, info};
use gpui_kit::component::{Root, Theme, TitleBar};
use gpui_kit::{AppContext, WindowDecorations, WindowOptions};

use crate::virtual_machine::DashboardView;

pub fn run() {
    info!("Starting VirtDesk...");

    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    app.run(move |cx| {
        gpui_kit::init(cx);
        // theme::init(cx);

        cx.spawn(async move |cx| {
            
            let window_options = WindowOptions {
                titlebar: Some(TitleBar::title_bar_options()),
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            };

            if let Err(err) = cx.open_window(window_options, |window, cx| {
                Theme::sync_system_appearance(Some(window), cx);

                window.observe_window_appearance(|window, cx| {
                    Theme::sync_system_appearance(Some(window), cx);
                }).detach();
                
                let dashboard = cx.new(|_| DashboardView::new());
                cx.new(|cx| Root::new(dashboard, window, cx))
            }) {
                error!(error = ?err, "Failed to initialize window");
                std::process::exit(1);
            }
        })
        .detach();
    });
}
