use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    // Create a new key value store
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.store.get(&key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}
