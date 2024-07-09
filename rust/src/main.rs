#![allow(unused)]

use render_cdk::environment_management::prelude::*;
use render_cdk::resource_management::prelude::*;
use tokio::main;

/// Examples
#[main]
async fn main() {
    // let services = ServiceManager::list_all_services("20").await;
    let services = ServiceManager::find_service_by_name_and_type("whoami", "static_site");
}
