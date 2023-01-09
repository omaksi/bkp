use std::path::PathBuf;

use serde::Deserialize;

use crate::storage::fs::{list_files_in_dir, read_file_to_string};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_name: String,
    // pub server_name: String,
    pub included_paths: Vec<String>,
    pub excluded_paths: Vec<String>,

    pub pre_backup_script: String,
    pub post_backup_script: String,
    pub pre_restore_script: String,
    pub post_restore_script: String,

    // pub incremental_backup_interval_days: i32,
    // pub full_backup_periods: Vec<i32>,
    // pub backup_start_time: String,
    pub local_storage_location: String,
    // pub remote_storage_address: String,
    // pub remote_location: String,
}

pub fn parse_configs(path: PathBuf) -> Vec<Config> {
    let config_files = list_files_in_dir(path);

    let mut configs: Vec<Config> = Vec::new();

    for config_file in config_files.unwrap() {
        let config = read_file_to_string(config_file.as_path());
        match toml::from_str(config.as_str()) {
            Ok(config) => configs.push(config),
            Err(e) => println!("Error parsing config file: {}", e),
        }
    }

    configs
}
