use super::Dependencies;
use super::implementations::database_client::BTreeMapDatabaseClient;
use super::implementations::http_client::ReqwestHttpClient;

pub struct ProdDependencies {
    database_client: BTreeMapDatabaseClient,
    http_client: ReqwestHttpClient,
}

impl Dependencies for ProdDependencies {
    type DatabaseClient = BTreeMapDatabaseClient;
    type HttpClient = ReqwestHttpClient;

    fn new() -> Self {
        ProdDependencies {
            database_client: BTreeMapDatabaseClient::new(),
            http_client: ReqwestHttpClient::new(),
        }
    }

    fn database_client(&mut self) -> &mut Self::DatabaseClient {
        &mut self.database_client
    }

    fn http_client(&self) -> &Self::HttpClient {
        &self.http_client
    }
}
