use std::io::{self, Read, Write};
use std::fs::File;
use crate::Server;
use clap::Args;
use serde::{Deserialize, Serialize};
use ssh2::Session;

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct Connect {
    #[arg(short, long, required = true)]
    id: usize
}

pub fn run(args: &Connect) {
    let password = rpassword::prompt_password("Enter password: ").unwrap();
    let file = File::open("servers.json").unwrap();
    let servers: Vec<Server> = serde_json::from_reader(file).unwrap();
    let found_server = servers
        .into_iter()
        .find(|server| server.id == args.id)
        .unwrap();

    let tcp_stream =
        std::net::TcpStream::connect(format!("{}:{}", &found_server.host, &found_server.port)).unwrap();

    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp_stream);
    session.handshake().unwrap();

    match session.userauth_password(&found_server.username, &password) {
        Ok(_) => {
            println!("Connected to {}", &found_server.name);

            let mut channel = session.channel_session().unwrap();
            channel.request_pty("xterm", None, None).unwrap();
            channel.shell().unwrap();

            match channel {
                Ok(mut channel) => {

                }
            }
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    };

}
