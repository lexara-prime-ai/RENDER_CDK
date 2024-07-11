#![allow(unused)]
use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self};
use std::sync::Arc;

use crate::environment_management::prelude::*;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct State {
    pub CLIENT: Arc<reqwest::Client>,
    pub API_KEY: String,
}

impl State {
    pub async fn init() -> Self {
        let client = reqwest::Client::new();
        let api_key = EnvironmentManager::retrieve_api_key().API_KEY;

        /// This method returns an instance of the applications current [State].
        Self {
            CLIENT: client.into(),
            API_KEY: api_key.trim().to_string(),
        }
    }
}
