use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};

// Define the config struct
struct Config {
    settings: HashMap<String, String>,
    access_count: AtomicU32, // Track number of times config is accessed
}

impl Config {
    /// Method to get a setting by key
    fn get(&self, key: &str) -> Option<&String> {
        self.access_count.fetch_add(1, Ordering::SeqCst);
        // Strongest memory ordering (sequanrtial consistency)
        self.settings.get(key)
    }

    /// Get the counter
    fn get_access_count(&self) -> u32 {
        self.access_count.load(Ordering::SeqCst)
    }
}

/// Create a lazily-initialized static instance of Config
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    // Simulate loading settings from a file or environment
    let mut settings = HashMap::new();
    settings.insert("theme".to_string(), "dark".to_string());

    Config {
        settings,
        access_count: AtomicU32::new(0),
    }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        // Access the singleton instance
        if let Some(theme) = CONFIG.get("theme") {
            assert_eq!(theme, "dark");
        }
    }
}
