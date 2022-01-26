use std::fs;
use std::path::PathBuf;

use conf::Configuration;

use log::{info, LevelFilter};
use parse::LogFile;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod conf;
mod parse;
mod sftp;

fn main() {
	TermLogger::init(
		LevelFilter::Trace,
		Config::default(),
		TerminalMode::Stdout,
		ColorChoice::Auto,
	)
	.expect("Failed to configure logger");

	let config = Configuration::read().expect("Failed to read configuration file");
	info!("Loaded configuration file");

	sftp::setup(&config).expect("Failed to setup for sftp usage");
	info!("Setup for SFTP");

	info!("Starting SFTP");
	let removed = sftp::sync(&config).expect("Failed to run sftp");
	info!(
		"Transferred {}files",
		if removed { "and removed " } else { "" }
	);

	let mut real_logs = 0;
	for raw_file in fs::read_dir(
		&config
			.destination_folder
			.join(&config.source_folder.iter().last().unwrap()),
	)
	.expect("Failed to read destination directory")
	{
		let file = raw_file.expect("Failed to load log file");
		if file
			.metadata()
			.expect("Failed to load metadata for file")
			.is_file() && file.file_name() != *".DS_Store"
		{
			let log_file = LogFile::parse(fs::read_to_string(file.path()).unwrap_or_else(|_| {
				panic!(
					"Failed to read from log file: {}",
					file.file_name().to_str().unwrap()
				)
			}));
			if log_file.real {
				let event_folder = &config.destination_folder.as_path().join(
					config
						.real_logs_location
						.as_ref()
						.unwrap_or(&PathBuf::from(log_file.event_name.as_ref().unwrap())),
				);
				let new_filename = log_file.new_filename();
				fs::create_dir_all(event_folder).expect("Failed to create folder for event");
				fs::rename(file.path(), event_folder.join(log_file.new_filename()))
					.expect("Failed to rename real log file to new name");
				info!(
					"Detected real log file: {} => {}",
					file.file_name().to_str().unwrap(),
					&new_filename.to_str().unwrap()
				);
				real_logs += 1;
			} else {
				fs::remove_file(file.path()).expect("Failed to delete fake log file");
			}
		}
	}
	println!();
	info!("Found {} real log files", real_logs);
}
