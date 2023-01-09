use std::path::PathBuf;

use serde::Deserialize;

use crate::storage::fs::{list_files_in_dir, read_file_to_string};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_name: String,

    pub app_root: String,
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

const CONFIG_FILES_LOCATION: &str = "../testdata/config";

fn parse_configs(path: PathBuf) -> Vec<Config> {
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

pub fn get_all_configs() -> Vec<Config> {
    parse_configs(PathBuf::from(CONFIG_FILES_LOCATION))
}

pub fn get_config_from_app_name(app_name: &String) -> Config {
    let configs = parse_configs(PathBuf::from(CONFIG_FILES_LOCATION));

    for config in configs {
        if config.app_name == *app_name {
            return config;
        }
    }

    panic!("No config found for app_name: {}", app_name);
}
