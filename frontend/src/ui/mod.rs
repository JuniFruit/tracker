use eframe::{
    egui::{self, CentralPanel, Context, FontDefinitions, ScrollArea, Style, TextStyle, Visuals},
    epaint::{Color32, FontFamily, FontId},
    App, CreationContext,
};

use self::app_list::{AppList, AppListItem};

pub mod app_list;
pub mod panels;

const X_PADDING: f32 = 2.0;
const Y_PADDING: f32 = 4.0;
const HEADING_COLOR: Color32 = Color32::from_rgb(255, 255, 255); // White
const SUB_HEADING_COLOR: Color32 = Color32::from_rgb(143, 143, 143); // Grey
const BTN_TEXT: Color32 = HEADING_COLOR;
const BTN_BG: Color32 = Color32::from_rgb(0, 0, 0);
const MAIN_BG: Color32 = Color32::from_rgb(30, 31, 34); // Grey-black

pub struct Main {
    frames: u64,
}

impl Main {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        Main { frames: 0 }
    }
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
