use clap::{Subcommand, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub config_path: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long)]
    pub json: Option<bool>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    List(List),
    #[command(subcommand)]
    Add(Add),
    #[command(subcommand)]
    Push(Push),
    #[command(subcommand)]
    Pop(Pop),
}

#[derive(Subcommand)]
pub enum Push {
    Task {}
}

#[derive(Subcommand)]
pub enum Pop {}

#[derive(Subcommand)]
pub enum List {
    Project {}
}

#[derive(Subcommand)]
pub enum Add {
    Task {
        content: String
    },
    Project {},
}

