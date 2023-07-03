use eframe::{
    egui::{Label, Layout, RichText, ScrollArea, Separator, Ui},
    emath::Align,
};
use tracker::{store::apps_store::use_apps_store, tracking::badges::Badge};

use super::configs::{HEADING_COLOR, SUB_HEADING_COLOR};

pub struct BadgesPage {
    list: Vec<AppItem>,
}

impl BadgesPage {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.vertical_centered(|ui| ui.heading("Badges"));
        ui.add(Separator::default().spacing(20.0));

        self.make_list();
        let is_loading = use_apps_store()
            .lock()
            .unwrap()
            .selector()
            .is_fetching_tracked;
        if is_loading {
            ui.layout().horizontal_align();
            ui.label("Loading");
            return;
        } else if self.list.len() == 0 {
            self.render_if_empty(ui);
            return;
        };

        self.render_list(ui);
    }

    fn make_list(&mut self) {
        if use_apps_store()
            .lock()
            .unwrap()
            .selector()
            .tracked_apps
            .len()
            != self.list.len()
        {
            self.list = vec![];
            for item in &use_apps_store().lock().unwrap().selector().tracked_apps {
                AppItem::new(&item.display_name, item.badges.clone());
            }
        }
    }

    fn render_list(&mut self, ui: &mut Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &mut self.list {
                item.render(ui);

                ui.separator();
            }
        });
    }

    fn render_if_empty(&self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(10.0);
            ui.add(Label::new(
                RichText::new("No apps are being tracked").color(SUB_HEADING_COLOR),
            ));
        });
    }
}

struct AppItem {
    name: String,
    badges: Vec<Badge>,
}

impl AppItem {
    fn new(name: &str, badges: Vec<Badge>) -> Self {
        Self {
            name: name.to_owned(),
            badges,
        }
    }

    fn render(&self, ui: &mut Ui) {
        ui.add_space(5.0);
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.colored_label(HEADING_COLOR, format!("App: {}", &self.name));
            })
        });
    }

    fn render_badges(&self, ui: &mut Ui) {}
}
