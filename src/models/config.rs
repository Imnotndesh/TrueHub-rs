use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub server_url: String,
    #[serde(default)]
    pub insecure: bool,
    #[serde(default = "default_enable_ping")]
    pub enable_ping: bool,
    #[serde(default = "default_ping_timeout_ms")]
    pub ping_timeout_ms: u64,
    #[serde(default = "default_connection_timeout_ms")]
    pub connection_timeout_ms: u64,
    #[serde(default = "default_enable_debug_logging")]
    pub enable_debug_logging: bool,
}

fn default_enable_ping() -> bool { true }
fn default_ping_timeout_ms() -> u64 { 5000 }
fn default_connection_timeout_ms() -> u64 { 10000 }
fn default_enable_debug_logging() -> bool { true }

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_url: String::new(),
            insecure: false,
            enable_ping: true,
            ping_timeout_ms: 5000,
            connection_timeout_ms: 10000,
            enable_debug_logging: true,
        }
    }
}