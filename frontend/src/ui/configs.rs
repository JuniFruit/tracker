use eframe::{
    egui::{
        style::{Selection, Widgets},
        Context, FontData, FontDefinitions, Frame, Margin, TextStyle, Visuals,
    },
    emath::Vec2,
    epaint::{Color32, FontFamily, FontId, Rounding, Shadow, Stroke},
    NativeOptions,
};

use super::utils::shade_color;

/* File with ui app configs and user configs structs */

/* UI configs */

pub const X_PADDING: f32 = 2.0;
pub const Y_PADDING: f32 = 4.0;

const FRAME_ROUNDING: Rounding = Rounding {
    nw: 1.0,
    ne: 1.0,
    sw: 1.0,
    se: 1.0,
};

pub const DEFAULT_SHADOW: Shadow = Shadow {
    extrusion: 5.0,
    color: MAIN_BG,
};

pub const MAIN_TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
pub const HEADING_COLOR: Color32 = Color32::from_rgb(255, 255, 255); // White
pub const SUB_HEADING_COLOR: Color32 = Color32::from_rgb(143, 143, 143); // Grey
pub const BTN_TEXT: Color32 = HEADING_COLOR;
pub const ACCENT: Color32 = Color32::from_rgb(253, 121, 73); // Orange-fire;
pub const COMPLIMENTARY: Color32 = Color32::from_rgb(252, 127, 73); // Yellow;
pub const ADDITIONAL: Color32 = Color32::from_rgb(66, 108, 255); //Bluish;
pub const ADDITIONAL_2: Color32 = Color32::from_rgb(159, 63, 191); // Purple;
pub const MAIN_BG: Color32 = Color32::from_rgb(10, 10, 10); // Black
pub const ERROR_COLOR: Color32 = Color32::from_rgb(254, 0, 0); //Red

pub fn get_def_frame(ctx: &Context) -> Frame {
    let my_frame = Frame {
        inner_margin: Margin::symmetric(5.0, 2.0),
        outer_margin: 0.5.into(),
        rounding: 10.0.into(),
        shadow: Shadow::small_dark(),
        stroke: Stroke::new(0.0, MAIN_BG),
        fill: ctx.style().visuals.faint_bg_color,
    };

    my_frame
}

pub fn get_win_options() -> NativeOptions {
    NativeOptions {
        always_on_top: false,
        initial_window_size: Some(Vec2 { x: 400., y: 600. }),
        min_window_size: Some(Vec2 { x: 400., y: 600. }),
        max_window_size: None,
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        decorated: false,
        centered: true,
        resizable: true,
        app_id: Some("app-tracker".to_string()),
        ..Default::default()
    }
}

pub fn configure_visuals(ctx: &Context) {
    let visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(MAIN_TEXT_COLOR),
        widgets: Widgets::default(),
        selection: Selection::default(),
        hyperlink_color: shade_color(ACCENT.to_tuple(), 0.05),
        faint_bg_color: shade_color(MAIN_BG.to_tuple(), 0.05),
        extreme_bg_color: MAIN_BG,
        code_bg_color: MAIN_BG,
        warn_fg_color: ADDITIONAL,
        error_fg_color: ERROR_COLOR,
        window_rounding: FRAME_ROUNDING,
        window_shadow: Shadow::small_dark(),
        window_fill: MAIN_BG,
        window_stroke: Stroke {
            width: 2.0,
            color: shade_color(MAIN_BG.to_tuple(), 0.2),
        },
        menu_rounding: FRAME_ROUNDING,
        panel_fill: MAIN_BG,
        popup_shadow: Shadow::small_dark(),
        resize_corner_size: 4.0,
        text_cursor_width: 2.0,
        text_cursor_preview: true,
        clip_rect_margin: 0.0,
        button_frame: true,
        collapsing_header_frame: false,
        indent_has_left_vline: false,
        striped: false,
        slider_trailing_fill: false,
    };
    ctx.set_visuals(visuals);
}

pub fn configure_fonts(ctx: &Context) {
    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        format!("OperatorMono-Medium"),
        FontData::from_static(include_bytes!("../../assets/fonts/OperatorMono-Medium.otf")),
    );

    // Put my font first (highest priority) for proportional text:
    font_def
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "OperatorMono-Medium".to_owned());

    // font_def my font as last fallback for monospace:
    font_def
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("OperatorMono-Medium".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(font_def);
}

#[inline]
pub fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
pub fn heading3() -> TextStyle {
    TextStyle::Name("Heading3".into())
}

pub fn configure_text_styles(ctx: &Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();

    ctx.set_style(style);
}

/* User configs */

pub struct UserConfig {
    username: String,
    is_logged: bool,
}
