use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlError};
use std::{fs::OpenOptions, io::Read, path::PathBuf};
use xdg::{BaseDirectories, BaseDirectoriesError};

pub const APPNAME: &str = "holepunch";

impl From<BaseDirectoriesError> for Error {
    fn from(value: BaseDirectoriesError) -> Self {
        Self::Config(format!("Configuration error: {value:?}"))
    }
}

impl From<KdlError> for Error {
    fn from(value: KdlError) -> Self {
        Self::ConfigParse(format!("Error parsing profile config: {value}"))
    }
}

fn config_dir() -> Result<BaseDirectories> {
    Ok(BaseDirectories::with_prefix(APPNAME)?)
}

pub(crate) fn config_dir_path() -> Result<PathBuf> {
    config_dir()?
        .get_config_dirs()
        .first()
        .cloned()
        .ok_or(Error::Config(format!("No config dirs available")))
}

/// `ca_key_file` returns a absolute [`PathBuf`]` to the CA signing key within the
/// `XDG_CONFIG_HOME` / holepunch.
pub(crate) fn ca_key_file() -> Result<PathBuf> {
    Ok(config_dir()?.place_config_file("root-ca.key")?)
}

pub(crate) fn ca_certificate_file() -> Result<PathBuf> {
    Ok(config_dir()?.place_config_file("root-ca.cert")?)
}

fn default_config_path() -> Result<PathBuf> {
    Ok(config_dir()?.place_config_file("config.kdl")?)
}

pub(crate) fn load_config(config_path: Option<PathBuf>) -> Result<KdlDocument> {
    let config_path = config_path.unwrap_or(default_config_path()?);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.parse::<KdlDocument>()?)
}
