use std::collections::HashMap;

/// A simple Key-Value store
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Create a new key value store
    /// ### Example
    /// let mut store = KvStore::new();
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// Add a new key-value pair to the store.
    /// If the key already exists in the store, it simply updates the value with it.
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Get a value corresponding to the given key
    /// If the key doesn't exist, it returns None.
    pub fn get(&self, key: String) -> Option<String> {
        match self.store.get(&key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    /// Removes key and corresponding value
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
