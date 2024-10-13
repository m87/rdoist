use clap::Parser;
use ureq::Agent;

mod project;
mod cli;
mod config;
mod list_cmd;
mod add_cmd;
mod parser;
mod task;
use cli::Cli;
use cli::Commands;


fn main() {
    let cli = Cli::parse();
    let config = config::load_config(&cli);
    let agent: Agent = ureq::AgentBuilder::new()
        .build();

    match cli.command {
        Some(Commands::List(target)) => list_cmd::run(&agent, &config.api.token, &target),
        Some(Commands::Add(target)) => add_cmd::run(&agent, &config.api.token, &target),
        Some(Commands::Push(target)) => panic!(),
        Some(Commands::Pop(target)) => panic!(),
        None => panic!()
    }
}
