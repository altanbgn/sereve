use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};
use tabled::Table;
use std::fs::File;

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct List {}

pub fn run(_: &List) {
    let file = File::open("servers.json").unwrap();
    let servers: Vec<Server> = serde_json::from_reader(file).unwrap();

    let mut servers_table: Vec<Server> = vec![];

    servers.into_iter().for_each(|server| {
        servers_table.push(Server::create_table(
            &server.name,
            &server.ip,
            &server.port.to_string(),
        ));
    });

    println!("{}", Table::new(&servers_table).to_string());
}
