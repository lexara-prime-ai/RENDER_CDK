#![allow(non_snake_case)]
#![allow(unused)]

use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use super::config::Conf;
use super::models::postgres::PostgresConf;
use super::storage::IpAllowList;
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

        /////////////////////////
        //// Authorization
        ////////////////////////

        // To do -> Store and retrieve all credentials from Garage Object Storage e.g emails, passwords, api keys etc.
        let owner_id = Info::get_owner_id().await.OWNER_ID;

        ////////////////////////////
        //// [CONFIG] validation.
        ///////////////////////////
        if CONFIG.database.is_some() {
            let api_url = format!("{}{}", BASE_URL, "/postgres");

            //////////////////////////
            //// Initialize [PAYLOAD]
            /*
            * Payload struct.

               pub struct PostgresConf {
                   pub databaseName: Option<String>,
                   pub databaseUser: Option<String>,
                   pub enableHighAvailability: bool,
                   pub plan: String,
                   pub version: String,
                   pub name: String,
                   pub ownerId: String,
                   pub ipAllowList: Option<IpAllowList>
               }
            */

            let payload = PostgresConf {
                databaseName: CONFIG.database.clone().unwrap().databaseName,
                databaseUser: CONFIG.database.clone().unwrap().databaseUser,
                enableHighAvailability: CONFIG.database.clone().unwrap().enableHighAvailability,
                plan: CONFIG.database.clone().unwrap().plan,
                version: CONFIG.database.clone().unwrap().version,
                name: CONFIG.database.clone().unwrap().name,
                ownerId: owner_id,
                ipAllowList: Some(IpAllowList {
                    cidrBlock: CONFIG.database.clone().unwrap().cidrBlock,
                    description: CONFIG.database.clone().unwrap().accessLevelDescription,
                }),
            }
            .CONVERT_TO_JSON_STRING();
            //////////////////////////

            //////////////////////
            //// [DEBUG] logs.
            //////////////////////
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
                .post(api_url)
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
                Ok(result)
            } else {
                println!("{:?}", payload);
                LOGGER::INFO(
                    "[POSTGRES] :: Deployment failed. -> ",
                    "FAILED",
                    LogLevel::CRITICAL,
                );
                Err(anyhow::anyhow!(
                    "Request failed with status: {:?}",
                    response
                ))
            }
        } else if CONFIG.redis.is_some() {
            let api_url = format!("{}{}", BASE_URL, "/redis");
            let payload = serde_json::to_string_pretty(&CONFIG.redis).unwrap();

            //////////////////////
            //// [DEBUG] logs.
            //////////////////////
            LOGGER::INFO(
                "[REQUEST] :: Creating request -> ",
                &api_url,
                LogLevel::WARN,
            );

            let response = client
                .post(api_url)
                .header(ACCEPT, "application/json")
                .header(CONTENT_TYPE, "application/json")
                .header(AUTHORIZATION, format!("Bearer {}", api_key))
                .body(payload)
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
                Ok(result)
            } else {
                LOGGER::INFO(
                    "[REDIS] :: Deployment failed. -> ",
                    "FAILED",
                    LogLevel::CRITICAL,
                );
                Err(anyhow::anyhow!(
                    "Request failed with status: {:?}",
                    response
                ))
            }
        } else {
            LOGGER::INFO(
                "[INFO] :: No configuration to process. -> ",
                "SKIPPED",
                LogLevel::WARN,
            );
            Err(anyhow::anyhow!("No configuration to process."))
        }
    }
}
