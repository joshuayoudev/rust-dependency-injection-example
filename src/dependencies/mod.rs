pub mod implementations;
pub mod traits;

// Note 1: Technically, we do not need to lock the module behind a feature flag. But it is best
//       practice to remove any dead code in production environments. Adding the feature flag here,
//       as well as on each non-production dependency, ensures that the compiler removes the code
//       from the binary.
//
// Note 2: Code locked behind feature flags will be grayed out / lose intellisense support. A
//         workaround is to use the `any()` check and include the `test` flag. IDEs by default seem
//         to use the `test` flag during development, thus keeping the intellisense.
#[cfg(any(feature = "dev", test))]
mod dev_dependencies;

mod prod_dependencies;

// Note: In this case, we cannot use the `any()` + `test` trick as it will cause the IDE to see 2
//       `ServiceDependencies`
#[cfg(feature = "dev")]
pub use dev_dependencies::DevDependencies as ServiceDependencies;

// Note: We must include this `not()` check, otherwise the program will see multiple
//       `ServiceDependencies` for any non-production environments
#[cfg(not(any(feature = "dev")))]
pub use prod_dependencies::ProdDependencies as ServiceDependencies;

pub trait Dependencies {
    type DatabaseClient: traits::DatabaseClient;
    type HttpClient: traits::HttpClient;

    fn new() -> Self;

    fn database_client(&mut self) -> &mut Self::DatabaseClient;
    fn http_client(&self) -> &Self::HttpClient;
}
