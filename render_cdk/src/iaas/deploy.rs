#![allow(non_snake_case)]
// #![allow(unused)]
use crate::state_management::prelude::*;

use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use super::config::Conf;
use super::db::DatabaseConf;
use super::redis::RedisConf;
use crate::state_management::state::State;

// [DEBUG] utils.
use crate::logger::prelude::*;
use crate::utils::stringify::Stringify;

const BASE_URL: &str = "https://api.render.com/v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deploy {
    config: Conf,
}

pub trait DeploymentOperations {
    fn deploy_configuration(
        config: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}

impl DeploymentOperations for Deploy {
    async fn deploy_configuration(config_path: &str) -> Result<String, Error> {
        let config = Conf::read_configuration_file(config_path).unwrap();

        // -> Skip processing.
        if config.database.is_none() {
            ////////////////////////
            // Debug logs.
            ///////////////////////
            LOGGER::INFO(
                "\n -> Skipping [postgres] [CONFIG]\n",
                "[Reason] -> No database configuration provided...",
                LogLevel::SUCCESS,
            );
        }

        // -> Skip processing.
        if config.redis.is_none() {
            ////////////////////////
            // Debug logs.
            ///////////////////////
            LOGGER::INFO(
                "\n -> Skipping [redis] [CONFIG]\n",
                "[Reason] -> No redis configuration provided...",
                LogLevel::SUCCESS,
            );
        }

        /*****************************************************
         *
            curl --request GET \
            --url 'https://api.render.com/v1/postgres' \
            --header 'Accept: application/json' \
            --header 'Content-Type: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'
            --data '{
                "databaseName": "randomly generated",
                "databaseUser": "randomly generated",
                "enableHighAvailability": false,
                "plan": "free",
                "version": "11"
            }'

        *****************************************************************/

        //////////////////////////////
        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!("{}{}", BASE_URL, "/postgres");

        //////////////////////////////
        ////// [DEBUG] logs. /////////
        //////////////////////////////
        LOGGER::INFO("Processing [REQUEST] -> ", &api_url, LogLevel::WARN);
        // LOGGER::INFO("Processing [REQUEST] -> ", &api_key, LogLevel::WARN);
        //////////////////////////////

        // Process retrieved deployment configuration.
        if config.database.is_some() {
            ////////////////////////
            // Debug logs.
            ///////////////////////
            LOGGER::INFO(
                "\n -> Processing configuration...\n\n",
                &config.CONVERT_TO_JSON_STRING(),
                LogLevel::WARN,
            );

            let response = client
                .post(api_url)
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", api_key))
                .body(config.database.CONVERT_TO_JSON_STRING())
                .send()
                .await
                .context("<deploy.rs> -> Error processing request.")?;

            ////////////////////////////
            if response.status().is_success() {
                let result = response
                    .text()
                    .await
                    .context("<deploy.rs> -> Error parsing response.")?;
                LOGGER::INFO("[RESPONSE]", &result, LogLevel::SUCCESS);
                Ok(result)
            } else {
                LOGGER::INFO("[RESPONSE STATUS]", "FAILED", LogLevel::CRITICAL);
                Err(anyhow::anyhow!(
                    "Request failed with status: {:?}",
                    response
                ))
            }
        } else {
            ////////////////////////
            // Debug logs.
            ///////////////////////
            LOGGER::INFO(
                "\n -> No deployment configurations found...\n",
                "Config. [EMPTY]",
                LogLevel::WARN,
            );
            Err(anyhow::anyhow!("No deployment configurations found."))
        }
    }
}
