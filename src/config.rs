use std::{collections::HashMap, env::current_dir, io, path::PathBuf};

use directories_next::BaseDirs;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::profile::Profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default: Option<String>,
    pub profile: HashMap<String, Profile>,
}

pub fn get_config_path(filepath: Option<PathBuf>) -> Result<PathBuf, io::Error> {
    let name = env!("CARGO_PKG_NAME");
    if let Some(path) = filepath {
        if path.exists() {
            return Ok(path);
        } else {
            warn!(
                "config file {} does not exist; try to find fallback",
                path.as_path().display()
            );
        }
    }
    if let Ok(curr_dir) = current_dir() {
        for dir in curr_dir.ancestors() {
            let filepath = dir.join(format!(".{}.toml", name));
            if filepath.exists() {
                return Ok(filepath);
            }
        }
    };
    if let Some(base_dir) = BaseDirs::new() {
        let filepath = base_dir.config_dir().join(format!("{}.toml", name));
        if filepath.exists() {
            return Ok(filepath);
        }
        let filepath = base_dir.home_dir().join(format!(".{}.toml", name));
        if filepath.exists() {
            return Ok(filepath);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "config file not found"))
}
