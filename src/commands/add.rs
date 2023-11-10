use std::fs::{metadata, File, OpenOptions};
use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct Add {
    #[arg(short, long, required = true)]
    name: String,

    #[arg(short, long, required = true)]
    ip: String,

    #[arg(short, long, required = true)]
    port: u16,
}

pub fn run(args: &Add) {
    match metadata("servers.json") {
        Ok(_) => {
            let file = File::open("servers.json").unwrap();
            let mut servers: Vec<Server> = serde_json::from_reader(file).unwrap();

            servers.push(Server {
                name: args.name.clone(),
                ip: args.ip.clone(),
                port: args.port.clone(),
            });

            let file = OpenOptions::new()
                .write(true)
                .open("servers.json")
                .unwrap();

            match serde_json::to_writer(file, &servers) {
                Ok(_) => println!("Server added"),
                Err(e) => panic!("Error: {}", e),
            };
        }
        Err(e) => {
            match e.raw_os_error() {
                Some(2) => {
                    let servers = vec![Server {
                        name: args.name.clone(),
                        ip: args.ip.clone(),
                        port: args.port.clone(),
                        }];
                    let file = File::create("servers.json").unwrap();
                    match serde_json::to_writer(file, &servers) {
                        Ok(_) => println!("Server added"),
                        Err(e) => panic!("Error: {}", e),
                    }
                }
                _ => panic!("Error: {}", e)
            }
        }
    };
}
