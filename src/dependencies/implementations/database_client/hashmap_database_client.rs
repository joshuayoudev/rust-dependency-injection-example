use std::collections::HashMap;

use crate::dependencies::traits::DatabaseClient;
use crate::dependencies::traits::database_client::Key;
use crate::dependencies::traits::database_client::Value;

pub struct HashMapDatabaseClient {
    database: HashMap<String, String>,
}

impl HashMapDatabaseClient {
    pub fn new() -> HashMapDatabaseClient {
        HashMapDatabaseClient {
            database: HashMap::new(),
        }
    }
}

impl DatabaseClient for HashMapDatabaseClient {
    fn put(&mut self, key: Key, value: Value) {
        println!("[BETA] HashMapDatabaseClient");
        self.database.insert(key.0, value.0);
    }
}
