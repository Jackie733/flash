use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub ip: String,
    pub username: String,
    pub password: Option<String>,
    pub port: Option<u16>,
    pub remote_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub servers: HashMap<String, ServerConfig>,
    pub default: Option<DefaultConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultConfig {
    pub server: Option<String>,
}

impl Config {
    pub fn load() -> Result<Option<Self>> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
        let config: Config =
            toml::from_str(&content).with_context(|| "Failed to parse config file")?;

        Ok(Some(config))
    }

    pub fn create_example() -> Result<()> {
        let config_path = Self::get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let example_config = r#"# Flash configuration file
# You can define multiple servers and choose between them

[servers.home]
name = "Home Server"
ip = "192.168.1.100"
username = "admin"
password = "123"
port = 22
remote_path = "/home/admin"

[servers.work]
name = "Work Server"
ip = "10.0.0.5"
username = "deploy"
port = 2222
remote_path = "/opt/uploads"

[default]
server = "home"  # Default server to use
"#;

        fs::write(&config_path, example_config).with_context(|| {
            format!("Failed to write example config file to: {:?}", config_path)
        })?;
        println!("Example configuration file created at: {:?}", config_path);
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let local_config = PathBuf::from("flash.toml");
        if local_config.exists() {
            return Ok(local_config);
        }

        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

        Ok(config_dir.join("flash").join("config.toml"))
    }

    pub fn get_server(&self, name: &str) -> Option<&ServerConfig> {
        self.servers.get(name)
    }

    pub fn get_default_server(&self) -> Option<&ServerConfig> {
        let default_name = self.default.as_ref()?.server.as_ref()?;
        self.servers.get(default_name)
    }

    pub fn list_servers(&self) -> Vec<(&String, &ServerConfig)> {
        self.servers.iter().collect()
    }
}
