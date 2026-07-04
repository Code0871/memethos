use std::fs;
use toml;
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "src/config/config.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    server: Server,
    storage: Storage,
    logging: Logging,
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
                snapshot_interval: 60,
            },
            logging: Logging {
                log_level: "info".to_string(),
                log_format: "text".to_string(),
            }
        }
    }
}

impl Config {
    // TODO: Добавить обработку ошибок и загрузку по дефолту, если файл не найден или не верный формат
    pub fn load_from_file() -> Self {
        let config = fs::read_to_string(CONFIG_FILE_PATH).unwrap();

        let res: Config = toml::from_str(&config).unwrap();

        return res;
    }
}
