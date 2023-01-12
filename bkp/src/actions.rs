use std::{path::PathBuf, str::FromStr};

use crate::{
    backup::{
        do_full_backup, do_incremental_backup, get_all_local_backups,
        get_all_local_backups_for_app, get_last_full_backup_time, prune_local_backups, Backup,
        BackupType,
    },
    compress::decompress_archive,
    config::{get_all_configs, get_config_from_app_name},
    scripts::run_script,
    storage::s3::{download_backup_from_remote, get_all_remote_backups},
};

pub fn list(app_name: &Option<String>) {
    // println!("list");

    match app_name {
        Some(app_name) => {
            let config = get_config_from_app_name(&app_name);
            let backups = get_all_local_backups_for_app(&config);
            println!("{} Backups for {}", backups.len(), app_name);
            backups.iter().for_each(|backup| {
                println!("{:?} {}", backup.backup_type, backup.file_name);
            });
        }
        None => {
            println!("Listing all backups");

            let all_s3_backups = get_all_remote_backups();

            let all_local_backups = get_all_local_backups();

            let configs = get_all_configs();
            for config in configs {
                // let backups = get_all_local_backups_for_app(&config);

                println!("App: {}", config.app_name);
                let local_backups = all_local_backups
                    .iter()
                    .filter(|b| b.app_name == config.app_name)
                    .collect::<Vec<&Backup>>();

                let s3_backups = all_s3_backups
                    .iter()
                    .filter(|b| b.app_name == config.app_name)
                    .collect::<Vec<&Backup>>();

                println!("{} local backups", local_backups.len());
                local_backups.iter().for_each(|backup| {
                    println!("{:?} {}", backup.backup_type, backup.file_name);
                });
                println!("{} remote backups", s3_backups.len());
                s3_backups.iter().for_each(|backup| {
                    println!("{:?} {}", backup.backup_type, backup.file_name);
                });
            }
        }
    }
}

pub fn full_backup(app_name: &String) {
    let config = get_config_from_app_name(app_name);
    run_script(&config.pre_backup_script);
    do_full_backup(&config);
    run_script(&config.post_backup_script);
}

pub fn incremental_backup(app_name: &String) {
    let config = get_config_from_app_name(app_name);
    let last_backup_time = get_last_full_backup_time(&config);
    run_script(&config.pre_backup_script);
    do_incremental_backup(&config, last_backup_time);
    run_script(&config.post_backup_script);
}

pub fn restore(app_name: &String, backup_name: &String) -> () {
    println!("restore");
    let config = get_config_from_app_name(app_name);
    let local_backups = get_all_local_backups_for_app(&config);

    // filter backups until last full backup
    let mut backups_to_restore: Vec<Backup> = Vec::new();
    let mut found = false;
    for backup in local_backups {
        if backup.file_name == *backup_name {
            found = true;
        }
        if found {
            if backup.backup_type == BackupType::Full {
                backups_to_restore.push(backup);
                break;
            }
            backups_to_restore.push(backup);
        }
    }

    if !found {
        println!("Backup not found locally");

        let remote_backups = get_all_remote_backups();

        for backup in remote_backups {
            if backup.file_name == *backup_name {
                // found = true;
                download_backup_from_remote(&backup);
                backups_to_restore.push(backup);
                break;
            }
        }
    }

    backups_to_restore.reverse();

    println!("backups_to_restore: {:#?}", backups_to_restore);

    run_script(&config.pre_restore_script);

    for backup in backups_to_restore {
        decompress_archive(
            backup.path,
            PathBuf::from_str(config.app_root.as_str()).unwrap(),
        );
    }

    run_script(&config.post_restore_script);

    prune_local_backups(&config)
}
