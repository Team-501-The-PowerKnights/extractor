use std::path::PathBuf;
use std::{env, fs};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
	pub hostname: String,
	#[serde(default = "defaults::port")]
	pub port: u32,
	#[serde(default = "defaults::destination_folder")]
	pub destination_folder: PathBuf,
	pub source_folder: PathBuf,
	#[serde(default = "defaults::username")]
	pub username: String,
	#[serde(default = "defaults::password")]
	pub password: String,
	#[serde(default = "defaults::remove")]
	pub remove: bool,
	pub real_logs_location: Option<PathBuf>,
}

mod defaults {
	use std::path::PathBuf;

	pub fn password() -> String { String::from("") }
	pub fn port() -> u32 { 22 }
	pub fn username() -> String { String::from("lvuser") }
	pub fn destination_folder() -> PathBuf { dirs::desktop_dir().unwrap().join("extractor_logs") }
	pub fn remove() -> bool { false }
}

impl Configuration {
	pub fn read() -> Result<Self> {
		let loc = env::current_dir().unwrap().join("conf.toml");
		let content = fs::read_to_string(loc)?;
		Ok(toml::from_str(&content)?)
	}
}
