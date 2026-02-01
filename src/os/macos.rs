use std::{io, process::Command};

pub fn display_sleep_now() -> io::Result<()> {
    run("pmset", &["displaysleepnow"])
}

pub fn display_wake() -> io::Result<()> {
    run("caffeinate", &["-u", "-t", "1"])
}

fn run(cmd: &str, args: &[&str]) -> io::Result<()> {
    let st = Command::new(cmd).args(args).status()?;
    if st.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("{cmd} failed: {st}")))
    }
}