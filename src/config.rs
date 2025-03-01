use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::{self, Read, Write};
use crate::plugins::{PluginConfig, PluginType};
#[cfg(test)]
use crate::modes::ModeType;

/// Configuration for PhantomKeystroke
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Mode configuration
    #[serde(default)]
    pub mode: ModeConfig,
    
    /// Attribute mode configuration
    pub attribute: Option<AttributeConfig>,
    
    /// Plugin configuration
    #[serde(default)]
    pub plugin: Option<PluginConfig>,
}

/// Mode configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModeConfig {
    /// Mode type (1: Random, 2: Attribute)
    #[serde(default = "default_mode_type")]
    pub r#type: u8,
}

fn default_mode_type() -> u8 {
    1 // Default to Random mode
}

impl Default for ModeConfig {
    fn default() -> Self {
        ModeConfig {
            r#type: default_mode_type(),
        }
    }
}

/// Configuration for Attribute Mode
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttributeConfig {
    /// Country code (e.g., "DE" for Germany)
    pub country: String,
    
    /// Language code (e.g., "de" for German)
    pub language: String,
    
    /// Timezone offset (e.g., "+1" for CET)
    pub timezone: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mode: ModeConfig::default(),
            attribute: None,
            plugin: None,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
    
    /// Interactively configure attribute mode
    pub async fn configure_attribute(&mut self) -> Result<(), io::Error> {
        println!("Enter country code (DE, FR, RU, JP): ");
        io::stdout().flush()?;
        
        let mut country = String::new();
        io::stdin().read_line(&mut country)?;
        let country = country.trim().to_uppercase();
        
        println!("Enter language code (de, fr, ru, ja): ");
        io::stdout().flush()?;
        
        let mut language = String::new();
        io::stdin().read_line(&mut language)?;
        let language = language.trim().to_lowercase();
        
        println!("Enter timezone offset (e.g., +1): ");
        io::stdout().flush()?;
        
        let mut timezone = String::new();
        io::stdin().read_line(&mut timezone)?;
        let timezone = timezone.trim().to_string();
        
        self.attribute = Some(AttributeConfig {
            country,
            language,
            timezone,
        });
        
        Ok(())
    }
    
    /// Get the mode type from the configuration
    #[cfg(test)]
    pub fn get_mode_type(&self) -> ModeType {
        match self.mode.r#type {
            1 => ModeType::Random,
            2 => ModeType::Attribute,
            _ => {
                ModeType::Random
            }
        }
    }
    
    /// Configure a plugin
    pub async fn configure_plugin(&mut self, plugin_type: PluginType) -> Result<PluginConfig, io::Error> {
        let mut config = PluginConfig {
            name: plugin_type.as_str().to_string(),
            parameters: std::collections::HashMap::new(),
        };
        
        match plugin_type {
            PluginType::CobaltStrike => {
                println!("Enter Cobalt Strike endpoint URL (default: http://localhost:50050): ");
                io::stdout().flush()?;
                
                let mut endpoint = String::new();
                io::stdin().read_line(&mut endpoint)?;
                let endpoint = endpoint.trim();
                
                if !endpoint.is_empty() {
                    config.parameters.insert("endpoint".to_string(), endpoint.to_string());
                }
            },
            PluginType::Sliver => {
                println!("Enter Sliver server address (default: localhost:31337): ");
                io::stdout().flush()?;
                
                let mut address = String::new();
                io::stdin().read_line(&mut address)?;
                let address = address.trim();
                
                if !address.is_empty() {
                    config.parameters.insert("address".to_string(), address.to_string());
                }
            },
            PluginType::Mythic => {
                println!("Enter Mythic server URL (default: http://localhost:7443): ");
                io::stdout().flush()?;
                
                let mut url = String::new();
                io::stdin().read_line(&mut url)?;
                let url = url.trim();
                
                if !url.is_empty() {
                    config.parameters.insert("url".to_string(), url.to_string());
                }
            },
            PluginType::Custom => {
                println!("Enter custom plugin path: ");
                io::stdout().flush()?;
                
                let mut path = String::new();
                io::stdin().read_line(&mut path)?;
                let path = path.trim();
                
                if !path.is_empty() {
                    config.parameters.insert("path".to_string(), path.to_string());
                }
            },
            _ => {} // No configuration needed for null plugin
        }
        
        self.plugin = Some(config.clone());
        
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.attribute.is_none());
        assert_eq!(config.mode.r#type, 1);
    }
} 