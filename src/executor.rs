pub(crate) use crate::command::execute;

#[cfg(test)]
mod tests {
    pub(crate) use crate::command::{Command, Key, Response, Ttl, Value, execute};
    use crate::storage::Storage;
    use std::sync::RwLock;
    #[cfg(test)]
    struct TestStorage {
        last_call: RwLock<Option<String>>,
        response: Response,
    }
    #[cfg(test)]
    impl Storage for TestStorage {
        fn set(&self, _: Key, _: Value) -> Response {
            *self.last_call.write().unwrap() = Some("set".into());
            self.response.clone()
        }

        fn get(&self, _: Key) -> Response {
            *self.last_call.write().unwrap() = Some("get".into());
            self.response.clone()
        }

        fn del(&self, _: Key) -> Response {
            *self.last_call.write().unwrap() = Some("del".into());
            self.response.clone()
        }

        fn expire(&self, _: Key, _: Ttl) -> Response {
            *self.last_call.write().unwrap() = Some("expire".into());
            self.response.clone()
        }
        fn cleanup_expired(&self) {}
    }

    #[test]
    fn execute_set_calls_storage_set() {
        let storage = TestStorage {
            last_call: RwLock::new(None),
            response: Response::Deleted(true),
        };

        let cmd = Command::Set {
            key: Key::new("key".into()),
            value: Value::new(vec![]),
        };

        let result = execute(cmd, &storage);

        assert_eq!(result, Response::Deleted(true));
        assert_eq!(storage.last_call.read().unwrap().as_deref(), Some("set"));
    }

    #[test]
    fn execute_get_calls_storage_get() {
        let storage = TestStorage {
            last_call: RwLock::new(None),
            response: Response::Value(None),
        };

        let cmd = Command::Get {
            key: Key::new("key".into()),
        };

        let result = execute(cmd, &storage);

        assert_eq!(result, Response::Value(None));
        assert_eq!(storage.last_call.read().unwrap().as_deref(), Some("get"));
    }

    #[test]
    fn execute_del_calls_storage_del() {
        let storage = TestStorage {
            last_call: RwLock::new(None),
            response: Response::Deleted(false),
        };

        let cmd = Command::Del {
            key: Key::new("key".into()),
        };

        let result = execute(cmd, &storage);

        assert_eq!(result, Response::Deleted(false));
        assert_eq!(storage.last_call.read().unwrap().as_deref(), Some("del"));
    }

    #[test]
    fn execute_expire_calls_storage_expire() {
        let storage = TestStorage {
            last_call: RwLock::new(None),
            response: Response::Deleted(true),
        };

        let cmd = Command::Expire {
            key: Key::new("key".into()),
            ttl: Ttl::new(std::time::Duration::from_secs(1).into()),
        };

        let result = execute(cmd, &storage);

        assert_eq!(result, Response::Deleted(true));
        assert_eq!(storage.last_call.read().unwrap().as_deref(), Some("expire"));
    }
}
