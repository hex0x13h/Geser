use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Cache {
    text_cache: Arc<DashMap<String, String>>,
    binary_cache: Arc<DashMap<String, Vec<u8>>>,
}

impl Cache {
    // Creates a new Cache instance
    pub fn new() -> Self {
        Cache {
            text_cache: Arc::new(DashMap::new()),
            binary_cache: Arc::new(DashMap::new()),
        }
    }

    // Gets a cached text value by key
    pub fn get_text(&self, key: &str) -> Option<String> {
        self.text_cache.get(key).map(|v| v.value().clone())
    }

    // Sets a text value in the cache with a specified key
    pub fn set_text(&self, key: String, value: String) {
        self.text_cache.insert(key, value);
    }

    // Gets a cached binary value by key
    pub fn get_binary(&self, key: &str) -> Option<Vec<u8>> {
        self.binary_cache.get(key).map(|v| v.value().clone())
    }

    // Sets a binary value in the cache with a specified key
    pub fn set_binary(&self, key: String, value: Vec<u8>) {
        self.binary_cache.insert(key, value);
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;  // Import Cache struct from outer scope

    // Test text cache functionality
    #[test]
    fn test_text_cache() {
        let cache = Cache::new();
        cache.set_text("key1".to_string(), "value1".to_string());

        // Check if the cached value is correct
        assert_eq!(cache.get_text("key1"), Some("value1".to_string()));
        assert_eq!(cache.get_text("key2"), None); // Key "key2" doesn't exist
    }

    // Test binary cache functionality
    #[test]
    fn test_binary_cache() {
        let cache = Cache::new();
        cache.set_binary("key1".to_string(), vec![1, 2, 3, 4]);

        // Check if the cached binary data is correct
        assert_eq!(cache.get_binary("key1"), Some(vec![1, 2, 3, 4]));
        assert_eq!(cache.get_binary("key2"), None); // Key "key2" doesn't exist
    }
}
