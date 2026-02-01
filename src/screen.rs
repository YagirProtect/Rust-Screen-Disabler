use std::io;
use crate::os::windows;

pub fn active_screen() -> io::Result<()>{

    #[cfg(windows)]
    return windows::set_monitor_power(true);


    #[allow(unreachable_code)]
    Err(io::Error::new(io::ErrorKind::Other, "unsupported OS"))
}

pub fn disable_screen() -> io::Result<()> {

    #[cfg(windows)]
    return windows::set_monitor_power(false);

    #[allow(unreachable_code)]
    Err(io::Error::new(io::ErrorKind::Other, "unsupported OS"))
}