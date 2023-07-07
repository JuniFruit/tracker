use std::io::Error;
use std::mem::MaybeUninit;

use std::ptr::NonNull;
use std::time::{Duration, SystemTime};
use std::{io::Result, mem};
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, FILETIME, HMODULE};
use winapi::shared::ntdef::FALSE;
use winapi::um::minwinbase::STILL_ACTIVE;
use winapi::um::winnt::{self, MEMORY_BASIC_INFORMATION};

const MASK: DWORD = winnt::PAGE_EXECUTE_READWRITE
    | winnt::PAGE_EXECUTE_WRITECOPY
    | winnt::PAGE_READWRITE
    | winnt::PAGE_WRITECOPY;

const OPEN_PROCESS_RIGHTS: DWORD = winnt::PROCESS_QUERY_INFORMATION
    | winnt::PROCESS_QUERY_LIMITED_INFORMATION
    | winnt::PROCESS_VM_READ
    | winnt::PROCESS_VM_WRITE
    | winnt::PROCESS_VM_OPERATION;

#[derive(Clone, Debug)]
pub struct Process {
    pid: u32,
    handle: NonNull<c_void>,
    name: String,
}

impl Process {
    pub fn open_proc(pid: u32) -> Result<Self> {
        unsafe {
            NonNull::new(winapi::um::processthreadsapi::OpenProcess(
                OPEN_PROCESS_RIGHTS,
                FALSE as i32,
                pid,
            ))
            .map(|handle| Self {
                pid,
                handle,
                name: String::from("Unknown"),
            })
            .ok_or_else(Error::last_os_error)
        }
    }
    pub fn get_proc_name(&mut self) -> Result<&String> {
        if self.name != "Unknown" {
            return Ok(&self.name);
        }

        let mut module = MaybeUninit::<HMODULE>::uninit();
        let mut size = 0;
        if unsafe {
            winapi::um::psapi::EnumProcessModules(
                self.handle.as_ptr(),
                module.as_mut_ptr(),
                mem::size_of::<HMODULE>() as u32,
                &mut size,
            )
        } as u8
            == FALSE
        {
            return Err(Error::last_os_error());
        }

        let module = unsafe { module.assume_init() };
        let mut buffer = Vec::<u8>::with_capacity(64);
        let length = unsafe {
            winapi::um::psapi::GetModuleBaseNameA(
                self.handle.as_ptr(),
                module,
                buffer.as_mut_ptr().cast(),
                buffer.capacity() as u32,
            )
        };
        if length == 0 {
            return Err(Error::last_os_error());
        }
        unsafe { buffer.set_len(length as usize) }

        let name = String::from_utf8(buffer).unwrap();
        self.name = name;
        Ok(&self.name)
    }

    pub fn get_time(&self) -> Result<Duration> {
        let mut creation_time: FILETIME = create_def_filetime();
        let mut exit_time: FILETIME = create_def_filetime();
        let mut kernel_time: FILETIME = create_def_filetime();
        let mut user_time: FILETIME = create_def_filetime();

        if unsafe {
            winapi::um::processthreadsapi::GetProcessTimes(
                self.handle.as_ptr(),
                &mut creation_time,
                &mut exit_time,
                &mut kernel_time,
                &mut user_time,
            )
        } == 1
        {
            let current_time = SystemTime::now();
            let process_creation_time =
                filetime_to_systemtime(&creation_time) - Duration::from_secs(11644473600); //windows EPOCH difference in secs

            let result = match current_time.duration_since(process_creation_time) {
                Ok(n) => n,
                Err(e) => {
                    println!("Couldn't get value: {}.Defaulting duration to 0", e);
                    Duration::new(0, 0)
                }
            };
            Ok(result)
        } else {
            Err(Error::last_os_error())
        }
    }

    pub fn is_active(&self) -> Result<bool> {
        let mut exit_code: u32 = 0;

        if unsafe {
            winapi::um::processthreadsapi::GetExitCodeProcess(self.handle.as_ptr(), &mut exit_code)
        } == 0
        {
            return Err(Error::last_os_error());
        }

        if exit_code == STILL_ACTIVE {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /* Unused functions */

    pub fn read_memory(&self, addr: usize, n: usize) -> Result<Vec<u8>> {
        let mut buffer = Vec::<u8>::with_capacity(n);
        let mut read = 0;

        if unsafe {
            winapi::um::memoryapi::ReadProcessMemory(
                self.handle.as_ptr(),
                addr as *const _,
                buffer.as_mut_ptr().cast(),
                buffer.capacity(),
                &mut read,
            )
        } as u8
            == FALSE
        {
            return Err(Error::last_os_error());
        }
        unsafe { buffer.set_len(read as usize) };
        Ok(buffer)
    }

    pub fn rescan(&self, new_target: i32, locations: &mut Vec<usize>) {
        let target = new_target.to_ne_bytes();

        locations.retain(|loc| match self.read_memory(*loc, target.len()) {
            Ok(memory) => memory == target,
            Err(_) => false,
        })
    }

    pub fn write_memory(&self, addr: usize, value: &[u8]) -> Result<usize> {
        let mut written = 0;

        if unsafe {
            winapi::um::memoryapi::WriteProcessMemory(
                self.handle.as_ptr(),
                addr as *mut _,
                value.as_ptr().cast(),
                value.len(),
                &mut written,
            )
        } as u8
            == FALSE
        {
            return Err(Error::last_os_error());
        }
        Ok(written)
    }

    pub fn scan_memory(&self, target: i32) -> Vec<usize> {
        let target = target.to_ne_bytes();
        let regions: Vec<MEMORY_BASIC_INFORMATION> = self
            .read_memory_regions()
            .into_iter()
            .filter(|p| (p.Protect & MASK) != 0)
            .collect();

        let mut locations = Vec::with_capacity(regions.len());
        regions.into_iter().for_each(|region| {
            match self.read_memory(region.BaseAddress as _, region.RegionSize) {
                Ok(memory) => {
                    memory
                        .windows(target.len())
                        .enumerate()
                        .for_each(|(offset, window)| {
                            if window == target {
                                locations.push(region.BaseAddress as usize + offset);
                                println!(
                                    "Found exact value at [{:?}+{:x}]",
                                    region.BaseAddress, offset
                                );
                            }
                        })
                }
                Err(e) => eprintln!("Error accessing the mem region: {}", e),
            }
        });
        return locations;
    }

    pub fn read_memory_regions(&self) -> Vec<MEMORY_BASIC_INFORMATION> {
        let mut info = MaybeUninit::uninit();
        let mut regions: Vec<MEMORY_BASIC_INFORMATION> = Vec::new();
        let mut base = 0;

        loop {
            let written = unsafe {
                winapi::um::memoryapi::VirtualQueryEx(
                    self.handle.as_ptr(),
                    base as *const _,
                    info.as_mut_ptr(),
                    mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                )
            };

            if written == 0 {
                break regions;
            }
            let info = unsafe { info.assume_init() };
            base = info.BaseAddress as usize + info.RegionSize;
            regions.push(info);
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe { winapi::um::handleapi::CloseHandle(self.handle.as_mut()) };
    }
}

fn create_def_filetime() -> FILETIME {
    FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    }
}

fn filetime_to_systemtime(filetime: &FILETIME) -> SystemTime {
    let nanos =
        ((u64::from(filetime.dwHighDateTime) << 32) | u64::from(filetime.dwLowDateTime)) * 100;
    let duration = Duration::from_nanos(nanos);

    SystemTime::UNIX_EPOCH + duration
}

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
}

impl ProcessInfo {
    pub fn new(name: &str, pid: u32) -> Self {
        Self {
            name: name.to_string(),
            pid,
        }
    }
}
