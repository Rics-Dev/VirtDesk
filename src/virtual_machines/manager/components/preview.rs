use gpui_kit::{Div, FontWeight, Hsla, ParentElement, Styled, div, px};
use gpui_kit::{
    base::{h_flex, v_flex},
    component::scroll::ScrollableElement,
};

use super::status_dot;
use crate::theme::palette;
use crate::virtual_machines::manager::machine::MachineStatus;

fn terminal_line(text: &'static str, tint: Hsla) -> Div {
    div()
        .text_xs()
        .font_family("monospace")
        .text_color(tint)
        .child(text)
}

fn terminal_preview() -> Div {
    v_flex()
        .h(px(284.))
        .flex_shrink_0()
        .rounded_lg()
        .overflow_hidden()
        .bg(palette::current().preview_canvas)
        .border_1()
        .border_color(palette::current().preview_border)
        .child(
            h_flex()
                .h(px(34.))
                .px_3()
                .gap_2()
                .bg(palette::current().preview_surface)
                .border_b_1()
                .border_color(palette::current().preview_border)
                .child(
                    div()
                        .size(px(8.))
                        .rounded_full()
                        .bg(palette::current().accent_terminal_red),
                )
                .child(
                    div()
                        .size(px(8.))
                        .rounded_full()
                        .bg(palette::current().accent_terminal_yellow),
                )
                .child(
                    div()
                        .size(px(8.))
                        .rounded_full()
                        .bg(palette::current().accent_terminal_green),
                )
                .child(
                    div()
                        .min_w_0()
                        .pl_2()
                        .text_xs()
                        .font_family("monospace")
                        .text_color(palette::current().preview_text_dim)
                        .truncate()
                        .child("SSH  ·  root@192.168.1.101"),
                ),
        )
        .child(
            v_flex()
                .gap_1()
                .p_3()
                .child(terminal_line(
                    "Last login: Sat Sep 05 09:41:22 2026",
                    palette::current().preview_text_muted,
                ))
                .child(div().h(px(8.)))
                .child(terminal_line(
                    "root@prod-web-01:~# uptime",
                    palette::current().preview_text_success,
                ))
                .child(terminal_line(
                    " 10:48:12 up 4 days, 11:21, 1 user",
                    palette::current().preview_text,
                ))
                .child(terminal_line(
                    " load average: 0.95, 0.82, 0.68",
                    palette::current().preview_text,
                ))
                .child(div().h(px(8.)))
                .child(terminal_line(
                    "root@prod-web-01:~# df -h /",
                    palette::current().preview_text_success,
                ))
                .child(terminal_line(
                    "Filesystem  Size  Used  Avail  Use%",
                    palette::current().preview_text_dim,
                ))
                .child(terminal_line(
                    "/dev/sda1    60G   18G    40G   31% /",
                    palette::current().preview_text,
                ))
                .child(div().h(px(8.)))
                .child(
                    h_flex()
                        .gap_1()
                        .child(terminal_line(
                            "root@prod-web-01:~#",
                            palette::current().preview_text_success,
                        ))
                        .child(terminal_line("▍", palette::current().preview_text_success)),
                ),
        )
}

fn detail_field(label: &'static str, value: &'static str) -> Div {
    v_flex()
        .gap_1()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(palette::current().text_low)
                .child(label),
        )
        .child(
            div()
                .text_xs()
                .font_family("monospace")
                .text_color(palette::current().text_value)
                .child(value),
        )
}

pub(in crate::virtual_machines::manager) fn machine_detail_panel() -> Div {
    v_flex()
        .w_full()
        .h_full()
        .bg(palette::current().panel)
        .border_l_1()
        .border_color(palette::current().border)
        .child(
            h_flex()
                .h(px(52.))
                .flex_shrink_0()
                .justify_between()
                .px_4()
                .border_b_1()
                .border_color(palette::current().border)
                .child(
                    h_flex()
                        .gap_2()
                        .child(status_dot(MachineStatus::Running, 8.))
                        .child(
                            div()
                                .text_sm()
                                .font_family("monospace")
                                .text_color(palette::current().text_primary)
                                .child("prod-web-01"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(palette::current().text_muted)
                        .child("×"),
                ),
        )
        .child(
            v_flex()
                .flex_1()
                .min_h_0()
                .gap_4()
                .p_4()
                .overflow_y_scrollbar()
                .id("machine-detail-scroll")
                .child(
                    h_flex()
                        .justify_between()
                        .gap_2()
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(palette::current().text_primary)
                                        .child("Ubuntu 24.04"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(palette::current().text_muted)
                                        .child("Linux · 4 vCPU · 8 GB RAM"),
                                ),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .px_2()
                                .py_1()
                                .rounded_full()
                                .bg(palette::current().surface_success)
                                .text_xs()
                                .text_color(palette::current().accent_green_bright)
                                .child("Running"),
                        ),
                )
                .child(
                    h_flex()
                        .h(px(32.))
                        .flex_shrink_0()
                        .gap_4()
                        .border_b_1()
                        .border_color(palette::current().border_strong)
                        .child(
                            div()
                                .h_full()
                                .px_1()
                                .border_b_1()
                                .border_color(palette::current().accent_green_soft)
                                .text_xs()
                                .text_color(palette::current().text_primary_bright)
                                .child("SSH terminal"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(palette::current().text_muted)
                                .child("VM details"),
                        ),
                )
                .child(terminal_preview())
                .child(
                    h_flex()
                        .flex_wrap()
                        .gap_4()
                        .pt_1()
                        .child(detail_field("IP ADDRESS", "192.168.1.101"))
                        .child(detail_field("UPTIME", "4d 11h")),
                )
                .child(
                    div()
                        .h(px(40.))
                        .border_t_1()
                        .border_color(palette::current().border_subtle),
                ),
        )
        .child(
            h_flex()
                .h(px(34.))
                .flex_shrink_0()
                .gap_2()
                .px_4()
                .border_t_1()
                .border_color(palette::current().border)
                .child(status_dot(MachineStatus::Running, 7.))
                .child(
                    div()
                        .text_xs()
                        .text_color(palette::current().text_dim)
                        .child("Console preview · display only"),
                ),
        )
}
