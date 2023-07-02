use eframe::egui::Ui;

use super::Main;

/* Defines routes and reacts to changes in route rendering corresponding app page */
#[derive(PartialEq, Clone)]
pub enum Routes {
    Login,
    Home,
    Badges,
    NotTrackedApps,
}

pub fn outlet(app: &mut Main, ui: &mut Ui) {
    match &app.current_route {
        Routes::Login => (),
        Routes::Home => app.tracked_apps.render(ui),
        Routes::Badges => (),
        Routes::NotTrackedApps => app.untracked_apps.render(ui),
    };
}
