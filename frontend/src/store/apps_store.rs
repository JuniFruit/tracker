use std::{
    sync::{
        self,
        mpsc::{Receiver, Sender, TryRecvError},
        Mutex, MutexGuard,
    },
    thread::{self, JoinHandle},
};

use tracker::{get_running_procs, procs::ProcessInfo, tracking::TrackLog};

use super::{ReducerMsg, Store};

lazy_static! {
    static ref APPS_STORE: Mutex<Store<AppState, Actions>> =
        Mutex::new(Store::new(Box::new(reducer)));
}

#[derive(Default)]
pub struct AppState {
    tracked_apps: Vec<TrackLog>,
    untracked_apps: Vec<ProcessInfo>,
    is_fetching_tracked: bool,
    is_fetching_untracked: bool,
}

fn reducer(state: &mut AppState, msg: Actions) {
    match msg {
        Actions::GetTrackedApps => println!("Get tracked apps"),
        Actions::AddTrackedApp => println!("Add tracked app"),
        Actions::DeleteTrackedApp => println!("Delete tracked app"),
        Actions::GetUntrackedApps => {
            let (rx, tx) = sync::mpsc::channel();
            if state.is_fetching_untracked {
                async_get_data(state, tx);
            } else {
                fetch_data(state, rx);
            }
        }
        Actions::None => (),
    };
}

fn fetch_data(state: &mut AppState, rx: Sender<Vec<ProcessInfo>>) -> JoinHandle<()> {
    let handler = thread::spawn(move || {
        let mut tries: u8 = 0;

        while tries < 5 {
            match get_running_procs() {
                Ok(procs) => {
                    if let Err(e) = rx.send(procs) {
                        eprint!("Error sending Untracked AppList: {}", e);
                    };
                    break;
                }
                Err(e) => {
                    tries += 1;
                    eprint!("Couldn't get running processes: {}", e)
                }
            }
        }
    });
    state.is_fetching_untracked = true;
    handler
}

fn async_get_data(state: &mut AppState, tx: Receiver<Vec<ProcessInfo>>) {
    match tx.try_recv() {
        Ok(data) => {
            state.untracked_apps = data;
            state.is_fetching_untracked = false;
        }
        Err(e) => {
            if e == TryRecvError::Disconnected {
            } else {
                eprintln!("Couldn't recieve msg from untracked apps channel: {}", e)
            }
        }
    };
}

pub fn get_apps_store() -> MutexGuard<'static, Store<AppState, Actions>> {
    APPS_STORE.lock().unwrap()
}

#[derive(Clone, Copy)]
pub enum Actions {
    None,
    GetTrackedApps,
    GetUntrackedApps,
    AddTrackedApp,
    DeleteTrackedApp,
}
impl ReducerMsg for Actions {
    type Value = Actions;
}
