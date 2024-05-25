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
