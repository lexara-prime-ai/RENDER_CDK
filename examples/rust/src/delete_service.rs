use render_cdk::resource_management::prelude::*;
use render_cdk::resource_management::models::template::*;
use tokio::main;
use tokio::time::{sleep, Duration};

#[main]
async fn main() {
    //! This example demostrates how to create
    //!   a service and delete it.
    //!
    //!   Pre-requisites
    //!   - A valid [API_KEY] from Render Cloud.
    let deployment_config = Static {
        type_: "static_site".to_owned(),
        name: "test_deployment".to_owned(),
        repo: "https://github.com/lexara-prime-ai/SAMPLE_STATIC_SITE".to_owned(),
        auto_deploy: "yes".to_owned(), // By default, Render automatically deploys your service whenever you update its code or configuration.
        root_dir: Some("./public".to_owned()),
        service_details: Some(ServiceDetails {
            build_command: None, // Render runs this command to build your app before each deploy e.g npm run build, yarn build.
            publish_path: Some("./".to_owned()), // This will translate to /public/
            pull_request_previews_enabled: Some("yes".to_owned()),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Create and Deploy the service.
    ServiceManager::create_service(deployment_config)
        .await
        .unwrap();

    // Wait until the service is deployed.
    println!("\nWaiting for deployment...");
    sleep(Duration::from_secs(60)).await;

    // Deleting services.
    let _ = ServiceManager::delete_service("test_deployment", "static").await;
}