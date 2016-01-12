use std::sync::{Arc, Mutex};

use redis::{Connection, Commands};

use cache::Cache;

pub struct RedisCache {
    connection: Arc<Mutex<Connection>>
}

impl RedisCache {
    pub fn new(conn: Arc<Mutex<Connection>> ) -> RedisCache {
        RedisCache { connection: conn }
    }
}

impl Cache for RedisCache {
    fn get(&self, k: &String) -> Option<String> {
        self.connection.lock().unwrap().get(k.clone()).ok()
    }

    fn set(&self, k: &String, v: &String) {
       let _ = self.connection.lock().unwrap().set::<String, String,()>(k.clone(), v.clone());
    }
}

