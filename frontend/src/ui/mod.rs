mod app_list;
mod basics;
mod configs;
mod panels;
mod router;
mod utils;

use std::thread;

use self::{
    app_list::{AppList, NotTrackedAppList},
    configs::{configure_fonts, configure_text_styles, configure_visuals, get_win_options},
    panels::{footer, header, side_menu},
    router::{outlet, Routes},
};

use eframe::{
    egui::{self, CentralPanel},
    run_native, App, CreationContext,
};

/* Bootstrap file (entry point) of the app */

pub struct Main {
    frames: u64,
    current_route: Routes,
    tracked_apps: AppList,
    untracked_apps: NotTrackedAppList,
}

impl Main {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);
        configure_visuals(&cc.egui_ctx);

        Main {
            frames: 0,
            current_route: Routes::Home,
            tracked_apps: AppList::new(),
            untracked_apps: NotTrackedAppList::new(),
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
        footer(&ctx);
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
}

pub fn run_app() {
    run_native(
        "App Tracker",
        get_win_options(),
        Box::new(|cc| Box::new(Main::new(cc))),
    )
    .expect("Application Error");
}
