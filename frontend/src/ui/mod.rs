pub mod app_list;
mod basics;
pub mod panels;
mod router;
mod utils;

use self::{
    panels::{footer, header},
    router::{outlet, Routes},
    utils::shade_color,
};
use eframe::{
    egui::{
        self,
        style::{Selection, Widgets},
        CentralPanel, Context, FontDefinitions, Margin, TextStyle, Visuals,
    },
    epaint::{Color32, FontFamily, FontId, Rounding, Shadow, Stroke},
    App, CreationContext,
};

const X_PADDING: f32 = 2.0;
const Y_PADDING: f32 = 4.0;

const FRAME_ROUNDING: Rounding = Rounding {
    nw: 1.0,
    ne: 1.0,
    sw: 1.0,
    se: 1.0,
};

const DEFAULT_SHADOW: Shadow = Shadow {
    extrusion: 5.0,
    color: MAIN_BG,
};

const MAIN_TEXT_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
const HEADING_COLOR: Color32 = Color32::from_rgb(255, 255, 255); // White
const SUB_HEADING_COLOR: Color32 = Color32::from_rgb(143, 143, 143); // Grey
const BTN_TEXT: Color32 = HEADING_COLOR;
const ACCENT: Color32 = Color32::from_rgb(253, 121, 73); // Orange-fire;
const COMPLIMENTARY: Color32 = Color32::from_rgb(252, 127, 73); // Yellow;
const ADDITIONAL: Color32 = Color32::from_rgb(66, 108, 255); //Bluish;
const ADDITIONAL_2: Color32 = Color32::from_rgb(159, 63, 191); // Purple;
const MAIN_BG: Color32 = Color32::from_rgb(10, 10, 10); // Black
const ERROR_COLOR: Color32 = Color32::from_rgb(254, 0, 0); //Red

pub struct Main {
    frames: u64,
    current_route: Routes,
}

impl Main {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        configure_visuals(&cc.egui_ctx);

        Main {
            frames: 0,
            current_route: Routes::Home,
        }
    }

    pub fn change_route(&mut self, route: Routes) {
        self.current_route = route;
    }
}

impl App for Main {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        header(&ctx, frame, self);
        CentralPanel::default().show(ctx, |ui| outlet(&self.current_route, ui));
        footer(&ctx);
    }
}

pub fn get_def_frame(ctx: &Context) -> egui::Frame {
    let my_frame = egui::Frame {
        inner_margin: Margin::symmetric(5.0, 2.0),
        outer_margin: 0.5.into(),
        rounding: 10.0.into(),
        shadow: Shadow::small_dark(),
        stroke: Stroke::new(0.0, MAIN_BG),
        fill: ctx.style().visuals.window_fill(),
    };

    my_frame
}

fn configure_visuals(ctx: &Context) {
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
        window_shadow: DEFAULT_SHADOW,
        window_fill: MAIN_BG,
        window_stroke: Stroke {
            width: 5.0,
            color: shade_color(MAIN_BG.to_tuple(), 0.2),
        },
        menu_rounding: Rounding::default(),
        panel_fill: MAIN_BG,
        popup_shadow: DEFAULT_SHADOW,
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

fn configure_fonts(ctx: &Context) {
    let mut font_def = FontDefinitions::default();

    font_def.font_data.insert(
        format!("OperatorMono-Medium"),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/OperatorMono-Medium.otf")),
    );

    // Put my font first (highest priority) for proportional text:
    font_def
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "OperatorMono-Medium".to_owned());

    // font_def my font as last fallback for monospace:
    font_def
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("OperatorMono-Medium".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(font_def);
}

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("Heading3".into())
}

fn configure_text_styles(ctx: &Context) {
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
