use clap::{builder::StringValueParser, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to config file (default: ~/.config/rdoist.toml)
    #[arg(short, long)]
    pub config_path: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>
} 

#[derive(Subcommand)]
pub enum Commands {
    /// Elo 520
    #[command(subcommand)]
    Add(Add),

    #[command(subcommand)]
    List(List)
}

#[derive(Subcommand)]
pub enum Add {
    Task { 
        content: String
    }
}

#[derive(Subcommand)]
pub enum List {
    Project {}
}
