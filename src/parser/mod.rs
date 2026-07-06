use std::path::Path;

use crate::library::Metadata;

#[derive(Debug)]
pub struct ParsingError;

pub fn parse_album<P: AsRef<Path>>(path: P) -> Result<Metadata, ParsingError> {
    let raw = std::fs::read_to_string(path).unwrap();
    let metadata = toml::from_str(&raw).unwrap();
    Ok(metadata)
}
