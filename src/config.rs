use crate::error::{Error, Result};
use std::path::PathBuf;
use xdg::{BaseDirectories, BaseDirectoriesError};

pub const APPNAME: &str = "holepunch";

impl From<BaseDirectoriesError> for Error {
    fn from(value: BaseDirectoriesError) -> Self {
        Self::Config(format!("Configuration error: {value:?}"))
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
