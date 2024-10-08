#![allow(unused)]
use render_cdk::authentication::owner::Info;
use render_cdk::resource_management::models::template::*;
use render_cdk::resource_management::prelude::*;
use render_cdk::utils::config::Conf;

// [DEBUG] utils.
use colored::Colorize;
use render_cdk::logger::prelude::*;
use render_cdk::LOGGER;

use std::default::Default;
use tokio::main;
use tokio::time::{sleep, Duration};

/// Usage Examples.
#[main]
async fn main() {
    /*
      _____                 _
     / ____|               (_)
    | (___   ___ _ ____   ___  ___ ___  ___
     \___ \ / _ \ '__\ \ / / |/ __/ _ \/ __|
     ____) |  __/ |   \ V /| | (_|  __/\__ \
    |_____/ \___|_|    \_/ |_|\___\___||___/

    */

    // ServiceManager::list_all_services("50").await;
    // ServiceManager::list_services_with_status("suspended", "50").await;
    // ServiceManager::find_service_by_name_and_type("test_deployment", "static").await;
    // ServiceManager::find_service_by_region("oregon", "50").await;
    // ServiceManager::find_service_by_environment("image", "50").await;

    /*
     _____          _
    |  __ \        | |
    | |__) |__  ___| |_ __ _ _ __ ___  ___
    |  ___/ _ \/ __| __/ _` | '__/ _ \/ __|
    | |  | (_) \__ \ || (_| | | |  __/\__ \
    |_|   \___/|___/\__\__, |_|  \___||___/
                        __/ |
                       |___/

    */

    // ServiceManager::list_postgres_instances(true, "50").await;
    // ServiceManager::find_postgres_instance_by_name("fluentcomet", true, "50").await;
    // ServiceManager::find_postgres_instance_with_status("suspended", true, "50").await;

    /*
    _____          _ _
    |  __ \        | (_)
    | |__) |___  __| |_ ___
    |  _  // _ \/ _` | / __|
    | | \ \  __/ (_| | \__ \
    |_|  \_\___|\__,_|_|___/

    */

    // ServiceManager::find_redis_instance_by_name("cyberplasma", "50").await;

    /*
    _____          _ _
    |  __ \        | (_)
    | |__) |___  __| |_ ___
    |  _  // _ \/ _` | / __|
    | | \ \  __/ (_| | \__ \
    |_|  \_\___|\__,_|_|___/

    */

    // let instances = ServiceManager::find_redis_instance_by_name("cyberplasma", "50").await;

    // Retrieve Owner Id.
    // let owner = Info::get_owner_id().await;
    // println!("{}", owner);

    // let config = Conf::read_configuration_file("./samples/sample.conf").unwrap();

    /*

        * THE FOLLOWING EXAMPLE DEPLOYS A SIMPLE [STATIC] SITE.
        > REPO_URL - https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE

    */

    let static_site = Template {
        type_: "static_site".to_owned(),
        name: "test_static".to_owned(),
        repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
        auto_deploy: Some("yes".to_owned()),
        root_dir: Some("./public".to_owned()),
        service_details: Some(ServiceDetails {
            publish_path: Some("./".to_owned()),
            pull_request_previews_enabled: Some("yes".to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    };

    /*
        <build_command>
        Render runs this command to build your app before each deploy
        e.g npm run build, yarn build

        <publish_path>
        The path where the static site will be published
        This will translate to /public/

        <pull_request_previews_enabled>
        Whether pull request previews are enabled for this deployment.

    */

    /*

        * THE FOLLOWING EXAMPLE DEPLOYS A SIMPLE [WEB] SERVICE(NodeJs).
        > REPO_URL - https://github.com/lexara-prime-ai/SAMPLE_WEB_SERVICE

    */

    let web_service = Template {
        type_: "web_service".to_owned(),
        name: "test_web".to_owned(),
        repo: "https://github.com/lexara-prime-ai/SAMPLE_WEB_SERVICE".to_owned(),
        auto_deploy: Some("yes".to_owned()),
        root_dir: Some("./".to_owned()),
        service_details: Some(ServiceDetails {
            region: Some("oregon".to_owned()),
            plan: Some("starter".to_owned()),
            runtime: Some("node".to_owned()),
            num_instances: Some(1),
            env_specific_details: Some(EnvSpecificDetails {
                build_command: Some("yarn".to_owned()),
                start_command: Some("npm start".to_owned()),
            }),
            pull_request_previews_enabled: Some("yes".to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    };

    // DEPLOY A <static_site>.
    // ServiceManager::create_service(static_site).await;

    // DEPLOY A <web_service>.
    // ServiceManager::create_service(web_service).await;

    // Deploy existing configuration.
    // ServiceManager::deploy_configuration("./samples/sample.conf")
    //     .await
    //     .unwrap();

    // Deleting services.
    // Wait for the specified amount of time before deleting the deployed resources.
    // LOGGER!(
    //     ":: [Status] ::",
    //     "Waiting for deployment...",
    //     LogLevel::WARN
    // );

    /*

        DELETING SERVICES.

    */
    // sleep(Duration::from_secs(150)).await;
    // ServiceManager::delete_service("test_static", "static").await;
    // ServiceManager::delete_service("test_web", "web_service").await;
    // ServiceManager::delete_postgres_instance("gearednimbus").await;
    // ServiceManager::delete_redis_instance("cyberplasma").await;
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
    async fn test_list_all_postgres_instances() {
        let result = ServiceManager::list_postgres_instances(true, "10").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let instances = result.unwrap().to_string();
        assert!(!instances.is_empty());
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
    async fn test_find_postgres_instance_with_status() {
        let results =
            ServiceManager::find_postgres_instance_with_status("suspended", true, "50").await;
        // The result should be Ok().
        assert!(results.is_ok());

        // Validate content.
        let instances = results.unwrap().to_string();
        assert!(!instances.is_empty());
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
    async fn test_find_postgres_instance_by_name() {
        let result =
            ServiceManager::find_postgres_instance_by_name("fluentcomet", true, "100").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let instances = result.unwrap().to_string();
        assert!(!instances.is_empty());
    }

    #[tokio::test]
    async fn test_find_redis_instance_by_name() {
        let result = ServiceManager::find_redis_instance_by_name("fluentcomet", "100").await;
        // The result should be Ok().
        assert!(result.is_ok());

        // Validate content.
        let instances = result.unwrap().to_string();
        assert!(!instances.is_empty());
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
        let deployment_config = Template {
            type_: "static_site".to_owned(),
            name: "test_deployment".to_owned(),
            repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
            auto_deploy: Some("yes".to_owned()),
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
        assert!(auto_deploy.is_some(), "Auto deploy should be set.");

        assert_eq!(root_dir, Some("./public".to_owned()));
        assert_eq!(build_command, None);
        assert_eq!(publish_path, Some("./".to_owned()));
        assert_eq!(pull_request_reviews_enabled, Some("yes".to_owned()));
    }
}
