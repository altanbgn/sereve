use std::fs::{metadata, File, OpenOptions};
use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct Remove {
    #[arg(short, long, required = true)]
    id: usize
}

pub fn run(args: &Remove) {
    match metadata("servers.json") {
        Ok(_) => {
            let file = File::open("servers.json").unwrap();
            let mut servers: Vec<Server> = serde_json::from_reader(file).unwrap();
            servers.retain(|server| server.id != args.id);

            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("servers.json")
                .unwrap();

            match serde_json::to_writer_pretty(file, &servers) {
                Ok(_) => println!("Successfully removed!"),
                Err(e) => panic!("Error: {}", e),
            };

            println!("{:?}", servers);
        },
        Err(e) => {
            match e.raw_os_error() {
                Some(2) => {},
                _ => panic!("Error: {}", e),
            }
        }
    };
}
