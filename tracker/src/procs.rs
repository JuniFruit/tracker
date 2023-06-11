mod process;

pub use self::process::{Process, ProcessInfo};
use std::io::{Error, ErrorKind};
use std::{io::Result, mem};
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::FALSE;

pub fn enum_procs() -> Result<Vec<u32>> {
    let mut pids = Vec::<DWORD>::with_capacity(1024);
    let mut size = 0;

    if unsafe {
        winapi::um::psapi::EnumProcesses(
            pids.as_mut_ptr(),
            (pids.capacity() * mem::size_of::<DWORD>()) as u32,
            &mut size,
        )
    } as u8
        == FALSE
    {
        return Err(Error::last_os_error());
    }
    let count = size as usize / mem::size_of::<DWORD>();
    unsafe { pids.set_len(count) };
    Ok(pids)
}

pub fn enum_procs_by_name() -> Result<Vec<Process>> {
    let mut opened: u32 = 0;
    let mut tried: u32 = 0;

    let pids = enum_procs()?;
    let mut processes = Vec::with_capacity(pids.capacity());

    pids.into_iter()
        .for_each(|pid| match Process::open_proc(pid) {
            Ok(mut proc) => {
                match proc.get_proc_name() {
                    Ok(name) => {
                        println!("Active process pid: {},named: {}", pid, name)
                    }
                    Err(e) => {
                        // eprintln!("Couldn't get process name with pid: {}.Reason: {}", pid, e)
                    }
                }
                processes.push(proc);
                opened += 1;
            }
            Err(e) => {
                // println!("Failed to open process. Pid: {}.Reason: {}", pid, e);
                tried += 1;
            }
        });
    eprintln!("Enumerated. Opened successfully: {}/{}", opened, tried);
    if opened == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "App couldn't open any process. Try to launch the app with admin rights",
        ));
    }
    return Ok(processes);
}
