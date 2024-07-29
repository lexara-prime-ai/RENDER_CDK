#![allow(missing_docs)]
#![allow(unused)]

use dotenvy::dotenv;

#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct EnvironmentManager {
    pub API_KEY: String,
    pub OWNER_CREDENTIALS: String,
}

impl EnvironmentManager {
    pub fn retrieve_env_config() -> Self {
        dotenv().ok();

        let api_key = std::env::var_os("API_KEY").expect("[API_KEY] must be set.");
        let owner_credentials =
            std::env::var_os("OWNER_CREDENTIALS").expect("[OWNER_CREDENTIALS] must be set.");

        Self {
            API_KEY: api_key.into_string().unwrap(),
            OWNER_CREDENTIALS: owner_credentials.into_string().unwrap(),
        }
    }
}
