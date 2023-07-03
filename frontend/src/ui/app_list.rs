use eframe::{
    egui::{Context, Label, Layout, RichText, ScrollArea, Separator, Ui},
    emath::Align,
};
use tracker::store::{
    apps_store::{is_app_tracked, use_apps_store, Actions},
    user_store::use_user_store,
};

use super::{
    basics::{core_btn, input_field},
    configs::{ACCENT, ADDITIONAL_2, ERROR_COLOR, HEADING_COLOR, MAIN_BG, SUB_HEADING_COLOR},
    modals::{change_proc_name_modal, confirm_modal},
    utils::format_time,
};

/* Structs for ui list of applications that are being tracked by the app */

const PADDING: f32 = 5.0;

pub struct AppListItem {
    pub name: String,
    pub uptime: *const u64,
    pub display_name: String,
    is_running: *const bool,
    on_edit_modal_open: bool,
    new_display_name: String,
}

impl AppListItem {
    pub fn new(name: &str, uptime: &u64, display_name: &str, is_running: &bool) -> Self {
        Self {
            name: String::from(name),
            uptime,
            on_edit_modal_open: false,
            display_name: if display_name.trim() == "" {
                String::from(name)
            } else {
                String::from(display_name)
            },
            new_display_name: display_name.to_owned(),
            is_running,
        }
    }

    pub fn render(&mut self, ui: &mut Ui, on_delete: impl FnOnce(String, &str) -> ()) {
        ui.add_space(PADDING);
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.colored_label(HEADING_COLOR, format!("App: {}", &self.display_name));
                ui.colored_label(
                    unsafe {
                        if *self.is_running {
                            ACCENT
                        } else {
                            SUB_HEADING_COLOR
                        }
                    },
                    format!("Used for: {}", format_time(unsafe { *self.uptime })),
                );
            })
        });
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            let del_btn = core_btn(ui, ERROR_COLOR, "DELETE")
                .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

            let edit_btn = core_btn(ui, ADDITIONAL_2, "EDIT")
                .on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

            if edit_btn.clicked() {
                self.on_edit_modal_open = !self.on_edit_modal_open;
            }

            if del_btn.clicked() {
                on_delete(self.name.to_owned(), &self.display_name);
            }
        });

        if self.on_edit_modal_open {
            self.render_edit_modal(ui);
        }

        ui.add_space(PADDING);
    }
    fn render_edit_modal(&mut self, ui: &mut Ui) {
        let proc_name = self.name.to_owned();
        let display_name = self.display_name.to_owned();

        change_proc_name_modal(ui.ctx(), &mut self.new_display_name, |input| {
            if input.trim() != "" && display_name != *input {
                self.display_name = input.to_owned();
                use_apps_store()
                    .lock()
                    .unwrap()
                    .dispatch(Actions::ChangeTrackedAppName(
                        proc_name,
                        self.display_name.to_owned(),
                    ));
            };
            self.on_edit_modal_open = false;
        })
    }
}
/// Apps that our application is tracking. Added by user.
pub struct AppList {
    list: Vec<AppListItem>,
    on_delete_modal_open: bool,
    app_to_delete: String,
    app_to_delete_display_name: String,
}

impl AppList {
    pub fn new() -> Self {
        Self {
            list: vec![],
            on_delete_modal_open: false,
            app_to_delete: String::new(),
            app_to_delete_display_name: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Applications you use"));
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

        if self.on_delete_modal_open {
            self.render_confirm_modal(ui.ctx())
        }

        self.render_list(ui);

        ui.add_space(PADDING);
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
                self.list.push(AppListItem::new(
                    &item.process_name,
                    &item.uptime,
                    &item.display_name,
                    &item.is_running,
                ))
            }
        }
    }

    fn render_confirm_modal(&mut self, ctx: &Context) {
        let text = format!(
            "Are you sure you want to delete {} app. All data will be erased forever.",
            self.app_to_delete_display_name
        );

        let modal_ptr: *mut bool = &mut self.on_delete_modal_open;

        confirm_modal(
            ctx,
            &text,
            || {
                use_apps_store()
                    .lock()
                    .unwrap()
                    .dispatch(Actions::DeleteTrackedApp(self.app_to_delete.to_owned()));
                unsafe {
                    modal_ptr.write(false);
                }
            },
            || unsafe {
                modal_ptr.write(false);
            },
        )
    }

    fn render_list(&mut self, ui: &mut Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &mut self.list {
                item.render(ui, |proc_name, display_name| {
                    self.on_delete_modal_open = true;
                    self.app_to_delete = proc_name;
                    self.app_to_delete_display_name = display_name.to_owned();
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
#[derive(Clone)]
pub struct NotTrackedAppItem {
    name: String,
    is_added: bool,
}

impl NotTrackedAppItem {
    pub fn render(&self, ui: &mut Ui, on_add: impl FnOnce(String) -> ()) {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                ui.colored_label(HEADING_COLOR, &self.name);
            })
        });
        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            let text = if self.is_added { "ADDED" } else { "ADD" };
            let color = if self.is_added {
                SUB_HEADING_COLOR
            } else {
                ACCENT
            };
            let add_btn =
                core_btn(ui, color, text).on_hover_cursor(eframe::egui::CursorIcon::PointingHand);

            if add_btn.clicked() {
                if self.is_added {
                    return;
                }
                on_add(self.name.to_string());
            }
        });
    }
}

pub struct NotTrackedAppList {
    list: Vec<NotTrackedAppItem>,
    filtered: Vec<Option<NotTrackedAppItem>>,
    search_term: String,
}

impl NotTrackedAppList {
    pub fn new() -> Self {
        Self {
            list: vec![],
            filtered: vec![],
            search_term: String::new(),
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        ui.vertical_centered(|ui| ui.heading("Choose apps to track"));
        ui.add(Separator::default().spacing(20.0));
        ui.vertical_centered(|ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if input_field(ui, "Search", &mut self.search_term).changed() {
                    self.filter()
                }
            });
            ui.add_space(5.0);
            ui.separator();
        });

        self.use_load_data();

        let is_loading = use_apps_store()
            .lock()
            .unwrap()
            .selector()
            .is_fetching_untracked;

        if is_loading {
            ui.label("Loading");
        }
        if self.list.len() == 0 {
            self.render_if_empty(ui);
        } else {
            self.render_list(ui);
        }

        ui.add_space(PADDING);
    }

    fn use_load_data(&mut self) {
        if use_apps_store()
            .lock()
            .unwrap()
            .selector()
            .untracked_apps
            .len()
            == 0
        {
            use_apps_store()
                .lock()
                .unwrap()
                .dispatch(Actions::FetchUntrackedApps);
        } else {
            self.make_list()
        }
    }

    fn make_list(&mut self) {
        if use_apps_store()
            .lock()
            .unwrap()
            .selector()
            .untracked_apps
            .len()
            != self.list.len()
        {
            self.list = vec![];
            for item in &use_apps_store().lock().unwrap().selector().untracked_apps {
                self.list.push(NotTrackedAppItem {
                    name: item.name.clone(),
                    is_added: false,
                });
            }
            self.filter();
        }
    }

    fn render_list(&self, ui: &mut Ui) {
        ScrollArea::new([false, true]).show(ui, |ui| {
            for item in &self.filtered {
                if item.is_some() {
                    item.as_ref().unwrap().render(ui, |proc_name| {
                        use_apps_store()
                            .lock()
                            .unwrap()
                            .dispatch(Actions::AddTrackedApp(
                                use_user_store().selector().username.to_owned(),
                                proc_name,
                            ))
                    });
                };
            }
        });
    }

    fn filter(&mut self) {
        let list = &self.list;
        self.filtered = list
            .into_iter()
            .map(|item| {
                if item
                    .name
                    .to_lowercase()
                    .starts_with(&self.search_term.to_lowercase())
                {
                    Some(item.to_owned())
                } else {
                    None
                }
            })
            .collect()
    }

    fn render_if_empty(&self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.add_space(40.0);
            ui.add(Label::new(
                RichText::new("No apps running").color(SUB_HEADING_COLOR),
            ));
        });
    }
}
