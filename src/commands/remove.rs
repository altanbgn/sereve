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
    let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.servers.json";
    match metadata(&home_path) {
        Ok(_) => {
            let file = File::open(&home_path).unwrap();
            let mut servers: Vec<Server> = serde_json::from_reader(file).unwrap();
            servers.retain(|server| server.id != args.id);

            let file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&home_path)
                .unwrap();

            match serde_json::to_writer_pretty(file, &servers) {
                Ok(_) => println!("Successfully removed!"),
                Err(e) => panic!("Error: {}", e),
            };
        },
        Err(e) => {
            match e.raw_os_error() {
                Some(2) => {},
                _ => panic!("Error: {}", e),
            }
        }
    };
}
