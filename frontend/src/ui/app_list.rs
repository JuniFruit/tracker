use std::{
    sync::{
        self,
        mpsc::{Receiver, TryRecvError},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use eframe::{
    egui::{Button, Label, Layout, RichText, ScrollArea, Separator, Ui, Vec2},
    emath::Align,
};
use tracker::{
    get_running_procs, get_tracked_procs_by_user, procs::ProcessInfo, start_tracking,
    tracking::TrackLog,
};

use super::{
    configs::{UserConfig, ACCENT, HEADING_COLOR, SUB_HEADING_COLOR},
    utils::format_time,
};

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
        ui.colored_label(HEADING_COLOR, format!("App: {}", &self.name));
        ui.add_space(3.0);

        ui.colored_label(
            SUB_HEADING_COLOR,
            format!("Used for: {}", format_time(self.uptime)),
        );

        ui.add_space(PADDING);
    }
}
/// Apps that our application is tracking. Added by user.
pub struct AppList {
    pub list: Option<Vec<AppListItem>>,
    data_tx: Option<Receiver<Vec<TrackLog>>>,
}

impl AppList {
    pub fn new() -> Self {
        Self {
            list: None,
            data_tx: None,
        }
    }

    pub fn render(&mut self, ui: &mut Ui, config: &UserConfig) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Applications you use"));
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
            None => {
                self.async_get_data(&config.username);
                self.render_if_empty(ui);
            }
        }

        ui.add_space(PADDING);
    }

    fn async_get_data(&mut self, username: &str) {
        match &self.data_tx {
            Some(tx) => match tx.try_recv() {
                Ok(data) => {
                    self.list = Some(
                        data.into_iter()
                            .map(|p| AppListItem::new(&p.process_name, p.uptime))
                            .collect(),
                    );
                    self.data_tx = None;
                }
                Err(e) => {
                    eprintln!("Couldn't recieve msg from tracked apps channel: {}", e)
                }
            },
            None => {
                self.fetch_tracked_apps(username);
            }
        };
    }

    fn fetch_tracked_apps(&mut self, username: &str) -> JoinHandle<()> {
        let username = username.to_string();
        let (rx, tx) = sync::mpsc::channel();
        let handler = thread::spawn(move || {
            let mut tries: u8 = 0;

            while tries < 5 {
                match get_tracked_procs_by_user(&username) {
                    Ok(tracked_procs) => {
                        if let Err(e) = rx.send(tracked_procs) {
                            eprintln!("Error sending Tracked AppList: {}", e);
                        };
                        break;
                    }
                    Err(e) => {
                        tries += 1;
                        eprintln!("Couldn't get tracked processes: {}", e);
                        thread::sleep(Duration::from_secs(5));
                    }
                }
            }
        });
        self.data_tx = Some(tx);
        handler
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

/// Apps that are currently running in the system but not tracked by the app
pub struct NotTrackedAppItem {
    name: String,
    pid: u32,
}

impl NotTrackedAppItem {
    pub fn render(&self, ui: &mut Ui, on_add: impl FnOnce(String) -> ()) {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.colored_label(HEADING_COLOR, &self.name);
            })
        });
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            let add_btn = ui
                .add(
                    Button::new(RichText::new("ADD").size(15.0))
                        .min_size(Vec2::new(45.0, 25.0))
                        .rounding(5.0)
                        .fill(ACCENT),
                )
                .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

            if add_btn.clicked() {
                on_add(self.name.to_string());
            }
        });
    }
}

pub struct NotTrackedAppList {
    pub list: Option<Vec<NotTrackedAppItem>>,
    data_tx: Option<Receiver<Vec<ProcessInfo>>>,
}

impl NotTrackedAppList {
    pub fn new() -> Self {
        Self {
            list: None,
            data_tx: None,
        }
    }

    pub fn render(&mut self, ui: &mut Ui, config: &UserConfig) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Choose apps to track"));
        ui.add(Separator::default().spacing(20.0));

        match &self.list {
            Some(list) => {
                ScrollArea::new([false, true]).show(ui, |ui| {
                    for item in list {
                        item.render(ui, |proc_name| start_tracking(&proc_name, &config.username));

                        ui.separator();
                    }
                });
            }
            None => {
                self.async_get_data();
                self.render_if_empty(ui);
            }
        };

        ui.add_space(PADDING);
    }

    fn fetch_data(&mut self) -> JoinHandle<()> {
        let (rx, tx) = sync::mpsc::channel();
        let handler = thread::spawn(move || {
            let mut tries: u8 = 0;

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
        });
        self.data_tx = Some(tx);
        handler
    }

    fn async_get_data(&mut self) {
        match &self.data_tx {
            Some(tx) => match tx.try_recv() {
                Ok(data) => {
                    self.list = Some(
                        data.into_iter()
                            .map(|p| NotTrackedAppItem {
                                name: p.name,
                                pid: p.pid,
                            })
                            .collect(),
                    );
                    self.data_tx = None;
                }
                Err(e) => {
                    if e == TryRecvError::Disconnected {
                    } else {
                        eprintln!("Couldn't recieve msg from untracked apps channel: {}", e)
                    }
                }
            },
            None => {
                self.fetch_data();
            }
        };
    }

    fn render_if_empty(&self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(10.0);
            ui.add(Label::new(
                RichText::new("No apps found").color(SUB_HEADING_COLOR),
            ));
        });
    }
}
