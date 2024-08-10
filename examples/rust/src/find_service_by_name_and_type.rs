use render_cdk::resource_management::{prelude::*};
use tokio::main;

  
#[main]
async fn main() {
    //! This example demostrates how to retrieve
    //!   a list of services by name and type,
    //!   with a specified limit.
    //! 
    //!   Pre-requisites
    //!   - A valid [API_KEY] from Render Cloud.
    ServiceManager::find_service_by_name_and_type("test_deployment", "static").await.unwrap();
}