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

use crossterm::style::{PrintStyledContent, Stylize};

pub fn show_text() {

    print_banner();

    println!("This little tool will allow you to turn on and off the screen\nby pressing a key combination.\n");

    #[cfg(target_os = "windows")]
    {
        print!("{}", PrintStyledContent("[Ctrl + Alt + F11]".cyan().bold()));
        println!(" - Disable screen");
        print!("{}", PrintStyledContent("[Ctrl + Alt + F12]".cyan().bold()));
        println!(" - Enable screen");
        return;
    }

    #[cfg(target_os = "linux")]
    {
        print!("{}", PrintStyledContent("[Ctrl + Alt + K]".cyan().bold()));
        println!(" - Disable screen");
        print!("{}", PrintStyledContent("[Ctrl + Alt + L]".cyan().bold()));
        println!(" - Enable screen");
        return;
    }

    #[cfg(target_os = "macos")]
    {
        print!("{}", PrintStyledContent("[Control + Option + K]".cyan().bold()));
        println!(" - Disable screen");
        print!("{}", PrintStyledContent("[Control + Option + L]".cyan().bold()));
        println!(" - Enable screen");
        return;
    }
}
pub fn print_banner() {
    println!(r#"
███████╗ ██████╗██████╗ ███████╗███████╗███╗   ██╗
██╔════╝██╔════╝██╔══██╗██╔════╝██╔════╝████╗  ██║
███████╗██║     ██████╔╝█████╗  █████╗  ██╔██╗ ██║
╚════██║██║     ██╔══██╗██╔══╝  ██╔══╝  ██║╚██╗██║
███████║╚██████╗██║  ██║███████╗███████╗██║ ╚████║
╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═══╝
      ██████╗ ██╗███████╗ █████╗ ██████╗ ██╗     ███████╗██████╗
      ██╔══██╗██║██╔════╝██╔══██╗██╔══██╗██║     ██╔════╝██╔══██╗
      ██║  ██║██║███████╗███████║██████╔╝██║     █████╗  ██████╔╝
      ██║  ██║██║╚════██║██╔══██║██╔══██╗██║     ██╔══╝  ██╔══██╗
      ██████╔╝██║███████║██║  ██║██████╔╝███████╗███████╗██║  ██║
      ╚═════╝ ╚═╝╚══════╝╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝

"#);
}



