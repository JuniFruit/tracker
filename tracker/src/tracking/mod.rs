pub mod badges;

use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::{fs::File, thread, time::Duration};

use crate::store::apps_store::{use_apps_store, Actions};
use crate::win_funcs::enum_procs_by_name;

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
    get_tracker_thread_for_proc(proc_name)
}

fn get_tracker_thread_for_proc(proc_name: &str) -> Sender<String> {
    let (rx, tx) = mpsc::channel();
    let proc_name = proc_name.to_owned();

    thread::spawn(move || {
        let interval = Duration::from_secs(5);
        let mut elapsed: u64 = 0;

        let mut procs = enum_procs_by_name().unwrap();
        let mut target = procs.into_iter().find(|p| p.name() == proc_name);

        let mut prev_time: Option<u64> = None;

        let l = use_apps_store().selector().tracked_apps.len();

        for i in 0..l {
            if use_apps_store().selector().tracked_apps[i].process_name == proc_name {
                prev_time = Some(use_apps_store().selector().tracked_apps[i].uptime.clone());
                break;
            }
        }
        let mut total_time: u64 = if prev_time.is_some() {
            prev_time.unwrap()
        } else if target.is_some() {
            let t = target.as_ref().unwrap().get_time().unwrap().as_secs();
            t
        } else {
            0
        };

        loop {
            match tx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating tracking: {}", proc_name);
                    break;
                }
                Err(TryRecvError::Empty) => {}
            };
            if target.is_some() {
                let proc = target.as_ref().unwrap();
                if proc.is_active().unwrap_or(false) {
                    use_apps_store()
                        .dispatch(Actions::UpdateAppTime(proc_name.to_owned(), total_time));
                };
                if elapsed % 60 == 0 {
                    use_apps_store().dispatch(Actions::SaveData(proc_name.to_owned()));
                    procs = enum_procs_by_name().unwrap();
                    target = procs.into_iter().find(|p| p.name() == proc_name);
                };
            } else {
                println!(
                    "Terminating tracking: {}. Process is no longer running",
                    proc_name
                );
                break;
            };

            thread::sleep(interval);
            elapsed += interval.as_secs();
            total_time += interval.as_secs();
        }
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
}

impl TrackLog {
    pub fn new(username: &str, proc_name: &str, display_name: &str) -> Self {
        TrackLog {
            username: String::from(username),
            uptime: 0,
            badges: vec![],
            process_name: String::from(proc_name),
            display_name: display_name.to_owned(),
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
