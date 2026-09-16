use ::tracing::info;
use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    base::{StyledExt, h_flex, v_flex},
    component::ActiveTheme,
    component::{
        button::{Button, ButtonVariants},
        tag::Tag,
        TitleBar,
    },
    div,
};

use crate::qemu_backend::{QemuCmd, Format};

#[derive(Default)]
pub struct DashboardView;

impl DashboardView {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    fn render_content(cx: &Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            // 1. Title bar
            .child(
                TitleBar::new()
                    .child(
                        h_flex()
                            .w_full()
                            .items_center()
                            .justify_center()
                            .child("VirtDesk")
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .size_full()
                    .p_6()
                    .bg(cx.theme().background)
                    .text_color(cx.theme().foreground)
                    .items_center()
                    .justify_center()
                    .child(
                        Button::new("create-vm")
                            .primary()
                            .label("Create VM")
                            .on_click(|_, _, _| {
                                // we create here
                                launch_user_vm();
                            }),
                    ),
            )
    }
}

impl Render for DashboardView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Self::render_content(cx)
    }
}


pub fn launch_user_vm() -> std::io::Result<()> {
    let mut vm_process = QemuCmd::new()
        .enable_kvm()
        .memory(2048)
        .smp(2)
        .cdrom("/home/ric/Downloads/ISO/alpine-virt-3.24.1-x86_64.iso")
        .drive("/home/ric/Downloads/ISO/alpine.qcow2", Format::Qcow2)
        .display("default")
        .spawn()?;

    println!("VM started with PID: {}", vm_process.id());
    
    let status = vm_process.wait()?;
    println!("VM stopped with exit status: {status}");
    Ok(())
}
