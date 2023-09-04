use std::process::{Command, Stdio};

/// Get current logon username
pub fn get_username() -> std::io::Result<String> {
    let mut child_proc_output = Command::new("whoami").stdout(Stdio::piped()).output()?;

    Ok(String::from_utf8(child_proc_output.stdout).unwrap_or(String::from("Guest")))
}
