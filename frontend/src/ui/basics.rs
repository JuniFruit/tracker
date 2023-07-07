use eframe::{
    egui::{Button, Response, RichText, Ui},
    epaint::{Color32, Vec2},
};
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

pub fn input_field(ui: &mut Ui, label: &str, input: &mut String) -> Response {
    ui.label(label);
    ui.add_space(10.0);
    ui.text_edit_singleline(input)
}
