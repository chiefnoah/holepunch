#![allow(dead_code)]
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::config::APPNAME;
use crate::error::{Error, Result};
use rcgen::{CertificateParams, Error as RCGenError, KeyPair};

const CACOMMONNAME: &str = APPNAME;
const CACOUNTRY: &str = "NET";
// five years
const CAEXPIRY: usize = 1825;

impl From<RCGenError> for Error {
    fn from(value: RCGenError) -> Self {
        Self::Certificate(format!("Error with CA or chain: {value}"))
    }
}

/// Creates or reads a CA keypair and certificate.
pub(crate) fn ensure_ca(ca_key: PathBuf, ca_cert: PathBuf) -> Result<(KeyPair, CertificateParams)> {
    let mut did_generate_cert_keypair = false;
    let key_pair = if !ca_key.exists() {
        let key_pair = KeyPair::generate()?;
        let mut file = File::create(ca_key)?;
        file.write_all(key_pair.serialize_pem().as_bytes())?;
        did_generate_cert_keypair = true;
        key_pair
    } else {
        let mut file = File::open(ca_key)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        KeyPair::from_pem(&contents)?
    };

    let cert_params = if !ca_cert.exists() || did_generate_cert_keypair {
        let subjects = vec![CACOMMONNAME.to_string()];
        let cert = CertificateParams::new(subjects)?.self_signed(&key_pair)?;
        let mut file = File::create(ca_cert)?;
        file.write_all(cert.pem().as_bytes())?;
        cert.params().clone()
    } else {
        let mut file = File::open(ca_cert)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        CertificateParams::from_ca_cert_pem(&contents)?
    };
    Ok((key_pair, cert_params))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_create_ca() {}
}
