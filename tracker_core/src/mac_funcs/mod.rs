pub mod process;
pub mod user;

use std::result::Result;

use sysinfo::{PidExt, ProcessExt, System, SystemExt};

use self::process::ProcessInfo;

pub fn get_running_procs() -> Result<Vec<ProcessInfo>, Box<dyn std::error::Error>> {
    let mut sys = System::new();
    sys.refresh_all();

    let mut procs: Vec<ProcessInfo> = vec![];

    for (pid, process) in sys.processes() {
        procs.push(ProcessInfo::new(process.name(), pid.as_u32()));
    }

    Ok(procs)
}

pub fn hide_console_window() {}
