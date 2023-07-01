use std::{thread, time::Duration};
use store::apps_store::Actions;

use crate::store::apps_store::use_apps_store;

pub mod procs;
pub mod store;
pub mod tracking;
mod utils;

#[macro_use]
extern crate lazy_static;

pub fn init_data() {
    thread::spawn(move || {
        let mut tries: u8 = 0;

        loop {
            if tries > 5
                || use_apps_store().selector().is_error_tracked
                || use_apps_store().selector().tracked_apps.len() > 0
            {
                break;
            }
            use_apps_store().dispatch(Actions::FetchTrackedApps);
            tries += 1;
            thread::sleep(Duration::from_secs(1));
        }
        use_apps_store().dispatch(Actions::ResumeTrackingAll)
    });
}
