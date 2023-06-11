pub mod procs;
pub mod tracking;
mod utils;

use procs::{enum_procs_by_name, Process, ProcessInfo};
use std::error::Error;

use tracking::{get_stats_from_file, TrackLog};

pub fn get_running_procs() -> Result<Vec<ProcessInfo>, Box<dyn Error>> {
    match enum_procs_by_name() {
        Ok(procs) => Ok(procs
            .into_iter()
            .map(|p| ProcessInfo::new(p.name(), p.pid()))
            .collect()),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_tracked_procs_by_user(username: &str) -> Result<Vec<TrackLog>, Box<dyn Error>> {
    let procs = get_stats_from_file()?;
    Ok(procs
        .into_iter()
        .filter(|p| p.username == username)
        .collect())
}
