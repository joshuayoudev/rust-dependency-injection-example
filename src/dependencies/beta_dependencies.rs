use super::Dependencies;
use super::implementations::database_client::HashMapDatabaseClient;
use super::implementations::http_client::UreqHttpClient;

pub struct BetaDependencies {
    database_client: HashMapDatabaseClient,
    http_client: UreqHttpClient,
}

impl Dependencies for BetaDependencies {
    type DatabaseClient = HashMapDatabaseClient;
    type HttpClient = UreqHttpClient;

    fn new() -> Self {
        BetaDependencies {
            database_client: HashMapDatabaseClient::new(),
            http_client: UreqHttpClient::new(),
        }
    }

    fn database_client(&mut self) -> &mut Self::DatabaseClient {
        &mut self.database_client
    }

    fn http_client(&self) -> &Self::HttpClient {
        &self.http_client
    }
}
