use gpui_kit::base::input::InputState;
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::TitleBar;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::form::{field, v_form};
use gpui_kit::component::input::Input;
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, TitlebarOptions, Window, WindowBounds, WindowDecorations, WindowKind, WindowOptions, px, size,
};
use tracing::{error, info};

use crate::theme::palette;

pub struct CreateVmWindow {
    name: Entity<InputState>,
    cpu: Entity<InputState>,
    memory: Entity<InputState>,
    disk: Entity<InputState>,
}

impl CreateVmWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            name: cx.new(|cx| InputState::new(window, cx)),
            cpu: cx.new(|cx| InputState::new(window, cx)),
            memory: cx.new(|cx| InputState::new(window, cx)),
            disk: cx.new(|cx| InputState::new(window, cx)),
        }
    }

    /// Open the window. Call this wherever you used to call open_dialog.
    pub fn open(cx: &mut App) {
        let bounds = WindowBounds::centered(size(px(440.), px(420.)), cx);

        let options = WindowOptions {
                window_bounds: Some(bounds),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Create virtual machine")),
                    appears_transparent: true,
                    ..Default::default()
                }),
                kind: WindowKind::Dialog,
                is_resizable: false,
                window_min_size: Some(size(px(440.), px(420.))),
                ..Default::default()
            };

        let result = gpui_kit::open_window(options, cx, |window, cx| {
            cx.new(|cx| CreateVmWindow::new(window, cx))
        });

        if let Err(err) = result {
            error!("failed to open create-VM window: {err}");
        }
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = self.name.read(cx).value();
        let cpu = self.cpu.read(cx).value();
        let memory = self.memory.read(cx).value();
        let disk = self.disk.read(cx).value();

        if name.trim().is_empty() {
            return;
        }

        info!("Create VM: name={name}, cpu={cpu}, memory={memory}MB, disk={disk}GB");
        // TODO: real create logic
        window.remove_window();
    }
}

impl Render for CreateVmWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .bg(palette::current().canvas)
            .text_color(palette::current().text_primary)
            .p_4()
            .gap_4()
            .child(
                v_form()
                    .gap_3()
                    .child(
                        field()
                            .label("Name")
                            .required(true)
                            .child(Input::new(&self.name)),
                    )
                    .child(
                        field()
                            .label("CPU cores")
                            .required(true)
                            .description("Number of vCPUs")
                            .child(Input::new(&self.cpu)),
                    )
                    .child(
                        field()
                            .label("Memory (MB)")
                            .required(true)
                            .child(Input::new(&self.memory)),
                    )
                    .child(
                        field()
                            .label("Disk size (GB)")
                            .required(true)
                            .child(Input::new(&self.disk)),
                    ),
            )
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .justify_end()
                    .child(
                        Button::new("cancel")
                            .label("Cancel")
                            .on_click(|_, window, _cx| window.remove_window()),
                    )
                    .child(
                        Button::new("create")
                            .primary()
                            .label("Create")
                            .on_click(cx.listener(|this, _, window, cx| this.submit(window, cx))),
                    ),
            )
    }
}
