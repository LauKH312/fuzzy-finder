/// # Gruvbox theme for egui.
use egui::{epaint, style, Color32};

pub fn set_theme(ctx: &egui::Context, theme: Theme) {
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(egui::Visuals {
        override_text_color: Some(theme.text),
        hyperlink_color: theme.blue,
        faint_bg_color: theme.bg,
        extreme_bg_color: theme.bg2,
        code_bg_color: theme.bg2,
        warn_fg_color: theme.orange,
        error_fg_color: theme.red,
        window_fill: theme.bg,
        panel_fill: theme.bg,
        window_stroke: egui::Stroke {
            color: theme.bg2,
            ..old.window_stroke
        },
        widgets: style::Widgets {
            noninteractive: make_widget_visual(old.widgets.noninteractive, &theme, theme.bg),
            inactive: make_widget_visual(old.widgets.inactive, &theme, theme.bg1),
            hovered: make_widget_visual(old.widgets.hovered, &theme, theme.bg2),
            active: make_widget_visual(old.widgets.active, &theme, theme.bg3),
            open: make_widget_visual(old.widgets.open, &theme, theme.gray),
        },
        selection: style::Selection {
            bg_fill: theme
                .blue
                .linear_multiply(if theme == DARK { 0.4 } else { 0.2 }),
            stroke: egui::Stroke {
                color: theme.blue_light,
                ..old.selection.stroke
            },
        },
        window_shadow: epaint::Shadow {
            color: theme.bg,
            ..old.window_shadow
        },
        popup_shadow: epaint::Shadow {
            color: theme.bg,
            ..old.popup_shadow
        },
        ..old
    });
}

fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &Theme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.bg2,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text,
            ..old.fg_stroke
        },
        ..old
    }
}

/// The colors for a theme variant.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Theme {
    pub fg: Color32,
    pub bg: Color32,
    pub text: Color32,
    pub blue: Color32,
    pub blue_light: Color32,
    pub green: Color32,
    pub green_light: Color32,
    pub yellow: Color32,
    pub yellow_light: Color32,
    pub orange: Color32,
    pub orange_light: Color32,
    pub red: Color32,
    pub red_light: Color32,
    pub purple: Color32,
    pub purple_light: Color32,
    pub aqua: Color32,
    pub aqua_light: Color32,
    pub gray: Color32,
    pub gray_light: Color32,
    pub bg1: Color32,
    pub bg2: Color32,
    pub bg3: Color32,
}

const fn color_from_hex(hex: u32) -> Color32 {
    Color32::from_rgb(
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}

pub const DARK: Theme = Theme {
    fg: color_from_hex(0xebdbb2),
    bg: color_from_hex(0x282828),
    text: color_from_hex(0xebdbb2),
    blue: color_from_hex(0x458588),
    blue_light: color_from_hex(0x83a598),
    green: color_from_hex(0x98971a),
    green_light: color_from_hex(0xb8bb26),
    yellow: color_from_hex(0xd79921),
    yellow_light: color_from_hex(0xfabd2f),
    orange: color_from_hex(0xd65d0e),
    orange_light: color_from_hex(0xfe8019),
    red: color_from_hex(0xcc241d),
    red_light: color_from_hex(0xfb4934),
    purple: color_from_hex(0xb16286),
    purple_light: color_from_hex(0xd3869b),
    aqua: color_from_hex(0x689d6a),
    aqua_light: color_from_hex(0x8ec07c),
    gray: color_from_hex(0x928374),
    gray_light: color_from_hex(0xa89984),
    bg1: color_from_hex(0x3c3836),
    bg2: color_from_hex(0x504945),
    bg3: color_from_hex(0x665c54),
};
