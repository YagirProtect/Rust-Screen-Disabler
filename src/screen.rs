use std::io;
use crate::os;

pub fn active_screen() -> io::Result<()>{

    #[cfg(windows)]
    return os::windows::set_monitor_power(true);

    #[cfg(target_os = "macos")]
    return os::macos::display_sleep_now();

    #[cfg(target_os = "linux")]
    return os::linux::display_off_best_effort();


    #[allow(unreachable_code)]
    Err(io::Error::new(io::ErrorKind::Other, "unsupported OS"))
}

pub fn disable_screen() -> io::Result<()> {

    #[cfg(windows)]
    return os::windows::set_monitor_power(true);

    #[cfg(target_os = "macos")]
    return os::macos::display_wake();

    #[cfg(target_os = "linux")]
    return os::linux::display_on_best_effort();

    #[allow(unreachable_code)]
    Err(io::Error::new(io::ErrorKind::Other, "unsupported OS"))
}