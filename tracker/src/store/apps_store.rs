use std::{
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Mutex, MutexGuard,
    },
    thread::{self},
};

use crate::{
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
    pub error: Option<String>,
    tracked_tx: Option<Receiver<Vec<TrackLog>>>,
    untracked_tx: Option<Receiver<Vec<ProcessInfo>>>,
    channel_senders: Vec<ChannelSender>,
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
                        state.is_error_tracked = false;
                        state.is_fetching_tracked = false;
                    }
                    Err(e) => {
                        if e == TryRecvError::Empty {
                            state.is_error_tracked = true;
                            state.is_fetching_tracked = false;
                            eprintln!("Couldn't recieve msg from tracked apps channel: {}", e)
                        }
                    }
                }
            }
        }
        Actions::AddTrackedApp(username, proc_name) => {
            for i in 0..state.tracked_apps.len() {
                if state.tracked_apps[i].process_name == proc_name {
                    return;
                }
            }

            let rx = start_tracking(&proc_name);
            state
                .tracked_apps
                .push(TrackLog::new(&username, &proc_name, &proc_name));
            state
                .channel_senders
                .push(ChannelSender::new(&proc_name, rx));
        }
        Actions::SaveData(proc_name) => {
            let mut tracked_log: Option<&mut TrackLog> = None;

            for ind in 0..state.tracked_apps.len() {
                if state.tracked_apps[ind].process_name == proc_name {
                    tracked_log = Some(&mut state.tracked_apps[ind]);
                    break;
                }
            }
            match tracked_log {
                Some(log) => {
                    log.save_to_file();
                }
                _ => eprintln!("Cannot save tracked progress: {}. Not found.", proc_name),
            }
        }
        Actions::DeleteTrackedApp(proc_name) => {
            let mut rx: Option<Sender<String>> = None;
            for ind in 0..state.channel_senders.len() {
                if state.channel_senders[ind].proc_name == proc_name {
                    rx = Some(state.channel_senders[ind].rx.clone());
                    break;
                }
            }
            if rx.is_some() {
                if rx.unwrap().send("Terminate".to_owned()).is_err() {
                    return;
                } else {
                    for i in 0..state.tracked_apps.len() {
                        if state.tracked_apps[i].process_name == proc_name {
                            match state.tracked_apps[i].delete_from_file() {
                                Ok(_) => {
                                    state.tracked_apps.remove(i);
                                    break;
                                }
                                Err(e) => {
                                    println!(
                                        "Failed to delete track log {} from file. Reason: {}",
                                        proc_name, e
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        Actions::FetchUntrackedApps => {
            if !state.is_fetching_untracked {
                fetch_untracked_apps(state);
            } else if state.untracked_tx.is_some() {
                match state.untracked_tx.as_ref().unwrap().try_recv() {
                    Ok(data) => {
                        state.untracked_apps = data;
                        state.is_error_untracked = false;
                        state.is_fetching_untracked = false;
                    }
                    Err(e) => {
                        if e == TryRecvError::Empty {
                            state.is_error_untracked = true;
                            state.is_fetching_untracked = false;
                            eprintln!("Couldn't recieve msg from untracked apps channel: {}", e)
                        }
                    }
                };
            }
        }
        Actions::UpdateAppTime(proc_name, secs) => {
            let mut tracked_log: Option<&mut TrackLog> = None;

            for ind in 0..state.tracked_apps.len() {
                if state.tracked_apps[ind].process_name == proc_name {
                    tracked_log = Some(&mut state.tracked_apps[ind]);
                    break;
                }
            }
            match tracked_log {
                Some(log) => {
                    log.set_uptime(secs);
                }
                _ => eprintln!("Cannot update: {}. Not found.", proc_name),
            }
        }

        Actions::ResumeTrackingAll => {
            if state.tracked_apps.len() == 0 {
                return;
            };

            for i in 0..state.tracked_apps.len() {
                let app = &state.tracked_apps[i];
                let rx = start_tracking(&app.process_name);
                state
                    .channel_senders
                    .push(ChannelSender::new(&app.process_name, rx))
            }
        }
        Actions::SaveAllData => {
            if state.tracked_apps.len() == 0 {
                return;
            };

            for i in 0..state.tracked_apps.len() {
                let app = &state.tracked_apps[i];
                match app.save_to_file() {
                    Ok(_) => (),
                    Err(e) => println!("Error saving data for {}. Reason:{}", app.process_name, e),
                }
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
    state.is_error_untracked = false;
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
    state.is_fetching_tracked = true;
    state.is_error_tracked = false;
}

pub fn use_apps_store() -> MutexGuard<'static, Store<AppState, Actions>> {
    APPS_STORE.lock().unwrap()
}

#[derive(Clone)]
pub enum Actions {
    None,
    FetchTrackedApps,
    FetchUntrackedApps,
    AddTrackedApp(String, String),
    UpdateAppTime(String, u64),
    DeleteTrackedApp(String),
    SaveData(String),
    ResumeTrackingAll,
    SaveAllData,
}
impl ReducerMsg for Actions {
    type Value = Actions;
}

struct ChannelSender {
    proc_name: String,
    rx: Sender<String>,
}

impl ChannelSender {
    fn new(proc_name: &str, rx: Sender<String>) -> Self {
        Self {
            proc_name: proc_name.to_owned(),
            rx,
        }
    }
}
