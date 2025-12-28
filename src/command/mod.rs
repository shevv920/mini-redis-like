use crate::storage::Storage;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Ttl(Duration);
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Key(String);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value(Vec<u8>);

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Set { key: Key, value: Value },
    Get { key: Key },
    Del { key: Key },
    Expire { key: Key, ttl: Ttl },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Response {
    Value(Option<Value>),
    Ok,
    Deleted(bool),
    Error(String),
}

pub fn execute(command: Command, storage: &dyn Storage) -> Response {
    match command {
        Command::Set { key, value } => storage.set(key, value),
        Command::Get { key } => storage.get(key),
        Command::Del { key } => storage.del(key),
        Command::Expire { key, ttl } => storage.expire(key, ttl),
    }
}

impl From<Ttl> for Duration {
    fn from(ttl: Ttl) -> Self {
        ttl.0
    }
}

impl Key {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    #[cfg(test)]
    pub fn from(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl Value {
    pub fn new(value: Vec<u8>) -> Self {
        Self(value)
    }

    #[cfg(test)]
    pub fn from(value: impl Into<Vec<u8>>) -> Self {
        Self(value.into())
    }
}

impl Ttl {
    pub fn new(value: Duration) -> Self {
        Self(value)
    }
    pub fn from_millis(millis: u64) -> Self {
        Self(Duration::from_millis(millis))
    }
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Response::Ok => b"+OK\n".to_vec(),
            Response::Value(Some(v)) => {
                let mut out = b"+".to_vec();
                out.extend_from_slice(&v.0);
                out.push(b'\n');
                out
            }
            Response::Value(None) => b"$-1\n".to_vec(),
            Response::Deleted(true) => b":1\n".to_vec(),
            Response::Deleted(false) => b":0\n".to_vec(),
            Response::Error(str) => {
                let mut out = b"-".to_vec();
                out.extend_from_slice(str.as_bytes());
                out.push(b'\n');
                out
            }
        }
    }
}
