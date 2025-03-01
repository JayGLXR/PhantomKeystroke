use std::collections::HashMap;

/// Represents a complete attribution persona
pub struct Persona {
    /// Country code (e.g., "RU", "CN", "IR")
    pub country_code: String,
    
    /// Language code (e.g., "ru", "zh", "fa") 
    pub language_code: String,
    
    /// Timezone offset from UTC
    pub timezone_offset: i32,
    
    /// Common command patterns
    #[allow(dead_code)]
    pub command_patterns: HashMap<String, String>,
    
    /// Working hours (start hour, end hour)
    pub working_hours: (u8, u8),
    
    /// Weekend days (0 = Monday, 6 = Sunday) 0-indexed
    pub weekend_days: Vec<u8>,
    
    /// Holiday dates specific to the persona (month, day)
    pub holidays: Vec<(u8, u8)>,
}

impl Persona {
    /// Create a Russian APT persona
    pub fn russian_apt() -> Self {
        let mut command_patterns = HashMap::new();
        command_patterns.insert("dir".to_string(), "ls".to_string());
        command_patterns.insert("ipconfig".to_string(), "ifconfig".to_string());
        command_patterns.insert("color".to_string(), "colour".to_string());
        
        Persona {
            country_code: "RU".to_string(),
            language_code: "ru".to_string(),
            timezone_offset: 3,
            command_patterns,
            working_hours: (9, 18),
            weekend_days: vec![5, 6], // Saturday, Sunday
            holidays: vec![
                (1, 1),  // New Year
                (1, 2),  // New Year holiday
                (1, 7),  // Orthodox Christmas 
                (2, 23), // Defender of the Fatherland
                (3, 8),  // International Women's Day
                (5, 1),  // Spring and Labor Day
                (5, 9),  // Victory Day
                (6, 12), // Russia Day
                (11, 4), // Unity Day
            ],
        }
    }
    
    /// Create an Iranian hacker persona
    pub fn iranian_apt() -> Self {
        let mut command_patterns = HashMap::new();
        command_patterns.insert("dir".to_string(), "ls".to_string());
        command_patterns.insert("type".to_string(), "cat".to_string());
        
        Persona {
            country_code: "IR".to_string(),
            language_code: "fa".to_string(),
            timezone_offset: 3,
            command_patterns,
            working_hours: (8, 16),
            weekend_days: vec![4, 5], // Friday, Saturday (Iranian weekend)
            holidays: vec![
                (1, 1),   // Nowruz (Persian New Year)
                (1, 2),   // Nowruz holiday
                (1, 3),   // Nowruz holiday
                (1, 4),   // Nowruz holiday
                (1, 13),  // Nature Day
                (2, 11),  // Islamic Revolution Victory Day
                (3, 21),  // Oil Nationalization Day
                (6, 5),   // Khomeini Death Anniversary
                (11, 22), // Islamic Republic Day
            ],
        }
    }
    
    /// Create a Chinese APT persona
    pub fn chinese_apt() -> Self {
        let mut command_patterns = HashMap::new();
        command_patterns.insert("dir".to_string(), "ls".to_string());
        command_patterns.insert("type".to_string(), "cat".to_string());
        
        Persona {
            country_code: "CN".to_string(),
            language_code: "zh".to_string(),
            timezone_offset: 8,
            command_patterns,
            working_hours: (9, 18),
            weekend_days: vec![5, 6], // Saturday, Sunday
            holidays: vec![
                (1, 1),   // New Year
                (2, 1),   // Spring Festival (approximate, varies by lunar calendar)
                (2, 2),   // Spring Festival holiday
                (2, 3),   // Spring Festival holiday
                (5, 1),   // Labor Day
                (10, 1),  // National Day
                (10, 2),  // National Day holiday
                (10, 3),  // National Day holiday
            ],
        }
    }
    
    /// Create a North Korean APT persona
    pub fn north_korean_apt() -> Self {
        let mut command_patterns = HashMap::new();
        command_patterns.insert("dir".to_string(), "ls".to_string());
        command_patterns.insert("type".to_string(), "cat".to_string());
        
        Persona {
            country_code: "KP".to_string(),
            language_code: "ko".to_string(),
            timezone_offset: 9,
            command_patterns,
            working_hours: (8, 17),
            weekend_days: vec![6], // Sunday only
            holidays: vec![
                (1, 1),    // New Year
                (2, 16),   // Kim Jong Il Birthday
                (4, 15),   // Kim Il Sung Birthday
                (7, 27),   // Victory Day
                (9, 9),    // Day of the Foundation of the Republic
                (10, 10),  // Party Foundation Day
                (12, 17),  // Kim Jong Il Death Anniversary
            ],
        }
    }
    
    /// Create a generic Western (US/UK) persona for comparison
    pub fn western_apt() -> Self {
        let mut command_patterns = HashMap::new();
        command_patterns.insert("ls".to_string(), "dir".to_string());
        command_patterns.insert("cat".to_string(), "type".to_string());
        
        Persona {
            country_code: "US".to_string(),
            language_code: "en".to_string(),
            timezone_offset: -5, // Eastern Time
            command_patterns,
            working_hours: (9, 17),
            weekend_days: vec![5, 6], // Saturday, Sunday
            holidays: vec![
                (1, 1),   // New Year's Day
                (5, 25),  // Memorial Day (approximate)
                (7, 4),   // Independence Day
                (9, 7),   // Labor Day (approximate)
                (11, 11), // Veterans Day
                (11, 26), // Thanksgiving (approximate)
                (12, 25), // Christmas
            ],
        }
    }
    
    /// Get a persona by country code
    pub fn by_country_code(country_code: &str) -> Option<Self> {
        match country_code.to_uppercase().as_str() {
            "RU" => Some(Self::russian_apt()),
            "IR" => Some(Self::iranian_apt()),
            "CN" => Some(Self::chinese_apt()),
            "KP" => Some(Self::north_korean_apt()),
            "US" => Some(Self::western_apt()),
            _ => None,
        }
    }
    
    /// Get timezone offset
    pub fn get_timezone_offset(&self) -> i32 {
        self.timezone_offset
    }
    
    /// Get country code
    pub fn get_country_code(&self) -> &str {
        &self.country_code
    }
    
    /// Get language code
    pub fn get_language_code(&self) -> &str {
        &self.language_code
    }
    
    /// Get working hours
    pub fn get_working_hours(&self) -> (u8, u8) {
        self.working_hours
    }
    
    /// Get weekend days
    pub fn get_weekend_days(&self) -> &[u8] {
        &self.weekend_days
    }
    
    /// Get holidays
    pub fn get_holidays(&self) -> &[(u8, u8)] {
        &self.holidays
    }
    
    /// Get command patterns
    #[allow(dead_code)]
    pub fn get_command_patterns(&self) -> &HashMap<String, String> {
        &self.command_patterns
    }
}