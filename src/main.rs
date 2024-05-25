use render_cdk::environment_management::prelude::*;

fn main() {
    let api_token = EnvironmentManager::retrieve_api_key().API_KEY;
    println!("{}", api_token);
}
