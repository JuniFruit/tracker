use eframe::{
    egui::{Button, Label, Layout, RichText, ScrollArea, Separator, Ui, Vec2},
    emath::Align,
};
use tracker::{
    store::{
        apps_store::{use_apps_store, Actions},
        user_store::use_user_store,
    },
    tracking::TrackLog,
};

use super::{
    basics::core_btn,
    configs::{ACCENT, ERROR_COLOR, HEADING_COLOR, SUB_HEADING_COLOR},
    utils::format_time,
};

/* Structs for ui list of applications that are being tracked by the app */

const PADDING: f32 = 5.0;

pub struct AppListItem {
    pub name: String,
    pub uptime: *const u64,
}

impl AppListItem {
    pub fn new(name: &str, uptime: &u64) -> Self {
        Self {
            name: String::from(name),
            uptime,
        }
    }

    pub fn render(&self, ui: &mut Ui, on_delete: impl FnOnce(String) -> ()) {
        ui.add_space(PADDING);
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.colored_label(HEADING_COLOR, format!("App: {}", &self.name));
                ui.colored_label(
                    SUB_HEADING_COLOR,
                    format!("Used for: {}", format_time(unsafe { *self.uptime })),
                );
            })
        });
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            let del_btn = core_btn(ui, ERROR_COLOR, "DELETE")
                .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

            if del_btn.clicked() {
                on_delete(self.name.to_owned());
            }
        });

        ui.add_space(PADDING);
    }
}
/// Apps that our application is tracking. Added by user.
pub struct AppList {
    list: Vec<AppListItem>,
}

impl AppList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Applications you use"));
        ui.add(Separator::default().spacing(20.0));

        self.make_list();
        let is_loading = use_apps_store().selector().is_fetching_tracked;
        if is_loading {
            ui.label("Loading");
            return;
        } else if self.list.len() == 0 {
            self.render_if_empty(ui);
            return;
        };

        self.render_list(ui);

        ui.add_space(PADDING);
    }
    fn make_list(&mut self) {
        if use_apps_store().selector().tracked_apps.len() != self.list.len() {
            self.list = vec![];
            for item in &use_apps_store().selector().tracked_apps {
                self.list
                    .push(AppListItem::new(&item.process_name, &item.uptime))
            }
        }
    }

    fn render_list(&self, ui: &mut Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &self.list {
                item.render(ui, |proc_name| {
                    use_apps_store().dispatch(Actions::DeleteTrackedApp(proc_name));
                });

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
            let add_btn =
                core_btn(ui, ACCENT, "ADD").on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

            if add_btn.clicked() {
                on_add(self.name.to_string());
            }
        });
    }
}

pub struct NotTrackedAppList {
    pub list: Vec<NotTrackedAppItem>,
}

impl NotTrackedAppList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Choose apps to track"));
        ui.add(Separator::default().spacing(20.0));

        self.use_load_data();

        let is_loading = use_apps_store().selector().is_fetching_untracked;

        if is_loading {
            ui.label("Loading");
        } else if self.list.len() == 0 {
            self.render_if_empty(ui);
        };

        self.render_list(ui);

        ui.add_space(PADDING);
    }

    fn use_load_data(&mut self) {
        if use_apps_store().selector().untracked_apps.len() == 0 {
            use_apps_store().dispatch(Actions::FetchUntrackedApps);
        } else {
            self.make_list()
        }
    }

    fn make_list(&mut self) {
        if use_apps_store().selector().untracked_apps.len() == self.list.len() {
            return;
        }

        for item in &use_apps_store().selector().untracked_apps {
            self.list.push(NotTrackedAppItem {
                name: item.name.clone(),
                pid: item.pid,
            });
        }
    }

    fn render_list(&self, ui: &mut Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &self.list {
                item.render(ui, |proc_name| {
                    use_apps_store().dispatch(Actions::AddTrackedApp(
                        use_user_store().selector().username.to_owned(),
                        proc_name,
                    ))
                });

                ui.separator();
            }
        });
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
