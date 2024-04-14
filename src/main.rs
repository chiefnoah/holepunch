mod ca;
mod config;
mod error;

use crate::config::config_dir_path;
use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

fn main() {
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
        return
    }

}
