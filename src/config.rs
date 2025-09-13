use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
            
            let config: Config = serde_json::from_str(&content)
                .with_context(|| "Failed to parse config file")?;
            
            Ok(config)
        } else {
            // Create default config
            let config = Config {
                api_key: String::new(),
            };
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize config")?;
        
        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;
        
        Ok(())
    }

    pub fn set_api_key(&mut self, api_key: String) -> Result<()> {
        self.api_key = api_key;
        self.save()
    }
}

fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
        .context("Unable to determine config directory")?;
    
    Ok(config_dir.join("gemini-chat-tui").join("config.json"))
}

pub fn prompt_for_api_key() -> Result<String> {
    println!("🚀 Welcome to Gemini Chat TUI!");
    println!();
    println!("To get started, you need a Gemini API key:");
    println!("1. Go to https://aistudio.google.com/app/apikey");
    println!("2. Create a new API key");
    println!("3. Paste it below");
    println!();
    print!("Enter your Gemini API key: ");
    
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let api_key = input.trim().to_string();
    
    if api_key.is_empty() {
        anyhow::bail!("API key cannot be empty");
    }
    
    println!("✅ API key saved! Starting the chat...");
    println!();
    
    Ok(api_key)
}