mod commands;

use serde::{Deserialize, Serialize};
use clap::Parser;
use tabled::{Tabled};
use commands::MainCommand;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "A basic example of clap usage.")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<MainCommand>
}

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Server {
    name: String,
    ip: String,
    port: u16,
}

impl Server {
    pub fn create_table(name: &str, ip: &str, port: &str) -> Self {
        Self {
            name: name.to_string(),
            ip: ip.to_string(),
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
        _ => println!("No command provided"),
    }
}
