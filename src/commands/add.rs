use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::fs::{metadata, File, OpenOptions};

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct Add {
    #[arg(long, required = true)]
    name: String,

    #[arg(long, required = true)]
    username: String,

    #[arg(long, required = true)]
    host: String,

    #[arg(long, required = true)]
    port: u16,
}

pub fn run(args: &Add) {
    let home_path = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/servers.json";

    match metadata(&home_path) {
        Ok(_) => {
            let file = File::open(&home_path).unwrap();
            let mut servers: Vec<Server> = serde_json::from_reader(file).unwrap();

            servers.push(Server {
                id: servers.len(),
                name: args.name.clone(),
                username: args.username.clone(),
                host: args.host.clone(),
                port: args.port.clone(),
            });

            let file = OpenOptions::new().write(true).open(&home_path).unwrap();
            match serde_json::to_writer(file, &servers) {
                Ok(_) => println!("Server added"),
                Err(e) => panic!("Error: {}", e),
            };
        }
        Err(e) => match e.raw_os_error() {
            Some(2) => {
                let servers = vec![Server {
                    id: 0,
                    name: args.name.clone(),
                    username: args.username.clone(),
                    host: args.host.clone(),
                    port: args.port.clone(),
                }];
                let file = File::create(&home_path).unwrap();
                match serde_json::to_writer(file, &servers) {
                    Ok(_) => println!("Server added"),
                    Err(e) => panic!("Error: {}", e),
                }
            }
            _ => panic!("Error: {}", e),
        },
    };
}
