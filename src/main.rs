#![allow(dead_code, unused_variables)]
mod ca;
mod config;
mod error;

use crate::config::{ca_certificate_file, ca_key_file, config_dir_path};
use crate::error::Result;
use ca::{ensure_ca, load_ca};
use clap::{arg, command, value_parser, Command};
use config::load_config;
use std::path::PathBuf;

fn main() -> Result<()> {
    let matches = command!()
        .arg(
            arg!(-c --config <FOLDER> "Configuration folder to use")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .subcommand(
            Command::new("init-ca")
                .about("Initializes the certificate authority")
                .arg(
                    arg!(-f --force "Force regenerate CA. This will overwrite existing config.")
                        .required(false)
                        .value_parser(value_parser!(bool)),
                ),
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

    Ok(())
}
