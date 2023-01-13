extern crate tar;
use log::{error, info};
use std::{fs::File, path::PathBuf};
use tar::{Archive, Builder};

// todo: convert to Result
pub fn compress_files(archive: &PathBuf, paths: &Vec<PathBuf>) {
    let tar_file = match File::create(archive) {
        Ok(file) => file,
        Err(e) => panic!("Error creating archive: {}", e),
    };

    info!("Creating archive: {}", archive.display());

    let mut tar_builder = Builder::new(tar_file);

    for path in paths {
        info!("Adding path to archive: {}", path.display());
        match tar_builder.append_path(path) {
            Ok(_) => (),
            Err(e) => error!("Error appending path to archive: {}", e),
        }
    }

    match tar_builder.finish() {
        Ok(_) => (),
        Err(e) => error!("Error finishing archive: {}", e),
    }

    // let tar_gz_file = File::create("foo.tar.gz").unwrap();
    // let mut gz_writer = flate2::write::GzEncoder::new(tar_gz_file, flate2::Compression::default());
    // // gz_writer.write_all(tar_file.read).unwrap();
    // tar_file.read_to_end(gz_writer.).unwrap();
}

pub fn decompress_archive(archive: PathBuf, app_root: PathBuf) {
    let tar_file = File::open(archive).unwrap();

    let mut tar_archive = Archive::new(tar_file);

    match tar_archive.unpack(app_root) {
        Ok(_) => println!("Backup unpacked successfully"),
        Err(e) => println!("Error unpacking archive: {}", e),
    }
}
