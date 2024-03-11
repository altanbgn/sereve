use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::process::Command;

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct Connect {
    #[arg(short, long, required = true)]
    id: usize
}

pub fn run(args: &Connect) {
    let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/servers.json";
    let file = File::open(home_path).unwrap();
    let servers: Vec<Server> = serde_json::from_reader(file).unwrap();
    let found_server = servers
        .into_iter()
        .find(|server| server.id == args.id)
        .unwrap();

    let mut child = Command::new("ssh")
        .arg(format!("{}@{}", &found_server.username, &found_server.host))
        .arg("-p")
        .arg(format!("{}", &found_server.port))
        .spawn()
        .expect("Failed to connect to server");

    child.wait().expect("Failed to wait on child");
}

