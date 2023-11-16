use std::process::Command;
use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct Connect {
    #[arg(short, long, required = true)]
    id: usize,
}

pub fn run(args: &Connect) {
    let file = File::open("servers.json").unwrap();
    let servers: Vec<Server> = serde_json::from_reader(file).unwrap();
    let found_server = servers
        .into_iter()
        .find(|server| server.id == args.id)
        .unwrap();

    let _ = Command::new("ssh")
        .arg(format!("{}@{}", found_server.username, found_server.host))
        .spawn()
        .expect("Failed to execute command");
}
