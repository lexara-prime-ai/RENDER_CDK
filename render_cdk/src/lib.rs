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

//! # Render CDK
//! Render CDK provides a streamlined interface for interacting with Render Cloud,
//! a platform that allows you to build, deploy, and scale your applications with ease.
//! This crate abstracts Render's API, facilitating effortless programmatic interaction
//! with Render's robust cloud infrastructure.
//!
//! [website]: https://cdk-c1wu.onrender.com/
//!
//! Reference documentation can be found on the [website].
//!
//! Render CDK comprises several modules that encapsulate
//! the functionalities of the Render API, enabling you to manage services, databases,
//! and other cloud resources through simple Rust code.
//!
//! ## Usage
//! Add `tokio` and `render_cdk` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1.37.0", features = ["full"] }
//! render_cdk = "0.0.20"
//! ```
//!
//! ## Examples
//! The following examples demonstrate various ways to interact with Render Cloud using this crate.
//!
//! ### 1. Querying for deployed services
//! Retrieve deployed services using different filters like status, region, environment, or by name.
//!
//! ```ignore
//! use render_cdk::resource_management::services::service_manager::ServiceManager;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     // List all services, limiting to 50 results
//!     ServiceManager::list_all_services("50").await.unwrap();
//!     
//!     // List services by status (e.g., suspended)
//!     ServiceManager::list_services_with_status("suspended", "50").await.unwrap();
//!
//!     // Find a specific service by name and type
//!     ServiceManager::find_service_by_name_and_type("whoami", "web_service").await.unwrap();
//!
//!     // Find services by region
//!     ServiceManager::find_service_by_region("oregon", "10").await.unwrap();
//!
//!     // Find services by environment
//!     ServiceManager::find_service_by_environment("image", "10").await.unwrap();
//! }
//! ```
//!
//! ### 2. Deleting a service
//! Delete a specific service, such as a web service or static site.
//!
//! ```ignore
//! use render_cdk::resource_management::services::service_manager::ServiceManager;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     // Delete a web service
//!     ServiceManager::delete_service("test_web", "web_service").await.unwrap();
//!
//!     // Delete a static site
//!     ServiceManager::delete_service("test_static", "static").await.unwrap();
//! }
//! ```
//!
//! ### 3. Working with Postgres databases
//! Manage Postgres databases within your Render account, listing, searching, or filtering by status.
//!
//! ```ignore
//! use render_cdk::resource_management::services::service_manager::ServiceManager;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     // List all Postgres instances, limit results to 50
//!     ServiceManager::list_postgres_instances(true, "50").await.unwrap();
//!
//!     // Find a Postgres instance by name
//!     ServiceManager::find_postgres_instance_by_name("agilecomet", true, "100").await.unwrap();
//!
//!     // Find Postgres instances by status (e.g., suspended)
//!     ServiceManager::find_postgres_instance_with_status("suspended", true, "50").await.unwrap();
//! }
//! ```
//!
//! ### 4. Deploying a static site
//! This example demonstrates how to deploy a simple static site using Render.
//!
//! ```ignore
//! use render_cdk::resource_management::templates::{Template, ServiceDetails};
//!
//! let static_site = Template {
//!     type_: "static_site".to_owned(),
//!     name: "test_static".to_owned(),
//!     repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
//!     auto_deploy: Some("yes".to_owned()),
//!     root_dir: Some("./public".to_owned()),
//!     service_details: Some(ServiceDetails {
//!         publish_path: Some("./".to_owned()),
//!         pull_request_previews_enabled: Some("yes".to_owned()),
//!         ..Default::default()
//!     }),
//!     ..Default::default()
//! };
//!
//! // Deploy the static site
//! ServiceManager::create_service(static_site).await.unwrap();
//! ```
//!
//! ### 5. Deploying a web service (Node.js)
//! This example demonstrates deploying a web service, specifically a Node.js application.
//!
//! ```ignore
//! use render_cdk::resource_management::templates::{Template, ServiceDetails, EnvSpecificDetails};
//!
//! let web_service = Template {
//!     type_: "web_service".to_owned(),
//!     name: "test_web".to_owned(),
//!     repo: "https://github.com/lexara-prime-ai/SAMPLE_WEB_SERVICE".to_owned(),
//!     auto_deploy: Some("yes".to_owned()),
//!     root_dir: Some("./".to_owned()),
//!     service_details: Some(ServiceDetails {
//!         region: Some("oregon".to_owned()),
//!         plan: Some("starter".to_owned()),
//!         runtime: Some("node".to_owned()),
//!         num_instances: Some(1),
//!         env_specific_details: Some(EnvSpecificDetails {
//!             build_command: Some("yarn".to_owned()),
//!             start_command: Some("npm start".to_owned()),
//!         }),
//!         pull_request_previews_enabled: Some("yes".to_owned()),
//!         ..Default::default()
//!     }),
//!     ..Default::default()
//! };
//!
//! // Deploy the web service
//! ServiceManager::create_service(web_service).await.unwrap();
//! ```
//!
//! ### 6. Using configuration files for resource provisioning
//! You can use `.conf` files to provision resources on Render. The following example shows
//! how to load and deploy an existing configuration file.
//!
//! ```ignore
//! use render_cdk::iaas::config::Conf;
//! use tokio::main;
//!
//! #[main]
//! async fn main() {
//!     // Read the configuration file
//!     let config = Conf::read_configuration_file("./samples/sample.conf").unwrap();
//!     println!("Loaded Configuration: {:?}", config);
//!
//!     // Deploy the configuration
//!     ServiceManager::deploy_configuration("./samples/sample.conf").await.unwrap();
//! }
//! ```

//! ## 7. Deploying services via .conf files.
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
//! ## 8. Deploying the configuration.
//! The above configuration can be deployed by running the following code snippet.
//!
//! ```ignore
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
//! ## 9. Deleting services.
//! This example demonstrates how to use the ServiceManager to delete various services.
//! 
//! ```ignore
//! use render_cdk::service_management::ServiceManager;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Delete a static site deployment.
//!     ServiceManager::delete_service("test_deployment", "static").await;
//!
//!    // Delete a web service deployment.
//!     ServiceManager::delete_service("test_deployment", "web_service").await;
//!
//!     // Delete a postgres instance.
//!     ServiceManager::delete_postgres_instance("test_postgres").await;
//!
//!     // Delete a redis instance.
//!     ServiceManager::delete_redis_instance("test_redis").await;
//! }
//!


pub mod authentication;
pub mod environment_management;
pub mod logger;
pub mod resource_management;
pub mod state_management;
pub mod utils;
