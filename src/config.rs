

use serde::Deserialize;
use std::net::SocketAddr;
use std::path::PathBuf;
use clap::Parser;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ServerConfig {
    /// IP:port to listen on, e.g. 127.0.0.1:8080.
    pub listen: SocketAddr,

    /// HTTPS Certificate.
    pub ssl_certificate: Option<PathBuf>,

    /// HTTPS private key.
    pub ssl_certificate_key: Option<PathBuf>,

    /// Insecure HTTP.
    #[serde(default)]
    pub insecure: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct HttpConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct KfdConfig {
    #[serde(default="default_bool_true")]
    pub daemon: bool,

    #[serde(default="default_log_level")]
    pub log_level: String,

    pub work_dir: Option<String>,

    #[serde(default="default_user")]
    pub user: String,

    #[serde(default="default_user")]
    pub group: String,

    /// Configuration for the KFD Http Server
    pub http: HttpConfig,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to a KFD config file. Supported formats: TOML, YAML, JSON and possibly other formats
    /// supported by the `config` crate.
    #[arg(short, long, env = "KFD_CONFIG_FILE")]
    pub config_file: String,
}

fn default_user() -> String {
    "nobody".to_string()
}

fn default_bool_true() -> bool {
    true
}

fn default_log_level() -> String {
    "info".to_string()
}
