//! Contains loading logic for various file types.
//!
//! Currently supported:
//! - `json` and `json5`
//! - `ron`
//! - `toml`
//! - `yaml`

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use std::path::Path;

lazy_static! {
    /// Contains all currently supported configuration file types.
    ///
    /// This vector is used to detect and parse the right config file format.
    pub(crate) static ref CONFIG_FILE_EXTENSIONS: Vec<&'static str> = {
        let mut vec = Vec::new();

        if cfg!(feature = "config-json") {
            vec.push("json");
        }
        if cfg!(feature = "config-json5") {
            vec.push("json5");
        }
        if cfg!(feature = "config-ron") {
            vec.push("ron");
        }
        if cfg!(feature = "config-toml") {
            vec.push("toml");
        }
        if cfg!(feature = "config-yaml") {
            vec.push("yaml");
            vec.push("yml");
        }

        vec
    };
}

/// Reads and parses the config file at a given path.
///
/// # Arguments
///
/// - `path`: A reference to the config file's path.
///
/// # Returns
///
/// The parsed config file. It's up to the user to denote the type of the config file.
pub fn parse<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let file_extension = path
        .as_ref()
        .extension()
        .ok_or_else(|| anyhow!("Failed to retrieve config file extension"))?
        .to_str()
        .ok_or_else(|| anyhow!("Failed to convert config file extension to str"))?;
    println!("extension: {}", &file_extension);

    match file_extension {
        #[cfg(feature = "config-json")]
        "json" => parse_json(path),

        #[cfg(feature = "config-json5")]
        "json5" => parse_json5(path),

        #[cfg(feature = "config-ron")]
        "ron" => parse_ron(path),

        #[cfg(feature = "config-toml")]
        "toml" => parse_toml(path),

        #[cfg(feature = "config-yaml")]
        "yaml" | "yml" => parse_yaml(path),

        _ => unreachable!(),
    }
}

#[cfg(feature = "config-json")]
fn parse_json<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let file = File::open(path.as_ref())?;
    Ok(serde_json::from_reader(file)?)
}

#[cfg(feature = "config-json5")]
fn parse_json5<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let file = File::open(path.as_ref())?;
    Ok(serde_json::from_reader(file)?)
}

#[cfg(feature = "config-ron")]
fn parse_ron<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let mut file = File::open(path.as_ref())?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    Ok(ron::from_str(&content)?)
}

#[cfg(feature = "config-toml")]
fn parse_toml<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let mut file = File::open(path.as_ref())?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;
    println!("toml");
    Ok(toml::from_str(&content)?)
}

#[cfg(feature = "config-yaml")]
fn parse_yaml<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let file = File::open(path.as_ref())?;
    Ok(serde_yaml::from_reader(file)?)
}
