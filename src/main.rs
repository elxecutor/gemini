mod app;
mod config;
mod demo;
mod gemini;
mod ui;

use anyhow::Result;
use clap::Parser;
use config::Config;

#[derive(Parser)]
#[command(name = "gemini-chat-tui")]
#[command(about = "A crazy awesome TUI for chatting with Google's Gemini AI")]
struct Cli {
    /// Set the API key (will be saved for future use)
    #[arg(long)]
    api_key: Option<String>,
    
    /// Reset configuration (will prompt for API key again)
    #[arg(long)]
    reset_config: bool,
    
    /// Run in demo mode (shows UI without API key)
    #[arg(long)]
    demo: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Run demo mode if requested
    if cli.demo {
        println!("Running in demo mode - showing UI with sample messages");
        println!("Press any key to exit demo mode");
        std::thread::sleep(std::time::Duration::from_secs(2));
        return demo::run_demo();
    }
    
    let mut config = if cli.reset_config {
        Config { api_key: String::new() }
    } else {
        Config::load().unwrap_or_else(|_| Config { api_key: String::new() })
    };
    
    // Handle API key setup
    if let Some(api_key) = cli.api_key {
        config.set_api_key(api_key)?;
    } else if config.api_key.is_empty() {
        let api_key = config::prompt_for_api_key()?;
        config.set_api_key(api_key)?;
    }
    
    // Start the TUI application
    let mut app = app::App::new(config.api_key);
    app.run().await?;
    
    Ok(())
}
