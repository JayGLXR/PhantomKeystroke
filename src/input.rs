use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::io;
use std::time::Duration;
use std::hash::{Hash, Hasher};

/// Handler for keyboard input
pub struct KeyboardInput {}

impl KeyboardInput {
    /// Create a new keyboard input handler
    pub fn new() -> Self {
        KeyboardInput {}
    }
    
    /// Read a single key press
    pub async fn read_key(&self) -> Result<Key, io::Error> {
        // Enable raw mode for direct key capture
        crossterm::terminal::enable_raw_mode()?;
        
        let mut key = None;
        
        // Poll for an event with a timeout
        while key.is_none() {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    let key_code = key_event.code;
                    key = Some(Key::from(key_code));
                    
                    // Handle Ctrl+C
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) && key_event.code == KeyCode::Char('c') {
                        crossterm::terminal::disable_raw_mode()?;
                        return Err(io::Error::new(io::ErrorKind::Interrupted, "Interrupted"));
                    }
                }
            }
            
            // Allow other async tasks to run
            tokio::task::yield_now().await;
        }
        
        // Disable raw mode
        crossterm::terminal::disable_raw_mode()?;
        
        Ok(key.unwrap())
    }
}

/// A simplified key code for internal use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Tab,
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Function(u8),
    Other,
}

// Implement Hash trait for Key enum
impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use discriminant to hash the enum variant
        std::mem::discriminant(self).hash(state);
        
        // Hash the contained value for variants with data
        match self {
            Key::Char(c) => c.hash(state),
            Key::Function(n) => n.hash(state),
            _ => {}
        }
    }
}

impl From<KeyCode> for Key {
    fn from(key_code: KeyCode) -> Self {
        match key_code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Enter => Key::Enter,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Tab => Key::Tab,
            KeyCode::Esc => Key::Escape,
            KeyCode::Up => Key::ArrowUp,
            KeyCode::Down => Key::ArrowDown,
            KeyCode::Left => Key::ArrowLeft,
            KeyCode::Right => Key::ArrowRight,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::Delete => Key::Delete,
            KeyCode::F(n) => Key::Function(n),
            _ => Key::Other,
        }
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        match self {
            Key::Char(c) => c.to_string(),
            Key::Enter => "\n".to_string(),
            Key::Tab => "\t".to_string(),
            Key::Backspace => "[BACKSPACE]".to_string(),
            Key::Escape => "[ESC]".to_string(),
            Key::ArrowUp => "[UP]".to_string(),
            Key::ArrowDown => "[DOWN]".to_string(),
            Key::ArrowLeft => "[LEFT]".to_string(),
            Key::ArrowRight => "[RIGHT]".to_string(),
            Key::Home => "[HOME]".to_string(),
            Key::End => "[END]".to_string(),
            Key::PageUp => "[PGUP]".to_string(),
            Key::PageDown => "[PGDN]".to_string(),
            Key::Delete => "[DEL]".to_string(),
            Key::Function(n) => format!("[F{}]", n),
            Key::Other => "[KEY]".to_string(),
        }
    }
} 