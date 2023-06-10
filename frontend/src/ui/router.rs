use eframe::egui::{Ui, Widget};

use super::{
    app_list::{AppList, NotTrackedAppList},
    Main,
};

/* Defines routes and reacts to changes in route rendering corresponding app page */

pub enum Routes {
    Login,
    Home,
    AppPage,
    NotTrackedApps,
}

pub fn outlet(app: &mut Main, ui: &mut Ui) {
    match app.current_route {
        Routes::Login => (),
        Routes::Home => AppList::new().render(ui),
        Routes::AppPage => (),
        Routes::NotTrackedApps => NotTrackedAppList::new().render(ui),
    };
}
