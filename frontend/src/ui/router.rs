use eframe::egui::Ui;

use super::app_list::{AppList, NotTrackedAppList};

pub enum Routes {
    Login,
    Home,
    AppPage,
    NotTrackedApps,
}

pub fn outlet(route: &Routes, ui: &mut Ui) {
    match route {
        Routes::Login => (),
        Routes::Home => AppList::new().render(ui),
        Routes::AppPage => (),
        Routes::NotTrackedApps => NotTrackedAppList::new().render(ui),
    };
}
