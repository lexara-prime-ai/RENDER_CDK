use render_cdk::resource_management::prelude::*;
use tokio::main;

#[main]
async fn main() {
    //! This example demostrates how to retrieve
    //!   a list of services based on status,
    //!   with a specified limit.
    //!
    //!   Pre-requisites
    //!   - A valid [API_KEY] from Render Cloud.
    ServiceManager::list_services_with_status("suspended", "50")
        .await
        .unwrap();
}
