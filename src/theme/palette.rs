use std::sync::{
    OnceLock,
    atomic::{AtomicBool, Ordering},
};

use gpui_kit::{Hsla, rgb};

static IS_DARK: AtomicBool = AtomicBool::new(true);
static DARK: OnceLock<Palette> = OnceLock::new();
static LIGHT: OnceLock<Palette> = OnceLock::new();

/// Colors shared by VirtDesk's screens.
///
/// Components use semantic color names instead of raw hex values, while
/// keeping the palette flat so colors are easy to find and use.
pub(crate) struct Palette {
    pub canvas: Hsla,
    pub sidebar: Hsla,
    pub panel: Hsla,
    pub surface: Hsla,
    pub surface_raised: Hsla,
    pub surface_selected: Hsla,
    pub surface_card_selected: Hsla,
    pub surface_control: Hsla,
    pub surface_active: Hsla,
    pub surface_chip: Hsla,
    pub surface_success: Hsla,

    pub border: Hsla,
    pub border_subtle: Hsla,
    pub border_strong: Hsla,
    pub border_selected: Hsla,

    pub text_primary: Hsla,
    pub text_primary_bright: Hsla,
    pub text_secondary: Hsla,
    pub text_tertiary: Hsla,
    pub text_muted: Hsla,
    pub text_dim: Hsla,
    pub text_low: Hsla,
    pub text_faint: Hsla,
    pub text_icon: Hsla,
    pub text_status: Hsla,
    pub text_success: Hsla,
    pub text_error: Hsla,
    pub text_value: Hsla,

    pub status_running: Hsla,
    pub status_stopped: Hsla,
    pub status_suspended: Hsla,
    pub status_error: Hsla,

    pub accent_blue: Hsla,
    pub accent_red: Hsla,
    pub accent_green: Hsla,
    pub accent_gold: Hsla,
    pub accent_green_bright: Hsla,
    pub accent_green_soft: Hsla,
    pub accent_terminal_red: Hsla,
    pub accent_terminal_yellow: Hsla,
    pub accent_terminal_green: Hsla,

    pub preview_canvas: Hsla,
    pub preview_surface: Hsla,
    pub preview_border: Hsla,
    pub preview_text: Hsla,
    pub preview_text_muted: Hsla,
    pub preview_text_dim: Hsla,
    pub preview_text_success: Hsla,
}

impl Palette {
    fn dark() -> Self {
        Self {
            canvas: color(0x0b0c0e),
            sidebar: color(0x111317),
            panel: color(0x14161b),
            surface: color(0x191c22),
            surface_raised: color(0x1f232b),
            surface_selected: color(0x202a24),
            surface_card_selected: color(0x1b241e),
            surface_control: color(0x222630),
            surface_active: color(0x2a2f3d),
            surface_chip: color(0x17221c),
            surface_success: color(0x142820),

            border: color(0x262a35),
            border_subtle: color(0x1e212b),
            border_strong: color(0x363c4d),
            border_selected: color(0x2e5c43),

            text_primary: color(0xf0f2f5),
            text_primary_bright: color(0xffffff),
            text_secondary: color(0x9fa6b2),
            text_tertiary: color(0x858d9a),
            text_muted: color(0x6a7280),
            text_dim: color(0x565d6a),
            text_low: color(0x454b56),
            text_faint: color(0x333842),
            text_icon: color(0x9fa6b2),
            text_status: color(0x78808d),
            text_success: color(0x4ade80),
            text_error: color(0xf87171),
            text_value: color(0x38bdf8),

            status_running: color(0x4ade80),
            status_stopped: color(0x6a7280),
            status_suspended: color(0xfacc15),
            status_error: color(0xf87171),

            accent_blue: color(0x60a5fa),
            accent_red: color(0xf87171),
            accent_green: color(0x4ade80),
            accent_gold: color(0xfacc15),
            accent_green_bright: color(0x22c55e),
            accent_green_soft: color(0x166534),
            accent_terminal_red: color(0xef4444),
            accent_terminal_yellow: color(0xeab308),
            accent_terminal_green: color(0x22c55e),

            preview_canvas: color(0x08090a),
            preview_surface: color(0x121418),
            preview_border: color(0x262a35),
            preview_text: color(0xd1d5db),
            preview_text_muted: color(0x858d9a),
            preview_text_dim: color(0x565d6a),
            preview_text_success: color(0x4ade80),
        }
    }

    fn light() -> Self {
        Self {
            canvas: color(0xf6f7f9),
            sidebar: color(0xffffff),
            panel: color(0xffffff),
            surface: color(0xf0f2f5),
            surface_raised: color(0xf5f6f8),
            surface_selected: color(0xdbe3ed),
            surface_card_selected: color(0xf1f3f5),
            surface_control: color(0xe2e7ed),
            surface_active: color(0xd0d7e1),
            surface_chip: color(0xe5f4ec),
            surface_success: color(0xe0e4e9),

            border: color(0xcad2dc),
            border_subtle: color(0xd8dde3),
            border_strong: color(0xb9d8c4),
            border_selected: color(0xe6f3ea),

            text_primary: color(0x1a1d21),
            text_primary_bright: color(0x1a1d21),
            text_secondary: color(0x3c434d),
            text_tertiary: color(0x606a76),
            text_muted: color(0x6e7783),
            text_dim: color(0x737d89),
            text_low: color(0x7c8692),
            text_faint: color(0x929ba6),
            text_icon: color(0x68727e),
            text_status: color(0x5d6874),
            text_success: color(0x218252),
            text_error: color(0xc23d38),
            text_value: color(0x4b5563),

            status_running: color(0x218252),
            status_stopped: color(0x707883),
            status_suspended: color(0x9a6c00),
            status_error: color(0xc23d38),

            accent_blue: color(0x2876ce),
            accent_red: color(0xc74641),
            accent_green: color(0x218252),
            accent_gold: color(0x8a6b00),
            accent_green_bright: color(0x218252),
            accent_green_soft: color(0x218252),
            accent_terminal_red: color(0xc23d38),
            accent_terminal_yellow: color(0x9a6c00),
            accent_terminal_green: color(0x218252),

            preview_canvas: color(0x0d0f12),
            preview_surface: color(0x181a1e),
            preview_border: color(0x30343a),
            preview_text: color(0xc2c5ca),
            preview_text_muted: color(0x858991),
            preview_text_dim: color(0x696d74),
            preview_text_success: color(0x64d2a1),
        }
    }
}

/// Set the active appearance.
pub(crate) fn set_dark(is_dark: bool) {
    IS_DARK.store(is_dark, Ordering::Release);
}

/// Get the palette for the current appearance.
pub(crate) fn current() -> &'static Palette {
    if IS_DARK.load(Ordering::Acquire) {
        DARK.get_or_init(Palette::dark)
    } else {
        LIGHT.get_or_init(Palette::light)
    }
}

fn color(hex: u32) -> Hsla {
    Hsla::from(rgb(hex))
}