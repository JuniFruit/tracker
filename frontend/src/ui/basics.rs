use eframe::egui::{Button, RichText, Ui};
/* Small ui widgets */

pub fn icon_button(ui: &mut Ui, text: &str, on_hover_text: &str, on_click: impl FnOnce() -> ()) {
    let size: f32 = 12.0;

    let btn = ui
        .add(Button::new(RichText::new(text).size(size)))
        .on_hover_text(on_hover_text)
        .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

    if btn.clicked() {
        on_click();
    }
}

pub fn nav_button() {}
