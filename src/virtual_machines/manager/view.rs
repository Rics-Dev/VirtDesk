use gpui_kit::base::{Selectable, h_flex, v_flex};
use gpui_kit::component::button::ButtonCustomVariant;
use gpui_kit::{Context, Div, IntoElement, ParentElement, Render, Styled, Window, px};

use gpui_kit::component::resizable::{h_resizable, resizable_panel};

use gpui_kit::component::{
    Sizable as _,
    button::{Button, ButtonVariants as _},
};
use gpui_kit_assets::IconName;

use super::{
    components::{
        machine_detail_panel, machine_grid, machine_list, sidebar, status_bar, title_bar,
        workspace_header,
    },
    palette::color,
    responsive::ResponsiveLayout,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum MachineViewMode {
    #[default]
    Grid,
    List,
}

#[derive(Default)]
pub struct VirtualMachineManager {
    view_mode: MachineViewMode,
    show_sidebar: bool,
    show_vm_panel: bool,
}

impl VirtualMachineManager {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            view_mode: MachineViewMode::List,
            show_sidebar: true,
            show_vm_panel: false,
        }
    }

    fn view_mode_button(
        &self,
        mode: MachineViewMode,
        label: &'static str,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let id = match mode {
            MachineViewMode::Grid => "grid-view-button",
            MachineViewMode::List => "list-view-button",
        };
        let custom_style = ButtonCustomVariant::new(cx)
            .color(color(0x24262a))
            .foreground(color(0x92969e))
            .active(color(0x34363b));

        Button::new(id)
            .custom(custom_style)
            .selected(self.view_mode == mode)
            .cursor_pointer()
            .label(label)
            .icon(if id == "grid-view-button" {
                IconName::LayoutGrid
            } else {
                IconName::LayoutList
            })
            .on_click(cx.listener(move |this, _, _, cx| {
                this.view_mode = mode;
                cx.notify();
            }))
    }

    fn view_mode_switcher(&self, cx: &Context<Self>) -> Div {
        h_flex()
            .gap_1()
            .p_1()
            .rounded_lg()
            .bg(color(0x1d1e21))
            .border_1()
            .border_color(color(0x2c2e33))
            .child(self.view_mode_button(MachineViewMode::List, "List", cx))
            .child(self.view_mode_button(MachineViewMode::Grid, "Grid", cx))
    }

    fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.show_sidebar = !self.show_sidebar;
        cx.notify();
    }

    fn sidebar_button(&self, cx: &Context<Self>) -> Button {
        Button::new("sidebar")
            .small()
            .text()
            .icon(if self.show_sidebar {
                IconName::PanelLeftClose
            } else {
                IconName::PanelLeftOpen
            })
            .on_click(cx.listener(|this, _, _, cx| this.toggle_sidebar(cx)))
    }

    fn toggle_vm_panel(&mut self, cx: &mut Context<Self>) {
        self.show_vm_panel = !self.show_vm_panel;
        cx.notify();
    }

    fn vm_panel_button(&self, cx: &Context<Self>) -> Button {
        Button::new("detail-panel")
            .small()
            .text()
            .icon(if self.show_vm_panel {
                IconName::PanelRightClose
            } else {
                IconName::PanelRightOpen
            })
            .on_click(cx.listener(|this, _, _, cx| this.toggle_vm_panel(cx)))
    }
}

impl Render for VirtualMachineManager {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let responsive = ResponsiveLayout::for_viewport(
            window.viewport_size().width,
            self.view_mode == MachineViewMode::Grid,
        );
        let switcher = self.view_mode_switcher(cx);
        let sidebar_button = self.sidebar_button(cx);
        let vm_panel_button = self.vm_panel_button(cx);
        let machine_content = match self.view_mode {
            MachineViewMode::Grid => v_flex()
                .flex_1()
                .min_h_0()
                .child(machine_grid(responsive.grid_columns)),
            MachineViewMode::List => v_flex().flex_1().min_h_0().child(machine_list(&responsive)),
        };

        let workspace = v_flex()
            .flex_1()
            .min_w_0()
            .min_h_0()
            .gap_1()
            .p_3()
            .child(workspace_header(switcher, &responsive))
            .child(machine_content);

        let content = h_resizable("vm-manager-layout")
            .child(
                resizable_panel()
                    .visible(self.show_sidebar)
                    .size(px(210.))
                    .size_range(px(210.)..px(300.))
                    .flex_none()
                    .child(sidebar()),
            )
            .child(resizable_panel().child(workspace))
            .child(
                resizable_panel()
                    .visible(self.show_vm_panel)
                    .size(px(320.))
                    .size_range(px(320.)..px(420.))
                    .flex_none()
                    .child(machine_detail_panel()),
            );

        // let mut content = h_flex().flex_1().min_w_0().min_h_0().items_stretch();
        // // if responsive.show_sidebar {
        // content = content.child(sidebar());
        // // }
        // content = content.child(workspace);
        // content = content.child(machine_vm_panel());

        v_flex()
            .size_full()
            .bg(color(0x141518))
            .text_color(color(0xd9dadd))
            .child(title_bar())
            .child(content)
            .child(status_bar(sidebar_button, vm_panel_button))
    }
}
