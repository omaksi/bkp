use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::{
    compress::compress_files,
    config::{parse_configs, Config},
    storage::fs::{
        filter_files_newer_than, filter_files_with_extension, get_list_of_paths, list_files_in_dir,
    },
    time::parse_timestamp,
};

pub fn automatic_backup() {
    let configs = parse_configs(PathBuf::from("../testdata/config"));

    // print number of found configs
    println!("Found {} configs", configs.len());
    // println!("{:?}", configs);

    // for each config:
    // find existing backups
    for config in configs {
        println!("Config: {}", config.app_name);
        let files = match list_files_in_dir(config.local_storage_location.clone().into()) {
            Ok(files) => files,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // filter files using filter_files_with_extension and "tar"
        let files = match filter_files_with_extension(files, "tar") {
            Ok(files) => files,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        // print number of found backups
        println!("Found {} backups", files.len());

        // parse filenames using parse_backup_filename and order by timestamp
        let mut files = files
            .iter()
            .map(|file| parse_backup_filename(file.to_str().unwrap()))
            .collect::<Vec<(&str, String, String, DateTime<Utc>)>>();

        files.sort_by_key(|f| f.3);

        // reverse order of files vec
        files.reverse();

        println!("{:?}", files);

        // if there are no backups, do a full backup
        if files.len() == 0 {
            do_full_backup(&config);
            return;
        }

        // do_full_backup(&config);w
        do_incremental_backup(&config, files[0].3);
    }
}

pub fn do_full_backup(config: &Config) {
    // let paths = match list_files_in_dir(config.included_paths[0].clone().into()) {
    //     Ok(paths) => paths,
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         return;
    //     }
    // };

    let paths = get_list_of_paths(config.included_paths.clone(), config.excluded_paths.clone());

    do_backup(config, &paths, "full");
}

pub fn do_incremental_backup(config: &Config, last_backup_time: DateTime<Utc>) {
    // let paths = match list_files_in_dir(config.included_paths[0].clone().into()) {
    //     Ok(paths) => paths,
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         return;
    //     }
    // };

    let paths = get_list_of_paths(config.included_paths.clone(), config.excluded_paths.clone());

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

    do_backup(config, &paths, "incremental");
}

fn do_backup(config: &Config, paths: &Vec<PathBuf>, backup_type: &str) {
    // if paths is empty, return with message
    if paths.is_empty() {
        println!("No files to backup");
        return;
    }

    // save current dir
    let current_dir = std::env::current_dir().unwrap();

    // set current dir to config.included_paths[0]
    std::env::set_current_dir(config.included_paths[0].clone()).unwrap();

    println!("Found {} files", paths.len());
    println!("{:?}", paths);

    // remove prefix config.included_paths[0] from paths
    let paths = paths
        .iter()
        .map(|path| {
            path.strip_prefix(config.included_paths[0].clone())
                .unwrap()
                .to_path_buf()
        })
        .collect::<Vec<PathBuf>>();

    println!("{:?}", paths);

    // create path for new backup file, it should be config.local_storage_location + app_name + timestamp + .tar
    let backup_file_path = PathBuf::from(config.local_storage_location.clone())
        .join(config.app_name.clone() + "_" + backup_type + "_" + Utc::now().to_rfc3339().as_str())
        .with_extension("tar");

    println!("Backup file path: {:?}", backup_file_path);

    println!("Compressing {} files", paths.len());

    // call compress function with backup_file_path and paths
    compress_files(backup_file_path, &paths);

    // restore current dir
    std::env::set_current_dir(current_dir).unwrap();
}

pub fn parse_backup_filename(filename: &str) -> (&str, String, String, DateTime<Utc>) {
    let filename_parts: Vec<&str> = filename.split('.').collect();
    let name_parts: Vec<&str> = filename_parts[0].split('_').collect();

    let name = name_parts[0].to_string();
    let backup_type = name_parts[1].to_string();
    let timestamp = match parse_timestamp(name_parts[2].to_string() + "Z") {
        Some(timestamp) => timestamp,
        None => panic!("Unable to parse timestamp from filename"),
    };

    (filename, name, backup_type, timestamp)
}
