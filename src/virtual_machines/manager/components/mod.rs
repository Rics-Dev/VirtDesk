mod chrome;
mod machine_views;
mod preview;

pub(super) use chrome::{sidebar, status_bar, title_bar, workspace_header};
pub(super) use machine_views::{machine_grid, machine_list};
pub(super) use preview::machine_detail_panel;

use gpui_kit::{Div, Hsla, Styled, div, px};

use super::machine::MachineStatus;
use crate::theme::palette;

pub(super) fn status_color(status: MachineStatus) -> Hsla {
    match status {
        MachineStatus::Running => palette::current().status_running,
        MachineStatus::Stopped => palette::current().status_stopped,
        MachineStatus::Suspended => palette::current().status_suspended,
        MachineStatus::Error => palette::current().status_error,
    }
}

pub(super) fn status_dot(status: MachineStatus, size: f32) -> Div {
    div()
        .size(px(size))
        .flex_shrink_0()
        .rounded_full()
        .bg(status_color(status))
}
