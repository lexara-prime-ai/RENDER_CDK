#![allow(unused)]
// #![deny(missing_docs)]

use render_cdk::environment_management::prelude::*;
use render_cdk::iaas::prelude::*;
use render_cdk::resource_management::{self, models::prelude::*, prelude::*};
use render_cdk::state_management::prelude::*;

// [DEBUG] utils.
use render_cdk::logger::prelude::*;

use tokio::main;

/// Examples
#[main]
async fn main() {
    /// Examples
    /// 1. Querying for deployed Services.
    ///
    /// List all Services.
    // let services = ServiceManager::list_all_services("50").await;

    // List all Services that are suspended/not_suspended.
    let services = ServiceManager::list_all_suspended_services("suspended", "50").await;

    // List all Services by Name and Type.
    // let services = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;

    // List all Services by Region.
    // let services = ServiceManager::find_service_by_region("oregon", "10").await;

    // List all Services by Environment.
    // let services = ServiceManager::find_service_by_environment("image", "10").await;
    ////////////////////////////////////////////////
    //
    // 2. Using simple .conf files for resource provisioning.
    // let config = config::Conf::read_configuration_file().unwrap();
    // println!("Sample Configuration: {:?}\n", config);

    // 3. Retrieve a list of authorized 'users'.
    // let authorized_user = Owner::list_authorized_users("irfanghat@gmail.com", "100")
    //     .await
    //     .unwrap();

    ////////////////////////////
    // [DEBUG] logs.
    ///////////////////////////
    // println!("Owner Info.: {:?}\n", authorized_user);

    ///////////////////////////
    // Retrieving the <owner_id>. This is used to tie a <resource> to the user who created it.
    // let owner_id = authorized_user
    //     .get(0)
    //     .map(|owner_response| owner_response.owner.id.clone())
    //     .expect("No authorized users found.");

    // /// 4. Creating services.
    // The following is a sample deployment configuration.
    // let deployment_config = template::Template {
    //     type_: "static_site".to_owned(), // Options ->
    //     name: "test_deployment".to_owned(),
    //     owner_id,
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

    ///////////////////////////////
    // Other sample configurations.
    ///////////////////////////////
    // let template_with_image = Template {
    //     type_: "static_site".to_owned(),
    //     name: "test".to_owned(),
    //     owner_id: "test".to_owned(),
    //     repo: "httpe".to_owned(),
    //     auto_deploy: true,
    //     branch: Some("main".to_owned()),
    //     image: Some(Image {
    //         owner_id: "owner123".to_owned(),
    //         registry_credential_id: "cred123".to_owned(),
    //         image_path: "path/to/image".to_owned(),
    //     }),
    //     build_filter: BuildFilter {
    //         Initialize your fields here.
    //     },
    //     root_dir: "./".to_owned(),
    //     env_vars: vec![
    //         EnvVar {
    //             key: "EXAMPLE".to_owned(),
    //             value: Some("EXAMPLE".to_owned()),
    //             generate_value: false,
    //         }
    //     ],
    //     secret_files: vec![],
    //     service_details: ServiceDetails {
    //         Initialize your fields here
    //     },
    // };

    // This example doesn't contain an image, branch, env_vars and secret_files.
    //
    // let template_without_image = Template {
    //     type_: "static_site".to_owned(),
    //     name: "test".to_owned(),
    //     owner_id: "test".to_owned(),
    //     repo: "httpe".to_owned(),
    //     auto_deploy: true,
    //     branch: None,
    //     image: None,
    //     build_filter: BuildFilter {
    //         Initialize your fields here
    //     },
    //     root_dir: "./".to_owned(),
    //    env_vars: vec![],
    //     secret_files: vec![],
    //     service_details: ServiceDetails {
    //         Initialize your fields here
    //     },
    // };
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
