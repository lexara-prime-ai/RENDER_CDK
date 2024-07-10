#![allow(unused)]

use render_cdk::environment_management::prelude::*;
use render_cdk::resource_management::prelude::*;
use tokio::main;

/// Examples
#[main]
async fn main() {
    /// List all Services.
    // let services = ServiceManager::list_all_services("20").await;

    // List all Services by Name and Type.
    // let services = ServiceManager::find_service_by_name_and_type("whoami", "web_service").await;

    // 'https://api.render.com/v1/services?region=oregon&limit=20'
    let services = ServiceManager::find_service_by_region("oregon", "10").await;
}
