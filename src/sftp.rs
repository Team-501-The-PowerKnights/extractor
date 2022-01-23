use std::fs::create_dir_all;
use std::io::Write;
use std::process::Command;

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

/// Sync the files and then delete them from the source if enabled in configuration file.
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
	if config.remove {
		let mut remove_command = Command::new("sftp").arg(&loc).spawn()?;
		remove_command
			.stdin
			.as_mut()
			.unwrap()
			.write_all("rm logfile-*".as_bytes())?;
		remove_command.wait()?;
	}
	Ok(())
}
