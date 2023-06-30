use std::{
    sync::{
        mpsc::{channel, Receiver},
        Mutex, MutexGuard,
    },
    thread::{self},
};

use tracker::{
    procs::{get_running_procs, process::ProcessInfo},
    tracking::{get_tracked_procs_by_user, start_tracking, TrackLog},
};

use super::{user_store::use_user_store, ReducerMsg, Store};

lazy_static! {
    static ref APPS_STORE: Mutex<Store<AppState, Actions>> =
        Mutex::new(Store::new(Box::new(reducer)));
}

#[derive(Default)]
pub struct AppState {
    pub tracked_apps: Vec<TrackLog>,
    pub untracked_apps: Vec<ProcessInfo>,
    pub is_fetching_tracked: bool,
    pub is_fetching_untracked: bool,
    pub is_error_untracked: bool,
    pub is_error_tracked: bool,
    tracked_tx: Option<Receiver<Vec<TrackLog>>>,
    untracked_tx: Option<Receiver<Vec<ProcessInfo>>>,
}

fn reducer(state: &mut AppState, msg: Actions) {
    match msg {
        Actions::FetchTrackedApps => {
            if !state.is_fetching_tracked {
                fetch_tracked_apps(state)
            } else if state.tracked_tx.is_some() {
                match state.tracked_tx.as_ref().unwrap().try_recv() {
                    Ok(data) => {
                        state.tracked_apps = data;
                        state.is_fetching_tracked = false
                    }
                    Err(e) => {
                        eprintln!("Couldn't recieve msg from tracked apps channel: {}", e)
                    }
                }
            }
        }
        Actions::AddTrackedApp(username, proc_name) => {
            start_tracking(proc_name, username);
        }
        Actions::DeleteTrackedApp => println!("Delete tracked app"),
        Actions::FetchUntrackedApps => {
            if !state.is_fetching_untracked {
                fetch_untracked_apps(state);
            } else if state.untracked_tx.is_some() {
                match state.untracked_tx.as_ref().unwrap().try_recv() {
                    Ok(data) => {
                        state.untracked_apps = data;
                        state.is_fetching_untracked = false;
                    }
                    Err(e) => {
                        eprintln!("Couldn't recieve msg from untracked apps channel: {}", e)
                    }
                };
            }
        }

        Actions::None => (),
    };
}

fn fetch_untracked_apps(state: &mut AppState) {
    let (rx, tx) = channel();
    thread::spawn(move || match get_running_procs() {
        Ok(procs) => {
            if let Err(e) = rx.send(procs) {
                eprint!("Error sending Untracked AppList: {}", e);
            };
        }
        Err(e) => {
            eprint!("Couldn't get running processes: {}", e)
        }
    });
    state.untracked_tx = Some(tx);
    state.is_fetching_untracked = true;
}

fn fetch_tracked_apps(state: &mut AppState) {
    let (rx, tx) = channel();
    thread::spawn(
        move || match get_tracked_procs_by_user(&use_user_store().selector().username) {
            Ok(tracked_procs) => {
                if let Err(e) = rx.send(tracked_procs) {
                    eprintln!("Error sending Tracked AppList: {}", e);
                };
            }
            Err(e) => {
                eprintln!("Couldn't get tracked processes: {}", e);
            }
        },
    );
    state.tracked_tx = Some(tx);
    state.is_fetching_tracked = true
}

pub fn use_apps_store() -> MutexGuard<'static, Store<AppState, Actions>> {
    APPS_STORE.lock().unwrap()
}

#[derive(Clone, Copy)]
pub enum Actions {
    None,
    FetchTrackedApps,
    FetchUntrackedApps,
    AddTrackedApp(&str, &str),
    DeleteTrackedApp,
}
impl ReducerMsg for Actions {
    type Value = Actions;
}
