use crate::dependencies::traits::DatabaseClient;
use crate::dependencies::traits::database_client::Key;
use crate::dependencies::traits::database_client::Value;

pub struct TestDatabaseClient {
    database: Vec<(String, String)>,
}

impl TestDatabaseClient {
    pub fn new() -> TestDatabaseClient {
        TestDatabaseClient { database: Vec::new() }
    }
}

impl DatabaseClient for TestDatabaseClient {
    fn put(&mut self, key: Key, value: Value) {
        println!("[TEST] TestDatabaseClient");
        self.database.push((key.0, value.0));
    }
}
