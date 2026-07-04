use std::{fs, io::Error};
use toml;
use serde::Deserialize;
use crate::{ConfigError};

const CONFIG_FILE_PATH: &str = "src/config/config.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: Server,
    pub storage: Storage,
    pub logging: Logging,
    pub wal: Wal
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
    pub log_level: String,
    pub log_format: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Wal {
    pub enabled: bool,
    pub dir: String,
    pub file_prefix: String,
    pub max_wal_files: u32,
    pub max_size_limit: u64,
    pub sync_interval_ms: u64,
    pub max_batch_size: u32,
    pub flush_on_shutdown: bool,
    pub auto_recover: bool,
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
                log_level: "info".to_string(),
                log_format: "text".to_string(),
            },
            wal: Wal {
                enabled: true,
                dir: "./wal".to_string(),
                file_prefix: "wal_".to_string(),
                max_wal_files: 10,
                max_size_limit: 1024 * 1024 * 1024, // 1GB
                sync_interval_ms: 100,
                max_batch_size: 10000,
                flush_on_shutdown: true,
                auto_recover: true,
            }
        }
    }
}

impl Config {
    // TODO: Добавить обработку ошибок и загрузку по дефолту, если файл не найден или не верный формат
    pub fn load_from_file() -> Result<Self, ConfigError> {
        
        let parse_config = fs::read_to_string(CONFIG_FILE_PATH)?;

        let conf: Config = toml::from_str(&parse_config)?;

        Ok(conf)
    }
}
