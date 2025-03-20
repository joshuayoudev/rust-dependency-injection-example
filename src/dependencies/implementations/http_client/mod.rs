mod reqwest_http_client;

#[cfg(test)]
mod test_http_client;

#[cfg(any(feature = "dev", test))]
mod ureq_http_client;

pub use reqwest_http_client::ReqwestHttpClient;

#[cfg(test)]
pub use test_http_client::TestHttpClient;

#[cfg(any(feature = "dev", test))]
pub use ureq_http_client::UreqHttpClient;
