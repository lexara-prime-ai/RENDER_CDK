![crates.io](https://img.shields.io/crates/v/render_cdk.svg)

# Technical Documentation

## Configuration

To configure the environment variables for use with `render_cdk`, you need to set the `API_KEY` and `OWNER_CREDENTIALS` environment variables. You can do this by creating a `.env` file in the root of your project with the following content:

```.env
API_KEY=rnd_xxxxXXXXxxxxXXXXxxxXX
OWNER_CREDENTIALS=<render>@<email>.com
```

Make sure to replace `rnd_xxxxXXXXxxxxXXXXxxxXX` with your actual Render API key.

### Installation

Add `render_cdk` to your `Cargo.toml`:

```toml
[dependencies]
render_cdk = "0.0.16"
```

* _Alternatively_, running at the `cargo add render_cdk` **root** of your project will also add **render_cdk** to your project.

### Usage Examples

Here are basic examples of how to use the `render_cdk` crate to interact with [Render Cloud](https://render.com/):

### 1. Querying for Deployed Services

You can easily retrieve information about your deployed services using the `ServiceManager` module. Below are examples of how to query services based on various criteria.

```rust
use render_cdk::environment_management::prelude::*;
use render_cdk::resource_management::prelude::*;
use tokio::main;

#[main]
async fn main() {
    // List all deployed services, limiting the result to 10.
    let services = ServiceManager::list_all_services("10").await;

    // List all services with the status "suspended", limiting the result to 50.
    let services = ServiceManager::list_services_with_status("suspended", "50").await;

    // Find a specific service by its name and type.
    let services = ServiceManager::find_service_by_name_and_type("my_api", "web_service").await;

    // Find services deployed in a specific region (e.g., Oregon), limiting the result to 10.
    let services = ServiceManager::find_service_by_region("oregon", "10").await;

    // Find services based on the environment they are deployed in, limiting the result to 10.
    let services = ServiceManager::find_service_by_environment("image", "10").await;

    // Deleting services by name and type.
    ServiceManager::delete_service("my_api", "web_service").await;
}
``` 

### 2. Retrieving Owner Information

You can retrieve the owner ID of the current account with a simple API call.

```rust
use render_cdk::environment_management::prelude::*;

#[tokio::main]
async fn main() {
    // Retrieve the owner ID of the current Render account.
    let owner = Info::get_owner_id().await;
    println!("{}", owner);
}
``` 

### 3. Reading Configuration Files

The `Conf` module allows you to read configuration files that define deployment settings.

```rust
use render_cdk::configuration::Conf;

fn main() {
    // Read and parse a configuration file from a specified path.
    let config = Conf::read_configuration_file("./samples/sample.conf").unwrap();

    // Use the configuration as needed...
}
``` 

### 4. Deploying a Static Site

The following example demonstrates how to deploy a simple static site using a configuration template.

```rust
use render_cdk::deployment::Template;
use render_cdk::service_management::{ServiceDetails, ServiceManager};

#[tokio::main]
async fn main() {
    let deployment_config = Template {
        type_: "static_site".to_owned(),
        name: "test_deployment".to_owned(),
        repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
        auto_deploy: "yes".to_owned(),
        root_dir: Some("./public".to_owned()),
        service_details: Some(ServiceDetails {
            publish_path: Some("./".to_owned()),
            pull_request_previews_enabled: Some("yes".to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Deploy the static site with the specified configuration.
    ServiceManager::create_static_site(deployment_config).await.unwrap();
}
```

**Description of Fields:**

-   **build_command**: The command Render runs to build your app before each deploy, e.g., `npm run build` or `yarn build`.
-   **publish_path**: The directory path where the static site will be published, e.g., `/public/`.
-   **pull_request_previews_enabled**: Indicates whether pull request previews are enabled for this deployment.

### 5. Deploying a Web Service

Here’s an example of deploying a simple Node.js web service.

```rust
use render_cdk::deployment::Template;
use render_cdk::service_management::{EnvSpecificDetails, ServiceDetails, ServiceManager};

#[tokio::main]
async fn main() {
    let deployment_config = Template {
        type_: "web_service".to_owned(),
        name: "test_deployment".to_owned(),
        repo: "https://github.com/lexara-prime-ai/SAMPLE_WEB_SERVICE".to_owned(),
        auto_deploy: "yes".to_owned(),
        root_dir: Some("./".to_owned()),
        service_details: Some(ServiceDetails {
            region: "oregon".to_owned(),
            plan: "starter".to_owned(),
            runtime: "node".to_owned(),
            num_instances: 1,
            env_specific_details: Some(EnvSpecificDetails {
                build_command: Some("yarn".to_owned()),
                start_command: Some("npm start".to_owned()),
            }),
            pull_request_previews_enabled: Some("yes".to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Deploy the web service with the specified configuration.
    ServiceManager::create_web_service(deployment_config).await.unwrap();
}
``` 

### 6. Deploying an Existing Configuration

If you already have a configuration file, you can deploy it directly using the `ServiceManager`.

```rust
use render_cdk::service_management::ServiceManager;

#[tokio::main]
async fn main() {
    // Deploy services as specified in the configuration file.
    ServiceManager::deploy_configuration("./samples/sample.conf").await.unwrap();
}
``` 

### 7. Deleting Services

Finally, you can delete services that are no longer needed.

```rust
use render_cdk::service_management::ServiceManager;

#[tokio::main]
async fn main() {
    // Delete a static site deployment.
    ServiceManager::delete_service("test_deployment", "static").await;

    // Delete a web service deployment.
    ServiceManager::delete_service("test_deployment", "web_service").await;
}
``` 

----------

### 8. Using Simple .conf Files for Resource Provisioning

* `.conf` files offer a convenient alternative to _programmatic resource provisioning_, allowing you to **define** and **manage** resources through _simple_ configuration settings.

### Configuration File Example

The following is a _sample configuration__ file. This will be used to provision a managed **Postgres** instance and a managed **Redis** instance.

* `[database]` section specifies the configuration for a managed Postgres instance.

* The `name` and `user` fields should be filled with the desired **database name** and **user**.
`enable_high_availability` indicates whether high availability should be enabled.
`plan` specifies the pricing plan for the instance, and `version` indicates the Postgres version.

```toml
# The following is a sample configuration file.
# This will be used to provision a
# managed postgres instance and managed redis instance.
[database]
databaseName = ""                   # Replace with the desired database name 
databaseUser = ""                   # Set to true to enable high availability
enableHighAvailability = false      # Pricing plan for the database instance
plan = "starter"
version = "12"                      # Postgres version
name = ""
# The following portion enables <public> access.
cidrBlocks = [
    { cidrBlock = "0.0.0.0/0", description = "Everywhere" }
    # { cidrBlock = "0.0.0.0/0", description = "Everywhere" },
    # { cidrBlock = "0.0.0.0/0", description = "Everywhere" }
    # Add more CIDR blocks here...
]
```

**_Note_** Blank fields will be  **autogenerated**.

* `[redis]` section specifies the configuration for a managed **Redis** instance.
 `plan` specifies the pricing plan for the instance.

```toml
[redis]
name = ""
plan = "starter"              # Pricing plan for the Redis instance` 
```

### Explanation

* **[database] Section**:

  * **name**: The name of the Postgres database.
  * **user**: The user for the Postgres database.
  * **enable_high_availability**: Boolean value to enable or disable high availability for the database.
  * **plan**: The pricing plan for the Postgres instance. Options may include "starter", "standard", "premium", etc.

    > **_Note_** **free** plan will result in failed deployments.

  * **version**: The version of Postgres to be used.
* **[redis] Section**:
  * **name**: The name of the Redis instance.
  * **plan**: The pricing plan for the Redis instance. Options may include "free", "standard", "premium", etc.

    > **_Note_** **free** plan will result in failed deployments.

This configuration file allows you to easily set up _**managed database**_ and _**caching**_ services with _specific plans_ and options suited to your project's needs.

```rust
use render_cdk::config::Conf;

fn main() {
    let config = Conf::read_configuration_file().unwrap();
    println!("Sample Configuration: {:?}\n", config);
}
```

----------

This documentation should help guide you through the basic usage of the `render_cdk` crate for managing your services on Render Cloud.