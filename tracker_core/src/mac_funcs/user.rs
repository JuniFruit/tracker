use crate::mac_funcs::process::child_stream_to_vec;
use std::process::Command;

/// Get current logon username
pub fn get_username() -> std::io::Result<String> {
    let mut child_proc = Command::new("id").arg("-un").spawn().unwrap();
    let out = child_stream_to_vec(child_proc.stdout.take().expect("!stdout"));
    Ok(String::from_utf8(out).unwrap())
}
