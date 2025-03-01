use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

/// Handler for terminal output
pub struct OutputHandler {
    buffer: String,
    detailed_buffer: String,
    quiet_mode: bool,
}

impl OutputHandler {
    /// Create a new output handler
    pub fn new(quiet_mode: bool) -> Self {
        OutputHandler {
            buffer: String::new(),
            detailed_buffer: String::new(),
            quiet_mode,
        }
    }
    
    /// Clear the terminal screen
    pub fn clear_screen(&self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        stdout.flush()?;
        Ok(())
    }
    
    /// Display the input, output, and timestamp
    pub fn display(&mut self, input: &str, output: &str, timestamp: &str) -> Result<(), io::Error> {
        // In quiet mode, don't display individual keystrokes
        if !self.quiet_mode {
            let mut stdout = io::stdout();
            
            // Display the input/output pair with timestamp
            execute!(
                stdout,
                SetForegroundColor(Color::Green),
                Print(format!("[Input: {}] ", input)),
                SetForegroundColor(Color::Blue),
                Print(format!("-> [Output: {}] ", output)),
                SetForegroundColor(Color::Yellow),
                Print(format!("[Time: {}]\n", timestamp)),
                ResetColor
            )?;
            
            stdout.flush()?;
        }
        
        // Save to detailed buffer for API version display
        self.add_to_detailed_buffer(input, output, timestamp);
        
        // Store the output in buffer for plaintext summary
        if !input.starts_with('[') {  // Skip metadata inputs like [OPSEC WARNING]
            // Handle special cases for better formatting
            if input == "\n" {
                self.add_to_buffer("\n");
            } else if input == "[BACKSPACE]" {
                // Remove the last character from buffer if there is one
                if !self.buffer.is_empty() {
                    self.buffer.pop();
                }
            } else {
                // Add regular characters to buffer
                self.add_to_buffer(output);
            }
        }
        
        Ok(())
    }
    
    /// Add text to the buffer
    pub fn add_to_buffer(&mut self, text: &str) {
        self.buffer.push_str(text);
    }
    
    /// Add detailed entry with timestamps to the detailed buffer
    pub fn add_to_detailed_buffer(&mut self, input: &str, output: &str, timestamp: &str) {
        let entry = format!("[Input: {}] -> [Output: {}] [Time: {}]\n", 
                          input, output, timestamp);
        self.detailed_buffer.push_str(&entry);
    }
    
    /// Clear the buffer
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
        self.detailed_buffer.clear();
    }
    
    /// Get the current buffer content
    #[allow(dead_code)]
    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }
    
    /// Check if quiet mode is enabled
    pub fn is_quiet_mode(&self) -> bool {
        self.quiet_mode
    }
    
    /// Display the summary of all outputs as plain text
    #[allow(dead_code)]
    pub fn display_summary(&self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        
        // Clear screen first
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        
        // Display the plaintext result prominently
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("==================================================\n"),
            Print("                PLAINTEXT OUTPUT                   \n"),
            Print("==================================================\n\n"),
            SetForegroundColor(Color::White),
            Print(self.buffer.clone()),
            Print("\n\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("Press [Enter] to see detailed API version with timestamps...\n"),
            ResetColor
        )?;
        
        stdout.flush()?;
        
        // Wait for Enter to be pressed
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        Ok(())
    }
    
    /// Display the detailed API version with all timestamps
    pub fn display_detailed_summary(&self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        
        // Clear screen first
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        
        // Display the detailed API version
        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("==================================================\n"),
            Print("                DETAILED API VERSION               \n"),
            Print("==================================================\n\n"),
            SetForegroundColor(Color::White),
            Print(self.detailed_buffer.clone()),
            Print("\n\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("Session completed. Press Enter to exit...\n"),
            ResetColor
        )?;
        
        stdout.flush()?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_buffer_operations() {
        let mut handler = OutputHandler::new();
        assert_eq!(handler.get_buffer(), "");
        
        handler.add_to_buffer("Hello");
        assert_eq!(handler.get_buffer(), "Hello");
        
        handler.add_to_buffer(" World");
        assert_eq!(handler.get_buffer(), "Hello World");
        
        handler.clear_buffer();
        assert_eq!(handler.get_buffer(), "");
    }
}