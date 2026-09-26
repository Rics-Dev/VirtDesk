use std::rc::Rc;

use gpui_kit::{
    Div, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement, Styled,
    UniformListScrollHandle, div, px, uniform_list,
};
use gpui_kit::{
    base::{h_flex, v_flex},
    component::scroll::ScrollableElement,
};

use super::{status_color, status_dot};
use crate::virtual_machines::manager::{
    machine::{MachineStatus, MachineSummary},
    palette::color,
};

fn machine_metric(label: &'static str, value: &'static str, fill: f32, tint: Hsla) -> Div {
    h_flex()
        .h(px(15.))
        .gap_2()
        .child(
            div()
                .w(px(30.))
                .text_xs()
                .text_color(color(0x777b83))
                .child(label),
        )
        .child(
            div()
                .w(px(68.))
                .h(px(4.))
                .rounded_full()
                .bg(color(0x303237))
                .child(div().w(px(fill)).h(px(4.)).rounded_full().bg(tint)),
        )
        .child(
            div()
                .text_xs()
                .font_family("monospace")
                .text_color(color(0x9a9da4))
                .child(value),
        )
}

fn machine_card(machine: &MachineSummary) -> Div {
    let selected = machine.name == "prod-web-01";
    let card_background = if selected {
        color(0x202522)
    } else {
        color(0x1d1e21)
    };
    let card_border = if selected {
        color(0x315542)
    } else {
        color(0x2a2c30)
    };
    let cpu_tint = if machine.cpu_fill >= 68. {
        color(0xe0645d)
    } else {
        color(0x4e9df4)
    };

    v_flex()
        .h(px(108.))
        .flex_1()
        .min_w_0()
        .gap_1()
        .p_3()
        .rounded_lg()
        .bg(card_background)
        .border_1()
        .border_color(card_border)
        .child(
            h_flex()
                .gap_2()
                .child(status_dot(machine.status, 8.))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_sm()
                        .font_family("monospace")
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(color(0xd9dadd))
                        .truncate()
                        .child(machine.name),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(if machine.starred {
                            color(0xd7c16b)
                        } else {
                            color(0x53565d)
                        })
                        .child(if machine.starred { "★" } else { "" }),
                ),
        )
        .child(
            h_flex()
                .gap_2()
                .child(
                    div()
                        .size(px(15.))
                        .flex()
                        .flex_shrink_0()
                        .rounded_sm()
                        .bg(color(0x292b30))
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(color(0x9da1a8))
                        .items_center()
                        .justify_center()
                        .child(machine.os_mark),
                )
                .child(
                    div()
                        .min_w_0()
                        .text_xs()
                        .text_color(color(0x858991))
                        .truncate()
                        .child(machine.operating_system),
                ),
        )
        .child(machine_metric(
            "CPU",
            machine.cpu_label,
            machine.cpu_fill,
            cpu_tint,
        ))
        .child(machine_metric(
            "MEM",
            machine.memory_label,
            machine.memory_fill,
            color(0x49c28d),
        ))
}

pub(in crate::virtual_machines::manager) fn machine_grid(
    machines: Rc<Vec<MachineSummary>>,
    columns: usize,
) -> impl IntoElement {
    let columns = columns.max(1);
    let mut grid = v_flex().flex_1().min_h_0().gap_3().pr_1();
    for (row_index, machine_row) in machines.chunks(columns).enumerate() {
        let mut row = h_flex()
            .h(px(108.))
            .gap_3()
            .items_stretch()
            .flex_shrink_0()
            .id(("machine-grid-row", row_index));
        for machine in machine_row {
            row = row.child(machine_card(machine));
        }
        grid = grid.child(row);
    }
    grid.overflow_y_scrollbar().id("machine-grid")
}

fn table_column_header(label: &'static str, width: f32) -> Div {
    div()
        .w(px(width))
        .flex_shrink_0()
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(color(0x777b83))
        .child(label)
}

fn table_name_cell(machine: &MachineSummary) -> Div {
    h_flex()
        .w(px(200.))
        .flex_shrink_0()
        .gap_2()
        .child(status_dot(machine.status, 8.))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .font_family("monospace")
                .text_color(color(0xd9dadd))
                .truncate()
                .child(machine.name),
        )
}

fn table_os_cell(machine: &MachineSummary) -> Div {
    h_flex().w(px(160.)).flex_shrink_0().gap_2().child(
        div()
            .min_w_0()
            .text_xs()
            .text_color(color(0x969aa1))
            .truncate()
            .child(machine.operating_system),
    )
}

fn table_cpu_cell(machine: &MachineSummary) -> Div {
    h_flex()
        .w(px(110.))
        .flex_shrink_0()
        .gap_2()
        .child(
            div()
                .w(px(46.))
                .h(px(4.))
                .rounded_full()
                .bg(color(0x303237))
                .child(
                    div()
                        .w(px(machine.cpu_fill.min(46.)))
                        .h(px(4.))
                        .rounded_full()
                        .bg(if machine.cpu_fill >= 68. {
                            color(0xe0645d)
                        } else {
                            color(0x4e9df4)
                        }),
                ),
        )
        .child(
            div()
                .text_xs()
                .font_family("monospace")
                .text_color(color(0x9a9da4))
                .child(machine.cpu_label),
        )
}

fn table_memory_cell(machine: &MachineSummary) -> Div {
    h_flex()
        .w(px(160.))
        .flex_shrink_0()
        .gap_2()
        .child(
            div()
                .w(px(42.))
                .h(px(4.))
                .rounded_full()
                .bg(color(0x303237))
                .child(
                    div()
                        .w(px(machine.memory_fill.min(42.)))
                        .h(px(4.))
                        .rounded_full()
                        .bg(color(0x49c28d)),
                ),
        )
        .child(
            div()
                .text_xs()
                .font_family("monospace")
                .text_color(color(0x9a9da4))
                .child(machine.memory_label),
        )
}

fn table_text_cell(text: &'static str, width: f32) -> Div {
    div()
        .w(px(width))
        .flex_shrink_0()
        .text_xs()
        .font_family("monospace")
        .text_color(color(0x858991))
        .truncate()
        .child(text)
}

fn table_status_cell(status: MachineStatus) -> Div {
    h_flex()
        .w(px(102.))
        .flex_shrink_0()
        .gap_2()
        .child(status_dot(status, 7.))
        .child(
            div()
                .text_xs()
                .text_color(status_color(status))
                .child(status.label()),
        )
}

fn machine_table_row(machine: &MachineSummary) -> Div {
    let mut row = h_flex()
        .h(px(46.))
        .px_3()
        .gap_2()
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(color(0x292b30))
        .child(table_name_cell(machine));

    row = row.child(table_os_cell(machine));
    row = row.child(table_cpu_cell(machine));
    row = row.child(table_memory_cell(machine));
    row = row.child(table_text_cell(machine.ip_address, 170.));
    row = row.child(table_text_cell(machine.uptime, 92.));
    row.child(table_status_cell(machine.status))
}

pub(in crate::virtual_machines::manager) fn machine_list(machines: Rc<Vec<MachineSummary>>) -> Div {
    let mut header = h_flex()
        .h(px(34.))
        .px_3()
        .gap_2()
        .items_center()
        .justify_between()
        .border_b_1()
        .border_color(color(0x303237))
        .child(table_column_header("NAME", 230.));

    header = header.child(table_column_header("SYSTEM", 160.));
    header = header.child(table_column_header("CPU", 110.));
    header = header.child(table_column_header("MEMORY", 160.));
    header = header.child(table_column_header("IP ADDRESS", 170.));
    header = header.child(table_column_header("UPTIME", 92.));
    header = header.child(table_column_header("STATUS", 102.));

    let mut rows = v_flex().flex_1().min_h_0();
    for (index, machine) in machines.iter().enumerate() {
        rows = rows.child(machine_table_row(machine).id(("machine-list-row", index)));
    }

    v_flex().flex_1().min_h_0().overflow_hidden().child(
        v_flex()
            .id("machine-list")
            .w_full()
            .flex_1()
            .min_h_0()
            .overflow_x_scrollbar()
            .child(header)
            .child(rows.overflow_y_scrollbar()),
    )

    // v_flex()
    //     .flex_1()
    //     .min_h_0()
    //     .overflow_hidden()
    //     .child(header)
    //     .child(rows.overflow_y_scrollbar().id("machine-list"))
}
