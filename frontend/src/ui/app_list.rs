use std::time::SystemTime;

use eframe::egui::{Button, Context, ScrollArea, Separator, Ui, Widget};

use super::{BTN_BG, HEADING_COLOR, SUB_HEADING_COLOR};

/* Structs for ui list of application that is being tracked by the app */

const PADDING: f32 = 5.0;

pub struct AppListItem {
    pub name: String,
    pub uptime: u64,
}

impl AppListItem {
    pub fn new(name: &str, uptime: u64) -> Self {
        AppListItem {
            name: String::from(name),
            uptime,
        }
    }

    pub fn render(&self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.colored_label(HEADING_COLOR, &self.name);
        ui.add_space(3.0);

        ui.colored_label(SUB_HEADING_COLOR, format!("Total used: {}", &self.uptime));

        ui.add_space(PADDING);
    }
}

pub struct AppList {
    pub list: Vec<AppListItem>,
}

impl AppList {
    pub fn new() -> Self {
        let list_iter = (0..20).map(|item| AppListItem::new(&format!("name: {item}"), item));

        AppList {
            list: Vec::from_iter(list_iter),
        }
    }

    pub fn render(&self, ui: &mut Ui) {
        ui.vertical_centered(|ui| ui.heading("Applications you use"));
        ui.add_space(PADDING);
        ui.add(Separator::default().spacing(20.0));

        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &self.list {
                item.render(ui);
                ui.separator();
            }
        });

        ui.add_space(PADDING);
    }
}
