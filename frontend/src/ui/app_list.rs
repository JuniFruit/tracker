use eframe::{
    egui::{Button, Label, Layout, RichText, ScrollArea, Separator, Ui, Vec2},
    emath::Align,
};

use crate::store::{
    apps_store::{use_apps_store, Actions},
    user_store::use_user_store,
};

use super::{
    configs::{ACCENT, HEADING_COLOR, SUB_HEADING_COLOR},
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
    pub list: Vec<AppListItem>,
}

impl AppList {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Applications you use"));
        ui.add(Separator::default().spacing(20.0));

        self.use_load_data();

        let is_loading = use_apps_store().selector().is_fetching_tracked;
        let is_error = use_apps_store().selector().is_error_tracked;

        if is_loading {
            ui.label("Loading");
        } else if is_error {
            self.render_if_empty(ui);
        };

        self.render_list(ui);

        ui.add_space(PADDING);
    }

    fn use_load_data(&mut self) {
        if use_apps_store().selector().tracked_apps.len() == 0 {
            use_apps_store().dispatch(Actions::FetchTrackedApps);
        } else {
            self.make_list()
        }
    }

    fn make_list(&mut self) {
        if self.list.len() > 0 {
            return;
        }

        for item in &use_apps_store().selector().tracked_apps {
            self.list
                .push(AppListItem::new(&item.process_name, item.uptime));
        }
    }

    fn render_list(&self, ui: &mut Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &self.list {
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
    pub list: Vec<NotTrackedAppItem>,
    calls: u64,
}

impl NotTrackedAppList {
    pub fn new() -> Self {
        Self {
            list: vec![],
            calls: 0,
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Choose apps to track"));
        ui.add(Separator::default().spacing(20.0));

        self.use_load_data();

        let is_loading = use_apps_store().selector().is_fetching_untracked;
        let is_error = use_apps_store().selector().is_error_untracked;

        if is_loading {
            ui.label("Loading");
        } else if is_error {
            self.render_if_empty(ui);
        };

        self.render_list(ui);

        ui.add_space(PADDING);
    }

    fn use_load_data(&mut self) {
        self.calls += 1;
        println!("Calls: {}", self.calls);
        if use_apps_store().selector().untracked_apps.len() == 0 {
            use_apps_store().dispatch(Actions::FetchUntrackedApps);
        } else {
            self.make_list()
        }
    }

    fn make_list(&mut self) {
        if self.list.len() > 0 {
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
                        &use_user_store().selector().username,
                        &proc_name,
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
