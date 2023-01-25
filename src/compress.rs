extern crate tar;
use log::{error, info};
use std::io::{self};
use std::{fs::File, path::PathBuf};
use tar::{Archive, Builder};

use crate::backup::get_backup_path_with_extension;

// todo: convert to Result
pub fn compress_files(archive_path: &PathBuf, paths: &Vec<PathBuf>) {
    println!("archive_path: {:?}", archive_path);

    let tar_archive_path = get_backup_path_with_extension(archive_path, ".tar");

    let tar_file_writer = match File::create(&tar_archive_path) {
        Ok(file) => file,
        Err(e) => panic!("Error creating archive_path: {}", e),
    };

    info!("Creating archive: {}", tar_archive_path.display());

    let mut tar_builder = Builder::new(tar_file_writer.try_clone().unwrap());

    for path in paths {
        info!("Adding path to archive: {}", path.display());
        match tar_builder.append_path(path) {
            Ok(_) => (),
            Err(e) => error!("Error appending path to archive: {}", e),
        }
    }

    match tar_builder.finish() {
        Ok(_) => (),
        Err(e) => {
            error!("Error finishing archive: {}", e);
            return;
        }
    }

    let mut tar_file_reader = File::open(&tar_archive_path).unwrap();

    let gz_archive = get_backup_path_with_extension(archive_path, ".tar.gz");

    let tar_gz_file = File::create(gz_archive).unwrap();
    let mut gz_encoder = flate2::write::GzEncoder::new(tar_gz_file, flate2::Compression::default());

    io::copy(&mut tar_file_reader, &mut gz_encoder).unwrap();

    gz_encoder.try_finish().unwrap();
    gz_encoder.finish().unwrap();

    std::fs::remove_file(&tar_archive_path).unwrap();

    info!("Archive created successfully");
}

pub fn decompress_archive(archive: PathBuf, app_root: PathBuf) {
    let mut tar_gz_file_reader = File::open(&archive).unwrap();

    let tar_file_path = archive.with_file_name("tmp.tar");
    let tar_file_writer = match File::create(&tar_file_path) {
        Ok(file) => file,
        Err(e) => panic!("Error creating archive_path: {}", e),
    };

    let mut gz_decoder = flate2::write::GzDecoder::new(&tar_file_writer);

    io::copy(&mut tar_gz_file_reader, &mut gz_decoder).unwrap();

    gz_decoder.try_finish().unwrap();
    gz_decoder.finish().unwrap();

    let tar_file_reader = File::open(&tar_file_path).unwrap();

    let mut tar_archive = Archive::new(tar_file_reader);

    match tar_archive.unpack(app_root) {
        Ok(_) => println!("Backup unpacked successfully"),
        Err(e) => println!("Error unpacking archive: {}", e),
    }

    std::fs::remove_file(&tar_file_path).unwrap();
}
