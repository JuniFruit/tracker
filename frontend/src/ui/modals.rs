use eframe::egui::{self, Context, Layout, Ui};
use tracker::store::apps_store::{use_apps_store, Actions};

use super::{basics::core_btn, configs::ADDITIONAL_2, Main};

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
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down(eframe::emath::Align::Center), |ui| {
                ui.add_space(35.0);
                ui.label(text);
                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.horizontal_centered(|ui| {
                        if core_btn(ui, ADDITIONAL_2, "No").clicked() {
                            on_cancel();
                        };
                        ui.spacing();
                        if core_btn(ui, ADDITIONAL_2, "Yes").clicked() {
                            on_confirm();
                        };
                    });
                });

                ui.add_space(35.0)
            })
        });
}
