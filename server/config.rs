use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Config {}

/// An error returned when parsing the configuration file fails.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ParseConfigError;

impl fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "failed to parse the configuration file".fmt(f)
    }
}

impl std::error::Error for ParseConfigError {}

pub fn parse_config(path: PathBuf) -> Result<Config, ParseConfigError> {
    let result = fs::read_to_string(path);
    let contents = match result {
        Ok(contents) => contents,
        Err(_) => return Err(ParseConfigError), // TODO: Embed error message.
    };
    let result: Result<Config, _> = toml::from_str(contents.as_str());
    let config: Config = match result {
        Ok(config) => config,
        Err(_) => return Err(ParseConfigError), // TODO: Embed error message.
    };
    Ok(config)
}
