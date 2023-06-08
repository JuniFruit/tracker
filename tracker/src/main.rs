mod error;
mod procs;
mod tracking;
mod utils;

use tracking::get_tracker_thread_for_proc;

use crate::{procs::enum_procs_by_name, utils::read_line};
use std::{
    self,
    sync::{self},
    thread::JoinHandle,
};

fn main() {
    let procs = enum_procs_by_name().unwrap();

    let (sender, receiver) = sync::mpsc::channel();

    let mut handles: Vec<JoinHandle<()>> = vec![];

    loop {
        let procs = &procs;
        let mut user_pid = String::with_capacity(10);
        println!("Please enter pid of application you want to track: ");
        read_line(&mut user_pid);

        let user_pid_parsed = user_pid.trim().parse().unwrap_or(0);

        match procs.into_iter().find(|p| p.pid() == user_pid_parsed) {
            Some(p) => {
                if p.name() != "Unknown" {
                    handles.push(get_tracker_thread_for_proc(sender, receiver, "", p.name()));
                    break;
                } else {
                    eprintln!("Couldn't get name of process PID: {}", p.pid())
                }
            }
            _ => eprintln!("Such process is not found. Please check the PID and try again"),
        }
    }

    for handle in handles {
        handle.join().unwrap()
    }
}
