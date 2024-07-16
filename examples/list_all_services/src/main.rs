use render_cdk::resource_management::{prelude::*};
use tokio::main;


  
#[main]
async fn main() {
    //! This example demostrates how to retrieve
    //!   a list of services, with a specified limit.
    //! 
    //!   Pre-requisites
    //!   - A valid [API_KEY] from Render Cloud.
    let _services = ServiceManager::list_all_services("50").await;
    println!("Done processing [request].");
}