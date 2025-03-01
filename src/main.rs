// Make sure our module declarations match the lib.rs file
mod config;
mod input;
mod output;
mod obfuscation;
mod logging;
mod cleanup;
mod modes;
mod plugins;
mod command;
mod persona;

use crate::config::Config;
use crate::modes::{Mode, ModeType};
use crate::plugins::{PluginManager, PluginType};

use clap::Parser;
use log::{error, info, LevelFilter};
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// PhantomKeystroke - An open-source, pure Rust application for keyboard input obfuscation
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Configuration file path
    #[clap(short, long)]
    config: Option<PathBuf>,

    /// Run in random mode
    #[clap(short, long)]
    random: bool,

    /// Run in attribute mode
    #[clap(short = 't', long)]
    attribute: bool,
    
    /// Plugin to use (null, cobaltstrike, sliver, mythic, custom)
    #[clap(short, long)]
    plugin: Option<String>,

    /// Log to file instead of stdout
    #[clap(short, long)]
    log_file: bool,
    
    /// Run in verbose mode (shows individual keystrokes with timing)
    #[clap(short, long)]
    verbose: bool,
    
    /// Target attribution country code
    #[clap(short, long, long_help = "Target country for attribution fingerprinting:
    ru - Russian
    zh - Chinese
    ko - Korean
    de - German
    fr - French
    ar - Arabic
    fa - Persian (Iran)
    random - Random attribution (default: Russian)")]
    attribution: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to PhantomKeystroke!");
    
    let args = Args::parse();
    
    // Set up cleanup on Ctrl+C
    let running = Arc::new(Mutex::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut running = r.lock().await;
            *running = false;
            
            println!("\nInterrupted. Clean up now? [y/n]: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            if input.trim().to_lowercase() == "y" {
                println!("Cleaning up...");
                let _ = cleanup::perform_cleanup().await;
                println!("Session ended unexpectedly");
            } else {
                println!("Exiting without cleanup");
            }
            
            std::process::exit(0);
        });
    })
    .expect("Error setting Ctrl-C handler");
    
    // Initialize logging
    let log_to_file = if args.log_file {
        true
    } else if !args.random && !args.attribute {
        println!("Log to stdout? [y/n]: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        input.trim().to_lowercase() != "y"
    } else {
        false
    };
    
    if log_to_file {
        let log_path = "/tmp/phantomkeystroke.log";
        WriteLogger::init(
            LevelFilter::Info,
            ConfigBuilder::new().build(),
            File::create(log_path)?,
        )?;
        println!("Logging to {}", log_path);
    } else {
        TermLogger::init(
            LevelFilter::Info,
            ConfigBuilder::new().build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )?;
    }
    
    info!("PhantomKeystroke started");
    
    // Load configuration
    let mut config = if let Some(config_path) = args.config.clone() {
        Config::from_file(config_path)?
    } else {
        // Check for default config.toml in current directory
        let default_path = PathBuf::from("config.toml");
        if default_path.exists() {
            Config::from_file(default_path)?
        } else {
            Config::default()
        }
    };
    
    // Determine mode
    let mode_type = if args.random {
        ModeType::Random
    } else if args.attribute {
        ModeType::Attribute
    } else if let Some(mode_type) = config.mode.r#type.try_into().ok().and_then(|t: u8| match t {
        1 => Some(ModeType::Random),
        2 => Some(ModeType::Attribute),
        _ => None,
    }) {
        mode_type
    } else {
        // Interactive mode selection
        println!("Select mode:");
        println!("1: Random Mode (unpredictable obfuscation)");
        println!("2: Attribute Mode (region-specific emulation)");
        
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => ModeType::Random,
            "2" => ModeType::Attribute,
            _ => {
                error!("Invalid mode selection, defaulting to Random Mode");
                ModeType::Random
            }
        }
    };
    
    // Determine plugin
    let plugin_type = if let Some(plugin_name) = &args.plugin {
        match PluginType::from_str(plugin_name) {
            Some(pt) => pt,
            None => {
                error!("Invalid plugin name: {}, defaulting to null plugin", plugin_name);
                PluginType::Null
            }
        }
    } else if let Some(plugin_config) = &config.plugin {
        match PluginType::from_str(&plugin_config.name) {
            Some(pt) => pt,
            None => {
                error!("Invalid plugin name in config: {}, defaulting to null plugin", plugin_config.name);
                PluginType::Null
            }
        }
    } else {
        // Interactive plugin selection
        println!("Select plugin:");
        println!("1: Null Plugin (standalone terminal mode)");
        println!("2: Cobalt Strike Plugin");
        println!("3: Sliver Plugin");
        println!("4: Mythic Plugin");
        println!("5: Custom Plugin");
        
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => PluginType::Null,
            "2" => PluginType::CobaltStrike,
            "3" => PluginType::Sliver,
            "4" => PluginType::Mythic,
            "5" => PluginType::Custom,
            _ => {
                error!("Invalid plugin selection, defaulting to Null Plugin");
                PluginType::Null
            }
        }
    };
    
    // Configure the plugin if needed
    let plugin_config = if plugin_type != PluginType::Null && config.plugin.is_none() {
        Some(config.configure_plugin(plugin_type).await?)
    } else {
        config.plugin.clone()
    };
    
    // Initialize plugin
    let plugin_manager = PluginManager::new(plugin_type, plugin_config).await?;
    
    info!("Using plugin: {}", plugin_manager.plugin().name());
    
    // Determine attribution target
    let attribution_target = args.attribution.as_deref().unwrap_or("ru"); // Default to Russian
    
    // Display information about the session
    println!("Mode: {}", match mode_type {
        ModeType::Random => "Random",
        ModeType::Attribute => "Attribute",
    });
    println!("Plugin: {}", plugin_manager.plugin().name());
    println!("Attribution: {}", match attribution_target {
        "ru" => "Russian",
        "zh" => "Chinese",
        "ko" => "Korean",
        "de" => "German",
        "fr" => "French",
        "ar" => "Arabic",
        "fa" => "Persian",
        "random" => "Random",
        _ => attribution_target,
    });
    println!("Display: {}", if args.verbose { "Verbose (with keystrokes)" } else { "Quiet (output only)" });
    println!("---");
    
    // Initialize selected mode (quiet mode is now default, verbose is opt-in)
    let mut mode = Mode::new(mode_type, config, plugin_manager, !args.verbose, attribution_target).await?;
    
    // Run the application
    let output_summary = mode.run(running).await?;
    
    // Display the plaintext output summary
    println!("\n==== PLAINTEXT OUTPUT ====");
    println!("{}", output_summary);
    println!("==========================\n");
    
    // Give the user a moment to see the results before exiting
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(())
}
