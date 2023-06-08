use crate::ui::{
    app_list::AppList,
    panels::{footer, header},
    Main,
};
use eframe::{egui::CentralPanel, App};
mod app_list;

impl App for Main {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            header(ui);
            AppList::new().render(ui);
            footer(ui);
        });
    }
}
