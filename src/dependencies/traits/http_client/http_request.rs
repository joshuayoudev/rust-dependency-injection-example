pub struct HttpRequest {
    uri: String,
}

impl HttpRequest {
    pub fn new(uri: String) -> HttpRequest {
        HttpRequest { uri }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}
