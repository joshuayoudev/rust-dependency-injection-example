pub mod implementations;
pub mod traits;

#[cfg(any(feature = "dev", test))]
mod dev_dependencies;

mod prod_dependencies;

#[cfg(feature = "dev")]
pub use dev_dependencies::DevDependencies as ServiceDependencies;

#[cfg(not(feature = "dev"))]
pub use prod_dependencies::ProdDependencies as ServiceDependencies;

pub trait Dependencies {
    type DatabaseClient: traits::DatabaseClient;
    type HttpClient: traits::HttpClient;

    fn database_client(&mut self) -> &mut Self::DatabaseClient;
    fn http_client(&self) -> &Self::HttpClient;
}
