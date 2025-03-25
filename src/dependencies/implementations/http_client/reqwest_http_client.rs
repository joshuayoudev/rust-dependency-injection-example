use crate::dependencies::traits::HttpClient;
use crate::dependencies::traits::http_client::HttpRequest;
use crate::dependencies::traits::http_client::HttpResponse;

pub struct ReqwestHttpClient {
    client: reqwest::Client,
}

impl ReqwestHttpClient {
    pub fn new() -> ReqwestHttpClient {
        ReqwestHttpClient {
            client: reqwest::Client::new(),
        }
    }
}

impl HttpClient for ReqwestHttpClient {
    async fn get(&self, request: HttpRequest) -> HttpResponse {
        println!("[PROD] ReqwestHttpClient");

        let response = self.client.get(request.uri()).send().await;

        if let Err(error) = response {
            panic!("{}", error);
        }

        let body = response.unwrap().bytes().await;

        if let Err(error) = body {
            panic!("{}", error);
        }

        HttpResponse::new(body.unwrap().to_vec())
    }
}
