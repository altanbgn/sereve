use crate;
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
    match metadata("servers.json") {
        Ok(_) => {
            let file = File::open("servers.json").unwrap();
            let mut servers: Vec<Servers>
        }
    }
}
