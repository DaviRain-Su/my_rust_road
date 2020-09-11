use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Debug)]
pub struct KvStore {
    kv_item: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            kv_item: HashMap::new(),
        }
    }

    /// Set the value of a string key to a string
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, _key: String, _value: String) {
        if self.kv_item.contains_key(&_key) {
            *self.kv_item.get_mut(&_key).unwrap() = _value;
        } else {
            self.kv_item.insert(_key, _value);
        }
    }

    /// Get the string value of the a string key. If the key does not exist, return None.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, _key: String) -> Option<String> {
        self.kv_item.get(&_key).cloned()
    }

    /// Remove a given key.
    pub fn remove(&mut self, _key: String) {
        self.kv_item.remove(&_key);
    }
}
