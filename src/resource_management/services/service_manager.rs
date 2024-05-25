use anyhow::{Context, Result};
use reqwest::header::AUTHORIZATION;

const BASE_URL: &str = "https://api.render.com/v1";

pub struct ServiceManager;

impl ServiceManager {
    pub fn list_all_services(api_key: String) {
        let client = reqwest::Client::new();

        let api_url = format!("{}{}", BASE_URL, "services?limit=20");

        println!("{}", api_url);
    }
}
