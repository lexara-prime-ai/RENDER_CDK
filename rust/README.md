
### Configuration

To configure the environment variables for use with the `render_cdk`, you need to set the `API_KEY` environment variable. You can do this by creating a `.env` file in the root of your project with the following content:

```.env
API_KEY=rnd_xxxxXXXXxxxxXXXXxxxXX
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
[dependencies]
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