use std::collections::HashMap;

pub struct KvStore {
    hashmap: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            hashmap: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: String, value: String) {
        self.hashmap.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        self.hashmap.get(&key).map(|val| val.to_string())
    }
    pub fn remove(&mut self, key: String) {
        self.hashmap.remove(&key);
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}
