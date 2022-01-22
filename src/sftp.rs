use std::{fs::create_dir_all, io::Write, process::Command};

use anyhow::Result;
use log::info;
use which::which;

use crate::conf::Configuration;

pub fn setup(config: &Configuration) -> Result<()> {
    if !config.destination.exists() {
        create_dir_all(&config.destination)?;
        info!("Destination directory didn't exist so it was created");
    }
    which("sftp")?;
    Ok(())
}

pub fn sync(config: &Configuration) -> Result<()> {
    let loc = format!(
        "{}@{}.local:{}",
        config.username, config.hostname, config.source
    );
    Command::new("sftp")
        .arg("-r")
        .arg(&loc)
        .arg(&config.destination)
        .status()?;
    let mut second_command = Command::new("sftp").arg(&loc).spawn()?;
    second_command
        .stdin
        .as_mut()
        .unwrap()
        .write_all("rm logfile-*".as_bytes())?;
    second_command.wait()?;
    Ok(())
}
