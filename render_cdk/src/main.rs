#![allow(unused)]
use render_cdk::authentication::owner::Info;
use render_cdk::resource_management::models::template::*;
use render_cdk::resource_management::prelude::*;
use render_cdk::utils::config::Conf;

// [DEBUG] utils.
use render_cdk::logger::prelude::*;

use std::default::Default;
use tokio::main;

/// Usage Examples.
#[main]
async fn main() {
    // let services = ServiceManager::list_all_services("50").await;
    // let services = ServiceManager::list_services_with_status("suspended", "50").await;
    // let services = ServiceManager::find_service_by_name_and_type("test_deployment", "static").await;
    // let services = ServiceManager::find_service_by_region("oregon", "10").await;
    // let services = ServiceManager::find_service_by_environment("image", "10").await;
    // let owner = Info::get_owner_id().await;

    // let config = Conf::read_configuration_file("./samples/sample.conf").unwrap();

    // let deployment_config = Static {
    //     type_: "static_site".to_owned(),
    //     name: "test_deployment".to_owned(),
    //     repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
    //     auto_deploy: "yes".to_owned(), // By default, Render automatically deploys your service whenever you update its code or configuration.
    //     root_dir: Some("./public".to_owned()),
    //     service_details: Some(ServiceDetails {
    //         build_command: None, // Render runs this command to build your app before each deploy e.g npm run build, yarn build.
    //         publish_path: Some("./".to_owned()), // This will translate to /public/
    //         pull_request_previews_enabled: Some("yes".to_owned()),
    //         ..Default::default()
    //     }),
    //     ..Default::default()
    // };

    // ServiceManager::create_static_site(deployment_config)
    //     .await
    //     .unwrap();

    // Deploy existing configuration.
    // ServiceManager::deploy_configuration("./samples/sample.conf")
    //     .await
    //     .unwrap();

    // Deleting services.
    // ServiceManager::delete_service("test_deployment", "static").await;
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
///    let services = result.unwrap().to_string();
///     assert!(!services.is_empty());
/// }
///
/// More tests...

#[cfg(test)]
mod regression_tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    ///////////////////////
    // Service Management.
    ////////////////////////
    #[tokio::test]
    async fn test_list_all_services() {
        let result = ServiceManager::list_all_services("10").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let services = result.unwrap().to_string();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_list_services_with_status() {
        let results = ServiceManager::list_services_with_status("suspended", "10").await;
        // The result should be Ok().
        assert!(results.is_ok());

        // Validate content.
        let services = results.unwrap().to_string();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_find_service_by_name_and_type() {
        let result = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let services = result.unwrap().to_string();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_find_service_by_region() {
        let result = ServiceManager::find_service_by_region("oregon", "10").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let services = result.unwrap().to_string();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_find_service_by_environment() {
        let result = ServiceManager::find_service_by_environment("image", "10").await;

        // The reult should be Ok().
        assert!(result.is_ok());

        // Validate data.
        let services = result.unwrap().to_string();
        assert!(!services.is_empty());
    }

    #[tokio::test]
    async fn test_create_static() {
        let deployment_config = Static {
            type_: "static_site".to_owned(),
            name: "test_deployment".to_owned(),
            repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
            auto_deploy: "yes".to_owned(),
            root_dir: Some("./public".to_owned()),
            service_details: Some(ServiceDetails {
                build_command: None,
                publish_path: Some("./".to_owned()),
                pull_request_previews_enabled: Some("yes".to_owned()),
                ..Default::default()
            }),
            ..Default::default()
        };

        // Assertions.
        let service_type = deployment_config.type_;
        let service_name = deployment_config.name;
        let repo_url = deployment_config.repo;
        let auto_deploy = deployment_config.auto_deploy;
        let root_dir = deployment_config.root_dir;
        let build_command = deployment_config
            .service_details
            .clone()
            .unwrap()
            .build_command;
        let publish_path = deployment_config
            .service_details
            .clone()
            .unwrap()
            .publish_path;
        let pull_request_reviews_enabled = deployment_config
            .service_details
            .clone()
            .unwrap()
            .pull_request_previews_enabled;

        assert!(!service_type.is_empty(), "Service type should be set.");
        assert!(!service_name.is_empty(), "Service name should be set.");
        assert!(!repo_url.is_empty(), "Repo url should be set.");
        assert!(!auto_deploy.is_empty(), "Auto deploy should be set.");

        assert_eq!(root_dir, Some("./public".to_owned()));
        assert_eq!(build_command, None);
        assert_eq!(publish_path, Some("./".to_owned()));
        assert_eq!(pull_request_reviews_enabled, Some("yes".to_owned()));
    }
}
