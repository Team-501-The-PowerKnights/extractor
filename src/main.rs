use conf::Configuration;

use log::{info, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

mod conf;
mod sftp;

fn main() {
	TermLogger::init(
		LevelFilter::Trace,
		Config::default(),
		TerminalMode::Stdout,
		ColorChoice::Auto,
	)
	.expect("Failed to configure logger");

	let config = Configuration::read().expect("Failed to read from configuration file");
	info!("Loaded configuration file");
	sftp::setup(&config).expect("Failed to setup for sftp usage");
	info!("Setup for SFTP");
	sftp::sync(&config).expect("Failed to run sftp");
	info!("SUCCESS\n\tTransferred and removed files")
}
