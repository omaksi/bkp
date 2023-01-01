use clap::{Args, Parser, Subcommand};

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
    // Stash(Stash),
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

    println!("{:?}", args);

    match &args.command {
        Some(Commands::Backup(backup)) => {
            println!("Backup command");
            match &backup.command {
                Some(BackupTypes::Full { app_name }) => {
                    println!("Full command");
                    println!("{:?}", app_name);
                }
                Some(BackupTypes::Incremental { app_name }) => {
                    println!("Incremental command");
                    println!("{:?}", app_name);
                }
                None => {
                    println!("No backup type");
                }
            }
        }
        None => {
            println!("No command");
        }
    }
}
