use std::{io, process::Command};

pub fn dpms_off() -> io::Result<()> {
    run("xset", &["dpms", "force", "off"])
}

pub fn dpms_on() -> io::Result<()> {
    run("xset", &["dpms", "force", "on"])
}

fn run(cmd: &str, args: &[&str]) -> io::Result<()> {
    let st = Command::new(cmd).args(args).status()?;
    if st.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, format!("{cmd} failed: {st}")))
    }
}