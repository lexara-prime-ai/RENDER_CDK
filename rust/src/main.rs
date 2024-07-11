#![allow(unused)]

use render_cdk::environment_management::prelude::*;
use render_cdk::resource_management::prelude::*;
use tokio::main;

/// Examples
#[main]
async fn main() {
    /// List all Services.
    // let services = ServiceManager::list_all_services("20").await;

    // List all Services by Name and Type.
    // let services = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;

    // List all Services by Region.
    // let services = ServiceManager::find_service_by_region("oregon", "10").await;

    // List all Services by Environment.
    let services = ServiceManager::find_service_by_environment("image", "10").await;
}



/// Checks for regression of service management functions
///
/// These checks are there to validate that it is functioning properly
/// and returning the right results, after which we shall describe each test case.
/// List all Services.
///
/// This test confirms if the function list_all_services returns all services available.
/// 
/// #[tokio::test]
/// async fn test_list_all_services() {
/// let services = ServiceManager::list_all_services("20").await;
/// assert!(services.is_ok());
/// let services = services.unwrap();
/// assert!(!services.is_empty());
/// }
///

#[#[cfg(tests)]]
mod tests {
    u
}

