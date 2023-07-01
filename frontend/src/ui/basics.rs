use super::configs::ACCENT;
use eframe::{
    egui::{Button, Context, Image, Response, RichText, Sense, Ui},
    epaint::{vec2, Color32, Vec2},
};
use egui_extras::{self, RetainedImage};
/* Small ui widgets */

pub fn text_small_button(
    ui: &mut Ui,
    text: &str,
    on_hover_text: Option<&str>,
    on_click: impl FnOnce() -> (),
) {
    let size: f32 = 12.0;

    let btn = ui
        .add(Button::new(RichText::new(text).size(size)))
        .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

    if btn.clicked() {
        on_click();
    };

    if on_hover_text != None {
        btn.on_hover_text(on_hover_text.unwrap());
    };
}

pub fn core_btn(ui: &mut Ui, color: Color32, text: &str) -> Response {
    let add_btn = ui
        .add(
            Button::new(RichText::new(text).size(15.0))
                .min_size(Vec2::new(45.0, 25.0))
                .rounding(5.0)
                .fill(color),
        )
        .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

    add_btn
}

pub fn logo_btn(ui: &mut Ui, on_click: impl FnOnce() -> ()) {
    let logo = get_icon_img(ui.ctx(), &ImgIcons::HomeIcon, Some(25.0))
        .sense(Sense::click())
        .bg_fill(ACCENT);

    let logo_btn = ui
        .add(logo)
        .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

    if logo_btn.clicked() {
        on_click()
    }
}

pub enum ImgIcons {
    HomeIcon,
    ListIcon,
    TestIcon,
}
/// Returns [`RetainedImage`] from icon path.

pub fn svg_icon(icon: &ImgIcons) -> RetainedImage {
    let fit_to_size = egui_extras::image::FitTo::Original;

    match icon {
        ImgIcons::HomeIcon => RetainedImage::from_svg_bytes_with_size(
            "home_icon",
            include_bytes!("../../assets/icons/home.svg"),
            fit_to_size,
        ),
        ImgIcons::ListIcon => RetainedImage::from_svg_bytes_with_size(
            "list_icon",
            include_bytes!("../../assets/icons/list.svg"),
            fit_to_size,
        ),
        ImgIcons::TestIcon => RetainedImage::from_svg_bytes_with_size(
            "test_icon",
            include_bytes!("../../assets/icons/horse-icon.svg"),
            fit_to_size,
        ),
    }
    .unwrap()
}
/// Returns [`Image`] with default size of 35.0
pub fn get_icon_img(ctx: &Context, icon: &ImgIcons, size: Option<f32>) -> Image {
    let size = if size != None {
        vec2(size.unwrap(), size.unwrap())
    } else {
        vec2(25.0, 25.0)
    };

    let icon = svg_icon(icon);
    Image::new(icon.texture_id(ctx), size)
}
