use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configuration {
	pub hostname: String,
	#[serde(default = "defaults::port")]
	pub port: u32,
	#[serde(default = "defaults::destination")]
	pub destination: PathBuf,
	pub source: String,
	#[serde(default = "defaults::username")]
	pub username: String,
	#[serde(default = "defaults::password")]
	pub password: String,
	#[serde(default = "defaults::remove")]
	pub remove: bool,
}

mod defaults {
	use std::path::PathBuf;

	pub fn password() -> String { String::from("") }
	pub fn port() -> u32 { 22 }
	pub fn username() -> String { String::from("lvuser") }
	pub fn destination() -> PathBuf { dirs::desktop_dir().unwrap().join("extractor_logs") }
	pub fn remove() -> bool { false }
}

impl Configuration {
	pub fn read() -> Result<Self> {
		let loc = dirs::home_dir()
			.unwrap()
			.join(".config")
			.join("extractor")
			.join("conf.toml");
		let content = fs::read_to_string(loc)?;
		Ok(toml::from_str(&content)?)
	}
}
