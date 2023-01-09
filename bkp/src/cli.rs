use clap::{Args, Parser, Subcommand};

use crate::actions::{full_backup, incremental_backup, list, restore};

// use crate::actions::{full_backup_action, incremental_backup_action, list, restore};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Backs apps up according config file
    Backup(Backup),
    Restore {
        app_name: String,
        backup_name: String,
    },
    List {
        app_name: Option<String>,
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

    // println!("{:?}", args);

    match &args.command {
        Some(Commands::Backup(backup)) => {
            // println!("Automatic backup command");
            // automatic_backup();

            match &backup.command {
                Some(BackupTypes::Full { app_name }) => {
                    println!("Running full backup of {}", app_name);

                    full_backup(app_name);
                }
                Some(BackupTypes::Incremental { app_name }) => {
                    println!("Running incremental backup of {}", app_name);

                    incremental_backup(app_name);
                }
                None => {
                    println!("Please specify backup type");
                }
            }
        }
        Some(Commands::Restore {
            app_name,
            backup_name,
        }) => {
            println!("Running restore of {} from {}", app_name, backup_name);

            restore(app_name.clone(), backup_name.clone());
        }
        Some(Commands::List { app_name }) => {
            list(app_name);
        }
        None => {
            // println!("No command");
        }
    }
}
