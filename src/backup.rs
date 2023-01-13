use std::path::PathBuf;

use chrono::{DateTime, Utc};
use log::info;

use crate::{
    compress::compress_files,
    config::Config,
    globalconfig::GLOBAL_CONFIG,
    storage::{
        fs::{delete_file, filter_files_newer_than, get_files_to_backup, list_files_in_dir},
        s3::{delete_backup_from_remote, get_all_remote_backups, upload_backup_to_remote},
    },
    time::parse_timestamp,
};

#[derive(Debug, PartialEq)]
pub enum BackupType {
    Full,
    Incremental,
}

#[derive(Debug)]
pub struct Backup {
    pub app_name: String,
    pub server_name: String,
    pub path: PathBuf,
    pub file_name: String,
    pub backup_type: BackupType,
    pub time: DateTime<Utc>,
}

// backup naming [app_name]_[server_name]_[backup_type]_[timestamp].tar.gz
pub fn parse_backup_from_path(path: &PathBuf) -> Backup {
    // println!("Parsing backup from path: {:?}", path);
    let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

    let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();
    let parts = file_stem.split("_").collect::<Vec<&str>>();
    let app_name = parts[0].to_string();
    let server_name = parts[1].to_string();
    let backup_type = match parts[2] {
        "full" => BackupType::Full,
        "incremental" => BackupType::Incremental,
        _ => panic!("Unknown backup type"),
    };
    let time = parse_timestamp(parts[3].to_string()).unwrap();

    Backup {
        path: path.clone(),
        file_name,
        app_name,
        server_name,
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

pub fn get_all_local_backups() -> Vec<Backup> {
    let files = list_files_in_dir(GLOBAL_CONFIG.local_storage_location.clone().into()).unwrap();

    let mut backups = parse_backups_from_paths(files);

    backups.sort_by_key(|b| b.time);

    backups
}

pub fn get_all_local_backups_for_app(config: &Config) -> Vec<Backup> {
    let backups = get_all_local_backups();

    // filter files beginning with app_name
    let backups: Vec<Backup> = backups
        .into_iter()
        .filter(|b| b.app_name == config.app_name)
        .collect();

    // println!("Found {} backups", backups.len());

    backups
}

pub fn do_full_backup(config: &Config) {
    let paths = get_files_to_backup(
        config.app_root.clone(),
        config.included_paths.clone(),
        config.excluded_paths.clone(),
    );

    do_backup(config, &paths, "full");

    prune_local_backups(config);
    prune_remote_backups(config);
}

pub fn do_incremental_backup(config: &Config, last_backup_time: &DateTime<Utc>) {
    let paths = get_files_to_backup(
        config.app_root.clone(),
        config.included_paths.clone(),
        config.excluded_paths.clone(),
    );

    // println!("Paths: {:?}", paths);
    // println!("Last backup time: {:?}", last_backup_time);

    // filter paths using filter_files_newer_than and lastBackupTime
    let paths = match filter_files_newer_than(&paths, last_backup_time) {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error: Couldn't filter paths based on modified time {}", e);
            return;
        }
    };

    // println!("Filtered paths: {:?}", paths);

    do_backup(config, &paths, "incremental");
}

fn do_backup(config: &Config, paths: &Vec<PathBuf>, backup_type: &str) {
    // if paths is empty, return with message
    if paths.is_empty() {
        panic!("No files to backup");
    }

    // save current dir
    let current_dir = std::env::current_dir().unwrap();

    // set current dir to app_root
    std::env::set_current_dir(config.app_root.clone()).unwrap();

    // println!("Found {} files", paths.len());
    // println!("{:?}", paths);

    let backup_file_name = config.app_name.clone()
        + "_"
        + config.server_name.as_str()
        + "_"
        + backup_type
        + "_"
        + Utc::now().to_rfc3339().as_str()
        + ".tar";

    // create path for new backup file, it should be config.local_storage_location + app_name + timestamp + .tar
    let backup_file_path =
        PathBuf::from(GLOBAL_CONFIG.local_storage_location.clone()).join(backup_file_name.as_str());

    // println!("Backup file path: {:?}", backup_file_path);

    // remove prefix from paths
    let paths = paths
        .iter()
        .map(|p| {
            p.strip_prefix(config.app_root.clone())
                .unwrap()
                .to_path_buf()
        })
        .collect::<Vec<PathBuf>>();

    info!("Compressing {} files", paths.len());

    // call compress function with backup_file_path and paths
    compress_files(&backup_file_path, &paths);

    // upload file to s3
    upload_backup_to_remote(backup_file_path, backup_file_name);

    // restore current dir
    std::env::set_current_dir(current_dir).unwrap();
}

pub fn get_last_full_backup_time(config: &Config) -> DateTime<Utc> {
    let backups = get_all_local_backups_for_app(config);

    let last_full_backup = backups
        .into_iter()
        .filter(|b| b.backup_type == BackupType::Full)
        .last()
        .unwrap();

    last_full_backup.time
}

pub fn prune_local_backups(config: &Config) {
    let backups = get_all_local_backups_for_app(config);

    let mut backups_to_keep = config.keep_full_local_backups;

    for backup in backups {
        if backup.backup_type == BackupType::Full {
            if backups_to_keep > 0 {
                backups_to_keep -= 1;
            } else {
                info!("Deleting local backup: {:?}", backup.path);
                delete_file(&backup.path);
            }
        }
    }
}

fn prune_remote_backups(config: &Config) {
    let backups = get_all_remote_backups();

    let mut backups_to_keep = config.keep_full_remote_backups;

    for backup in backups {
        if backup.backup_type == BackupType::Full {
            if backups_to_keep > 0 {
                backups_to_keep -= 1;
            } else {
                info!("Deleting remote backup: {:?}", backup.path);
                delete_backup_from_remote(&backup);
            }
        }
    }
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
