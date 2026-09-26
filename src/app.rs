use gpui_kit_assets::{AllAssets};
use ::tracing::{error, info};
use gpui_kit::component::{Root, Theme, TitleBar};
use gpui_kit::{AppContext, WindowBounds, WindowDecorations, WindowOptions, px, size};

use crate::virtual_machines::VirtualMachineManager;

pub fn run() {
    info!("Starting VirtDesk...");

    let app = gpui_kit::application().with_assets(AllAssets);

    app.run(move |cx| {
        gpui_kit::init(cx);
        // theme::init(cx);

        let window_bounds = WindowBounds::centered(size(px(1360.), px(840.)), cx);

        // match Connect::open(Some("qemu:///session")) {
        //     Ok(mut conn) => {
        //         info!("Connected to qemu:///session");
        //         // Use `conn` here...
        //         // When done: let _ = conn.close();
        //         let domains = conn.list_all_domains(0);

        //         match domains {
        //             Ok(domain_list) => {
        //                 for domain in domain_list {
        //                     match domain.get_name() {
        //                         Ok(name) => info!("Domain name: {}", name),
        //                         Err(e) => error!("Failed to get domain name: {}", e),
        //                     }
        //                 }
        //             }
        //             Err(err) => {
        //                 error!("error retrieving domains: {}", err);
        //             }
        //         }

        //         let _ = conn.close();
        //     }
        //     Err(e) => {
        //         error!("Failed to connect to qemu:///session: {}", e);
        //     }
        // }

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                window_bounds: Some(window_bounds),
                window_min_size: Some(size(px(760.), px(560.))),
                titlebar: Some(TitleBar::title_bar_options()),
                window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            };

            if let Err(err) = cx.open_window(window_options, |window, cx| {
                Theme::sync_system_appearance(Some(window), cx);

                window
                    .observe_window_appearance(|window, cx| {
                        Theme::sync_system_appearance(Some(window), cx);
                    })
                    .detach();

                let machine_manager = cx.new(|_| VirtualMachineManager::new());
                cx.new(|cx| Root::new(machine_manager, window, cx))
            }) {
                error!(error = ?err, "Failed to initialize window");
                std::process::exit(1);
            }
        })
        .detach();

        // Spawn the qemu-system-x86_64 process
        // let mut child = Command::new("qemu-system-x86_64")
        //     .arg("-enable-kvm")
        //     .args(["-m", "2048"])
        //     .args(["-smp", "2"])
        //     .args(["-boot", "d"])
        //     .args(["-cdrom", "/home/ric/Downloads/ISO/alpine-virt-3.24.1-x86_64.iso"])
        //     .args(["-drive", "file=/home/ric/Downloads/ISO/alpine.qcow2,format=qcow2"])
        //     .args(["-display", "default"])
        //     // Optional: Inherit stdout/stderr so you can see boot errors in your terminal
        //     .stdout(Stdio::inherit())
        //     .stderr(Stdio::inherit())
        //     .spawn(); // Spawns the process in the background

        // println!("VM started successfully with PID: {}", child.id());

        // Wait for the VM to be closed by the user
        // let status = child.wait()?;

        // println!("QEMU exited with status: {}", status);
    });
}
