use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Port(u16);
impl Default for Port {
    fn default() -> Self {
        Self(3030)
    }
}
struct IPV4 {}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigError {
    InvalidPort(u16),
}

impl Port {
    pub fn new(port: u16) -> Result<(), ConfigError> {
        if port > 65535 || port < 1025 {
            return Err(ConfigError::InvalidPort(port));
        } else {
            Ok(())
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServerConfig {
    pub fqdn: Option<String>,
    pub main_port: Option<Port>,
    pub pool_size: Option<u16>,
}
