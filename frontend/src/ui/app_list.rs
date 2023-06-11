use std::{
    sync,
    thread::{self, JoinHandle},
    time::Duration,
};

use eframe::{
    egui::{panel::Side, Button, Label, RichText, ScrollArea, Separator, SidePanel, Ui},
    epaint::FontId,
};
use tracker::{get_running_procs, procs::Process};

use super::configs::{HEADING_COLOR, SUB_HEADING_COLOR};

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
/// Apps that our application is tracking. Added by user.
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

/// Apps that are currently running in the system but not tracked by the app
pub struct NotTrackedAppItem {
    name: String,
    pid: u32,
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
    pub list: Option<Vec<NotTrackedAppItem>>,
}

impl NotTrackedAppList {
    pub fn new() -> Self {
        Self { list: None }
    }

    pub fn fetch_data(&mut self) {
        let (rx, tx) = sync::mpsc::channel();

        let handler = thread::spawn(move || {
            let mut tries: u8 = 0;
            thread::sleep(Duration::new(5, 0));
            while tries < 5 {
                match get_running_procs() {
                    Ok(procs) => {
                        if let Err(e) = rx.send(procs) {
                            eprint!("Error sending Untracked AppList: {}", e);
                        };
                        break;
                    }
                    Err(e) => {
                        tries += 1;
                        eprint!("Couldn't get running processes: {}", e)
                    }
                }
            }
        })
        .join();
        match tx.try_recv() {
            Ok(procs) => {
                self.list = Some(
                    procs
                        .into_iter()
                        .map(|p| NotTrackedAppItem {
                            name: p.name,
                            pid: p.pid,
                        })
                        .collect(),
                )
            }
            Err(e) => {
                eprint!("Couldn't receive Untracked app list: {}", e);
            }
        };
    }

    pub fn render(&self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Choose apps to track"));
        ui.add(Separator::default().spacing(20.0));

        match &self.list {
            Some(list) => {
                ScrollArea::new([false, true]).show(ui, |ui| {
                    for item in list {
                        item.render(ui);
                        ui.separator();
                    }
                });
            }
            None => self.render_if_empty(ui),
        };

        ui.add_space(PADDING);
    }

    fn render_if_empty(&self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add(Label::new(
                RichText::new("No apps found").color(SUB_HEADING_COLOR),
            ));
        });
    }
}
