mod btreemap_database_client;

#[cfg(any(feature = "beta", test))]
mod hashmap_database_client;

#[cfg(test)]
mod test_database_client;

pub use btreemap_database_client::BTreeMapDatabaseClient;

#[cfg(any(feature = "beta", test))]
pub use hashmap_database_client::HashMapDatabaseClient;

#[cfg(test)]
pub use test_database_client::TestDatabaseClient;
