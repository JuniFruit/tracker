use std::{thread, time::Duration};

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
        use_user_store()
            .lock()
            .unwrap()
            .dispatch(UserActions::InitConfig);

        let mut tries: u8 = 0;
        // fetch prev tracking data
        loop {
            if tries > 5
                || use_apps_store().lock().unwrap().selector().is_error_tracked
                || use_apps_store()
                    .lock()
                    .unwrap()
                    .selector()
                    .tracked_apps
                    .len()
                    > 0
            {
                break;
            }
            use_apps_store()
                .lock()
                .unwrap()
                .dispatch(Actions::FetchTrackedApps);
            thread::sleep(Duration::from_secs(1));
            tries += 1;
        }
    });

    start_supervisor_thread();
}
