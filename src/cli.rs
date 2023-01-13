use clap::{Args, Parser, Subcommand};
use log::{error, info};

use crate::actions::{full_backup, incremental_backup, list, restore};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Lists all backups
    List { app_name: Option<String> },
    /// Backs apps up according to config file
    Backup(Backup),
    /// Restores an app from a specific backup
    Restore {
        app_name: String,
        backup_name: String,
    },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Backup {
    #[command(subcommand)]
    command: Option<BackupTypes>,
}

#[derive(Debug, Subcommand)]
enum BackupTypes {
    Full { app_name: String },
    Incremental { app_name: String },
}

pub fn parse_args() -> () {
    let args = Cli::parse();

    // info!("{:?}", args);

    match &args.command {
        Some(Commands::Backup(backup)) => {
            // info!("Automatic backup command");
            // automatic_backup();

            match &backup.command {
                Some(BackupTypes::Full { app_name }) => {
                    info!("Running full backup of {}", app_name);

                    full_backup(app_name);
                }
                Some(BackupTypes::Incremental { app_name }) => {
                    info!("Running incremental backup of {}", app_name);

                    incremental_backup(app_name);
                }
                None => {
                    info!("Please specify backup type");
                }
            }
        }
        Some(Commands::Restore {
            app_name,
            backup_name,
        }) => {
            info!("Running restore of {} from {}", app_name, backup_name);

            restore(app_name, backup_name);
        }
        Some(Commands::List { app_name }) => {
            list(app_name);
        }
        None => {
            error!("No command");
        }
    }
}
