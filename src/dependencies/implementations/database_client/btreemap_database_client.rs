use std::collections::BTreeMap;

use crate::dependencies::traits::database_client::Key;
use crate::dependencies::traits::database_client::Value;
use crate::dependencies::traits::DatabaseClient;

pub struct BTreeMapDatabaseClient {
    database: BTreeMap<String, String>,
}

impl BTreeMapDatabaseClient {
    pub fn new() -> BTreeMapDatabaseClient {
        BTreeMapDatabaseClient {
            database: BTreeMap::new(),
        }
    }
}

impl DatabaseClient for BTreeMapDatabaseClient {
    fn put(&mut self, key: Key, value: Value) {
        println!("[PROD] BTreeMapDatabaseClient");
        self.database.insert(key.0, value.0);
    }
}
