use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

/// Holds all “from‐ENV” server settings.
#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub cert_path: String,
    pub key_path: String,
}

impl Config {
    /// Load HOST, PORT, TLS_CERT, TLS_KEY (with defaults) from .env or environment.
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok(); // load .env if present

        Ok(Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8443".into())
                .parse()
                .expect("PORT must be an integer"),
            cert_path: env::var("TLS_CERT").unwrap_or_else(|_| "cert.pem".into()),
            key_path: env::var("TLS_KEY").unwrap_or_else(|_| "key.pem".into()),
        })
    }

    /// Returns a “host:port” string to bind to.
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// A global, lazily‐initialized Config instance.
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_env().expect("Invalid environment"));
