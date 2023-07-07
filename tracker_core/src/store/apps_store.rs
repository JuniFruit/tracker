use std::{
    borrow::Borrow,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
    thread,
};

use crate::{
    tracking::{badges::Badge, get_tracked_procs_by_user, start_tracking, TrackLog},
    win_funcs::{get_running_procs, process::ProcessInfo},
};

use super::{user_store::use_user_store, ReducerMsg, Store};

lazy_static! {
    static ref APPS_STORE: Arc<Mutex<Store<AppState, Actions>>> =
        Arc::new(Mutex::new(Store::new(Box::new(reducer))));
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
                        if data.len() == 0 {
                            state.is_error_tracked = true;
                        } else {
                            state.is_error_tracked = false;
                        }
                        state.is_fetching_tracked = false;
                        state.tracked_apps = data;
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
            /* Clean listener channel */
            if rx.is_some() {
                rx.unwrap().send("Terminate".to_owned());
            };

            /* Delete all data from file */
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
        Actions::AddBadgeToProc(badge, proc_name) => {
            let mut log: Option<&mut TrackLog> = None;

            for i in 0..state.tracked_apps.len() {
                if state.tracked_apps[i].process_name == proc_name {
                    log = Some(&mut state.tracked_apps[i]);
                    break;
                }
            }
            let log = log.unwrap();
            let is_added = log.badges.iter().find(|b| b.rank == badge.rank).is_some();

            if !is_added {
                log.badges.push(badge);
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
        Actions::ChangeTrackedAppName(proc_name, new_display_name) => {
            let mut tracked_log: Option<&mut TrackLog> = None;

            for ind in 0..state.tracked_apps.len() {
                if state.tracked_apps[ind].process_name == proc_name {
                    tracked_log = Some(&mut state.tracked_apps[ind]);
                    break;
                }
            }
            match tracked_log {
                Some(log) => log.set_display_name(&new_display_name),
                _ => eprintln!("Cannot change display name: {}. Not found", proc_name),
            }
        }

        Actions::PauseTracking(proc_name) => {
            println!("Pause tracking: {}", proc_name);
            /* Clear channel listener */
            for ind in 0..state.channel_senders.len() {
                if state.channel_senders[ind].proc_name == proc_name {
                    state.channel_senders.remove(ind);
                };
            }
            /* Update running status in tracklog for UI */
            for ind in 0..state.tracked_apps.len() {
                let app = &mut state.tracked_apps[ind];
                if app.process_name == proc_name {
                    app.is_running = false;
                }
            }
        }
        Actions::ResumeTracking(proc_name) => {
            println!("Resume tracking: {}", proc_name);
            for i in 0..state.tracked_apps.len() {
                let app = &mut state.tracked_apps[i];
                if app.process_name == proc_name {
                    let rx = start_tracking(&app.process_name);
                    app.is_running = true;
                    state
                        .channel_senders
                        .push(ChannelSender::new(&app.process_name, rx));
                    break;
                }
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
        Actions::QueryUntrackedApps => {
            state.untracked_apps = get_running_procs().unwrap_or(vec![]);
        }
        Actions::CleanErrorMsg => state.error = None,
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

pub fn use_apps_store() -> Arc<Mutex<Store<AppState, Actions>>> {
    APPS_STORE.clone()
}

pub fn is_app_tracked(proc_name: &str) -> bool {
    let len = use_apps_store()
        .lock()
        .unwrap()
        .selector()
        .tracked_apps
        .len();

    for i in 0..len {
        if use_apps_store().lock().unwrap().selector().tracked_apps[i].process_name == proc_name {
            return true;
        }
    }
    false
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
    CleanErrorMsg,
    SaveAllData,
    ChangeTrackedAppName(String, String),
    QueryUntrackedApps,
    PauseTracking(String),
    ResumeTracking(String),
    AddBadgeToProc(Badge, String),
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
