#[derive(Default, Debug)]
pub struct Log {
	pub real: bool,
	pub event_name: Option<String>,
	pub match_type: Option<String>,
	pub match_number: Option<u32>,
	pub replay_number: Option<u32>,
	pub alliance: Option<String>,
	pub location: Option<u32>,
}

impl Log {
	pub fn parse(content: String) -> Self {
		let mut log = Log::default();
		for line in content.lines() {
			let words: Vec<&str> = line.split(" ").filter(|w| w != &" " && w != &"").collect();
			if words.len() > 5 {
				let value = words
					.iter()
					.skip(5)
					.map(|&w| w)
					.collect::<Vec<&str>>()
					.join(" ")
					.to_string();
				match words.get(4).unwrap().trim_end_matches(':') {
					"EventName" => log.event_name = Some(value),
					"MatchType" => log.match_type = Some(value),
					"MatchNumber" => log.match_number = Some(value.parse::<u32>().unwrap()),
					"ReplayNumber" => log.replay_number = Some(value.parse::<u32>().unwrap()),
					"Alliance" => log.alliance = Some(value),
					"Location" => log.location = Some(value.parse::<u32>().unwrap()),
					_ => (),
				}
			}
		}

		log.real = log.event_name.is_some()
			&& log.match_type.is_some()
			&& log.match_number.is_some()
			&& log.replay_number.is_some()
			&& log.replay_number.is_some()
			&& log.alliance.is_some()
			&& log.location.is_some();

		log
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use std::path::Path;

	use anyhow::Result;

	use super::Log;

	const TEST_LOG_DIR: &str = "./test_logs/";

	#[test]
	fn test_real_parse() -> Result<()> {
		let real_files = fs::read_dir(Path::new(TEST_LOG_DIR).join("real"))?;
		for real_file in real_files {
			let file = real_file?;
			let log = Log::parse(fs::read_to_string(&file.path())?);
			assert!(log.real);
			match file.file_name().to_str().unwrap() {
				"1.log" => {
					assert_eq!(log.event_name, Some(String::from("NHBB")));
					assert_eq!(log.match_type, Some(String::from("Qualification")));
					assert_eq!(log.match_number, Some(6));
					assert_eq!(log.replay_number, Some(1));
					assert_eq!(log.alliance, Some(String::from("Red")));
					assert_eq!(log.location, Some(1));
				}
				"2.log" => {
					assert_eq!(log.event_name, Some(String::from("NHBB Worlds")));
					assert_eq!(log.match_type, Some(String::from("Qualification")));
					assert_eq!(log.match_number, Some(13));
					assert_eq!(log.replay_number, Some(1));
					assert_eq!(log.alliance, Some(String::from("Red")));
					assert_eq!(log.location, Some(3));
				}
				"3.log" => {
					assert_eq!(log.event_name, Some(String::from("NHBB")));
					assert_eq!(log.match_type, Some(String::from("Elimination")));
					assert_eq!(log.match_number, Some(8));
					assert_eq!(log.replay_number, Some(1));
					assert_eq!(log.alliance, Some(String::from("Blue")));
					assert_eq!(log.location, Some(1));
				}
				"4.log" => {
					assert_eq!(log.event_name, Some(String::from("NHBB")));
					assert_eq!(log.match_type, Some(String::from("Elimination")));
					assert_eq!(log.match_number, Some(4));
					assert_eq!(log.replay_number, Some(1));
					assert_eq!(log.alliance, Some(String::from("Blue")));
					assert_eq!(log.location, Some(1));
				}
				"5.log" => {
					assert_eq!(log.event_name, Some(String::from("NHBB")));
					assert_eq!(log.match_type, Some(String::from("Qualification")));
					assert_eq!(log.match_number, Some(23));
					assert_eq!(log.replay_number, Some(1));
					assert_eq!(log.alliance, Some(String::from("Blue")));
					assert_eq!(log.location, Some(1));
				}
				"6.log" => {
					assert_eq!(log.event_name, Some(String::from("NHBB")));
					assert_eq!(log.match_type, Some(String::from("Qualification")));
					assert_eq!(log.match_number, Some(16));
					assert_eq!(log.replay_number, Some(1));
					assert_eq!(log.alliance, Some(String::from("Blue")));
					assert_eq!(log.location, Some(2));
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
			assert!(!Log::parse(fs::read_to_string(fake_file?.path())?).real)
		}
		Ok(())
	}
}
