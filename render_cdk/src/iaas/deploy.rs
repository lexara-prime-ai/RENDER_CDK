#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(unused)]

use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use super::config::Conf;
use super::models::postgres::PostgresConf;
use super::models::redis::RedisConf;
use crate::authentication::owner::*;
use crate::state_management::state::{Owner, State};

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
        config_path: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}

impl DeploymentOperations for Deploy {
    async fn deploy_configuration(config_path: &str) -> Result<String, Error> {
        let state = State::init().await;
        let client = state.CLIENT;
        let api_key = state.API_KEY;
        let CONFIG = Conf::read_configuration_file(config_path).unwrap();

        LOGGER::INFO(
            "Retrieving [CONFIG] -> ",
            &CONFIG.CONVERT_TO_JSON_STRING(),
            LogLevel::WARN,
        );

        // Authorization.
        let owner_id = Info::get_owner_id().await;

        // [CONFIG] validation.
        let mut results = Vec::new();

        if CONFIG.database.is_some() {
            let api_url = format!("{}{}", BASE_URL, "/postgres");

            let payload = PostgresConf {
                databaseName: CONFIG.database.clone().unwrap().databaseName,
                databaseUser: CONFIG.database.clone().unwrap().databaseUser,
                enableHighAvailability: CONFIG.database.clone().unwrap().enableHighAvailability,
                plan: CONFIG.database.clone().unwrap().plan,
                version: CONFIG.database.clone().unwrap().version,
                name: CONFIG.database.clone().unwrap().name,
                ownerId: owner_id.clone(),
                ipAllowList: Some(CONFIG.database.clone().unwrap().cidrBlocks),
            }
            .CONVERT_TO_JSON_STRING();

            LOGGER::INFO(
                "[REQUEST] :: Creating request -> ",
                &api_url,
                LogLevel::WARN,
            );

            LOGGER::INFO(
                "[PAYLOAD] :: Creating request -> ",
                &payload,
                LogLevel::WARN,
            );

            let response = client
                .post(&api_url)
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", api_key))
                .body(payload.clone())
                .send()
                .await
                .context("Config :: [POSTGRES] -> Error processing request.")?;

            if response.status().is_success() {
                let result = response.text().await.context("Error parsing response.")?;
                LOGGER::INFO(
                    "[POSTGRES] :: Deployment successful. -> ",
                    &result.CONVERT_TO_JSON_STRING(),
                    LogLevel::SUCCESS,
                );
                results.push(Ok(result));
            } else {
                LOGGER::INFO(
                    "[POSTGRES] :: Deployment failed. -> ",
                    "FAILED",
                    LogLevel::CRITICAL,
                );
                results.push(Err(anyhow::anyhow!(
                    "Request failed with status: {:?}",
                    response
                )));
            }
        }

        if CONFIG.redis.is_some() {
            let api_url = format!("{}{}", BASE_URL, "/redis");

            let payload = RedisConf {
                name: CONFIG.redis.clone().unwrap().name,
                ownerId: owner_id.clone(),
                plan: CONFIG.redis.clone().unwrap().plan,
                ipAllowList: Some(CONFIG.redis.clone().unwrap().cidrBlocks),
            }
            .CONVERT_TO_JSON_STRING();

            LOGGER::INFO(
                "[REQUEST] :: Creating request -> ",
                &api_url,
                LogLevel::WARN,
            );

            LOGGER::INFO(
                "[PAYLOAD] :: Creating request -> ",
                &payload,
                LogLevel::WARN,
            );

            let response = client
                .post(&api_url)
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", api_key))
                .body(payload.clone())
                .send()
                .await
                .context("Config :: [REDIS] -> Error processing request.")?;

            if response.status().is_success() {
                let result = response.text().await.context("Error parsing response.")?;
                LOGGER::INFO(
                    "[REDIS] :: Deployment successful. -> ",
                    &result.CONVERT_TO_JSON_STRING(),
                    LogLevel::SUCCESS,
                );
                results.push(Ok(result));
            } else {
                LOGGER::INFO(
                    "[REDIS] :: Deployment failed. -> ",
                    "FAILED",
                    LogLevel::CRITICAL,
                );
                results.push(Err(anyhow::anyhow!(
                    "Request failed with status: {:?}",
                    response
                )));
            }
        }

        if results.is_empty() {
            LOGGER::INFO(
                "[INFO] :: No configuration to process. -> ",
                "SKIPPED",
                LogLevel::WARN,
            );
            Err(anyhow::anyhow!("No configuration to process."))
        } else {
            results
                .into_iter()
                .find(|r| r.is_err())
                .unwrap_or_else(|| Ok("All deployments successful".to_string()))
        }
    }
}
