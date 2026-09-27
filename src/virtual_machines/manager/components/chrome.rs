use gpui_kit::{Div, FontWeight, ParentElement, Styled, div, px};
use gpui_kit::{
    base::{h_flex, v_flex},
    component::TitleBar,
};

use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::{Icon, IconName, Sizable as _};
use tracing::info;

use super::status_dot;
use crate::theme::palette;
use crate::virtual_machines::manager::machine::MachineStatus;

fn sidebar_item(mark: &'static str, label: &'static str, count: &'static str, active: bool) -> Div {
    let background = if active {
        palette::current().surface_selected
    } else {
        palette::current().sidebar
    };
    let foreground = if active {
        palette::current().text_primary_bright
    } else {
        palette::current().text_secondary
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
                .text_color(palette::current().text_icon)
                .child(mark),
        )
        .child(div().flex_1().text_sm().child(label))
        .child(
            div()
                .text_xs()
                .text_color(palette::current().text_muted)
                .child(count),
        )
}

fn sidebar_group(title: &'static str) -> Div {
    div()
        .pt_5()
        .pb_2()
        .px_2()
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(palette::current().text_low)
        .child(title)
}

pub(in crate::virtual_machines::manager) fn sidebar() -> Div {
    v_flex()
        .w_full()
        .h_full()
        .gap_1()
        .p_3()
        .bg(palette::current().sidebar)
        .border_r_1()
        .shadow_xl()
        .border_color(palette::current().border)
        .child(
            h_flex()
                .gap_2()
                .px_2()
                .pb_5()
                .child(
                    div()
                        .size(px(28.))
                        .flex()
                        .rounded_sm()
                        .bg(palette::current().surface_success)
                        .text_color(palette::current().text_success)
                        .font_weight(FontWeight::BOLD)
                        .items_center()
                        .justify_center()
                        .child("V"),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(palette::current().text_primary_bright)
                        .child("VirtDesk"),
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
                .bg(palette::current().surface_raised)
                .border_1()
                .border_color(palette::current().border)
                .child(
                    h_flex()
                        .gap_2()
                        .child(status_dot(MachineStatus::Running, 7.))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(palette::current().text_status)
                                .child("Local host connected"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette::current().text_muted)
                        .child("qemu:///session"),
                ),
        )
}

pub(in crate::virtual_machines::manager) fn title_bar() -> TitleBar {
    TitleBar::new()
        .bg(palette::current().surface)
        .border_color(palette::current().border)
        .child(
            h_flex()
                .w_full()
                .h_full()
                .justify_between()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(palette::current().text_primary)
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

pub(in crate::virtual_machines::manager) fn workspace_header(view_switcher: Div) -> Div {
    let title = v_flex()
        .gap_1()
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(palette::current().text_primary_bright)
                .child("All machines"),
        )
        .child(
            div()
                .text_xs()
                .text_color(palette::current().text_dim)
                .child("Manage and connect to your virtual environments"),
        );

    let controls = h_flex().flex_shrink_0().gap_2();
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
        .bg(palette::current().panel)
        .border_t_1()
        .border_color(palette::current().border)
        .text_xs()
        .text_color(palette::current().text_muted)
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
                        .child(
                            div()
                                .text_color(palette::current().text_status)
                                .child("8 running"),
                        ),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(status_dot(MachineStatus::Error, 6.))
                        .child(
                            div()
                                .text_color(palette::current().text_error)
                                .child("1 error"),
                        ),
                )
                .child(toggle_detail_panel_button),
        )
}
