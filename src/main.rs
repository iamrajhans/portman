mod cli;
mod commands;
mod config;
mod output;
mod process;
mod scanner;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List {
            range,
            filter,
            common,
            format,
        } => {
            commands::list::execute(range, filter, common, format).await?;
        }
        Commands::Kill { ports, force } => {
            commands::kill::execute(ports, force).await?;
        }
        Commands::Check { ports } => {
            let all_available = commands::check::execute(ports).await?;
            if !all_available {
                std::process::exit(1);
            }
        }
        Commands::Watch { config } => {
            commands::watch::execute(config).await?;
        }
        Commands::Free { common, force } => {
            commands::free::execute(common, force).await?;
        }
        Commands::Init { force } => {
            commands::init::execute(force).await?;
        }
        Commands::History { limit } => {
            commands::history::execute(limit).await?;
        }
    }

    Ok(())
}
