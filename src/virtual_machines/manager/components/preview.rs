use gpui_kit::{Div, FontWeight, Hsla, ParentElement, Styled, div, px};
use gpui_kit::{
    base::{h_flex, v_flex},
    component::scroll::ScrollableElement,
};

use super::status_dot;
use crate::virtual_machines::manager::{machine::MachineStatus, palette::color};

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
        .bg(color(0x0d0f12))
        .border_1()
        .border_color(color(0x30343a))
        .child(
            h_flex()
                .h(px(34.))
                .px_3()
                .gap_2()
                .bg(color(0x181a1e))
                .border_b_1()
                .border_color(color(0x2b2e33))
                .child(div().size(px(8.)).rounded_full().bg(color(0xe45d58)))
                .child(div().size(px(8.)).rounded_full().bg(color(0xd6ae42)))
                .child(div().size(px(8.)).rounded_full().bg(color(0x49c28d)))
                .child(
                    div()
                        .min_w_0()
                        .pl_2()
                        .text_xs()
                        .font_family("monospace")
                        .text_color(color(0x858991))
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
                    color(0x777b83),
                ))
                .child(div().h(px(8.)))
                .child(terminal_line("root@prod-web-01:~# uptime", color(0x64d2a1)))
                .child(terminal_line(
                    " 10:48:12 up 4 days, 11:21, 1 user",
                    color(0xc2c5ca),
                ))
                .child(terminal_line(
                    " load average: 0.95, 0.82, 0.68",
                    color(0xc2c5ca),
                ))
                .child(div().h(px(8.)))
                .child(terminal_line(
                    "root@prod-web-01:~# df -h /",
                    color(0x64d2a1),
                ))
                .child(terminal_line(
                    "Filesystem  Size  Used  Avail  Use%",
                    color(0x858991),
                ))
                .child(terminal_line(
                    "/dev/sda1    60G   18G    40G   31% /",
                    color(0xc2c5ca),
                ))
                .child(div().h(px(8.)))
                .child(
                    h_flex()
                        .gap_1()
                        .child(terminal_line("root@prod-web-01:~#", color(0x64d2a1)))
                        .child(terminal_line("▍", color(0x64d2a1))),
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
                .text_color(color(0x696d74))
                .child(label),
        )
        .child(
            div()
                .text_xs()
                .font_family("monospace")
                .text_color(color(0xb7bbc2))
                .child(value),
        )
}

pub(in crate::virtual_machines::manager) fn machine_detail_panel() -> Div {
    v_flex()
        .w_full()
        .h_full()
        .bg(color(0x191a1d))
        .border_l_1()
        .border_color(color(0x2a2c30))
        .child(
            h_flex()
                .h(px(52.))
                .flex_shrink_0()
                .justify_between()
                .px_4()
                .border_b_1()
                .border_color(color(0x2a2c30))
                .child(
                    h_flex()
                        .gap_2()
                        .child(status_dot(MachineStatus::Running, 8.))
                        .child(
                            div()
                                .text_sm()
                                .font_family("monospace")
                                .text_color(color(0xd9dadd))
                                .child("prod-web-01"),
                        ),
                )
                .child(div().text_xs().text_color(color(0x777b83)).child("×")),
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
                                        .text_color(color(0xd9dadd))
                                        .child("Ubuntu 24.04"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(color(0x777b83))
                                        .child("Linux · 4 vCPU · 8 GB RAM"),
                                ),
                        )
                        .child(
                            div()
                                .flex_shrink_0()
                                .px_2()
                                .py_1()
                                .rounded_full()
                                .bg(color(0x18372e))
                                .text_xs()
                                .text_color(color(0x71d5a9))
                                .child("Running"),
                        ),
                )
                .child(
                    h_flex()
                        .h(px(32.))
                        .flex_shrink_0()
                        .gap_4()
                        .border_b_1()
                        .border_color(color(0x303237))
                        .child(
                            div()
                                .h_full()
                                .px_1()
                                .border_b_1()
                                .border_color(color(0x59c798))
                                .text_xs()
                                .text_color(color(0xe0e2e6))
                                .child("SSH terminal"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(color(0x777b83))
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
                .child(div().h(px(40.)).border_t_1().border_color(color(0x292b30))),
        )
        .child(
            h_flex()
                .h(px(34.))
                .flex_shrink_0()
                .gap_2()
                .px_4()
                .border_t_1()
                .border_color(color(0x2a2c30))
                .child(status_dot(MachineStatus::Running, 7.))
                .child(
                    div()
                        .text_xs()
                        .text_color(color(0x858991))
                        .child("Console preview · display only"),
                ),
        )
}
