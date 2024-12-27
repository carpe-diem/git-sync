mod config;

use clap::{Parser, Subcommand};
use std::io;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure the application
    Setup,
    /// Synchronize notes with GitHub
    Sync,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup => {
            let config = config::Config::setup()?;
            println!("\nCurrent configuration:");
            println!("{:#?}", config);
        }
        Commands::Sync => {
            let _config =
                config::Config::load()?.expect("No configuration found. Please run 'setup' first");
            println!("Synchronizing notes...");
            // TODO: Implement synchronization
        }
    }

    Ok(())
}
