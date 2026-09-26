use gpui_kit::{Pixels, px};

#[derive(Clone, Copy)]
pub(super) struct ResponsiveLayout {
    pub(super) show_sidebar: bool,
    pub(super) show_vm_panel: bool,
    pub(super) grid_columns: usize,
}

impl ResponsiveLayout {
    pub(super) fn for_viewport(width: Pixels, is_grid_view: bool) -> Self {
        let show_vm_panel = is_grid_view && width >= px(1480.);
        let grid_columns = if width >= px(1440.) {
            3
        } else if width >= px(1060.) {
            2
        } else {
            1
        };

        Self {
            show_sidebar: width >= px(980.),
            show_vm_panel: width >= px(1480.),
            grid_columns,
        }
    }
}
