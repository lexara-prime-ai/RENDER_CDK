[Render](https://render.com/) is an all-in-one cloud platform designed to streamline the **development** and **deployment** of your apps and websites. It offers seamless integration with **free TLS certificates**, **a global CDN**, **private networks**, and **automatic deployments directly from Git**. _Build_, _run_, and _scale_ with ease, all in one **unified** solution.

### Getting Started
#### Configuration

To configure the environment variables for use with `render_cdk`, you need to set the `API_KEY` and `OWNER_CREDENTIALS` environment variables. You can do this by creating a `.env` file in the root of your project with the following content:

```toml
API_KEY=rnd_xxxxXXXXxxxxXXXXxxxXX
OWNER_CREDENTIALS=<render>@<email>.com
```

Make sure to replace `rnd_xxxxXXXXxxxxXXXXxxxXX` with your actual Render API key.

### Installation

Add `render_cdk` to your `Cargo.toml`:

```toml
[dependencies]
render_cdk = "0.0.21"
```

* Alternatively, running `cargo add render_cdk` at the root of your project will also add it to your project.

## Building from source
To build both the `rust` crate and `cpp` library, simply clone the repository, and run `make release-build`.

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
    // List all deployed services, limiting the result to 50.
    ServiceManager::list_all_services("50").await;

    // List all services with the status "suspended", limiting the result to 50.
    ServiceManager::list_services_with_status("suspended", "50").await;

    // Find a specific service by its name and type.
    ServiceManager::find_service_by_name_and_type("my_api", "web_service").await;

    // Find services deployed in a specific region (e.g., Oregon), limiting the result to 50.
    ServiceManager::find_service_by_region("oregon", "50").await;

    // Find services based on the environment they are deployed in, limiting the result to 50.
    ServiceManager::find_service_by_environment("image", "50").await;

    // Deleting a web service by name and type.
    ServiceManager::delete_service("my_api", "web_service").await.unwrap();

    // Deleting a static site by name and type.
    ServiceManager::delete_service("my_static_site", "static").await.unwrap();

    // List all Postgres database instances, limiting the result to 50.
    ServiceManager::list_postgres_instances(true, "50").await.unwrap();

    // Find a specific Postgres database instance by name.
    ServiceManager::find_postgres_instance_by_name("my_database", true, "50").await.unwrap();

    // Find Postgres database instances with a specific status (e.g., suspended), limiting the result to 50.
    ServiceManager::find_postgres_instance_with_status("suspended", true, "50").await.unwrap();

    // Find redis instances by name.
    ServiceManager::find_redis_instance_by_name("my_redis_instance", "50").await;
}
``` 

### 2. Deleting Services

You can delete services that are no longer needed as well.

```rust
use render_cdk::service_management::ServiceManager;

#[tokio::main]
async fn main() {
    // Delete a static site deployment.
    ServiceManager::delete_service("test_deployment", "static").await;

    // Delete a web service deployment.
    ServiceManager::delete_service("test_deployment", "web_service").await;

    // Delete a postgres instance.
    ServiceManager::delete_postgres_instance("test_postgres").await;

    // Delete a redis instance.
    ServiceManager::delete_redis_instance("test_redis").await;
}
``` 

### 3. Reading Configuration Files

The `Conf` module allows you to read configuration files that define deployment settings.

```rust
use render_cdk::configuration::Conf;

fn main() {
    // Read and parse a configuration file from a specified path.
    let config = Conf::read_configuration_file("./samples/sample.conf").unwrap();

    // Process the configuration as needed...
}
``` 

### 4. Deploying an Existing Configuration

If you already have a configuration file, you can deploy it directly using the `ServiceManager`.

```rust
use render_cdk::service_management::ServiceManager;

#[tokio::main]
async fn main() {
    // Deploy services as specified in the configuration file.
    ServiceManager::deploy_configuration("./samples/sample.conf").await.unwrap();
}
``` 

### 5. Deploying a Static Site

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
    ServiceManager::create_service(deployment_config).await.unwrap();
}
```


**Description of Fields:**

-   **build_command**: The command Render runs to build your app before each deploy, e.g., `npm run build` or `yarn build`.
-   **publish_path**: The directory path where the static site will be published, e.g., `/public/`.
-   **pull_request_previews_enabled**: Indicates whether pull request previews are enabled for this deployment.

### 6. Deploying a Web Service

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

### 7. Retrieving Owner Information

Finally, you can retrieve the owner ID of the current account with a simple API call.

```rust
use render_cdk::environment_management::prelude::*;

#[tokio::main]
async fn main() {
    // Retrieve the owner ID of the current Render account.
    Info::get_owner_id().await;
}
``` 

----------

### 8. Using Simple .conf Files for Resource Provisioning

* `.conf` files offer a convenient alternative to _programmatic resource provisioning_, allowing you to define and manage resources through _simple_ configuration settings.

### Configuration File Example

The following is a _sample configuration_ file that will be used to provision a managed _Postgres_ instance and a managed _Redis_ instance.

* The `[database]` section specifies the configuration for a managed **Postgres** instance:
 * The `name` and `user` fields should be filled with the desired database name and user.
 * `enable_high_availability` indicates whether high availability should be enabled.
 * `plan` specifies the pricing plan for the instance, and `version` indicates the Postgres version.
 * `cidrBlocks` defines which IP ranges are allowed to access the database, using CIDR notation.

```toml
# Sample configuration file for provisioning
# managed Postgres and Redis instances.

[database]
databaseName = ""                   # Replace with the desired database name
databaseUser = ""                   # Replace with the desired user for the database
enableHighAvailability = false      # Set to true to enable high availability
plan = "starter"                    # Pricing plan for the database instance
version = "12"                      # Postgres version
name = ""

# The following portion enables access control via CIDR blocks
cidrBlocks = [
    { cidrBlock = "0.0.0.0/0", description = "Public access from anywhere" },
    # { cidrBlock = "192.168.1.0/24", description = "Office network" },
    # Add more CIDR blocks here as needed
]
``` 

**_Note_**: Any blank fields (such as `name` and `user`) will be **autogenerated** if not provided.

-   The `[redis]` section specifies the configuration for a managed **Redis** instance:
    -   `plan` specifies the pricing plan for the instance.
    -   `cidrBlocks` controls which IP ranges have access to the Redis instance, using CIDR notation.

```toml
[redis]
name = ""                           # Replace with the desired Redis instance name
plan = "starter"                    # Pricing plan for the Redis instance

# CIDR blocks for access control to Redis
cidrBlocks = [
    { cidrBlock = "0.0.0.0/0", description = "Public access from anywhere" },
    # { cidrBlock = "10.0.0.0/16", description = "Private network access" },
    # Add more CIDR blocks here as needed
]
``` 

### Explanation

-   **[database] Section**:
    
    -   **name**: The name of the Postgres database.
    -   **user**: The user for the Postgres database.
    -   **enable_high_availability**: Boolean value to enable or disable high availability for the database.
    -   **plan**: The pricing plan for the Postgres instance. Options may include "starter", "standard", "premium", etc.

        <br/>

        > **_Note_**: The **free** plan will result in failed deployments.

        <br/>
        
    -   **version**: The version of Postgres to be used.
    -   **cidrBlocks**: A list of CIDR blocks for controlling access to the database. This ensures that only allowed IP ranges can access the instance.
        -   **cidrBlock**: A string representing the range of allowed IPs in CIDR format (e.g., `0.0.0.0/0` for public access or `192.168.1.0/24` for a private network).
        -   **description**: A human-readable description of the CIDR block's purpose.
-   **[redis] Section**:
    
    -   **name**: The name of the Redis instance.
    -   **plan**: The pricing plan for the Redis instance. Options may include "starter", "standard", "premium", etc.

        <br/>
        
        > **_Note_**: The **free** plan will result in failed deployments.

        <br/>
        
    -   **cidrBlocks**: A list of CIDR blocks for controlling access to the Redis instance, similar to the database configuration.

This configuration file allows you to easily set up _**managed database**_ and _**caching**_ services with specific plans and access controls suited to your project's needs.

```rust
use render_cdk::config::Conf;

fn main() {
    let config = Conf::read_configuration_file().unwrap();
    println!("Sample Configuration: {:?}\n", config);
}
``` 

This guide should have given you a solid understanding of how to streamline your cloud development with Render Cloud using Render CDK. 

By leveraging Render CDK’s powerful features, you can simplify the process of provisioning, managing, and scaling your cloud resources. 

With its declarative approach and seamless integration, Render CDK empowers you to focus more on building and innovating, while it handles the complexities of cloud infrastructure.
