// use std::fs;
use std::{
    fs::{metadata, read_dir, File},
    io::{Error, Read},
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};

use glob::glob;

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

pub fn get_files_to_backup(
    app_root: String,
    included_paths: Vec<String>,
    excluded_paths: Vec<String>,
) -> Vec<PathBuf> {
    let mut included_pathbufs: Vec<PathBuf> = Vec::new();
    let mut excluded_pathbufs: Vec<PathBuf> = Vec::new();

    // get included_pathbufs using glob
    for path in included_paths {
        let full_path = app_root.clone() + &path;
        for entry in glob(&full_path).unwrap() {
            included_pathbufs.push(entry.unwrap());
        }
    }

    // println!("included_pathbufs: {:?}", included_pathbufs);

    // get excluded_pathbufs using glob
    for path in excluded_paths {
        let full_path = app_root.clone() + &path;
        for entry in glob(&full_path).unwrap() {
            excluded_pathbufs.push(entry.unwrap());
        }
    }

    // println!("excluded_pathbufs: {:?}", excluded_pathbufs);

    // remove excluded_pathbufs from included_pathbufs
    for excluded_pathbuf in excluded_pathbufs {
        included_pathbufs.retain(|pathbuf| pathbuf != &excluded_pathbuf);
    }

    // return included_pathbufs
    included_pathbufs
}

pub fn filter_files_newer_than(
    paths: &Vec<PathBuf>,
    time: DateTime<Utc>,
) -> Result<Vec<PathBuf>, Error> {
    let mut filtered_paths: Vec<PathBuf> = Vec::new();

    for path in paths {
        let last_modified = metadata(path)?.modified()?;
        let last_modified: chrono::DateTime<Utc> = DateTime::from(last_modified);

        if last_modified > time {
            filtered_paths.push(path.clone());
        }
    }

    Ok(filtered_paths)
}
