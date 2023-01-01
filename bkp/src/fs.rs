// use std::fs;
use std::{
    fs::{metadata, read_dir, File},
    io::{Error, Read},
    path::{Path, PathBuf},
    time::SystemTime,
};

use chrono::{DateTime, Utc};

use crate::time::parse_timestamp;

pub fn read_file_to_string(path: &Path) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn list_files_in_dir(dir: PathBuf) -> Result<Vec<PathBuf>, Error> {
    let mut paths: Vec<PathBuf> = Vec::new();
    list_files_rec(dir, &mut paths)?;
    Ok(paths)
}

fn list_files_rec(dir: PathBuf, mut paths: &mut Vec<PathBuf>) -> Result<(), Error> {
    for entry in read_dir(dir)? {
        let path = entry?.path();
        match path.is_dir() {
            true => list_files_rec(path, &mut paths)?,
            false => paths.push(path),
        }
    }
    Ok(())
}

pub fn filter_files_newer_than(
    paths: &Vec<PathBuf>,
    time: SystemTime,
) -> Result<Vec<PathBuf>, Error> {
    let mut filtered_paths: Vec<PathBuf> = Vec::new();

    for path in paths {
        let last_modified = metadata(path)?.modified()?;

        if last_modified > time {
            filtered_paths.push(path.clone());
        }
    }

    Ok(filtered_paths)
}

pub fn filter_files_with_extension(
    paths: Vec<PathBuf>,
    extension: &str,
) -> Result<Vec<PathBuf>, Error> {
    let mut filtered_paths: Vec<PathBuf> = Vec::new();

    for path in paths {
        match path.extension() {
            Some(path_extension) => {
                if path_extension == extension {
                    filtered_paths.push(path.clone());
                }
            }
            None => continue,
        }
    }

    Ok(filtered_paths)
}

// pub fn parse_backup_filename(filename: &str) -> (String, DateTime<Utc>) {
//     let filename_parts: Vec<&str> = filename.split('.').collect();
//     let name_parts: Vec<&str> = filename_parts[0].split('-').collect();

//     let name = name_parts[0].to_string();
//     let timestamp = match parse_timestamp(name_parts[1].to_string()) {
//         Some(timestamp) => timestamp,
//         None => panic!("Unable to parse timestamp from filename"),
//     };

//     (name, timestamp)
// }
