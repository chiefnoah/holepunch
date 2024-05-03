use crate::error::{Error, Result};
use kdl::{KdlDocument, KdlError, KdlNode};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};
use xdg::{BaseDirectories, BaseDirectoriesError};

pub const APPNAME: &str = "holepunch";

pub(crate) struct Config {
    pub ca: CAConfig,
    pub profiles: Vec<ProfileConfig>,
}

pub(crate) enum CAConfig {
    Unmanaged,
    Managed {
        certificate: PathBuf,
        key: PathBuf,
        crls: PathBuf,
    },
}

pub(crate) struct ProfileConfig {
    name: String,
    certificate: Option<PathBuf>,
}

fn type_error(section: &str, expected: &str) -> Error {
    Error::Config(format!(
        "Incorrect type for '{section}'. Expected `{expected}`."
    ))
}

fn missing_arg(key: &str, section: &str) -> Error {
    Error::Config(format!("Missing '{key}' arg for '{section}' section."))
}
// parse a KDL document into our config format.
impl TryFrom<KdlDocument> for Config {
    type Error = Error;

    fn try_from(doc: KdlDocument) -> std::result::Result<Self, Self::Error> {
        let caconfig_node = doc
            .get("ca")
            .ok_or(Error::Config(format!("Missing 'ca' section")))?;
        // whether we're managing certificates or not
        let managed: bool = caconfig_node
            .get("managed")
            .ok_or(missing_arg("managed", "ca"))?
            .value()
            .as_bool()
            .ok_or(type_error("ca managed=???", "bool"))?;
        let ca = if managed {
            let certificate: PathBuf = caconfig_node
                .get("certificate")
                .ok_or(missing_arg("certificate", "ca"))?
                .value()
                .as_string()
                .ok_or(type_error("ca certificate=???", "string"))?
                .into();
            let key: PathBuf = caconfig_node
                .get("key")
                .ok_or(missing_arg("key", "ca"))?
                .value()
                .as_string()
                .ok_or(type_error("ca key=???", "string"))?
                .into();
            let crls: PathBuf = caconfig_node
                .get("crls")
                .ok_or(missing_arg("crls", "ca"))?
                .value()
                .as_string()
                .ok_or(type_error("ca crls=???", "string"))?
                .into();
            CAConfig::Managed {
                certificate,
                key,
                crls,
            }
        } else {
            CAConfig::Unmanaged
        };
        // if any of the profiles fail to parse, error out
        // See:
        // [this](https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect)
        // for details. It's kinda nifty!
        let profiles: Vec<ProfileConfig> = doc
            .nodes()
            .iter()
            .map(ProfileConfig::try_from)
            .collect::<Result<Vec<ProfileConfig>>>()?;
        Ok(Config { ca, profiles })
    }
}

impl TryFrom<&KdlNode> for ProfileConfig {
    type Error = Error;

    fn try_from(node: &KdlNode) -> std::result::Result<Self, Self::Error> {
        let name: String = node.name().value().to_string();
        let certificate = if let Some(certificate) = node.get("certificate") {
            Some(
                certificate
                    .value()
                    .as_string()
                    .ok_or(type_error("profile certificate=???", "string"))?
                    .into(),
            )
        } else {
            None
        };
        Ok(ProfileConfig { name, certificate })
    }
}

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
    Ok(config_dir()?.get_config_home())
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
        .create(true)
        .open(&config_path)?;
    if config_path.exists() {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents.parse::<KdlDocument>()?)
    } else {
        let config = default_config();
        file.write_all(config.to_string().as_bytes())?;
        Ok(config)
    }
}

fn default_config() -> KdlDocument {
    r#"
// The ca may be externally managed
ca managed=true \
   certificate="./root-ca.cert" \
   key="./root-ca.key" \
   crls="./root-ca.crl"


profiles {
    default
}
    "#
    .parse::<KdlDocument>()
    .unwrap()
}
