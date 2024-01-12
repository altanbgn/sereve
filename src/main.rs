mod commands;

use clap::Parser;
use commands::MainCommand;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "An ssh client for my personal use :). For now only supports adding, removing, listing and connecting to servers.")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<MainCommand>,
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Server {
    id: usize,
    name: String,
    username: String,
    host: String,
    port: u16,
}

impl Server {
    pub fn create_table(id: &str, name: &str, username: &str, host: &str, port: &str) -> Self {
        Self {
            id: id.parse::<usize>().unwrap(),
            name: name.to_string(),
            username: username.to_string(),
            host: host.to_string(),
            port: port.parse::<u16>().unwrap(),
        }
    }
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(MainCommand::Add(args)) => commands::add::run(args),
        Some(MainCommand::Remove(args)) => commands::remove::run(args),
        Some(MainCommand::List(args)) => commands::list::run(args),
        Some(MainCommand::Connect(args)) => commands::connect::run(args),
        _ => println!("No command provided"),
    }
}
