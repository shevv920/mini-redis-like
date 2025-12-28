use crate::command::{Key, Response, Ttl, Value};

pub mod memory;
pub trait Storage: Send + Sync {
    fn set(&self, key: Key, value: Value) -> Response;
    fn get(&self, key: Key) -> Response;
    fn del(&self, key: Key) -> Response;
    fn expire(&self, key: Key, ttl: Ttl) -> Response;
    fn cleanup_expired(&self);
}
