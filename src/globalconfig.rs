use std::process::exit;

use log::error;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::storage::fs::read_file_to_string;
#[derive(Deserialize, Debug)]
pub struct GlobalConfig {
    pub config_files_location: String,
    pub local_storage_location: String,
    pub remote_storage_address: String,
    pub remote_storage_access_id: String,
    pub remote_storage_secret_key: String,
    pub log_file_location: String,
}
pub static GLOBAL_CONFIG: Lazy<GlobalConfig> = Lazy::new(|| load_global_config());

const GLOBAL_CONFIG_FILENAME: &str = ".bkpconfig";

pub fn load_global_config() -> GlobalConfig {
    let home_dir = home::home_dir().unwrap().join(GLOBAL_CONFIG_FILENAME);
    let config = read_file_to_string(&home_dir);
    match toml::from_str(config.as_str()) {
        Ok(config) => config,
        Err(e) => {
            error!("Error parsing global config file: {}", e);
            exit(1)
        }
    }
}

// fn get_global_config() -> GlobalConfig {
//     GlobalConfig {
//         config_files_location: String::from("../testdata/config"),
//         local_storage_location: String::from(
//             "/Users/ondrej/Documents/GitHub/bkp/bkp/testdata/storage",
//         ),
//         remote_storage_address: String::from("http://localhost:9000"),
//         remote_storage_access_id: String::from("minioadmin"),
//         remote_storage_secret_key: String::from("minioadmin"),
//     }
// }
