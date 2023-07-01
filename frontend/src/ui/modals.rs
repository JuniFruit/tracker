use eframe::{
    egui::{self, Context, Layout},
    emath::{Align, Align2},
    epaint::Rounding,
};
use tracker::store::apps_store::{use_apps_store, Actions};

use super::{
    basics::core_btn,
    configs::{get_def_frame, ADDITIONAL_2, DEFAULT_SHADOW, ERROR_COLOR, FRAME_ROUNDING, MAIN_BG},
    utils::shade_color,
    Main,
};

pub fn confirm_close_modal(ctx: &Context, app: &mut Main, frame: &mut eframe::Frame) {
    let text =
        "Are you sure you want to quit? Apps are not being tracked when the application is closed.";

    confirm_modal(
        ctx,
        text,
        || {
            app.allow_close = true;
            use_apps_store().dispatch(Actions::SaveAllData);
            frame.close()
        },
        || app.on_close_dialog_open = false,
    );
}

pub fn confirm_modal(
    ctx: &Context,
    text: &str,
    on_confirm: impl FnOnce() -> (),
    on_cancel: impl FnOnce() -> (),
) {
    egui::Window::new("Confirm action")
        .resizable(false)
        .collapsible(false)
        .frame(
            get_def_frame(ctx)
                .fill(shade_color(MAIN_BG.to_tuple(), 0.03))
                .rounding(FRAME_ROUNDING)
                .shadow(DEFAULT_SHADOW),
        )
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down(eframe::emath::Align::Center), |ui| {
                ui.add_space(35.0);
                ui.label(text);
                ui.add_space(20.0);
                ui.with_layout(ui.layout().with_main_align(Align::Center), |ui| {
                    if core_btn(ui, ADDITIONAL_2, "No").clicked() {
                        on_cancel();
                    };

                    if core_btn(ui, ADDITIONAL_2, "Yes").clicked() {
                        on_confirm();
                    };
                });

                ui.add_space(35.0)
            })
        });
}

pub fn error_modal(ctx: &Context, text: &str, on_confirm: impl FnOnce() -> ()) {
    egui::Window::new("Error")
        .resizable(false)
        .collapsible(false)
        .frame(
            get_def_frame(ctx)
                .fill(shade_color(MAIN_BG.to_tuple(), 0.03))
                .rounding(FRAME_ROUNDING)
                .shadow(DEFAULT_SHADOW),
        )
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down(eframe::emath::Align::Center), |ui| {
                ui.add_space(35.0);
                ui.label(text);
                ui.add_space(20.0);
                ui.with_layout(ui.layout().with_main_align(Align::Center), |ui| {
                    if core_btn(ui, ERROR_COLOR, "Yes").clicked() {
                        on_confirm();
                    };
                });

                ui.add_space(35.0)
            })
        });
}
