use crate::procs::enum_procs_by_name;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::{
    fs::File,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::{Duration, SystemTime},
};

const STATS_PATH: &str = "./stats.json";

pub fn get_tracker_thread_for_proc(
    _sender: Sender<String>,
    _receiver: Receiver<String>,
    username: &str,
    proc_name: &str,
) -> thread::JoinHandle<()> {
    let mut track_log = TrackLog::new(username, proc_name);
    track_log.set_last_opened(SystemTime::now());

    thread::spawn(move || {
        let interval = Duration::from_secs(20);
        loop {
            let procs = enum_procs_by_name().unwrap();
            let target = procs
                .into_iter()
                .find(|p| p.name() == track_log.process_name);

            if target.is_some() {
                let proc = target.as_ref().unwrap();
                if proc.is_active().unwrap_or(false) {
                    track_log.set_uptime(proc.get_time().unwrap().as_secs());
                    match track_log.save_to_file() {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error saving data: {}", e),
                    }
                }
            }

            thread::sleep(interval);
            eprintln!(
                "Process {} has been up for: {}m",
                &track_log.process_name,
                &track_log.uptime / 60
            );
        }
    })
}
/// Returns locally saved stats in form of vector.
pub fn get_stats_from_file() -> Result<Vec<TrackLog>, Box<dyn Error>> {
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
    pub last_closed: SystemTime,
    pub last_opened: SystemTime,
    pub process_name: String,
    path: String,
}

impl TrackLog {
    pub fn new(username: &str, proc_name: &str) -> Self {
        TrackLog {
            username: String::from(username),
            uptime: 0,
            last_closed: SystemTime::now(),
            last_opened: SystemTime::now(),
            process_name: String::from(proc_name),
            path: STATS_PATH.to_string(),
        }
    }

    pub fn set_process_name(&mut self, new_name: &str) {
        self.process_name = String::from(new_name);
    }

    pub fn add_uptime(&mut self, seconds: u64) {
        self.uptime += seconds;
    }

    // if tracking is implemented by getting values from win api
    pub fn set_uptime(&mut self, seconds: u64) {
        self.uptime = seconds;
    }

    pub fn set_last_opened(&mut self, timestamp: SystemTime) {
        self.last_opened = timestamp
    }
    pub fn set_last_closed(&mut self, timestamp: SystemTime) {
        self.last_closed = timestamp
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
                prev_stats[ind].set_last_closed(self.last_closed);
                prev_stats[ind].set_last_opened(self.last_opened);
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
