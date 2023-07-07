mod app_list;
mod badges_page;
mod basics;
mod configs;
mod modals;
mod panels;
mod router;
mod utils;

use self::{
    app_list::{AppList, NotTrackedAppList},
    badges_page::BadgesPage,
    configs::{configure_fonts, configure_text_styles, configure_visuals, get_win_options},
    modals::confirm_close_modal,
    panels::{header, side_menu},
    router::{outlet, Routes},
};

use eframe::{
    egui::{self, CentralPanel},
    run_native, App, CreationContext,
};
use tracker::init_data;

/* Bootstrap file (entry point) of the app */

pub struct Main {
    current_route: Routes,
    tracked_apps: AppList,
    untracked_apps: NotTrackedAppList,
    badges_page: BadgesPage,
    on_close_dialog_open: bool,
    allow_close: bool,
}

impl Main {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        configure_visuals(&cc.egui_ctx);

        Main {
            current_route: Routes::Home,
            tracked_apps: AppList::new(),
            untracked_apps: NotTrackedAppList::new(),
            badges_page: BadgesPage::new(),
            on_close_dialog_open: false,
            allow_close: false,
        }
    }

    pub fn change_route(&mut self, route: Routes) {
        self.current_route = route;
    }
}

impl App for Main {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        header(&ctx, frame, self);
        side_menu(ctx, self);
        CentralPanel::default().show(ctx, |ui| outlet(self, ui));

        if self.on_close_dialog_open {
            confirm_close_modal(ctx, self, frame);
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    fn on_close_event(&mut self) -> bool {
        self.on_close_dialog_open = true;
        self.allow_close
    }
}

pub fn run_app() {
    init_data();
    run_native(
        "App Tracker",
        get_win_options(),
        Box::new(|cc| Box::new(Main::new(cc))),
    )
    .expect("Application Error");
}
