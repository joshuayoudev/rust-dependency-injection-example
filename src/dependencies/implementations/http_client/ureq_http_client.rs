use tokio::task::spawn_blocking;

use crate::dependencies::traits::HttpClient;
use crate::dependencies::traits::http_client::HttpRequest;
use crate::dependencies::traits::http_client::HttpResponse;

pub struct UreqHttpClient {
    client: ureq::Agent,
}

impl UreqHttpClient {
    pub fn new() -> UreqHttpClient {
        UreqHttpClient { client: ureq::agent() }
    }
}

impl HttpClient for UreqHttpClient {
    async fn get(&self, request: HttpRequest) -> HttpResponse {
        println!("[BETA] UreqHttpClient");

        let client = self.client.clone();

        let join_response = spawn_blocking(move || client.get(request.uri()).call()).await;

        if let Err(error) = join_response {
            panic!("{}", error);
        }

        let response = join_response.unwrap();

        if let Err(error) = response {
            panic!("{}", error);
        }

        let body = response.unwrap().body_mut().read_to_vec();

        if let Err(error) = body {
            panic!("{}", error);
        }

        HttpResponse::new(body.unwrap().to_vec())
    }
}
