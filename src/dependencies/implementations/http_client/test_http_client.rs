use crate::dependencies::traits::HttpClient;
use crate::dependencies::traits::http_client::HttpRequest;
use crate::dependencies::traits::http_client::HttpResponse;

pub const TEST_BODY: &str = "test body";

pub struct TestHttpClient;

impl TestHttpClient {
    pub fn new() -> TestHttpClient {
        TestHttpClient
    }
}

impl HttpClient for TestHttpClient {
    async fn get(&self, _request: HttpRequest) -> HttpResponse {
        println!("[TEST] TestHttpClient");

        HttpResponse::new(TEST_BODY.as_bytes().to_vec())
    }
}
