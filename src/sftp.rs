use std::fs;
use std::io::Write;
use std::process::Command;

use anyhow::{bail, Result};
use log::info;
use which::which;

use crate::conf::Configuration;

pub fn setup(config: &Configuration) -> Result<()> {
	if !config.destination_folder.exists() {
		fs::create_dir_all(&config.destination_folder)?;
		info!("Destination directory didn't exist so it was created");
	}
	which("sftp")?;
	Ok(())
}

/// Sync the files and then delete them from the source if enabled in configuration file.
pub fn sync(config: &Configuration) -> Result<bool> {
	let loc = format!(
		"{}@{}.local:{}",
		config.username,
		config.hostname,
		config.source_folder.display()
	);
	let status = Command::new("sftp")
		.arg("-r")
		.arg(&loc)
		.arg(&config.destination_folder)
		.status()?;
	if !status.success() {
		bail!("Failed to run stfp command with {}", status.to_string());
	}

	if config.remove {
		let mut remove_command = Command::new("sftp").arg(&loc).spawn()?;
		remove_command
			.stdin
			.as_mut()
			.unwrap()
			.write_all("rm logfile-*".as_bytes())?;
		let status = remove_command.wait()?;
		if !status.success() {
			bail!(
				"Failed to remove files from destination with {}",
				status.to_string()
			);
		}
		return Ok(true);
	}
	Ok(false)
}
