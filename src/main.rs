use self::dependencies::Dependencies;
use self::dependencies::ServiceDependencies;
use self::dependencies::traits::DatabaseClient;
use self::dependencies::traits::HttpClient;
use self::dependencies::traits::database_client::Key;
use self::dependencies::traits::database_client::Value;
use self::dependencies::traits::http_client::HttpRequest;

mod dependencies;

#[tokio::main]
async fn main() {
    run_service(ServiceDependencies::new()).await
}

async fn run_service(mut dependencies: impl Dependencies) {
    dependencies
        .database_client()
        .put(Key("foo".to_owned()), Value("bar".to_owned()));

    let _http_response = dependencies
        .http_client()
        .get(HttpRequest::new("https://docs.rs/".to_owned()))
        .await;
}

#[cfg(test)]
mod tests {
    use crate::dependencies::Dependencies;
    use crate::dependencies::implementations::database_client::TestDatabaseClient;
    use crate::dependencies::implementations::http_client::TestHttpClient;
    use crate::run_service;

    struct TestDependencies {
        database_client: TestDatabaseClient,
        http_client: TestHttpClient,
    }

    impl Dependencies for TestDependencies {
        type DatabaseClient = TestDatabaseClient;
        type HttpClient = TestHttpClient;

        fn new() -> Self {
            TestDependencies {
                database_client: TestDatabaseClient::new(),
                http_client: TestHttpClient::new(),
            }
        }

        fn database_client(&mut self) -> &mut Self::DatabaseClient {
            &mut self.database_client
        }

        fn http_client(&self) -> &Self::HttpClient {
            &self.http_client
        }
    }

    #[tokio::test]
    async fn test_run_service_with_mocked_dependencies() {
        let test_dependencies = TestDependencies::new();

        run_service(test_dependencies).await;
    }
}
