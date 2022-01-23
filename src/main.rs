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

	let log = LogFile::parse(
		fs::read_to_string("./test_logs/logfile-022.log").expect("Failed to read log file"),
	);
	println!("{:?}", log);

	// let config = Configuration::read().expect("Failed to read from configuration file");
	// info!("Loaded configuration file");
	// sftp::setup(&config).expect("Failed to setup for sftp usage");
	// info!("Setup for SFTP");
	// sftp::sync(&config).expect("Failed to run sftp");
	// info!("SUCCESS\n\tTransferred and removed files")
}
