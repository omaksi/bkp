use std::{path::PathBuf, time::SystemTime};

use chrono::Utc;

use crate::{
    compress::compress_files,
    config::Config,
    fs::{filter_files_newer_than, list_files_in_dir},
    time::get_current_timestamp,
};

pub fn do_full_backup(config: &Config) {
    // find existing backups
    let paths = match list_files_in_dir(config.included_paths[0].clone().into()) {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    do_backup(config, &paths);
}

pub fn do_incremental_backup(config: &Config, last_backup_time: SystemTime) {
    // find existing backups
    let paths = match list_files_in_dir(config.included_paths[0].clone().into()) {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("Paths: {:?}", paths);
    println!("Last backup time: {:?}", last_backup_time);

    // filter paths using filter_files_newer_than and lastBackupTime
    let paths = match filter_files_newer_than(&paths, last_backup_time) {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error: Couldn't filter paths based on modified time {}", e);
            return;
        }
    };

    println!("Filtered paths: {:?}", paths);

    do_backup(config, &paths);
}

fn do_backup(config: &Config, paths: &Vec<PathBuf>) {
    // if paths is empty, return with message
    if paths.is_empty() {
        println!("No files to backup");
        return;
    }

    // save current dir
    let current_dir = std::env::current_dir().unwrap();

    // set current dir to config.included_paths[0]
    std::env::set_current_dir(config.included_paths[0].clone()).unwrap();

    // remove prefix config.included_paths[0] from paths
    let paths = paths
        .iter()
        .map(|path| {
            path.strip_prefix(config.included_paths[0].clone())
                .unwrap()
                .to_path_buf()
        })
        .collect::<Vec<PathBuf>>();

    println!("Found {} files", paths.len());
    println!("{:?}", paths);

    // create path for new backup file, it should be config.local_storage_location + app_name + timestamp + .tar
    let backup_file_path = PathBuf::from(config.local_storage_location.clone())
        .join(config.app_name.clone() + "-" + Utc::now().to_rfc3339().as_str())
        .with_extension("tar");

    println!("Backup file path: {:?}", backup_file_path);

    println!("Compressing {} files", paths.len());

    // call compress function with backup_file_path and paths
    compress_files(backup_file_path, &paths);

    // restore current dir
    std::env::set_current_dir(current_dir).unwrap();
}
