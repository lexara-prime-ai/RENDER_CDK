# Render CDK Documentation

## Overview

`render_cdk` provides a streamlined interface for interacting with Render, a platform that allows you to build, deploy, and scale your apps with ease. This crate _abstracts_ Render's API, making it easier to work with Render cloud _**programmatically**_.

### Crate Information

-   **Name:** render_cdk
-   **Version:** 0.0.1
-   **Authors:** Irfan Ghat
-   **Description:** This crate provides a streamlined interface for interacting with Render, a platform that allows you to build, deploy, and scale your apps with ease.
-   **Homepage:** [Cloud Application Hosting for Developers | Render](https://render.com/)
-   **Repository:** [Render (github.com)](https://github.com/renderinc)
-   **License:** MIT

### Current Features

Work on the resource management module is currently under way. The API supports many of the same actions available from the Render Dashboard. It currently provides endpoints for managing:

-   Services
-   Deploys
-   Custom domains
-   Jobs

The CDK will provide an abstraction that will make it easier to work with the Render cloud programmatically.

## Environment Manager

### Code Sample

Here is an example of how to use the `EnvironmentManager` to retrieve your API key from environment variables:

```rust
#![allow(unused)]

use dotenv::dotenv;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct EnvironmentManager {
    pub API_KEY: String,
}

impl EnvironmentManager {
    pub fn retrieve_api_key() -> Self {
        dotenv().ok();

        let api_key = std::env::var_os("API_KEY").expect("[API_KEY] must be set.");

        Self {
            API_KEY: api_key.into_string().unwrap(),
        }
    }
}
``` 

### Configuration

To configure the environment variables for use with the `render_cdk`, you need to set the `API_KEY` environment variable. You can do this by creating a `.env` file in the root of your project with the following content:

```.env
`API_KEY=rnd_xxxxXXXXxxxxXXXXxxxXX
``` 

Make sure to replace `rnd_xxxxXXXXxxxxXXXXxxxXX` with your actual Render API key.

## Getting Started

### Prerequisites

Before using `render_cdk`, ensure you have the following installed:

-   Rust (latest stable version recommended)
-   Cargo (comes with Rust)

### Installation

Add `render_cdk` to your `Cargo.toml`:

```toml
`[dependencies]
render_cdk = "0.0.1"
``` 

### Usage

Here is a basic example of how to use the `render_cdk` crate to interact with Render's API:

```rust
use render_cdk::EnvironmentManager;

fn main() {
    let env_manager = EnvironmentManager::retrieve_api_key();
    println!("API Key: {}", env_manager.API_KEY);

    // Your code to interact with Render's API
}
``` 

## Service Manager

### Code Sample

Here's a sample using the `list_all_services` method from the `ServiceManager`:

```rust
#![allow(unused)]

use render_cdk::environment_management::prelude::*;
use render_cdk::resource_management::prelude::*;
use tokio::main;

#[main]
async fn main() {
    let services = ServiceManager::list_all_services("20").await;
    // Process the services as needed
}
``` 

## Contributing

Contributions are welcome! Please see the [repository](https://github.com/lexara-prime-ai/RENDER_CDK) for more information on how to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/lexara-prime-ai/MPESA_SDK/blob/master/LICENSE) file for details.

## Contact

For questions, issues, or suggestions, please open an issue on the [repository](https://github.com/lexara-prime-ai/RENDER_CDK).

----------

Thank you for using `render_cdk`! We hope this documentation helps you get started quickly.

----------
