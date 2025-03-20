pub struct HttpResponse {
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(body: Vec<u8>) -> HttpResponse {
        HttpResponse { body }
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
}
