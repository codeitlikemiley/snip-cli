use crate::helpers::expand_home_dir::expand_home_dir;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{stdout, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct SnipConfig {
    pub path: String,
}

impl SnipConfig {
    pub fn load(path: &str) -> anyhow::Result<SnipConfig> {
        let config_content = fs::read_to_string(path).context("Failed to read config file")?;
        let config: SnipConfig =
            serde_json::from_str(&config_content).context("Failed to parse config file")?;
        writeln!(stdout(), "{}", &config.path).unwrap();
        Ok(config)
    }

    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let config_content =
            serde_json::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(path, config_content).context("Failed to write config file")?;
        Ok(())
    }

    pub fn update_path(&mut self, new_path: String) {
        self.path = expand_home_dir(&new_path).to_string_lossy().into_owned();
    }
}
