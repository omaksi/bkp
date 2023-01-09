extern crate tar;

use std::{fs::File, path::PathBuf};
// use std::io::prelude::*;
use tar::{Archive, Builder};

// todo: convert to Result
pub fn compress_files(archive: PathBuf, paths: &Vec<PathBuf>) {
    let tar_file = match File::create(archive) {
        Ok(file) => file,
        Err(e) => panic!("Error creating archive: {}", e),
    };

    let mut tar_builder = Builder::new(tar_file);

    for path in paths {
        match tar_builder.append_path(path) {
            Ok(_) => (),
            Err(e) => println!("Error appending path to archive: {}", e),
        }
    }

    match tar_builder.finish() {
        Ok(_) => (),
        Err(e) => println!("Error finishing archive: {}", e),
    }
}

pub fn decompress_archive(archive: PathBuf, app_root: PathBuf) {
    let tar_file = File::open(archive).unwrap();

    let mut tar_archive = Archive::new(tar_file);

    match tar_archive.unpack(app_root) {
        Ok(_) => println!("Backup unpacked successfully"),
        Err(e) => println!("Error unpacking archive: {}", e),
    }
}
