use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Cache {
    text_cache: Arc<DashMap<String, String>>,
    binary_cache: Arc<DashMap<String, Vec<u8>>>,
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            text_cache: Arc::new(DashMap::new()),
            binary_cache: Arc::new(DashMap::new()),
        }
    }

    pub fn get_text(&self, key: &str) -> Option<String> {
        self.text_cache.get(key).map(|v| v.value().clone())
    }

    pub fn set_text(&self, key: String, value: String) {
        self.text_cache.insert(key, value);
    }

    pub fn get_binary(&self, key: &str) -> Option<Vec<u8>> {
        self.binary_cache.get(key).map(|v| v.value().clone())
    }

    pub fn set_binary(&self, key: String, value: Vec<u8>) {
        self.binary_cache.insert(key, value);
    }
}
