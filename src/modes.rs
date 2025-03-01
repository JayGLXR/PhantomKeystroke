use crate::config::Config;
use crate::input::KeyboardInput;
use crate::obfuscation::{KeyMapper, LanguageTransformer, TimestampEmulator};
use crate::output::OutputHandler;
use crate::plugins::PluginManager;
use crate::command::{CommandPreprocessor, OpsecValidator, CommandHistoryManager, OpsecValidationResult};
use crate::persona::Persona;

use log::{error, info, warn};
use rand::{thread_rng, Rng};
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time;

/// Mode types for PhantomKeystroke
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModeType {
    /// Random Mode - Applies unpredictable obfuscation
    Random,
    
    /// Attribute Mode - Emulates region-specific patterns
    Attribute,
}

/// Operational mode for PhantomKeystroke
pub struct Mode {
    mode_type: ModeType,
    key_mapper: KeyMapper,
    language_transformer: LanguageTransformer,
    timestamp_emulator: TimestampEmulator,
    input_handler: KeyboardInput,
    output_handler: OutputHandler,
    plugin_manager: PluginManager,
    command_preprocessor: CommandPreprocessor,
    command_history: Arc<Mutex<CommandHistoryManager>>,
    opsec_validator: Option<OpsecValidator>,
    #[allow(dead_code)]
    persona: Option<Persona>,
}

impl Mode {
    /// Create a new operational mode
    pub async fn new(
        mode_type: ModeType, 
        mut config: Config, 
        plugin_manager: PluginManager,
        quiet_mode: bool,
        attribution_target: &str
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Configure Attribute Mode if selected and not already configured
        if mode_type == ModeType::Attribute && config.attribute.is_none() {
            config.configure_attribute().await?;
        }
        
        let key_mapper = match mode_type {
            ModeType::Random => KeyMapper::random(),
            ModeType::Attribute => {
                if let Some(ref attr_config) = config.attribute {
                    KeyMapper::for_country(&attr_config.country)
                } else {
                    error!("Attribute configuration missing");
                    return Err("Attribute configuration missing".into());
                }
            }
        };
        
        let language_transformer = match mode_type {
            ModeType::Random => {
                // In random mode, we use language_transformer.random() for the base
                // language, but set attribution to the specific target
                if attribution_target != "random" {
                    let random_transformer = LanguageTransformer::random();
                    LanguageTransformer::with_attribution(
                        &random_transformer.get_language(),
                        attribution_target
                    )
                } else {
                    LanguageTransformer::random()
                }
            },
            ModeType::Attribute => {
                if let Some(ref attr_config) = config.attribute {
                    // In attribute mode, use both the language and attribution from config
                    if attribution_target != "random" {
                        LanguageTransformer::with_attribution(
                            &attr_config.language,
                            attribution_target
                        )
                    } else {
                        LanguageTransformer::for_language(&attr_config.language)
                    }
                } else {
                    error!("Attribute configuration missing");
                    return Err("Attribute configuration missing".into());
                }
            }
        };
        
        let timestamp_emulator = match mode_type {
            ModeType::Random => TimestampEmulator::random(),
            ModeType::Attribute => {
                if let Some(ref attr_config) = config.attribute {
                    TimestampEmulator::for_timezone(&attr_config.timezone)
                } else {
                    error!("Attribute configuration missing");
                    return Err("Attribute configuration missing".into());
                }
            }
        };
        
        let input_handler = KeyboardInput::new();
        let output_handler = OutputHandler::new(quiet_mode);
        let command_preprocessor = CommandPreprocessor::new(
            &key_mapper,
            &language_transformer,
            &timestamp_emulator,
        );
        let command_history = Arc::new(Mutex::new(CommandHistoryManager::new(100))); // Store last 100 commands
        
        // Set up persona and OPSEC validator for Attribute mode
        let (persona, opsec_validator) = if mode_type == ModeType::Attribute {
            if let Some(ref attr_config) = config.attribute {
                let persona: Option<Persona> = Persona::by_country_code(&attr_config.country);
                
                let validator = if let Some(ref p) = persona {
                    // Create validator with advanced configuration from persona
                    Some(OpsecValidator::with_config(
                        p.get_timezone_offset(),
                        p.get_country_code(),
                        p.get_language_code(),
                        p.get_working_hours(),
                        p.get_weekend_days().to_vec(),
                        p.get_holidays().to_vec(),
                    ))
                } else {
                    // Basic validator with just the essential info
                    Some(OpsecValidator::new(
                        attr_config.timezone.parse::<i32>().unwrap_or(0),
                        &attr_config.country,
                        &attr_config.language,
                    ))
                };
                
                (persona, validator)
            } else {
                (None, None)
            }
        } else {
            (None, None) // No persona or validator for Random mode
        };
        
        Ok(Mode {
            mode_type,
            key_mapper,
            language_transformer,
            timestamp_emulator,
            input_handler,
            output_handler,
            plugin_manager,
            command_preprocessor,
            command_history,
            opsec_validator,
            persona,
        })
    }
    
    /// Run the selected mode and return the plaintext output
    pub async fn run(&mut self, running: Arc<Mutex<bool>>) -> Result<String, Box<dyn std::error::Error>> {
        println!("Press Enter to start...");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        info!("Starting {} mode with {} plugin", 
              match self.mode_type {
                  ModeType::Random => "Random",
                  ModeType::Attribute => "Attribute",
              },
              self.plugin_manager.plugin().name());
        
        // Clear the screen and prepare for input
        self.output_handler.clear_screen()?;
        self.output_handler.clear_buffer();
        
        // Track the current command being built from keystrokes
        let mut current_command = String::new();
        
        while *running.lock().await {
            // Process input
            match self.input_handler.read_key().await {
                Ok(key) => {
                    // Apply key mapping
                    let obfuscated_key = self.key_mapper.map_key(key);
                    
                    // Get timestamp according to the emulated timezone
                    let timestamp = self.timestamp_emulator.get_timestamp();
                    
                    // Get the string representation of the key
                    let input_text = key.to_string();
                    
                    // Add to current command if it's a regular character
                    if input_text.len() == 1 && !input_text.contains('[') {
                        current_command.push_str(&input_text);
                    } else if input_text == "\n" {
                        // Process the complete command on Enter key
                        if !current_command.trim().is_empty() {
                            self.process_command(&current_command, &timestamp).await?;
                            current_command.clear();
                        }
                    } else if input_text == "[BACKSPACE]" && !current_command.is_empty() {
                        // Handle backspace
                        current_command.pop();
                    }
                    
                    // Get output text for display
                    let output_text = if input_text.trim().contains(' ') {
                        self.language_transformer.transform(&input_text)
                    } else {
                        obfuscated_key.to_string()
                    };
                    
                    // Realistic delay to simulate expert programmer/hacker typing
                    if self.mode_type == ModeType::Random {
                        // Expert typing speeds: 50-150ms base with occasional longer pauses
                        let mut delay_ms = thread_rng().gen_range(30..120);
                        
                        // Add realistic jitter: occasional brief pauses (10% chance)
                        if thread_rng().gen_ratio(1, 10) {
                            delay_ms = thread_rng().gen_range(150..350);
                        }
                        
                        // Very rarely (1% chance) simulate a short thinking pause
                        if thread_rng().gen_ratio(1, 100) {
                            delay_ms = thread_rng().gen_range(500..1000);
                        }
                        
                        time::sleep(Duration::from_millis(delay_ms)).await;
                    }
                    
                    // Output the transformed data
                    (&mut self.output_handler).display(&input_text, &output_text, &timestamp)?;
                }
                Err(e) => {
                    error!("Error reading input: {}", e);
                    break;
                }
            }
            
            // Check for data from the plugin
            if self.plugin_manager.plugin().name() != "null_plugin" {
                match self.plugin_manager.plugin().receive().await {
                    Ok(data) if !data.is_empty() => {
                        // Process received data
                        if let Ok(text) = String::from_utf8(data.clone()) {
                            info!("Received data from C2: {}", text);
                            
                            // Transform and display received data
                            let transformed = self.language_transformer.transform(&text);
                            let timestamp = self.timestamp_emulator.get_timestamp();
                            
                            (&mut self.output_handler).display("[C2]", &transformed, &timestamp)?;
                        } else {
                            // Handle binary data
                            info!("Received binary data from C2: {} bytes", data.len());
                        }
                    }
                    Err(e) => {
                        // Ignore common errors
                        if !e.to_string().contains("not fully implemented") {
                            error!("Error receiving data from C2: {}", e);
                        }
                    }
                    _ => {} // No data received
                }
            }
        }
        
        // Get the plaintext output
        let plaintext_output = self.output_handler.get_buffer().to_string();
        
        // Display detailed API version if not in quiet mode
        if !self.output_handler.is_quiet_mode() {
            self.output_handler.display_detailed_summary()?;
        }
        
        Ok(plaintext_output)
    }
    
    /// Process a complete command
    async fn process_command(&mut self, input: &str, timestamp: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Process the command through the CommandPreprocessor
        let command = self.command_preprocessor.process(input);
        
        // Log the command for debugging
        info!("[Command] Original: '{}' -> Transformed: '{}'", command.original, command.transformed);
        
        // Validate the command against OPSEC rules if in Attribute mode
        if let Some(validator) = &self.opsec_validator {
            match validator.validate(&command) {
                OpsecValidationResult::Valid => {
                    // Command is valid, proceed normally
                    info!("OPSEC validation passed for command: {}", input);
                },
                OpsecValidationResult::Warning(message) => {
                    // Display warning but allow the command
                    warn!("OPSEC warning: {}", message);
                    (&mut self.output_handler).display(
                        "[OPSEC WARNING]", 
                        &message, 
                        timestamp
                    )?;
                },
                OpsecValidationResult::Violation(message) => {
                    // Display error and block the command
                    error!("OPSEC violation: {}", message);
                    (&mut self.output_handler).display(
                        "[OPSEC VIOLATION]", 
                        &message, 
                        timestamp
                    )?;
                    return Ok(());
                }
            }
        }
        
        // Add to command history with cloned command
        let mut history_guard = self.command_history.lock().await;
        history_guard.add(command.clone());
        
        // If using a plugin other than null, send the transformed command with metadata
        if self.plugin_manager.plugin().name() != "null_plugin" {
            // Convert the string to bytes and send via the plugin with metadata
            let data = command.transformed.as_bytes();
            if let Err(e) = self.plugin_manager.plugin().send_with_metadata(data, &command.metadata).await {
                error!("Error sending data through plugin: {}", e);
                
                // Fall back to regular send if send_with_metadata fails
                if let Err(e) = self.plugin_manager.plugin().send(data).await {
                    error!("Error sending data through plugin (fallback): {}", e);
                }
            }
        }
        
        Ok(())
    }
} 