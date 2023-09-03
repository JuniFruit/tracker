use std::io::Read;

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
