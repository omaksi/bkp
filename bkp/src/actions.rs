use crate::{
    backup::{
        do_full_backup, do_incremental_backup, get_all_backups_for_app, get_last_full_backup_time,
        Backup,
    },
    config::{get_all_configs, get_config_from_app_name},
    scripts::run_script,
};

pub fn list(app_name: &Option<String>) {
    println!("list");

    match app_name {
        Some(app_name) => {
            println!("app_name: {}", app_name);
            let config = get_config_from_app_name(&app_name);
            let backups = get_all_backups_for_app(&config);
            println!("backups: {:#?}", backups);
        }
        None => {
            let configs = get_all_configs();
            for config in configs {
                println!("app_name: {}", config.app_name);
                let backups = get_all_backups_for_app(&config);
                println!("backups: {:#?}", backups);
            }
            println!("list all apps");
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

pub fn restore(app_name: String, backup_name: String) -> () {
    println!("restore");
    let config = get_config_from_app_name(&app_name);
    let backups = get_all_backups_for_app(&config);

    // filter backups until last full backup
    let mut backups_to_restore: Vec<Backup> = Vec::new();
    let mut found = false;
    for backup in backups {
        if backup.file_name == backup_name {
            found = true;
        }
        if found {
            if backup.backup_type == "full" {
                backups_to_restore.push(backup);
                break;
            }
            backups_to_restore.push(backup);
        }
    }

    println!("backups_to_restore: {:#?}", backups_to_restore);
}