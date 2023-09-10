use std::thread;

use crate::store::{apps_store::use_apps_store, user_store::use_user_store};
use store::{apps_store::Actions, user_store::UserActions};

use tracking::start_supervisor_thread;

pub mod mac_funcs;
pub mod store;
pub mod tracking;

#[macro_use]
extern crate lazy_static;

pub fn init_data() {
    thread::spawn(move || {
        // Init user related info
        use_user_store().dispatch(UserActions::InitConfig);

        // fetch prev tracking data

        if use_apps_store().lock().unwrap().selector().is_error_tracked
            || use_apps_store()
                .lock()
                .unwrap()
                .selector()
                .tracked_apps
                .len()
                > 0
        {
            return;
        }
        use_apps_store()
            .lock()
            .unwrap()
            .dispatch(Actions::FetchTrackedApps);
    });

    start_supervisor_thread();
}
