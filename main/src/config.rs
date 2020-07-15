use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub keys: Vec<KeyEntry>,
    pub taskbar: TaskbarConfig,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct KeyEntry {
    name: String,
    value: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TaskbarConfig {
    pub show: bool,
}
