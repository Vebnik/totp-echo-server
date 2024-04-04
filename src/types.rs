use clap::Parser;
use serde::{Deserialize, Serialize};

/// Simple http server for a get TOTP password
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Port
    #[arg(long, default_value_t = 8000)]
    pub port: i32,

    /// Host
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFA {
    pub code: String,
    pub ttl: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtpAuthCreate {
    pub url: String,
}