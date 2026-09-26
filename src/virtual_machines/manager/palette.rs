use gpui_kit::{Hsla, rgb};

pub(super) fn color(hex: u32) -> Hsla {
    Hsla::from(rgb(hex))
}
