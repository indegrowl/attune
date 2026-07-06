use serde::Deserialize;
use std::path::Path;

#[derive(Debug)]
pub struct ConfigError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub music_library_path: String,
}

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let raw = std::fs::read_to_string(path).unwrap();
    let config = toml::from_str(&raw).unwrap();
    Ok(config)
}
