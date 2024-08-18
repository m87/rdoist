use clap::Parser;
use ureq::Agent;

mod project;
mod cli;
mod config;
mod list_cmd;
use cli::Cli;
use cli::Commands;


fn main() {
    let cli = Cli::parse();
    let config = config::load_config(&cli);
    let agent: Agent = ureq::AgentBuilder::new()
        .build();
    
    match cli.command {
        Some(Commands::List(target)) => list_cmd::run(&agent, &target, &config.api.token),
        None => panic!()
    }
}
