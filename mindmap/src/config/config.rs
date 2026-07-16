use std::{fs};
use toml;
use serde::Deserialize;
use crate::{ConfigError};

const CONFIG_FILE_PATH: &str = "src/config/config.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
    pub storage: Storage,
    pub logging: Logging,
    pub wal: Wal,
    pub collection: Collection
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u32,
    pub workers: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Storage {
    pub data_dir: String,
    pub snapshot_interval: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Logging {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Wal {
    pub enabled: bool,
    pub dir: String,
    pub file_prefix: String,
    pub max_wal_files: u32,
    pub max_size_limit: u64,
    pub sync_interval_ms: u64,
    pub sync_timeout_ms: u64,
    pub max_batch_size: u32,
    pub flush_on_shutdown: bool,
    pub auto_recover: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Collection {
    pub dir: String,
    pub delete_dir: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: Server {
                host: "127.0.0.1".to_string(),
                port: 8071,
                workers: 4,
            },
            storage: Storage {
                data_dir: "./data".to_string(),
                snapshot_interval: 90,
            },
            logging: Logging {
                level: "info".to_string(),
                format: "text".to_string(),
            },
            wal: Wal {
                enabled: true,
                dir: "./wal".to_string(),
                file_prefix: "wal_".to_string(),
                max_wal_files: 10,
                max_size_limit: 1073741824, // 1GB
                sync_interval_ms: 100,
                sync_timeout_ms: 5000,
                max_batch_size: 10000,
                flush_on_shutdown: true,
                auto_recover: true,
            },
            collection: Collection {
                dir: "./data/collections".to_string(),
                delete_dir: "./data/del_collections".to_string(),
            }
        }
    }
}

impl Config {
    pub fn load_from_file() -> Result<Self, ConfigError> {
        let parse_config = fs::read_to_string(CONFIG_FILE_PATH)?;

        let conf: Config = toml::from_str(&parse_config)?;

        Ok(conf)
        
    }
}
