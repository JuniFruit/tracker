use crate::procs::enum_procs_by_name;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::io::Write;
use std::{
    fs::File,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, SystemTime},
};

pub fn get_tracker_thread_for_proc(
    sender: Sender<String>,
    receiver: Receiver<String>,
    username: &str,
    proc_name: &str,
) -> thread::JoinHandle<()> {
    let mut track_log = TrackLog::new(username, proc_name);
    track_log.set_last_opened(SystemTime::now());

    thread::spawn(move || {
        let interval = Duration::from_secs(10);
        loop {
            let received_msg = receiver.recv().unwrap_or(String::new());

            println!("Chosen name: {}", track_log.process_name);
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

#[derive(Debug, Deserialize, Serialize)]
struct TrackLog {
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
            path: String::from("./stats.json"),
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
        let mut file = File::create(&self.path)?;
        let serialized = serde_json::to_string_pretty(self)?;
        let content = serialized.as_bytes();

        let mut pos = 0;
        while pos < content.len() {
            let bytes_written = file.write(&content[pos..])?;
            pos += bytes_written;
        }
        Ok(())
    }
}

impl Drop for TrackLog {
    fn drop(&mut self) {
        self.set_last_closed(SystemTime::now());
        self.save_to_file().expect("Application error");
    }
}
