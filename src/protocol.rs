use crate::command::{Command, Key, Value, Ttl};
use std::time::Duration;

#[derive(Debug)]
pub enum ProtocolError {
    Command,
    Arity,
    Ttl,
}

pub fn parse(input: &[u8]) -> Result<Command, ProtocolError> {
    let line = std::str::from_utf8(input).map_err(|_| ProtocolError::Command)?;
    let mut parts = line.split_whitespace();

    let cmd = parts.next().ok_or(ProtocolError::Command)?;

    match cmd {
        "SET" => {
            let key = parts.next().ok_or(ProtocolError::Arity)?;
            let value = parts.next().ok_or(ProtocolError::Arity)?;
            Ok(Command::Set {
                key: Key::new(key.into()),
                value: Value::new(value.as_bytes().to_vec()),
            })
        }
        "GET" => {
            let key = parts.next().ok_or(ProtocolError::Arity)?;
            Ok(Command::Get {
                key: Key::new(key.into()),
            })
        }
        "DEL" => {
            let key = parts.next().ok_or(ProtocolError::Arity)?;
            Ok(Command::Del {
                key: Key::new(key.into()),
            })
        }
        "EXPIRE" => {
            let key = parts.next().ok_or(ProtocolError::Arity)?;
            let ttl = parts.next().ok_or(ProtocolError::Arity)?;
            let secs = ttl.parse::<u64>().map_err(|_| ProtocolError::Ttl)?;
            Ok(Command::Expire {
                key: Key::new(key.into()),
                ttl: Ttl::new(Duration::from_secs(secs)),
            })
        }
        _ => Err(ProtocolError::Command),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set() {
        let cmd = parse(b"SET a 1").unwrap();
        assert_eq!(
            cmd,
            Command::Set {
                key: Key::new("a".into()),
                value: Value::new(b"1".to_vec().into()),
            }
        );
    }

    #[test]
    fn parse_get() {
        let cmd = parse(b"GET a").unwrap();
        assert_eq!(cmd, Command::Get { key: Key::new("a".into()) });
    }

    #[test]
    fn parse_del() {
        let cmd = parse(b"DEL a").unwrap();
        assert_eq!(cmd, Command::Del { key: Key::new("a".into()) });
    }

    #[test]
    fn parse_expire() {
        let cmd = parse(b"EXPIRE a 5").unwrap();
        assert_eq!(
            cmd,
            Command::Expire {
                key: Key::new("a".into()),
                ttl: Ttl::new(Duration::from_secs(5).into()),
            }
        );
    }

    #[test]
    fn parse_unknown_fails() {
        assert!(parse(b"FOO a").is_err());
    }
}

