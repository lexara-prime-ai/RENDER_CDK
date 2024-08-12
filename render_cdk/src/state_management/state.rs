#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(unused)]
// [JSON] parsing.
use serde::{Deserialize, Serialize};

// Idiomatic [ERROR] handling.
use anyhow::{Context, Error, Ok, Result};

// HTTP.
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self};

use std::sync::Arc;

// [render_cdk] modules.
use crate::environment_management::prelude::*;

// Predefined [CONSTANTS].
const BASE_URL: &str = "https://api.render.com/v1";

///////////////////
///// [State] ////
///////////////////
#[derive(Debug, Clone)]
pub struct State {
    pub CLIENT: Arc<reqwest::Client>,
    pub API_KEY: String,
}

///////////////////
///// [Owner] ////
///////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Owner {
    pub id: String,
    pub name: String,
    pub email: String,
    pub twoFactorAuthEnabled: bool,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OwnerResponse {
    pub owner: Owner,
    pub cursor: String,
}

impl State {
    /// This method returns an instance of the applications current [State].
    pub async fn init() -> Self {
        let client = reqwest::Client::new();
        let api_key = EnvironmentManager::retrieve_env_config().API_KEY;

        Self {
            CLIENT: client.into(),
            API_KEY: api_key,
        }
    }
}

impl Owner {
    pub async fn list_authorized_users(
        email: &str,
        limit: &str,
    ) -> Result<Vec<OwnerResponse>, Error> {
        let client = State::init().await.CLIENT;
        let api_key = format!("Bearer {}", State::init().await.API_KEY);
        let api_url = format!("{}{}{}", BASE_URL, "/owners?limit=", limit);

        let response = client
            .get(api_url)
            .header("ACCEPT", "application/json")
            .header("AUTHORIZATION", api_key)
            .send()
            .await
            .context("Error sending request.")?;

        if response.status().is_success() {
            let authorized_users = response
                .json::<Vec<OwnerResponse>>()
                .await
                .context("Error parsing response.")?;

            // Filter for the user.
            let filtered_owners: Vec<OwnerResponse> = authorized_users
                .into_iter()
                .filter(|result| result.owner.email == email)
                .collect();

            /*
                Expected <response>.

                [
                    {
                        "owner": {
                        "id": "string",
                        "name": "string",
                        "email": "string",
                        "twoFactorAuthEnabled": true,
                        "type": "user"
                        },
                        "cursor": "string"
                    }
                ]
            */

            Ok(filtered_owners)
        } else {
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;

    #[tokio::test]
    async fn test_list_authorized_users() {
        let owner_credentials = EnvironmentManager::retrieve_env_config().OWNER_CREDENTIALS;
        let result = Owner::list_authorized_users(&owner_credentials, "100").await;

        // The result should be Ok().
        assert!(result.is_ok());
    }
}
