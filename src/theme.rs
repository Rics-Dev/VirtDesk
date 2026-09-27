pub(crate) mod palette;

use gpui_kit::{App, Window, WindowAppearance, component::Theme};

/// Keep the component library and VirtDesk's own palette in sync with the
/// appearance reported for this window.
pub(crate) fn sync_system_appearance(window: &mut Window, cx: &mut App) {
    palette::set_dark(matches!(
        window.appearance(),
        WindowAppearance::Dark | WindowAppearance::VibrantDark
    ));
    Theme::sync_system_appearance(Some(window), cx);
}
