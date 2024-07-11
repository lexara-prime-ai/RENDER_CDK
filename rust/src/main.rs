#![allow(unused)]

use render_cdk::environment_management::prelude::*;
use render_cdk::resource_management::prelude::*;
use tokio::main;

/// Examples
#[main]
async fn main() {
    /// Examples
    /// 1. Querying for deployed Services.
    /// 
    /// List all Services.
    let services = ServiceManager::list_all_services("20").await;

    /// List all Services by Name and Type.
    let services = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;

    /// List all Services by Region.
    let services = ServiceManager::find_service_by_region("oregon", "10").await;

    /// List all Services by Environment.
    let services = ServiceManager::find_service_by_environment("image", "10").await;
    ////////////////////////////////////////////////
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
/// let result = ServiceManager::list_all_services("10").await;
///     // The result should be Ok().
///     assert!(result.is_ok());
///
///    // Validate content.
///    let services = result.unwrap();
///     assert!(!services.is_empty());
/// }
///
///  #[tokio::test]
/// async fn test_find_service_by_name_and_type() {
///     let result = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;
///     // The result should be Ok().
///     assert!(result.is_ok());
///    // Validate content.
///     let services = result.unwrap();
///     assert!(!services.is_empty());
/// }
/// 
/// More tests...

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_all_services() {
        let result = ServiceManager::list_all_services("10").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let services = result.unwrap();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_find_service_by_name_and_type() {
        let result = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let services = result.unwrap();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_find_service_by_region() {
        let result = ServiceManager::find_service_by_region("oregon", "10").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let services = result.unwrap();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_find_service_by_environment() {
        let result = ServiceManager::find_service_by_environment("image", "10").await;

        // The reult should be Ok().
        assert!(result.is_ok());

        // Validate data.
        let services = result.unwrap();
        assert!(!services.is_empty());
    }
}
