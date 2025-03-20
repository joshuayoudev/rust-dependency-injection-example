mod http_request;
mod http_response;

pub use http_request::HttpRequest;
pub use http_response::HttpResponse;

pub trait HttpClient {
    async fn get(&self, request: HttpRequest) -> HttpResponse;
}
