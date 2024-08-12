#![allow(missing_docs)]
#![allow(unused)]
#![allow(unknown_lints, unexpected_cfgs)]
#![allow(
    clippy::cognitive_complexity,
    clippy::large_enum_variant,
    clippy::module_inception,
    clippy::needless_doctest_main
)]
#![warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![deny(unused_must_use)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![cfg_attr(loom, allow(dead_code, unreachable_pub))]

//! Render CDK provides a streamlined interface for interacting with Render Cloud,
//! a platform that allows you to build, deploy, and scale your applications with ease.
//! This crate abstracts Render's API, facilitating effortless programmatic interaction
//! with Render's robust cloud infrastructure.

//! [website]: https://cdk-c1wu.onrender.com/

//! ## Reference documentation can be found on the [website].

//! Render CDK comprises several modules that encapsulate
//! the functionalities of the Render API.

//! To get started, ensure that tokio and render_cdk are added to the dependencies in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1", features = ["full"] }
//! render_cdk = "0.14"
//! ```

//! Examples:
//! - Querying for deployed services.
//! - Using simple .conf files for resource provisioning.
//! - Retrieving the Owner Id to tie a resource to the user who created it.
//! - Creating and deploying services.

//! ## 1. Querying for deployed Services.
//! This example demonstrates how to retrieve a list of deployed services
//! based on specified criteria.
//!
//! ```rust,ignore
//! use render_cdk::resource_management::services::service_manager::ServiceManager;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     let services = ServiceManager::list_all_services("50").await;
//!     let services = ServiceManager::list_services_with_status("suspended", "50").await;
//!     let services = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;
//!     let services = ServiceManager::find_service_by_region("oregon", "10").await;
//!     let services = ServiceManager::find_service_by_environment("image", "10").await;
//!
//!     // Additional processing...
//!}
//! ```

//! ## 2. Using simple .conf files for resource provisioning.
//!
//! ```rust,ignore
//! use render_cdk::iaas::config::Conf;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     // You can manually preview the configuration files
//!     // to verify that the file has been setup properly...
//!     let config = config::Conf::read_configuration_file().unwrap();
//!     println!("Sample Configuration: {:?}\n", config);
//!
//!     // Additional processing...
//! }
//! ```

//! ## 3. Retrieving the owner_id.
//! The `owner_id` is used to tie the resource/service to the Owner
//! or Acting Service Principal.
//!
//! ```rust,ignore
//! use render_cdk::authentication::owner::Info;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     let owner_id = Info::get_owner_id().await;
//!     assert!(!owner_id.is_empty());
//! }
//! ```
//!
//! ## 4. Creating services.
//! The following is a sample deployment configuration.
//!
//! ```rust,ignore
//! use render_cdk::resource_management::{models::prelude::*, prelude::*};
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     let deployment_config = template::Template {
//!         type_: "static_site".to_owned(),
//!         name: "test_deployment".to_owned(),
//!         owner_id: "owner123",
//!         repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
//!         auto_deploy: "yes".to_owned(),
//!         branch: None,
//!         image: None,
//!         build_filter: None,
//!         root_dir: Some("./public".to_owned()),
//!         env_vars: vec![],
//!         secret_files: vec![],
//!         service_details: Some(ServiceDetails {
//!             build_command: None,
//!             headers: vec![],
//!             publish_path: Some("./".to_owned()),
//!             pull_request_previews_enabled: Some("yes".to_owned()),
//!             routes: vec![],
//!         }),
//!     };
//!
//!
//!     let service = ServiceManager::create_service(deployment_config)
//!         .await
//!         .unwrap();
//! }
//! ```

//! #### Here are other sample configurations that include image configurations.
//!
//! `NOTE` - The following section demostrates functionality that's currently under development.
//!
//! ```rust,ignore
//! #[main]
//! asycn fn main() {
//!     let template_with_image = Template {
//!         type_: "static_site".to_owned(),
//!         name: "test".to_owned(),
//!         owner_id: "test".to_owned(),
//!         repo: "https://link/to/your/repository".to_owned(),
//!         auto_deploy: true,
//!         branch: Some("main".to_owned()),
//!         image: Some(Image {
//!             owner_id: "owner123".to_owned(),
//!             registry_credential_id: "cred123".to_owned(),
//!             image_path: "path/to/image".to_owned(),
//!         }),
//!         build_filter: BuildFilter {
//!             // Initialize your fields here...
//!         },
//!         root_dir: "./".to_owned(),
//!         env_vars: vec![
//!             EnvVar {
//!                 key: "EXAMPLE".to_owned(),
//!                 value: Some("EXAMPLE".to_owned()),
//!                 generate_value: false,
//!             }
//!         ],
//!         secret_files: vec![],
//!         service_details: ServiceDetails {
//!             // Initialize your fields here...
//!         },
//!     };
//!
//!     // Deploy the configuration...
//!     let service = ServiceManager::create_service(template_with_image)
//!         .await
//!         .unwrap();
//!     
//!     assert!(service.is_ok());
//! }
//! ```

//!
//! #### The following example does not contain an image, branch, env_vars and secret_files.
//!
//! ```rust,ignore
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     let template_without_image = Template {
//!         type_: "static_site".to_owned(),
//!         name: "test".to_owned(),
//!         owner_id: "test".to_owned(),
//!         repo: "httpe".to_owned(),
//!         auto_deploy: true,
//!         branch: None,
//!         image: None,
//!         build_filter: BuildFilter {
//!             // Initialize your fields here...
//!         },
//!         root_dir: "./".to_owned(),
//!         env_vars: vec![],
//!         secret_files: vec![],
//!         service_details: ServiceDetails {
//!             // Initialize your fields here...
//!         },
//!     };
//!
//!     // Deploy the configuration...
//!     let service = ServiceManager::create_service(template_without_image)
//!         .await
//!         .unwrap();
//!
//!     assert!(services.is_ok());
//! }
//! ```

//! ## 5. Deploying services via .conf files.
//!
//! This method makes everything easier, the only thing you need to have setup is the
//! `.conf` file, your Render `API_KEY` and `OWNER_CREDENTIALS`
//! i.e the email that acts as the Service Principal on Render Cloud(Identity Access Management.)
//!
//! Here's a sample of a simple configuration file.
//!
//! ```toml
//! # The following is a sample configuration file.
//! # This will be used to provision a
//! # managed postgres instance and managed redis instance.
//! [database]
//! databaseName = ""
//! databaseUser = ""
//! enableHighAvailability = false
//! plan = "starter"
//! version = "12"
//! name = ""
//! # The following portion enables <public> access.
//! cidrBlocks = [
//!     { cidrBlock = "0.0.0.0/0", description = "Everywhere" }
//!     # { cidrBlock = "0.0.0.0/0", description = "Everywhere" },
//!     # { cidrBlock = "0.0.0.0/0", description = "Everywhere" }
//!     # Add more CIDR blocks here...
//! ]
//!
//! [redis]
//! # name = ""
//! # plan = "starter"
//! # cidrBlocks = [
//! #     { cidrBlock = "0.0.0.0/0", description = "Everywhere" }
//! #     # { cidrBlock = "0.0.0.0/0", description = "Everywhere" },
//! #     # { cidrBlock = "0.0.0.0/0", description = "Everywhere" }
//! #     # Add more CIDR blocks here...
//! # ]
//! ```
//!
//! ## 6. Deploying the configuration.
//! The above configuration can be deployed by running the following code snippet.
//!
//! ```rust,ignore
//! use render_cdk::iaas::config::Conf;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     // Specify the patch to the .conf file...
//!     let conf = Conf::read_configuration_file("./samples/sample.conf");
//!     let result = Deploy::deploy_configuration("./samples/sample.conf")
//!         .await
//!         .unwrap();
//!
//!     assert!(conf.is_ok());
//!     assert!(result.is_ok());
//! }
//! ```
//!
//! ## 7. Deleting deployed services.
//! The following example demonstrates how to delete deployed services.
//!
//! ```rust,ignore
//! use render_cdk::resource_management::prelude::*;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     // Provide the <service_name> and <service_type>...
//!     ServiceManager::delete_service("test_deployment", "static").await;
//! }
//! ```
//!

pub mod authentication;
pub mod environment_management;
pub mod logger;
pub mod resource_management;
pub mod state_management;
pub mod utils;
