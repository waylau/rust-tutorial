/// 实现KV数据库
use std::collections::HashMap;
use std::sync::Mutex;

// 使用Mutex保证线程安全
type KvStore = Mutex<HashMap<String, String>>;

pub struct KeyValueStore {
    store: KvStore,
}

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore {
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut map = self.store.lock().unwrap();
        map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.store.lock().unwrap();
        map.get(key).cloned()
    }
}
