use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};
use std::fs::File;
use tabled::Table;

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct List {}

pub fn run(_: &List) {
    let mut servers_table: Vec<Server> = vec![];
    match File::open("servers.json") {
        Ok(file) => {
            let servers: Vec<Server> = serde_json::from_reader(file).unwrap();

            servers.into_iter().for_each(|server| {
                servers_table.push(Server::create_table(
                    &server.id.to_string(),
                    &server.name,
                    &server.username,
                    &server.host,
                    &server.port.to_string(),
                ));
            });
        }
        _ => {}
    };

    println!("{}", Table::new(&servers_table).to_string());
}
