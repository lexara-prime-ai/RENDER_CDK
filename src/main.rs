use render_cdk::environment_management::prelude::*;
use render_cdk::resource_management::prelude::*;

fn main() {
    let api_token = EnvironmentManager::retrieve_api_key().API_KEY;
    println!("{}", api_token);

    ServiceManager::list_all_services("api_key".to_string(), "20");
}
