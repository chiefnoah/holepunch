#![allow(dead_code, unused_variables)]
mod ca;
mod config;
mod error;

use crate::error::Result;
use crate::{
    ca::ensure_ca,
    config::{ca_certificate_file, ca_key_file, config_dir_path},
};
use clap::{arg, command, value_parser, Command};
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
        .or(Some(&config_dir_path().expect("XDG config dir")));

    if let Some(init_ca) = matches.subcommand_matches("init-ca") {
        eprintln!("Initializing CA...");
        let (keys, ca_params) = ensure_ca(ca_key_file()?, ca_certificate_file()?)?;
    }
    Ok(())
}
