#![allow(dead_code, unused_variables)]
mod ca;
mod config;
mod error;

use crate::config::{ca_certificate_file, ca_key_file, config_dir_path};
use crate::error::Result;
use ca::{ensure_ca, load_ca};
use clap::{arg, command, value_parser, Command};
use config::load_config;
use std::net::TcpListener;
use std::path::PathBuf;

fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(-c --config <FOLDER> "Configuration folder to use")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("serve")
                .about("Runs holepunch in server mode")
                .arg(arg!(-p --port <PORT> "Specifies the port to bind to"))
                .arg(arg!(-a --address <ADDRESS> "Specifies the address to bind to")),
        )
        .subcommand(
            Command::new("connect")
                .about("Runs holepunch in client mode. Connects to specified")
                .arg(arg!(--stdin "Runs the client in stdin mode."))
                .arg(arg!(<ADDRESS> "The address of the server to connect to.")),
        )
        .get_matches();

    let config_path = matches
        .get_one::<PathBuf>("config")
        .cloned()
        .or(Some(config_dir_path().expect("XDG config dir")));
    let config = load_config(config_path)?;
    let managed: bool = config
        .get("ca")
        .map(|node| node["managed"].as_bool().unwrap_or(false))
        .unwrap_or(true);

    // TODO: load the ca_key_file and certificate_file paths from the config file
    let (keys, ca_params) = if managed {
        ensure_ca(ca_key_file()?, ca_certificate_file()?)?
    } else {
        load_ca(ca_key_file()?, ca_certificate_file()?)?
    };

    if let Some(server) = matches.subcommand_matches("serve") {
        let port = *matches.get_one::<u16>("port").unwrap_or(&5555);
        let addr = matches
            .get_one::<String>("address")
            .unwrap_or(&"0.0.0.0".to_string());
        let listener = TcpListener::bind(addr).expect("Unable to bind to port");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // we probably need to add an evelope around the messages, at least with a size
                    todo!()
                },
                Err(e) => eprintln!("Connection failed"),
            }
        }
    }

    Ok(())
}
