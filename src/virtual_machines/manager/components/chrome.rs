use gpui_kit::{Context, Div, FontWeight, ParentElement, Styled, div, px};
use gpui_kit::{
    base::{h_flex, v_flex},
    component::TitleBar,
};

use gpui_kit::component::{Icon, IconName};
use gpui_kit::component::{
    Sizable as _,
    button::{Button, ButtonGroup, ButtonVariants as _},
};
use tracing::info;

use super::status_dot;
use crate::virtual_machines::VirtualMachineManager;
use crate::virtual_machines::manager::{
    machine::MachineStatus, palette::color, responsive::ResponsiveLayout,
};

fn sidebar_item(mark: &'static str, label: &'static str, count: &'static str, active: bool) -> Div {
    let background = if active {
        color(0x292b30)
    } else {
        color(0x18191c)
    };
    let foreground = if active {
        color(0xe1e3e7)
    } else {
        color(0x9a9da4)
    };

    h_flex()
        .h(px(34.))
        .px_2()
        .gap_3()
        .rounded_md()
        .bg(background)
        .text_color(foreground)
        .child(
            div()
                .w(px(16.))
                .text_sm()
                .text_color(color(0x81858d))
                .child(mark),
        )
        .child(div().flex_1().text_sm().child(label))
        .child(div().text_xs().text_color(color(0x777b83)).child(count))
}

fn sidebar_group(title: &'static str) -> Div {
    div()
        .pt_5()
        .pb_2()
        .px_2()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(color(0x686c74))
        .child(title)
}

pub(in crate::virtual_machines::manager) fn sidebar() -> Div {
    v_flex()
        .w_full()
        .h_full()
        .gap_1()
        .p_3()
        .bg(color(0x18191c))
        .border_r_1()
        .border_color(color(0x2a2c30))
        .child(
            h_flex()
                .gap_2()
                .px_2()
                .pb_5()
                .child(
                    div()
                        .size(px(28.))
                        .flex()
                        .rounded_lg()
                        .bg(color(0x18372e))
                        .text_color(color(0x64d2a1))
                        .font_weight(FontWeight::BOLD)
                        .items_center()
                        .justify_center()
                        .child("V"),
                )
                .child(
                    v_flex()
                        .gap_0()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(color(0xe6e7e9))
                                .child("VirtDesk"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(color(0x777b83))
                                .child("QEMU / KVM"),
                        ),
                ),
        )
        .child(sidebar_item("▦", "All machines", "12", true))
        .child(sidebar_item("☆", "Starred", "2", false))
        .child(sidebar_item("◷", "Recent", "5", false))
        .child(sidebar_group("ENVIRONMENTS"))
        .child(sidebar_item("◈", "Production", "3", false))
        .child(sidebar_item("◈", "Staging", "2", false))
        .child(sidebar_item("◈", "Development", "4", false))
        .child(sidebar_item("◈", "Other", "3", false))
        .child(div().flex_1())
        .child(
            v_flex()
                .gap_2()
                .p_3()
                .rounded_lg()
                .bg(color(0x202125))
                .border_1()
                .border_color(color(0x2c2e33))
                .child(
                    h_flex()
                        .gap_2()
                        .child(status_dot(MachineStatus::Running, 7.))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(color(0xc8cbd0))
                                .child("Local host connected"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(color(0x777b83))
                        .child("qemu:///session"),
                ),
        )
}

pub(in crate::virtual_machines::manager) fn title_bar() -> TitleBar {
    TitleBar::new()
        .bg(color(0x1d1e21))
        .border_color(color(0x2a2c30))
        .child(
            h_flex()
                .w_full()
                .h_full()
                .justify_between()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(color(0xd5d7db))
                        .child("VirtDesk"),
                )
                .child(
                    Button::new("New VM")
                        .small()
                        .compact()
                        .primary()
                        .cursor_pointer()
                        .label("New VM")
                        .icon(Icon::new(IconName::Plus))
                        .on_click(|_, _, _| {
                            info!("New VM button clicked");
                        }),
                )
                .mr_3(),
        )
}

pub(in crate::virtual_machines::manager) fn workspace_header(
    view_switcher: Div,
    responsive: &ResponsiveLayout,
) -> Div {
    let title = if responsive.show_header_subtitle {
        v_flex()
            .gap_1()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(color(0xe5e7eb))
                    .child("All machines"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(color(0x858991))
                    .child("Manage and connect to your virtual environments"),
            )
    } else {
        v_flex().child(
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(color(0xe5e7eb))
                .child("All machines"),
        )
    };

    let mut controls = h_flex().flex_shrink_0().gap_2();
    controls = controls.child(
        div()
            .px_3()
            .py_2()
            .rounded_md()
            .bg(color(0x1d1e21))
            .border_1()
            .border_color(color(0x2c2e33))
            .text_xs()
            .text_color(color(0x858991))
            .child("⌕   Search machines"),
    );
    controls = controls.child(
        div()
            .px_2()
            .py_2()
            .rounded_md()
            .bg(color(0x1d1e21))
            .border_1()
            .border_color(color(0x2c2e33))
            .text_xs()
            .text_color(color(0x9a9da4))
            .child("☷  Filter"),
    );

    h_flex()
        .justify_between()
        .items_center()
        .gap_3()
        .pb_3()
        .child(title)
        .child(controls.child(view_switcher))
}

pub(in crate::virtual_machines::manager) fn status_bar(
    toggle_sidebar_button: Button,
    toggle_detail_panel_button: Button,
) -> Div {
    h_flex()
        .h(px(30.))
        .flex_shrink_0()
        .justify_between()
        .px_3()
        .bg(color(0x191a1d))
        .border_t_1()
        .border_color(color(0x2a2c30))
        .text_xs()
        .text_color(color(0x777b83))
        .child(
            h_flex()
                .gap_3()
                .child(toggle_sidebar_button)
                .child("12 virtual machines"),
        )
        .child(
            h_flex()
                .gap_4()
                .child(
                    h_flex()
                        .gap_1()
                        .child(status_dot(MachineStatus::Running, 6.))
                        .child(div().text_color(color(0x9da1a8)).child("8 running")),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(status_dot(MachineStatus::Error, 6.))
                        .child(div().text_color(color(0xe47a74)).child("1 error")),
                )
                .child(toggle_detail_panel_button),
        )
}
