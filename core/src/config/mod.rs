//!

use crate::debug::workspace_path;
use directories_next::ProjectDirs;
use file_types::CONFIG_FILE_EXTENSIONS;
use lazy_static::lazy_static;
use log::info;
use std::path::PathBuf;

mod file_types;
pub use file_types::parse;
use std::cmp::Ordering;

lazy_static! {
    /// One-time calculation of the config file path.
    static ref CONFIG_FILE_PATH: Option<PathBuf> = init_config_file_path();
}

#[cfg(feature = "config-toml")]
pub use toml;

/// Returns the config file path.
///
/// # Note
///
/// The value is only calculated once the first time this method is called.
pub fn config_file_path() -> Option<&'static PathBuf> {
    (*CONFIG_FILE_PATH).as_ref()
}

// Initializes the config file path.
fn init_config_file_path() -> Option<PathBuf> {
    if let Some(project_dirs) = ProjectDirs::from("net", "zerotask", "twm") {
        let config_dir = project_dirs.config_dir();

        // In the case the build is running in debug mode, a special config file is used for faster
        // iterations.
        if cfg!(debug_assertions) {
            println!("Running in debug, using debug configuration file");
            return Some(workspace_path().join("config.debug.toml"));
        }

        let mut config_files = Vec::new();

        for file_extension in &*CONFIG_FILE_EXTENSIONS {
            let config_file_path = config_dir.join(format!("config.{}", file_extension));
            if config_file_path.exists() {
                config_files.push(config_file_path);
            }
        }

        match config_files.len().cmp(&1) {
            Ordering::Equal => {
                let path = config_files.pop();
                info!("Using config file at {}", path.as_ref().unwrap().display());
                return path;
            }
            Ordering::Greater => panic!(
                "Only one config file is allowed, found {}",
                config_files.len()
            ),
            Ordering::Less => {
                panic!("No configuration file found!");
            }
        }
    }

    None
}
