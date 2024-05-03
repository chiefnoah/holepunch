use crate::config::Config;
use crate::error::Result;
use rustls::server::ClientCertVerifierBuilder;
use rustls::ServerConfig;

pub fn serve(config: Config) -> Result<()> {
    todo!();
    /*
    ServerConfig::builder()
        .with_client_cert_verifier(
            ClientCertVerifierBuilder::with_crls()
        )
        .with_single_cert(

        )
    */
}
