use eframe::egui::{Ui, Widget};

use super::{
    app_list::{AppList, NotTrackedAppList},
    Main,
};

/* Defines routes and reacts to changes in route rendering corresponding app page */
#[derive(PartialEq, Clone)]
pub enum Routes {
    Login,
    Home,
    AppPage,
    NotTrackedApps,
}

pub fn outlet(app: &mut Main, ui: &mut Ui) {
    match &app.current_route {
        Routes::Login => (),
        Routes::Home => app.tracked_apps.render(ui, &app.config),
        Routes::AppPage => (),
        Routes::NotTrackedApps => app.untracked_apps.render(ui, &app.config),
    };
}
