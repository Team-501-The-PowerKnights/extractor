use std::fs;

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

	// Moving log files to parent folder
	let transferred_folder = config
		.destination_folder
		.join(config.source_folder.iter().last().unwrap());
	println!("{}", transferred_folder.display());
	let log_files = fs::read_dir(&transferred_folder).expect("Failed to read transferred folder");
	for file in log_files {
		let verified_file = file.expect("Failed to load file");
		let file_path = &verified_file.path();
		fs::rename(
			file_path,
			file_path
				.parent()
				.unwrap()
				.parent()
				.unwrap()
				.join(verified_file.file_name()),
		)
		.expect("Failed to move log file up to source folder");
	}
	fs::remove_dir(&transferred_folder).expect("Failed to create transferred folder");

	// Find real log files to move it
	let mut real_logs = 0;
	for raw_file in
		fs::read_dir(&config.destination_folder).expect("Failed to read destination directory")
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
				let real_logs_folder = &config.real_logs_folder;
				let new_filename = log_file.new_filename();
				fs::create_dir_all(real_logs_folder)
					.expect("Failed to create folder for real log files");
				fs::rename(file.path(), real_logs_folder.join(log_file.new_filename()))
					.expect("Failed to rename real log file to new name");
				info!(
					"Detected real log file: {} => {}",
					file.file_name().to_str().unwrap(),
					&new_filename.to_str().unwrap()
				);
				real_logs += 1;
			}
		}
	}
	println!();
	info!("Found {} real log files", real_logs);
}
