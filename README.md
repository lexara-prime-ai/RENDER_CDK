# Render CDK Documentation

- Reference Documentation - [Reference](https://cdk-c1wu.onrender.com/)
- Technical Documentation(Rust) - [Markdown](https://github.com/lexara-prime-ai/RENDER_CDK/blob/master/rust/README.md)
- Technical Documentation(CPP) - _Coming Soon_

![docs](https://github.com/lexara-prime-ai/RENDER_CDK/blob/master/docs.jpeg?raw=true)

## Overview

`render_cdk` provides a streamlined interface for interacting with Render, a platform that allows you to build, deploy, and scale your apps with ease. This crate _abstracts_ Render's API, making it easier to work with Render cloud _**programmatically**_.

### Crate Information

- **Name:** render_cdk
- **Version:** 0.0.10
- **License:** MIT

### Current Features

Work on the resource management module is currently under way. The API supports many of the same actions available from the Render Dashboard. It currently provides endpoints for managing:

- Services
- Deploys
- Custom domains
- Jobs

The CDK will provide an abstraction that will make it easier to work with the Render cloud programmatically.

# Render CDK Crate Documentation

The `render_cdk` crate provides a comprehensive toolkit for managing cloud environments, IaaS, resource management, and state management within a Continuous Deployment/Continuous Integration (CDK) environment. This documentation provides an overview of the primary components and usage examples to help you get started.

## Modules

- **environment_management**: Tools and utilities for managing the deployment environment.
- **iaas**: Infrastructure as a Service functionalities.
- **resource_management**: Tools for managing resources, including models and utilities.
- **state_management**: Utilities for managing the state of deployed services.
- **logger**: Logging utilities for debugging and tracking operations.

## Usage

To use `render_cdk`, include the following in your `Cargo.toml`:

```toml
[dependencies]
render_cdk = "0.0.9"
```

## Examples

### Querying for Deployed Services

The following examples demonstrate how to query for various deployed services using the `ServiceManager`.

#### List all Services with a Specific Status

```rust
use render_cdk::iaas::prelude::*;
use tokio::main;

#[main]
async fn main() {
    let services = ServiceManager::list_services_with_status("suspended", "50").await;
}
```

#### List all Services by Name and Type

```rust
use render_cdk::iaas::prelude::*;
use tokio::main;

#[main]
async fn main() {
    let services = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;
}
```

#### List all Services by Region

```rust
use render_cdk::iaas::prelude::*;
use tokio::main;

#[main]
async fn main() {
    let services = ServiceManager::find_service_by_region("oregon", "10").await;
}
```

#### List all Services by Environment

```rust
use render_cdk::iaas::prelude::*;
use tokio::main;

#[main]
async fn main() {
    let services = ServiceManager::find_service_by_environment("image", "10").await;
}
```

### Using Configuration Files for Resource Provisioning

```rust
use render_cdk::resource_management::prelude::*;

#[main]
async fn main() {
    let config = config::Conf::read_configuration_file().unwrap();
    println!("Sample Configuration: {:?}\n", config);
}
```

### Retrieving a List of Authorized Users

```rust
use render_cdk::resource_management::prelude::*;

#[main]
async fn main() {
    let authorized_user = Owner::list_authorized_users("<user>@<email>.com", "100").await.unwrap();
    println!("Authorized User: {:?}", authorized_user);
}
```

### Creating Services

#### Sample Deployment Configuration

```rust
use render_cdk::resource_management::prelude::*;

#[main]
async fn main() {
    let deployment_config = template::Template {
        type_: "static_site".to_owned(),
        name: "test_deployment".to_owned(),
        owner_id: "owner_id_value".to_owned(),
        repo: "https://github.com/<username>/<repo_name>".to_owned(),
        auto_deploy: "yes".to_owned(),
        branch: None,
        image: None,
        build_filter: None,
        root_dir: Some("./public".to_owned()),
        env_vars: vec![],
        secret_files: vec![],
        service_details: Some(ServiceDetails {
            build_command: None,
            headers: vec![],
            publish_path: Some("./".to_owned()),
            pull_request_previews_enabled: Some("yes".to_owned()),
            routes: vec![],
        }),
    };

    let service = ServiceManager::create_service(deployment_config).await.unwrap();
    println!("Service Created: {:?}", service);
}
```

#### Sample Template with Image

```rust
use render_cdk::resource_management::prelude::*;

#[main]
async fn main() {
    let template_with_image = Template {
        type_: "static_site".to_owned(),
        name: "test".to_owned(),
        owner_id: "owner123".to_owned(),
        repo: "httpe".to_owned(),
        auto_deploy: true,
        branch: Some("main".to_owned()),
        image: Some(Image {
            owner_id: "owner123".to_owned(),
            registry_credential_id: "cred123".to_owned(),
            image_path: "path/to/image".to_owned(),
        }),
        build_filter: BuildFilter { /* Initialize your fields here */ },
        root_dir: "./".to_owned(),
        env_vars: vec![
            EnvVar {
                key: "EXAMPLE".to_owned(),
                value: Some("EXAMPLE".to_owned()),
                generate_value: false,
            }
        ],
        secret_files: vec![],
        service_details: ServiceDetails { /* Initialize your fields here */ },
    };
}
```

#### Sample Template without Image

```rust
use render_cdk::resource_management::prelude::*;

#[main]
async fn main() {
    let template_without_image = Template {
        type_: "static_site".to_owned(),
        name: "test".to_owned(),
        owner_id: "test".to_owned(),
        repo: "httpe".to_owned(),
        auto_deploy: true,
        branch: None,
        image: None,
        build_filter: BuildFilter { /* Initialize your fields here */ },
        root_dir: "./".to_owned(),
        env_vars: vec![],
        secret_files: vec![],
        service_details: ServiceDetails { /* Initialize your fields here */ },
    };
}
```

### Logging

```rust
use render_cdk::logger::prelude::*;

#[main]
async fn main() {
    LOGGER::INFO("Deployment Config. : ", &deployment_config.to_json_string(), LogLevel::WARN);
}
```

## Contributing

Contributions are welcome! Please see the [repository](CONTRIBUTING.md) for more information on how to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/lexara-prime-ai/MPESA_SDK/blob/master/LICENSE) file for details.

## Contact

For questions, issues, or suggestions, please open an issue on the [repository](https://github.com/lexara-prime-ai/RENDER_CDK).

Thank you for using `render_cdk`! We hope this documentation helps you get started quickly.
