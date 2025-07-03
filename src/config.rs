use crate::error::{AppError, AppResult as Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub database_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path().map_err(|e| AppError::Config(format!("resolve path: {e}")))?;

        if !path.exists() {
            create_default_config(&path)?;
        }

        let contents =
            fs::read_to_string(&path).map_err(|e| AppError::Config(format!("read config: {e}")))?;

        let config = toml::from_str(&contents)
            .map_err(|e| AppError::Config(format!("parse config: {e}")))?;

        Ok(config)
    }
}

fn config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "popplestones", "rolodex")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not determine config dir"))?;

    let path = proj_dirs.config_dir().join("config.toml");
    Ok(path)
}

fn create_default_config(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| AppError::Config(format!("create config dir: {e}")))?;
    }

    let default = Config {
        database_path: default_data_path()
            .ok_or_else(|| AppError::Config("could not determine data dir".into()))?,
    };

    let contents = toml::to_string_pretty(&default)
        .map_err(|e| AppError::Config(format!("serialize default config: {e}")))?;

    fs::write(path, contents)
        .map_err(|e| AppError::Config(format!("write default config: {e}")))?;

    Ok(())
}

fn default_data_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "popplestones", "rolodex")
        .map(|dirs| dirs.data_dir().join("rolodex.db"))
}
