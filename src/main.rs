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
