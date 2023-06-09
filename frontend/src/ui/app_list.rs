use super::{HEADING_COLOR, SUB_HEADING_COLOR};
use eframe::egui::{Button, Context, ScrollArea, Separator, Ui, Widget};

/* Structs for ui list of applications that are being tracked by the app */

const PADDING: f32 = 5.0;

pub struct AppListItem {
    pub name: String,
    pub uptime: u64,
}

impl AppListItem {
    pub fn new(name: &str, uptime: u64) -> Self {
        Self {
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

        Self {
            list: Vec::from_iter(list_iter),
        }
    }

    pub fn render(&self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Applications you use"));
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

pub struct NotTrackedAppItem {
    name: String,
}

impl NotTrackedAppItem {
    pub fn render(&self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.colored_label(HEADING_COLOR, &self.name);
        ui.add_space(3.0);

        let add_btn = ui.add(Button::new("Add"));

        ui.add_space(PADDING);
    }
}

pub struct NotTrackedAppList {
    list: Vec<NotTrackedAppItem>,
}

impl NotTrackedAppList {
    pub fn new() -> Self {
        let iter = (0..5)
            .map(|item| NotTrackedAppItem {
                name: format!("{}", item),
            })
            .into_iter();

        Self {
            list: Vec::from_iter(iter),
        }
    }

    pub fn render(&self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Choose apps to track"));
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
