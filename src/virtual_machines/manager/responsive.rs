use gpui_kit::{Pixels, px};

pub(super) struct ResponsiveLayout {
    pub(super) show_sidebar: bool,
    pub(super) show_detail_panel: bool,
    pub(super) show_header_search: bool,
    pub(super) show_header_filter: bool,
    pub(super) show_header_subtitle: bool,
    pub(super) grid_columns: usize,
    pub(super) show_os_column: bool,
    pub(super) show_cpu_column: bool,
    pub(super) show_memory_column: bool,
    pub(super) show_ip_column: bool,
    pub(super) show_uptime_column: bool,
}

impl ResponsiveLayout {
    pub(super) fn for_viewport(width: Pixels, is_grid_view: bool) -> Self {
        let show_detail_panel = is_grid_view && width >= px(1480.);
        let grid_columns = if width >= px(1440.) {
                        3
                    } else if width >= px(1060.) {
                        2
                    } else {
                        1
                    };
        // let grid_columns = if show_detail_panel {
        //     2
        // } else if width >= px(1440.) {
        //     3
        // } else if width >= px(1060.) {
        //     2
        // } else {
        //     1
        // };

        Self {
            show_sidebar: width >= px(940.),
            show_detail_panel,
            show_header_search: width >= px(1240.),
            show_header_filter: width >= px(1400.) && !show_detail_panel,
            show_header_subtitle: width >= px(900.),
            grid_columns,
            show_os_column: width >= px(740.),
            show_cpu_column: width >= px(820.),
            show_memory_column: width >= px(1160.),
            show_ip_column: width >= px(1320.),
            show_uptime_column: width >= px(1440.),
        }
    }
}
