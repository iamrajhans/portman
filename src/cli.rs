use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "portman")]
#[command(about = "A CLI tool for managing ports and processes on your system")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all occupied ports with process information
    List {
        /// Show only ports in the specified range (e.g., 3000-9000)
        #[arg(long, value_name = "START-END")]
        range: Option<String>,

        /// Filter processes by name (e.g., node, java, python)
        #[arg(long)]
        filter: Option<String>,

        /// Show only common development ports
        #[arg(long)]
        common: bool,

        /// Output format
        #[arg(long, value_enum, default_value = "table")]
        format: OutputFormat,
    },

    /// Kill process(es) using the specified port(s)
    Kill {
        /// Port number(s) to kill processes on
        ports: Vec<u16>,

        /// Kill without confirmation prompt
        #[arg(long, short)]
        force: bool,
    },

    /// Check if port(s) are available
    Check {
        /// Port numbers or ranges to check (e.g., 3000, 3000-3010)
        ports: Vec<String>,
    },

    /// Watch ports defined in configuration file
    Watch {
        /// Configuration file path (defaults to .portman.yaml)
        #[arg(short, long)]
        config: Option<String>,
    },

    /// Kill all processes on common development ports
    Free {
        /// Target common development ports only
        #[arg(long)]
        common: bool,

        /// Kill without confirmation prompt
        #[arg(long, short)]
        force: bool,
    },

    /// Initialize a .portman.yaml config file in current directory
    Init {
        /// Overwrite existing config file
        #[arg(long)]
        force: bool,
    },

    /// Show history of recent port management actions
    History {
        /// Number of recent actions to show
        #[arg(long, short, default_value = "10")]
        limit: usize,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}
