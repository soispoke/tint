use std::{collections::HashMap, sync::Mutex};

use crate::kv::KvStore;

/// Basic in-memory KV database implementation.
#[derive(Default)]
pub struct MemoryDatabase {
    store: Mutex<HashMap<Vec<u8>, Vec<u8>>>,
}

impl MemoryDatabase {
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl KvStore for MemoryDatabase {
    async fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        #[allow(clippy::unwrap_used)]
        let store = self.store.lock().unwrap();
        store.get(key).cloned()
    }

    async fn batch_put(&self, items: &[(&[u8], &[u8])]) {
        #[allow(clippy::unwrap_used)]
        let mut store = self.store.lock().unwrap();
        for (key, value) in items {
            store.insert(key.to_vec(), value.to_vec());
        }
    }

    async fn delete(&self, key: &[u8]) {
        #[allow(clippy::unwrap_used)]
        let mut store = self.store.lock().unwrap();
        store.remove(key);
    }
}
