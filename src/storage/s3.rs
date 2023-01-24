use std::path::PathBuf;

use log::info;
use s3::creds::Credentials;
use s3::region::Region;
use s3::Bucket;

use crate::{
    backup::{parse_backup_from_path, Backup},
    globalconfig::GLOBAL_CONFIG,
};

fn create_bucket() -> Bucket {
    Bucket::new(
        "bkp",
        Region::Custom {
            region: "eu-central-1".to_string(),
            endpoint: GLOBAL_CONFIG.remote_storage_address.to_string(),
        },
        Credentials::new(
            Some(&GLOBAL_CONFIG.remote_storage_access_id),
            Some(&GLOBAL_CONFIG.remote_storage_secret_key),
            None,
            None,
            None,
        )
        .unwrap(),
    )
    .unwrap()
    .with_path_style()
}

pub fn get_all_remote_backups() -> Vec<Backup> {
    let bucket = create_bucket();

    let list_response = bucket.list("/".to_string(), Some("/".to_string())).unwrap();

    // println!("s3 list response: {:?}", list_response[0].contents);
    let mut backups: Vec<Backup> = list_response[0]
        .contents
        .clone()
        .into_iter()
        .map(|s3object| parse_backup_from_path(&s3object.key.into()))
        .collect();

    backups.sort_by_key(|b| b.time);

    backups
}

pub fn upload_backup_to_remote(backup_file_path: PathBuf, backup_file_name: String) {
    let bucket = create_bucket();

    let mut reader = std::fs::File::open(&backup_file_path).unwrap();
    info!("Uploading backup to remote storage: {:?}", backup_file_path);
    bucket
        .put_object_stream(&mut reader, backup_file_name)
        .unwrap();
    // assert_eq!(response_data.status_code(), 200);
}

pub fn download_backup_from_remote(backup: &Backup) {
    let bucket = create_bucket();

    // create file writer
    let mut writer = std::fs::File::create(&backup.path).unwrap();

    bucket
        .get_object_stream(backup.path.to_str().unwrap(), &mut writer)
        .unwrap();
}

pub fn delete_backup_from_remote(backup: &Backup) {
    let bucket = create_bucket();

    let response_data = bucket
        .delete_object(&backup.path.to_str().unwrap())
        .unwrap();
    assert_eq!(response_data.status_code(), 204);
}

// fn parse_backup_from_s3object(path: &PathBuf) -> Backup {
//     let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

//     let parts = file_name.split("_").collect::<Vec<&str>>();
//     let app_name = parts[0].to_string();
//     let backup_type = parts[1].to_string();
//     let time = parse_timestamp(parts[2].to_string()).unwrap();

//     Backup {
//         path: path.clone(),
//         file_name,
//         app_name,
//         backup_type,
//         time,
//     }
// }
