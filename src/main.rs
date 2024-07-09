use clap::Parser;
use reqwest::Error;

mod list_cmd;
mod cli;
use cli::{Cli, Commands};

mod config;
mod project;
mod parser;
mod task;
mod todoist_client;
mod add_cmd;


use todoist_client::TodoistClient;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let config = config::load_config(&cli);
    let client = TodoistClient::new(config.api.token);

    match &cli.command {
        Some(Commands::List(target)) => list_cmd::run(&client, &target).await,
        Some(Commands::Add(target)) => add_cmd::run(&client, &target).await,
        None => {
            println!("");
        }
    }

    Ok(())
}
