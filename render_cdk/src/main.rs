#![allow(unused)]

use render_cdk::authentication::owner::Info;
use render_cdk::iaas::prelude::config::*;
use render_cdk::iaas::prelude::deploy::*;
use render_cdk::resource_management::models::template::*;
use render_cdk::resource_management::prelude::*;

// [DEBUG] utils.
use render_cdk::logger::prelude::*;

use tokio::main;

/// Usage Examples.
#[main]
async fn main() {
    // let services = ServiceManager::list_all_services("50").await;
    // let services = ServiceManager::list_services_with_status("suspended", "50").await;
    // let services = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;
    // let services = ServiceManager::find_service_by_region("oregon", "10").await;
    // let services = ServiceManager::find_service_by_environment("image", "10").await;

    // let config = Conf::read_configuration_file("./samples/sample.conf").unwrap();

    // let deployment_config = Template {
    //     type_: "static_site".to_owned(), // Options ->
    //     name: "test_deployment".to_owned(),
    //     owner_id: Info::get_owner_id().await,
    //     repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
    //     auto_deploy: "yes".to_owned(), // By default, Render automatically deploys your service whenever you update its code or configuration.
    //     branch: None,
    //     image: None,
    //     build_filter: None,
    //     root_dir: Some("./public".to_owned()),
    //     env_vars: vec![],
    //     secret_files: vec![],
    //     service_details: Some(ServiceDetails {
    //         build_command: None, // Render runs this command to build your app before each deploy e.g npm run build, yarn build.
    //         headers: vec![],
    //         publish_path: Some("./".to_owned()), // This will translate to /public/
    //         pull_request_previews_enabled: Some("yes".to_owned()),
    //         routes: vec![],
    //     }),
    // };

    //////////////////////////
    // [DEBUG] logs.
    /////////////////////////
    // LOGGER::INFO(
    //     "Deployment Config. : ",
    //     &deployment_config.to_json_string(),
    //     LogLevel::WARN,
    // );

    // let service = ServiceManager::create_service(deployment_config)
    //     .await
    //     .unwrap();

    // Deploy::deploy_configuration("./samples/sample.conf")
    //     .await
    //     .unwrap();
}

/// Mandatory Regression Tests.
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
/// More tests...

#[cfg(test)]
mod regression_tests {
    use super::*;

    ///////////////////////
    // Service Management.
    ////////////////////////
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
    async fn test_list_services_with_status() {
        let results = ServiceManager::list_services_with_status("suspended", "10").await;
        // The result should be Ok().
        assert!(results.is_ok());

        // Validate content.
        let services = results.unwrap();
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
