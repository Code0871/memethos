use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    ReadFile {
        path: String,
        source: std::io::Error,
    },

    #[error("Failed to parse config file {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Invalid config: {0}")]
    Invalid(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ConfigError>;