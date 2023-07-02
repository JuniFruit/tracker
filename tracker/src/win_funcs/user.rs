use std::io::Error;
use winapi::um::winbase::GetUserNameW;

/// Get current logon username
pub fn get_username() -> std::io::Result<String> {
    let mut username_buff: [u16; 256] = [0; 256];
    let mut pc_buff: u32 = 256;

    if unsafe { GetUserNameW(username_buff.as_mut_ptr().cast(), &mut pc_buff) } == 0 {
        return Err(Error::last_os_error());
    } else {
        Ok(String::from_utf16_lossy(
            &username_buff[0..(pc_buff - 1) as usize],
        ))
    }
}
