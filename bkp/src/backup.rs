use std::path::PathBuf;

use chrono::{DateTime, Utc};

use crate::{
    compress::compress_files,
    config::Config,
    storage::fs::{filter_files_newer_than, get_files_to_backup, list_files_in_dir},
    time::parse_timestamp,
};

#[derive(Debug)]
pub struct Backup {
    pub path: PathBuf,
    pub file_name: String,
    pub app_name: String,
    pub backup_type: String,
    pub time: DateTime<Utc>,
}

fn parse_backup_from_path(path: &PathBuf) -> Backup {
    let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();
    let parts = file_name.split("_").collect::<Vec<&str>>();
    let app_name = parts[0].to_string();
    let backup_type = parts[1].to_string();
    let time = parse_timestamp(parts[2].to_string() + "Z").unwrap();

    Backup {
        path: path.clone(),
        file_name,
        app_name,
        backup_type,
        time,
    }
}

fn parse_backups_from_paths(paths: Vec<PathBuf>) -> Vec<Backup> {
    paths
        .iter()
        .map(|path| parse_backup_from_path(path))
        .collect::<Vec<Backup>>()
}

pub fn get_all_backups_for_app(config: &Config) -> Vec<Backup> {
    // let config = get_config_from_app_name(app_name);

    let files = list_files_in_dir(config.local_storage_location.clone().into()).unwrap();

    let backups = parse_backups_from_paths(files);

    // filter files beginning with app_name
    let mut backups: Vec<Backup> = backups
        .into_iter()
        .filter(|b| b.app_name == config.app_name)
        .collect();

    println!("Found {} backups", backups.len());

    backups.sort_by_key(|b| b.time);

    backups
}

pub fn do_full_backup(config: &Config) {
    let paths = get_files_to_backup(
        config.app_root.clone(),
        config.included_paths.clone(),
        config.excluded_paths.clone(),
    );

    do_backup(config, &paths, "full");
}

pub fn do_incremental_backup(config: &Config, last_backup_time: DateTime<Utc>) {
    let paths = get_files_to_backup(
        config.app_root.clone(),
        config.included_paths.clone(),
        config.excluded_paths.clone(),
    );

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
        panic!("No files to backup");
    }

    // save current dir
    let current_dir = std::env::current_dir().unwrap();

    // set current dir to config.included_paths[0]
    std::env::set_current_dir(config.app_root.clone()).unwrap();

    println!("Found {} files", paths.len());
    println!("{:?}", paths);

    // create path for new backup file, it should be config.local_storage_location + app_name + timestamp + .tar
    let backup_file_path = PathBuf::from(config.local_storage_location.clone())
        .join(config.app_name.clone() + "_" + backup_type + "_" + Utc::now().to_rfc3339().as_str())
        .with_extension("tar");

    println!("Backup file path: {:?}", backup_file_path);

    // remove prefix from paths
    let paths = paths
        .iter()
        .map(|p| {
            p.strip_prefix(config.app_root.clone())
                .unwrap()
                .to_path_buf()
        })
        .collect::<Vec<PathBuf>>();

    println!("Compressing {} files", paths.len());

    // call compress function with backup_file_path and paths
    compress_files(backup_file_path, &paths);

    // restore current dir
    std::env::set_current_dir(current_dir).unwrap();
}

pub fn get_last_full_backup_time(config: &Config) -> DateTime<Utc> {
    let backups = get_all_backups_for_app(config);

    let last_full_backup = backups
        .into_iter()
        .filter(|b| b.backup_type == "full")
        .last()
        .unwrap();

    last_full_backup.time
}

// pub fn automatic_backup() {
//     let configs = parse_configs(PathBuf::from("../testdata/config"));

//     // print number of found configs
//     println!("Found {} configs", configs.len());
//     // println!("{:?}", configs);

//     // for each config:
//     // find existing backups
//     for config in configs {
//         println!("Config: {}", config.app_name);
//         let files = match list_files_in_dir(config.local_storage_location.clone().into()) {
//             Ok(files) => files,
//             Err(e) => {
//                 println!("Error: {}", e);
//                 continue;
//             }
//         };

//         // filter files using filter_files_with_extension and "tar"
//         let files = match filter_files_with_extension(files, "tar") {
//             Ok(files) => files,
//             Err(e) => {
//                 println!("Error: {}", e);
//                 continue;
//             }
//         };

//         // print number of found backups
//         println!("Found {} backups", files.len());

//         // parse filenames using parse_backup_filename and order by timestamp
//         let mut files = files
//             .iter()
//             .map(|file| parse_backup_filename(file.to_str().unwrap()))
//             .collect::<Vec<(&str, String, String, DateTime<Utc>)>>();

//         files.sort_by_key(|f| f.3);

//         // reverse order of files vec
//         files.reverse();

//         println!("{:?}", files);

//         // if there are no backups, do a full backup
//         if files.len() == 0 {
//             do_full_backup(&config);
//             return;
//         }

//         // do_full_backup(&config);w
//         do_incremental_backup(&config, files[0].3);
//     }
// }
