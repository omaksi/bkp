use std::{path::PathBuf, process::exit};

use log::error;
use serde::Deserialize;

use crate::{
    globalconfig::GLOBAL_CONFIG,
    storage::fs::{list_files_in_dir, read_file_to_string},
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_name: String,
    pub server_name: String,

    pub app_root: String,
    // pub server_name: String,
    pub included_paths: Vec<String>,
    pub excluded_paths: Vec<String>,

    pub pre_backup_script: String,
    pub post_backup_script: String,
    pub pre_restore_script: String,
    pub post_restore_script: String,

    pub keep_full_local_backups: i16,
    pub keep_full_remote_backups: i16,
}

fn parse_configs(path: PathBuf) -> Vec<Config> {
    let config_files = list_files_in_dir(path);

    let mut configs: Vec<Config> = Vec::new();

    for config_file in config_files.unwrap() {
        let config = read_file_to_string(config_file.as_path());
        match toml::from_str(config.as_str()) {
            Ok(config) => configs.push(config),
            Err(e) => {
                error!("Error parsing config file: {}", e);
                exit(1)
            }
        }
    }

    configs
}

pub fn get_all_configs() -> Vec<Config> {
    parse_configs(PathBuf::from(GLOBAL_CONFIG.config_files_location.clone()))
}

pub fn get_config_from_app_name(app_name: &String) -> Config {
    let configs = parse_configs(PathBuf::from(GLOBAL_CONFIG.config_files_location.clone()));

    for config in configs {
        if config.app_name == *app_name {
            return config;
        }
    }

    panic!("No config found for app_name: {}", app_name);
}
