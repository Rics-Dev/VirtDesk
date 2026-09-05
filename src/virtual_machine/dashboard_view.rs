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
            // 2. Main content
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
                    .child(Button::new("dashboard-action").label("Click Me"))
                    .child(
                        h_flex()
                            .gap_10()
                            .child(Tag::primary().child("Primary"))
                            .child(Tag::secondary().child("Secondary")),
                    )
                    .child("Hello")
                    .child(
                        Button::new("dashboard-confirm")
                            .primary()
                            .label("Let's Go!")
                            .on_click(|_, _, _| {
                                info!("dashboard action clicked");
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
