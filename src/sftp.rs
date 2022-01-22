use std::fs::create_dir_all;

use anyhow::Result;
use which::which;

use crate::conf::Configuration;

pub fn setup(config: Configuration) -> Result<()> {
    if !config.destination.exists() {
        create_dir_all(config.destination)?;
    }
    which("sftp")?;
    Ok(())
}
