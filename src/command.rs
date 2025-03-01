use chrono::{Local, Timelike, Datelike};
use log::debug;

use crate::obfuscation::{KeyMapper, LanguageTransformer, TimestampEmulator};
use crate::plugins::TransformationMetadata;

/// Represents a command with its associated metadata
#[derive(Clone)]
pub struct Command {
    /// Original command input by the operator
    pub original: String,
    
    /// Transformed command to be sent to C2
    pub transformed: String,
    
    /// Metadata used for the transformation
    pub metadata: TransformationMetadata,
}

/// Processes commands before they are sent to the C2 framework
pub struct CommandPreprocessor {
    key_mapper: KeyMapper,
    language_transformer: LanguageTransformer,
    timestamp_emulator: TimestampEmulator,
}

impl CommandPreprocessor {
    /// Create a new command preprocessor
    pub fn new(
        key_mapper: &KeyMapper,
        language_transformer: &LanguageTransformer,
        timestamp_emulator: &TimestampEmulator,
    ) -> Self {
        CommandPreprocessor {
            key_mapper: key_mapper.clone(),
            language_transformer: language_transformer.clone(),
            timestamp_emulator: timestamp_emulator.clone(),
        }
    }
    
    /// Process a command and attach transformation metadata
    pub fn process(&self, input: &str) -> Command {
        // Transform the command
        let transformed = self.language_transformer.transform(input);
        
        // Create metadata
        let metadata = TransformationMetadata::new(
            &self.key_mapper,
            &self.language_transformer,
            &self.timestamp_emulator,
        );
        
        Command {
            original: input.to_string(),
            transformed,
            metadata,
        }
    }
}

/// OPSEC validation result
#[derive(Debug, PartialEq)]
pub enum OpsecValidationResult {
    Valid,
    Warning(String),
    #[allow(dead_code)]
    Violation(String),
}

/// Validates commands against OPSEC rules
pub struct OpsecValidator {
    timezone_offset: i32,
    country_code: String,
    language_code: String,
    working_hours: (u8, u8),
    weekend_days: Vec<u8>,
    holidays: Vec<(u8, u8)>, // Month, day
}

impl OpsecValidator {
    /// Create a new OPSEC validator with basic configuration
    pub fn new(
        timezone_offset: i32,
        country_code: &str,
        language_code: &str,
    ) -> Self {
        OpsecValidator {
            timezone_offset,
            country_code: country_code.to_string(),
            language_code: language_code.to_string(),
            working_hours: (9, 17),
            weekend_days: vec![5, 6], // Saturday, Sunday by default
            holidays: Vec::new(),
        }
    }
    
    /// Create a new OPSEC validator with advanced configuration
    pub fn with_config(
        timezone_offset: i32,
        country_code: &str,
        language_code: &str,
        working_hours: (u8, u8),
        weekend_days: Vec<u8>,
        holidays: Vec<(u8, u8)>,
    ) -> Self {
        OpsecValidator {
            timezone_offset,
            country_code: country_code.to_string(),
            language_code: language_code.to_string(),
            working_hours,
            weekend_days,
            holidays,
        }
    }
    
    /// Validate a command against OPSEC rules
    pub fn validate(&self, command: &Command) -> OpsecValidationResult {
        // Check for timezone violations
        let now = Local::now();
        let local_hour = now.hour() as i32;
        let target_hour = (local_hour + self.timezone_offset).rem_euclid(24) as u8;
        
        // Check if operating outside business hours in target timezone
        if target_hour < self.working_hours.0 || target_hour > self.working_hours.1 {
            return OpsecValidationResult::Warning(
                format!("Operating outside business hours ({}:00) in target timezone", target_hour)
            );
        }
        
        // Check if operating on a weekend in target timezone
        let weekday = (now.weekday().number_from_monday() - 1) as u8; // 0-indexed weekday
        if self.weekend_days.contains(&weekday) {
            return OpsecValidationResult::Warning(
                format!("Operating on a weekend in target timezone")
            );
        }
        
        // Check if operating on a holiday in target timezone
        let month = now.month() as u8;
        let day = now.day() as u8;
        if self.holidays.contains(&(month, day)) {
            return OpsecValidationResult::Warning(
                format!("Operating on a holiday ({}/{}) in target timezone", month, day)
            );
        }
        
        // Language-specific checks
        self.language_specific_checks(command)
    }
    
    /// Perform language-specific validation checks
    fn language_specific_checks(&self, command: &Command) -> OpsecValidationResult {
        let cmd = command.original.to_lowercase();
        
        // Check for language inconsistencies
        match self.language_code.as_str() {
            "ru" => {
                // Check for American English spelling
                if cmd.contains("color") {
                    return OpsecValidationResult::Warning(
                        "Using American English spelling 'color' in Russian attribution profile (should be 'colour')".to_string()
                    );
                }
            },
            "de" => {
                // Check for improper path separators
                if cmd.contains("\\") {
                    return OpsecValidationResult::Warning(
                        "Using Windows path separators '\\' in German attribution profile".to_string()
                    );
                }
            },
            "fa" | "ar" => {
                // Check for left-to-right markers
                if cmd.contains("LRM") || cmd.contains("LEFT-TO-RIGHT") {
                    return OpsecValidationResult::Warning(
                        "Using LTR markers in RTL language attribution profile".to_string()
                    );
                }
            },
            "zh" => {
                // Check for Japanese characters in Chinese context
                if cmd.contains("の") || cmd.contains("は") {
                    return OpsecValidationResult::Warning(
                        "Using Japanese characters in Chinese attribution profile".to_string()
                    );
                }
            },
            _ => {}
        }
        
        // Check for US-centric directory names
        if self.country_code != "US" && (
            cmd.contains("/users/") || 
            cmd.contains("/desktop/") ||
            cmd.contains("/documents/")
        ) {
            return OpsecValidationResult::Warning(
                "Using English directory names in non-US attribution profile".to_string()
            );
        }
        
        OpsecValidationResult::Valid
    }
}

/// Manages command history
pub struct CommandHistoryManager {
    history: Vec<Command>,
    max_entries: usize,
}

impl CommandHistoryManager {
    /// Create a new command history manager
    pub fn new(max_entries: usize) -> Self {
        CommandHistoryManager {
            history: Vec::new(),
            max_entries,
        }
    }
    
    /// Add a command to history
    pub fn add(&mut self, command: Command) {
        self.history.push(command);
        
        // Trim history if it exceeds max entries
        if self.history.len() > self.max_entries {
            self.history.remove(0);
        }
    }
    
    /// Get command history (only transformed commands)
    #[allow(dead_code)]
    pub fn get_history(&self) -> Vec<String> {
        self.history.iter()
            .map(|cmd| cmd.transformed.clone())
            .collect()
    }
    
    /// Get command history with original commands
    #[allow(dead_code)]
    pub fn get_history_with_original(&self) -> Vec<(String, String)> {
        self.history.iter()
            .map(|cmd| (cmd.original.clone(), cmd.transformed.clone()))
            .collect()
    }
    
    /// Clear history
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.history.clear();
        debug!("Command history cleared");
    }
}