use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Instant};

use crate::command::{Key, Response, Ttl, Value};
use crate::storage::Storage;

struct Entry {
    value: Value,
    expires_at: Option<Instant>,
}

pub struct MemoryStorage {
    inner: RwLock<HashMap<Key, Entry>>,
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMap::new()),
        }
    }

    fn is_expired(entry: &Entry) -> bool {
        match entry.expires_at {
            Some(deadline) => Instant::now() >= deadline,
            None => false,
        }
    }
}


impl Storage for MemoryStorage {
    fn set(&self, key: Key, value: Value) -> Response {
        let mut map = self.inner.write().unwrap();
        map.insert(
            key,
            Entry {
                value,
                expires_at: None,
            },
        );
        Response::Ok
    }

    fn get(&self, key: Key) -> Response {
        let mut map = self.inner.write().unwrap();
        match map.get(&key) {
            Some(entry) if !Self::is_expired(entry) => Response::Value(Some(entry.value.clone())),
            _ => {
                map.remove(&key);
                Response::Value(None)
            }
        }
    }

    fn del(&self, key: Key) -> Response {
        let mut map = self.inner.write().unwrap();
        let existed = map.remove(&key).is_some();
        Response::Deleted(existed)
    }

    fn expire(&self, key: Key, ttl: Ttl) -> Response {
        let mut map = self.inner.write().unwrap();
        match map.get_mut(&key) {
            Some(entry) if !Self::is_expired(entry) => {
                entry.expires_at = Some(Instant::now() + ttl.into());
                Response::Ok
            }
            _ => Response::Deleted(false),
        }
    }

    fn cleanup_expired(&self) {
        let now = Instant::now();
        let mut map = self.inner.write().unwrap();
        map.retain(|_, entry| {
            match entry.expires_at {
                Some(t) => t > now,
                None => true,
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_then_get_returns_value() {
        let storage = MemoryStorage::new();

        let key = Key::new("a".to_string());
        let value = Value::new(vec![1, 2, 3]);

        storage.set(key.clone(), value.clone());

        let response = storage.get(key);

        assert_eq!(response, Response::Value(Some(value)));
    }

    #[tokio::test]
    async fn cleanup_removes_expired() {
        use std::time::Duration;
        let storage = MemoryStorage::new();
        storage.set(Key::from("a"), Value::from("1"));
        storage.expire(Key::from("a"), Ttl::from_millis(10));
        tokio::time::sleep(Duration::from_millis(20)).await;
        storage.cleanup_expired();
        assert_eq!(storage.get(Key::from("a")), Response::Value(None));
    }
}