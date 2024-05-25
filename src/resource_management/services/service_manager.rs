#![allow(unused)]
use anyhow::{Context, Result};
use reqwest::header::AUTHORIZATION;

const BASE_URL: &str = "https://api.render.com/v1";

#[derive(Debug)]
pub struct ServiceManager;

impl ServiceManager {
    pub fn list_all_services(api_key: String, limit: &str) -> String {
        let client = reqwest::Client::new();

        let api_url = format!("{}{}{}", BASE_URL, "/services?limit=", limit);

        println!("[REQUEST] -> {}", api_url);

        api_url
    }
}
