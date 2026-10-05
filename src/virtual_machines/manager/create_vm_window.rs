use gpui_kit::base::input::InputState;
use gpui_kit::base::slider::{SliderScale, SliderState};
use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::TitleBar;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::form::{field, v_form};
use gpui_kit::component::input::{
    Input, InputGroup, InputGroupAddon, InputGroupAddonAlignment, InputGroupButton, InputGroupInput,
};
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use gpui_kit::component::slider::Slider;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    App, AppContext, Bounds, Context, Entity, IntoElement, ParentElement, Render, SharedString,
    Styled, TitlebarOptions, Window, WindowBounds, WindowDecorations, WindowKind, WindowOptions,
    div, px, size,
};
use sysinfo::System;
use tracing::{error, info};

use crate::theme::palette;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MemoryUnit {
    Mb,
    Gb,
}

pub struct CreateVmWindow {
    name: Entity<InputState>,
    cpu: Entity<InputState>,
    disk: Entity<InputState>,
    slider_state: Entity<SliderState>,
    memory: Entity<InputState>,
    memory_unit: MemoryUnit,
}

impl CreateVmWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            name: cx.new(|cx| InputState::new(window, cx)),
            cpu: cx.new(|cx| InputState::new(window, cx)),
            disk: cx.new(|cx| InputState::new(window, cx)),
            slider_state: cx.new(|_| {
                SliderState::new()
                    .min(1.0)
                    .max(8.0)
                    .default_value(2.0)
                    .step(1.0)
                    .scale(SliderScale::Linear)
            }),
            memory: cx.new(|cx| {
                InputState::new(window, cx).placeholder("2")
                // optional: start with a default value
                // .default_value("2")  // if your API has this
            }),
            memory_unit: MemoryUnit::Gb, // default 2 GB
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
        let cpu_value = self.slider_state.read(cx).value().start() as i32;

        // let mut sys = System::new_all();

        // sys.refresh_all();

        // tracing::info!("NB CPUs: {}", sys.cpus().len());

        v_form()
            .bg(palette::current().canvas)
            .text_color(palette::current().text_primary)
            .p_4()
            .gap_4()
            .child(
                field()
                    .label("Name")
                    .child(Input::new(&self.name))
                    .required(true),
            )
            .child(
                field()
                    .label("ISO")
                    .required(true)
            )
            .child(
                field()
                    .label("CPU cores")
                    .description_fn(|_, _| {
                        div().child("Up to 8 available")
                    })
                    .required(true)
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                h_flex()
                                    .w_full()
                                    .items_center()
                                    .gap_3()
                                    .child(Slider::new(&self.slider_state).flex_1())
                                    .child(
                                        div()
                                            .min_w(px(28.))
                                            .text_right()
                                            .child(format!("{cpu_value}")),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .w_full()
                                    .justify_between()
                                    .text_xs()
                                    .text_color(palette::current().text_secondary)
                                    .child("1")
                                    .child("8"),
                            ),
                    ),
            )
            .child(
                field()
                    .label("Memory")
                    .description("RAM size for the virtual machine")
                    .required(true)
                    .child({
                        let unit_label = match self.memory_unit {
                            MemoryUnit::Mb => "MB",
                            MemoryUnit::Gb => "GB",
                        };
                        let view = cx.entity();
                        let is_mb = self.memory_unit == MemoryUnit::Mb;
                        let is_gb = self.memory_unit == MemoryUnit::Gb;

                        InputGroup::new("memory")
                            .input(InputGroupInput::new(&self.memory).aria_label("Memory amount"))
                            .addon(
                                InputGroupAddon::new("memory-unit")
                                    .align(InputGroupAddonAlignment::InlineEnd)
                                    .child(
                                        InputGroupButton::new("unit-dropdown")
                                            .label(unit_label)
                                            .dropdown_caret(true)
                                            .dropdown_menu(move |menu, window, _cx| {
                                                menu.item(
                                                    PopupMenuItem::new("MB")
                                                        .checked(is_mb)
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, _, cx| {
                                                                this.memory_unit = MemoryUnit::Mb;
                                                                cx.notify();
                                                            },
                                                        )),
                                                )
                                                .item(
                                                    PopupMenuItem::new("GB")
                                                        .checked(is_gb)
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, _, cx| {
                                                                this.memory_unit = MemoryUnit::Gb;
                                                                cx.notify();
                                                            },
                                                        )),
                                                )
                                            }),
                                    ),
                            )
                    }),
            )
            .child(
                field()
                    .label("Storage")
                    .description("Disk size of the Virtual Machine")
                    .required(true)
            )
    }
}
