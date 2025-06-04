use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{
    fs::File,
    io::{BufReader, Error as IoError, ErrorKind},
};

/// Load a TLS ServerConfig from the given cert file and key file paths.
///
/// Both files must be in PEM (privacy‐enhanced mail) format. You can generate them
/// via OpenSSL or certbot. Returns an `std::io::Result<ServerConfig>`.
pub fn load_tls(cert_path: &str, key_path: &str) -> std::io::Result<ServerConfig> {
    // Open the certificate and key files
    let mut cert_reader = BufReader::new(File::open(cert_path)?);
    let mut key_reader = BufReader::new(File::open(key_path)?);

    // Read all certificates (PEM) into a Vec<rustls::Certificate>
    let cert_chain: Vec<Certificate> = certs(&mut cert_reader)?
        .into_iter()
        .map(Certificate)
        .collect();

    // Read all private keys (PKCS#8) into Vec<rustls::PrivateKey>
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(&mut key_reader)?
        .into_iter()
        .map(PrivateKey)
        .collect();

    // Use the last private key in the file (if any)
    let priv_key = keys
        .pop()
        .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, "no private key found"))?;

    // Build a ServerConfig with safe defaults plus our certificate + key pair,
    // and no client authentication. This is standard for an HTTPS server.
    let server_config = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .unwrap() // these unwraps only fail if defaults are invalid
        .with_no_client_auth()
        .with_single_cert(cert_chain, priv_key)
        .map_err(|e| IoError::new(ErrorKind::InvalidInput, format!("TLS error: {e}")))?;

    Ok(server_config)
}
