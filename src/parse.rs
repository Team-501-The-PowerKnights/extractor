use std::path::PathBuf;

use chrono::Local;

#[derive(Default, Debug)]
pub struct LogFile {
	pub content: String,
	pub real: bool,
	pub event_name: Option<String>,
	pub match_type: Option<String>,
	pub match_number: Option<u32>,
	pub replay_number: Option<u32>,
	pub alliance: Option<String>,
	pub location: Option<u32>,
}

impl LogFile {
	pub fn parse(content: String) -> Self {
		let mut log_file = LogFile::default();
		for line in content.lines() {
			let words: Vec<&str> = line.split(' ').filter(|w| w != &" " && w != &"").collect();
			if words.len() > 5 {
				let value = words
					.iter()
					.skip(5)
					.copied()
					.collect::<Vec<&str>>()
					.join(" ")
					.to_string();
				match words.get(4).unwrap().trim_end_matches(':') {
					"EventName" => log_file.event_name = Some(value),
					"MatchType" => log_file.match_type = Some(value),
					"MatchNumber" => log_file.match_number = Some(value.parse::<u32>().unwrap()),
					"ReplayNumber" => log_file.replay_number = Some(value.parse::<u32>().unwrap()),
					"Alliance" => log_file.alliance = Some(value),
					"Location" => log_file.location = Some(value.parse::<u32>().unwrap()),
					_ => (),
				}
			}
		}

		log_file.content = content;
		log_file.real = log_file.event_name.is_some()
			&& log_file.match_type.is_some()
			&& log_file.match_number.is_some()
			&& log_file.replay_number.is_some()
			&& log_file.replay_number.is_some()
			&& log_file.alliance.is_some()
			&& log_file.location.is_some();

		log_file
	}

	pub fn new_filename(&self) -> PathBuf {
		PathBuf::from(format!(
			"{}_{}_{}-{:0>2}-{:0>2}_{}{}.log",
			Local::now().format("%Y%m%d"),
			self.event_name.as_ref().unwrap(),
			self.match_type.as_ref().unwrap().chars().next().unwrap(),
			self.match_number.unwrap(),
			self.replay_number.unwrap(),
			self.alliance.as_ref().unwrap().chars().next().unwrap(),
			self.location.unwrap()
		))
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use std::path::Path;

	use anyhow::Result;

	use super::LogFile;

	const TEST_LOG_DIR: &str = "./test_logs/";

	#[test]
	fn test_real_parse() -> Result<()> {
		let real_files = fs::read_dir(Path::new(TEST_LOG_DIR).join("real"))?;
		for real_file in real_files {
			let file = real_file?;
			let log_file = LogFile::parse(fs::read_to_string(&file.path())?);
			assert!(log_file.real);
			match file.file_name().to_str().unwrap() {
				"1.log" => {
					assert_eq!(log_file.event_name, Some(String::from("NHBB")));
					assert_eq!(log_file.match_type, Some(String::from("Qualification")));
					assert_eq!(log_file.match_number, Some(6));
					assert_eq!(log_file.replay_number, Some(1));
					assert_eq!(log_file.alliance, Some(String::from("Red")));
					assert_eq!(log_file.location, Some(1));
				}
				"2.log" => {
					assert_eq!(log_file.event_name, Some(String::from("NHBB Worlds")));
					assert_eq!(log_file.match_type, Some(String::from("Qualification")));
					assert_eq!(log_file.match_number, Some(13));
					assert_eq!(log_file.replay_number, Some(1));
					assert_eq!(log_file.alliance, Some(String::from("Red")));
					assert_eq!(log_file.location, Some(3));
				}
				"3.log" => {
					assert_eq!(log_file.event_name, Some(String::from("NHBB")));
					assert_eq!(log_file.match_type, Some(String::from("Elimination")));
					assert_eq!(log_file.match_number, Some(8));
					assert_eq!(log_file.replay_number, Some(1));
					assert_eq!(log_file.alliance, Some(String::from("Blue")));
					assert_eq!(log_file.location, Some(1));
				}
				"4.log" => {
					assert_eq!(log_file.event_name, Some(String::from("NHBB")));
					assert_eq!(log_file.match_type, Some(String::from("Elimination")));
					assert_eq!(log_file.match_number, Some(4));
					assert_eq!(log_file.replay_number, Some(1));
					assert_eq!(log_file.alliance, Some(String::from("Blue")));
					assert_eq!(log_file.location, Some(1));
				}
				"5.log" => {
					assert_eq!(log_file.event_name, Some(String::from("NHBB")));
					assert_eq!(log_file.match_type, Some(String::from("Qualification")));
					assert_eq!(log_file.match_number, Some(23));
					assert_eq!(log_file.replay_number, Some(1));
					assert_eq!(log_file.alliance, Some(String::from("Blue")));
					assert_eq!(log_file.location, Some(1));
				}
				"6.log" => {
					assert_eq!(log_file.event_name, Some(String::from("NHBB")));
					assert_eq!(log_file.match_type, Some(String::from("Qualification")));
					assert_eq!(log_file.match_number, Some(16));
					assert_eq!(log_file.replay_number, Some(1));
					assert_eq!(log_file.alliance, Some(String::from("Blue")));
					assert_eq!(log_file.location, Some(2));
				}
				_ => (),
			}
		}
		Ok(())
	}

	#[test]
	fn test_fake_parse() -> Result<()> {
		let fake_files = fs::read_dir(Path::new(TEST_LOG_DIR).join("fake"))?;
		for fake_file in fake_files {
			assert!(!LogFile::parse(fs::read_to_string(fake_file?.path())?).real)
		}
		Ok(())
	}

	#[test]
	fn test_new_filename() {
		assert!(LogFile {
			content: String::new(),
			real: true,
			event_name: Some(String::from("NHBB")),
			match_type: Some(String::from("Qualification")),
			match_number: Some(6),
			replay_number: Some(1),
			alliance: Some(String::from("Red")),
			location: Some(1),
		}
		.new_filename()
		.to_str()
		.unwrap()
		.ends_with("_NHBB_Q-06-01_R1.log"));

		assert!(LogFile {
			content: String::new(),
			real: true,
			event_name: Some(String::from("NHBB")),
			match_type: Some(String::from("Qualification")),
			match_number: Some(6),
			replay_number: Some(1),
			alliance: Some(String::from("Red")),
			location: Some(1),
		}
		.new_filename()
		.to_str()
		.unwrap()
		.ends_with("_NHBB_Q-06-01_R1.log"));
	}
}
