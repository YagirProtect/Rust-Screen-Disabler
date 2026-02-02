use std::io;
use crate::os;

pub fn active_screen() -> io::Result<()>{

    #[cfg(target_os = "windows")]
    return os::windows::set_monitor_power(true);

    #[cfg(target_os = "macos")]
    return os::macos::display_sleep_now();

    #[cfg(target_os = "linux")]
    return os::linux::dpms_on();


    #[allow(unreachable_code)]
    Err(io::Error::new(io::ErrorKind::Other, "unsupported OS"))
}

pub fn disable_screen() -> io::Result<()> {

    #[cfg(target_os = "windows")]
    return os::windows::set_monitor_power(false);

    #[cfg(target_os = "macos")]
    return os::macos::display_wake();

    #[cfg(target_os = "linux")]
    return os::linux::dpms_off();

    #[allow(unreachable_code)]
    Err(io::Error::new(io::ErrorKind::Other, "unsupported OS"))
}

pub fn show_text() {
    #[cfg(target_os = "windows")]
    {
        println!("Use Ctrl + Alt + F11 to off a screen");
        println!("Use Ctrl + Alt + F12 to on a screen");
        return;
    }


    #[cfg(target_os = "linux")]
    {
        println!("Use Ctrl + Alt + K to off a screen");
        println!("Use Ctrl + Alt + L to on a screen");
        return;
    }

    #[cfg(target_os = "macos")]
    {
        println!("Use Control + Option + F11 to off a screen");
        println!("Use Control + Option + F12 to on a screen");
        return;
    }
}


