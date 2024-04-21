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

pub(crate) fn create_ca(ca_key: PathBuf, ca_cert: PathBuf) -> Result<(KeyPair, CertificateParams)> {
    eprintln!("Initializing CA...");

    let key_pair = create_keypair(ca_key)?;
    let cert = create_certificate(&key_pair, ca_cert)?;

    Ok((key_pair, cert.clone()))
}

fn load_keypair(ca_key: PathBuf) -> Result<KeyPair> {
    let mut file = File::open(ca_key)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(KeyPair::from_pem(&contents)?)

}

fn create_keypair(ca_key: PathBuf) -> Result<KeyPair> {
    // Generate the keypair
    eprintln!("Generating root CA keypair...");
    let key_pair = KeyPair::generate()?;
    let mut file = File::create(ca_key)?;
    file.write_all(key_pair.serialize_pem().as_bytes())?;
    Ok(key_pair)
}

fn load_certificate(ca_cert: PathBuf) -> Result<CertificateParams> {
    let mut file = File::open(ca_cert)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(CertificateParams::from_ca_cert_pem(&contents)?)
}

fn create_certificate(key_pair: &KeyPair, ca_cert: PathBuf) -> Result<CertificateParams> {
    let san = format!("{CACOMMONNAME}-root");
    let subjects = vec![san.clone()];
    eprintln!("Generating root certificate with SAN {san}");
    let cert = CertificateParams::new(subjects)?.self_signed(&key_pair)?;
    let mut file = File::create(ca_cert)?;
    file.write_all(cert.pem().as_bytes())?;
    Ok(cert.params().clone())
}

pub(crate) fn load_ca(ca_key: PathBuf, ca_cert: PathBuf) -> Result<(KeyPair, CertificateParams)> {
    // Load keypair
    let key_pair = load_keypair(ca_key)?;
    let params = load_certificate(ca_cert)?;
    // Load certificate
    Ok((key_pair, params))
}

pub(crate) fn ensure_ca(ca_key: PathBuf, ca_cert: PathBuf) -> Result<(KeyPair, CertificateParams)> {
    let mut did_generate_key_pair = false;
    let key_pair = if !ca_key.exists() {
        did_generate_key_pair = true;
        create_keypair(ca_key)?
    } else {
        load_keypair(ca_key)?
    };

    let cert = if !ca_cert.exists() || did_generate_key_pair {
        create_certificate(&key_pair, ca_cert)?
    } else {
        load_certificate(ca_cert)?
    };
    Ok((key_pair, cert))
}
