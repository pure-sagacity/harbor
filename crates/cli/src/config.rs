use crate::Environment;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const MAIN_CONFIG_FILE: &str = ".harbor.toml";
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub name: String,
    pub version: String,
    pub default_env: Environment,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Toml(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "IO error: {}", err),
            ConfigError::Toml(err) => write!(f, "TOML error: {}", err),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(err) => Some(err),
            ConfigError::Toml(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Toml(err)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct HarborToml {
    #[serde(alias = "project")]
    name: String,
    version: String,
    config: Environment,
}

impl Config {
    pub fn from_repo_root(root: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let root = root.as_ref();
        let main = Self::read_main_file(root.join(MAIN_CONFIG_FILE))?;

        Ok(Config {
            name: main.name,
            version: main.version,
            default_env: main.config,
        })
    }

    pub fn read_main_file(path: impl AsRef<Path>) -> Result<HarborToml, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let parsed: HarborToml = toml::from_str(&contents)?;
        Ok(parsed)
    }

    pub fn merged_env_vars(&self, store_vars: &HashMap<String, String>) -> HashMap<String, String> {
        store_vars.clone()
    }
}
