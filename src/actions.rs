use std::{path::PathBuf, str::FromStr};

use log::{error, info};

use crate::{
    backup::{
        do_full_backup, do_incremental_backup, get_all_local_backups,
        get_all_local_backups_for_app, get_files_changed_since_backup, get_last_backup_time,
        prune_local_backups, prune_remote_backups, Backup, BackupType,
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
            info!("{} Backups for {}", backups.len(), app_name);
            backups.iter().for_each(|backup| {
                info!("{:?} {}", backup.backup_type, backup.file_name);
            });
        }
        None => {
            info!("--------------------------------------------");
            info!("Listing all backups from local applications");

            let all_remote_backups = get_all_remote_backups();

            let all_local_backups = get_all_local_backups();

            let configs = get_all_configs();
            for config in configs.to_vec() {
                // let backups = get_all_local_backups_for_app(&config);

                info!("App: {}", config.app_name);
                let local_backups = all_local_backups
                    .iter()
                    .filter(|b| b.app_name == config.app_name)
                    .collect::<Vec<&Backup>>();

                let remote_backups = all_remote_backups
                    .iter()
                    .filter(|b| b.app_name == config.app_name)
                    .collect::<Vec<&Backup>>();

                info!("{} local backups", local_backups.len());
                local_backups.iter().for_each(|backup| {
                    info!("{:?} {}", backup.backup_type, backup.file_name);
                });
                info!("{} remote backups", remote_backups.len());
                remote_backups.iter().for_each(|backup| {
                    info!("{:?} {}", backup.backup_type, backup.file_name);
                });
            }

            let mut remote_only_backups = all_remote_backups
                .iter()
                .filter(|b| configs.iter().all(|c| c.app_name != b.app_name))
                .collect::<Vec<&Backup>>();

            if remote_only_backups.len() > 0 {
                info!("--------------------------------------------");
                info!("Listing all backups from remote applications");
            } else {
                return;
            }

            remote_only_backups.sort_by_key(|b| b.app_name.clone());

            let mut remote_only_backups_unique_app_names = remote_only_backups
                .iter()
                .map(|b| b.app_name.clone())
                .collect::<Vec<String>>();

            remote_only_backups_unique_app_names.dedup();

            for app_name in remote_only_backups_unique_app_names {
                info!("App: {}", app_name);
                let mut remote_backups = remote_only_backups
                    .iter()
                    .filter(|b| b.app_name == app_name)
                    .collect::<Vec<&&Backup>>();

                remote_backups.sort_by_key(|b| b.file_name.clone());

                info!("{} remote backups", remote_backups.len());
                remote_backups.iter().for_each(|backup| {
                    info!("{:?} {}", backup.backup_type, backup.file_name);
                });
            }
        }
    }
}

pub fn full_backup(app_name: &String) {
    let config = get_config_from_app_name(app_name);
    // info!("Running full backup of {}", app_name);
    info!("Pre backup script: {:?}", config.pre_backup_script);
    run_script(&config.pre_backup_script);
    do_full_backup(&config);
    info!("Post backup script: {:?}", config.post_backup_script);
    run_script(&config.post_backup_script);
    info!("Pruning local backups");
    prune_local_backups(&config);
    info!("Pruning remote backups");
    prune_remote_backups(&config);
}

pub fn incremental_backup(app_name: &String) {
    let config = get_config_from_app_name(app_name);
    let last_backup_time = get_last_backup_time(&config);
    let files_changed_since_backup = get_files_changed_since_backup(&config, &last_backup_time);
    if files_changed_since_backup.len() == 0 {
        info!("No files changed since last backup, skipping incremental backup.");
        return;
    }
    info!("Pre backup script: {:?}", config.pre_backup_script);
    run_script(&config.pre_backup_script);
    do_incremental_backup(&config, &files_changed_since_backup);
    info!("Post backup script: {:?}", config.post_backup_script);
    run_script(&config.post_backup_script);
}

pub fn restore(app_name: &String, backup_name: &String) -> () {
    // println!("restore");
    info!("Restoring {} from {}", app_name, backup_name);
    let config = get_config_from_app_name(app_name);
    let local_backups = get_all_local_backups_for_app(&config);

    // filter backups until last full backup
    let mut backups_to_restore: Vec<Backup> = Vec::new();
    let mut found = false;
    for backup in local_backups {
        println!("{:?} {}", backup.backup_type, backup.file_name);
        if !found && backup.file_name == *backup_name {
            found = true;
        }

        if found {
            backups_to_restore.push(backup.clone());

            if backup.backup_type == BackupType::Full {
                break;
            }
        }
    }

    if !found {
        // println!("Backup not found locally");

        let remote_backups = get_all_remote_backups();

        for backup in remote_backups {
            if backup.file_name == *backup_name {
                found = true;
                download_backup_from_remote(&backup);
                backups_to_restore.push(backup);
                break;
            }
        }
    }

    if !found {
        error!("Couldn't find specified backup");
        return;
    }

    println!("Found {} backups to restore", backups_to_restore.len());
    for backup in &backups_to_restore {
        println!("{}", backup.file_name);
    }

    backups_to_restore.reverse();

    if &config.pre_restore_script == "" {
        info!("No pre restore script");
    } else {
        run_script(&config.pre_restore_script);
    }

    for backup in backups_to_restore {
        info!("Restoring {}", backup.file_name);
        decompress_archive(
            backup.path,
            PathBuf::from_str(config.app_root.as_str()).unwrap(),
        );
    }

    if &config.post_restore_script == "" {
        info!("No post restore script");
    } else {
        run_script(&config.post_restore_script);
    }

    prune_local_backups(&config)
}
