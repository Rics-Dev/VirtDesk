use ::tracing::{error, info};
use color_eyre::eyre::Result;
use gpui_kit::{
    AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, WindowOptions, base::{StyledExt, h_flex}, component::{
        Root,
        button::{Button, ButtonVariants},
        tag::Tag,
    }, div,
};

pub mod tracing;

pub struct HelloWorld;



impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(Button::new("btn").label("Click Me"))
            .child(
                h_flex()
                    .gap_10()
                    .child(Tag::primary().child("Primary"))
                    .child(Tag::secondary().child("Secondary")),
            )
            .child("Hello")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_, _, _| {
                        info!("clicked");
                    }),
            )

    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // let _guard = tracing::init_tracing();
    tracing::init_tracing();

    info!("Starting VirtDesk...");

    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            if let Err(err) = cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| HelloWorld);
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            }) {
                error!(error = ?err, "Failed to initialize window");
                std::process::exit(1);
            }
        })
        .detach();
    });

    Ok(())
}
