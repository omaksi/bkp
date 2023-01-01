extern crate tar;

use std::{fs, path::PathBuf};
// use std::io::prelude::*;
use tar::Builder;

// todo: convert to Result
pub fn compress_files(archive: PathBuf, paths: &Vec<PathBuf>) {
    let tar_file = match fs::File::create(archive) {
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
