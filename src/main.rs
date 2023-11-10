mod commands;

use serde::{Deserialize, Serialize};
use clap::Parser;
use commands::MainCommand;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "A basic example of clap usage.")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<MainCommand>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Server {
    name: String,
    ip: String,
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(MainCommand::Add(args)) => commands::add::run(args),
        Some(MainCommand::Remove(args)) => commands::remove::run(args),
        _ => println!("No command provided"),
    }

    println!("{:?}", cli);
}
