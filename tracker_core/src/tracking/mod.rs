pub mod badges;

use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::{fs::File, thread, time::Duration};

use crate::store::apps_store::{use_apps_store, Actions};
use crate::store::user_store::use_user_store;
use crate::tracking::badges::get_badge;

use self::badges::Badge;

const STATS_PATH: &str = "./stats.json";

pub fn get_tracked_procs_by_user(username: &str) -> Result<Vec<TrackLog>, Box<dyn Error>> {
    let procs = get_stats_from_file()?;
    Ok(procs
        .into_iter()
        .filter(|p| p.username == username)
        .collect())
}

pub fn start_tracking<'a>(proc_name: &'a str) -> Sender<String> {
    println!("Started tracking: {}", &proc_name);
    start_tracker_thread_for_proc(proc_name)
}
/// Query running processes and if the number is changed, check if need to start tracking a process
pub fn start_supervisor_thread() {
    thread::spawn(move || {
        let interval = Duration::from_secs(3);
        let mut prev_proc_num: u16 = 0;

        loop {
            // Query and update store with currently running procs
            use_apps_store()
                .lock()
                .unwrap()
                .dispatch(Actions::QueryUntrackedApps);

            thread::sleep(interval);

            let proc_num = use_apps_store()
                .lock()
                .unwrap()
                .selector()
                .untracked_apps
                .len() as u16;

            // Check if any of tracked procs launched to resume tracking
            if proc_num != prev_proc_num {
                let untracked = use_apps_store()
                    .lock()
                    .unwrap()
                    .selector()
                    .untracked_apps
                    .clone();
                let tracked = use_apps_store()
                    .lock()
                    .unwrap()
                    .selector()
                    .tracked_apps
                    .clone();

                tracked.into_iter().for_each(|l| {
                    if !l.is_running {
                        let is_restarted = untracked
                            .iter()
                            .find(|p| p.name == l.process_name)
                            .is_some();

                        if is_restarted {
                            use_apps_store()
                                .lock()
                                .unwrap()
                                .dispatch(Actions::ResumeTracking(l.process_name.to_owned()))
                        }
                    };
                });

                prev_proc_num = proc_num;
            }
        }
    });
}

fn start_tracker_thread_for_proc(proc_name: &str) -> Sender<String> {
    let (rx, tx) = mpsc::channel();
    let proc_name = proc_name.to_owned();

    thread::spawn(move || {
        fn check_is_proc_running(proc_name: &str) -> bool {
            use_apps_store()
                .lock()
                .unwrap()
                .selector()
                .untracked_apps
                .clone()
                .into_iter()
                .find(|p| p.name == proc_name)
                .is_some()
        }

        let interval = Duration::from_secs(5);
        let mut elapsed: u64 = 0;

        let store = use_apps_store().to_owned();
        let mut prev_proc_num = store.lock().unwrap().selector().untracked_apps.len() as u16;

        /* check if process was added earlier  */
        let stored_data = store
            .lock()
            .unwrap()
            .selector()
            .tracked_apps
            .clone()
            .into_iter()
            .find(|p| p.process_name == proc_name);

        let mut total_time: u64 = if stored_data.is_none() {
            0
        } else {
            let t = stored_data.as_ref().unwrap();
            t.uptime
        };

        let mut is_running = check_is_proc_running(&proc_name);

        loop {
            let proc_num = store.lock().unwrap().selector().untracked_apps.len() as u16;

            if prev_proc_num != proc_num {
                is_running = check_is_proc_running(&proc_name);
                prev_proc_num = proc_num;
            }
            /* Check if user terminated tracking (deleted by user) */
            match tx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("User is no longer tracking: {}", proc_name);
                    break;
                }
                Err(TryRecvError::Empty) => {}
            };
            /* Save uptime if process is still running, else save and break */
            if is_running {
                store
                    .lock()
                    .unwrap()
                    .dispatch(Actions::UpdateAppTime(proc_name.to_owned(), total_time));
            } else {
                store
                    .lock()
                    .unwrap()
                    .dispatch(Actions::PauseTracking(proc_name.to_owned()));
                store
                    .lock()
                    .unwrap()
                    .dispatch(Actions::SaveData(proc_name.to_owned()));
                break;
            }
            if elapsed % 120 == 0 {
                store
                    .lock()
                    .unwrap()
                    .dispatch(Actions::SaveData(proc_name.to_owned()));
            };

            /* Check badges */
            if elapsed % 300 == 0 {
                let badge = get_badge(total_time, &use_user_store().selector().username);

                if badge.is_some() {
                    store.lock().unwrap().dispatch(Actions::AddBadgeToProc(
                        badge.unwrap(),
                        proc_name.to_owned(),
                    ));
                }
            }

            thread::sleep(interval);
            elapsed += interval.as_secs();
            total_time += interval.as_secs();
        }
        println!("Tracking thread for: {} terminated", proc_name);
    });
    rx
}
/// Returns locally saved stats in form of vector.
fn get_stats_from_file() -> Result<Vec<TrackLog>, Box<dyn Error>> {
    File::open(STATS_PATH)?;

    let data = fs::read_to_string(STATS_PATH).expect("Unable to read file");
    let mut stats: Vec<TrackLog> = Vec::new();
    if data.trim().len() != 0 {
        stats = serde_json::from_str::<Vec<TrackLog>>(&data)?;
    };
    Ok(stats)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TrackLog {
    pub username: String,
    pub uptime: u64, // seconds
    pub badges: Vec<Badge>,
    pub process_name: String,
    pub display_name: String,
    pub is_running: bool,
}

impl TrackLog {
    pub fn new(username: &str, proc_name: &str, display_name: &str) -> Self {
        TrackLog {
            username: String::from(username),
            uptime: 0,
            badges: vec![],
            process_name: String::from(proc_name),
            display_name: display_name.to_owned(),
            is_running: true, // assumes when we create track log, process is running in sys
        }
    }

    pub fn add_uptime(&mut self, seconds: u64) {
        self.uptime += seconds;
    }

    pub fn set_uptime(&mut self, seconds: u64) {
        self.uptime = seconds;
    }

    pub fn set_display_name(&mut self, new_name: &str) {
        self.display_name = new_name.to_owned();
    }

    pub fn delete_from_file(&self) -> Result<(), Box<dyn Error>> {
        let mut prev_stats = get_stats_from_file()?;

        prev_stats.retain(|log| log.process_name != self.process_name);
        let serialized = serde_json::to_string_pretty(&prev_stats)?;
        fs::write(STATS_PATH, serialized)?;

        Ok(())
    }

    pub fn save_to_file(&self) -> Result<(), Box<dyn Error>> {
        let mut prev_stats = match get_stats_from_file() {
            Ok(data) => data,
            Err(_) => {
                if let Err(e) = File::create(STATS_PATH) {
                    eprintln!("Couldn't create a stat file: {}", e);
                };
                Vec::new()
            }
        };
        // if track log is already in file
        let mut is_in_file = false;
        for ind in 0..prev_stats.len() {
            let curr = &prev_stats[ind];
            if curr.process_name == self.process_name {
                prev_stats[ind].set_uptime(self.uptime);
                prev_stats[ind].set_display_name(&self.display_name);
                prev_stats[ind].badges = self.badges.to_owned();
                prev_stats[ind].is_running = false;
                is_in_file = true;
                break;
            } else {
                is_in_file = false;
            }
        }
        if !is_in_file {
            prev_stats.push(self.clone())
        };

        let serialized = serde_json::to_string_pretty(&prev_stats)?;

        fs::write(STATS_PATH, serialized)?;

        Ok(())
    }
}
