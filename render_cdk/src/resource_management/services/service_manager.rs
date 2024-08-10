#![allow(missing_docs)]
#![allow(unused)]
// [JSON] parsing.
extern crate serde;
extern crate serde_json;
use serde_json::Value;

// Idiomatic [ERROR] handling.
use anyhow::{Context, Error, Ok, Result};

// HTTP.
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

// [render_cdk] modules.
use crate::environment_management::prelude::EnvironmentManager;
use crate::resource_management::models::template::Template;
use crate::state_management::state::State;
use crate::authentication::owner::Info;

// [DEBUG] utils.
use crate::logger::prelude::*;
use crate::LOGGER;
use colored::Colorize;

// Predefined [CONSTANTS].
const BASE_URL: &str = "https://api.render.com/v1";

#[derive(Debug)]
pub struct ServiceManager;

pub trait ServiceManagerOperations {
    /// Querying services.
    fn list_all_services(
        limit: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
    fn list_services_with_status(
        service_status: &str,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
    fn find_service_by_name_and_type(
        service_name: &str,
        service_type: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
    fn find_service_by_region(
        service_region: &str,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
    fn find_service_by_environment(
        service_env: &str,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

    /// Creating services.
    fn create_service(
        deployment_config: Template,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;
}

impl ServiceManagerOperations for ServiceManager {
    /// List all resources.
    async fn list_all_services(limit: &str) -> Result<String, Error> {
        /*****************************************************
         *
            curl --request GET \
            --url 'https://api.render.com/v1/services?limit=20' \
            --header 'Accept: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!("{}{}{}", BASE_URL, "/services?limit=", limit);

        // [DEBUG] logs.
        LOGGER!("Processing [REQUEST] -> ", &api_url, LogLevel::WARN);
        // LOGGER!("Processing [REQUEST] -> ", &api_key, LogLevel::WARN);

        let response = client
            .get(api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error sending request.")?;

        // Validate [RESPONSE] status.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            LOGGER!("[RESPONSE] -> ", format!("{:#?}", data), LogLevel::SUCCESS);

            Ok(results)
        } else {
            LOGGER!("[RESPONSE STATUS] -> ", "FAILED", LogLevel::CRITICAL);
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }

    /// Finding all suspended services.
    /// Reqquired arguments: <service_status> i.e suspended/not_suspended.
    async fn list_services_with_status(service_status: &str, limit: &str) -> Result<String, Error> {
        /*****************************************************
         *
            curl --request GET \
            --url  'https://api.render.com/v1/services?suspended=suspended&limit=20' \
            --header 'Accept: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}",
            BASE_URL, "/services?suspended=", service_status, "&limit=", limit
        );

        // [DEBUG] logs.
        LOGGER!("Processing [REQUEST] -> ", &api_url, LogLevel::WARN);
        // LOGGER!("Processing [REQUEST] -> ", &api_key, LogLevel::WARN);

        let response = client
            .get(api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error sending request.")?;

        // Validate response status.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            LOGGER!("[RESPONSE] -> ", format!("{:#?}", data), LogLevel::SUCCESS);

            Ok(results)
        } else {
            LOGGER!("[RESPONSE STATUS] -> ", "FAILED", LogLevel::CRITICAL);
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }

    /// Finding services by type.
    /// Reqquired arguments: <service_type>
    async fn find_service_by_name_and_type(
        service_name: &str,
        service_type: &str,
    ) -> Result<String, Error> {
        /*****************************************************
         *
            curl --request GET \
                --url 'https://api.render.com/v1/services?name=test-service&type=static_site' \
                --header 'Accept: application/json' \
                --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}",
            BASE_URL, "/services?name=", service_name, "&type=", service_type
        );

        // [DEBUG] logs.
        LOGGER!("Processing [REQUEST] -> ", &api_url, LogLevel::WARN);
        // LOGGER!("Processing [REQUEST] -> ", &api_key, LogLevel::WARN);

        let response = client
            .get(api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error sending request.")?;

        // Validate response status.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            LOGGER!("[RESPONSE] -> ", format!("{:#?}", data), LogLevel::SUCCESS);

            Ok(results)
        } else {
            LOGGER!("[RESPONSE STATUS] -> ", "FAILED", LogLevel::CRITICAL);
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }

    /// Finding services by region.
    async fn find_service_by_region(service_region: &str, limit: &str) -> Result<String, Error> {
        /*****************************************************
         *
            curl --request GET \
                --url 'https://api.render.com/v1/services?region=oregon&limit=20' \
                --header 'Accept: application/json' \
                --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}",
            BASE_URL, "/services?region=", service_region, "&limit=", limit
        );

        // [DEBUG] logs.
        LOGGER!("Processing [REQUEST] -> ", &api_url, LogLevel::WARN);
        // LOGGER!("Processing [REQUEST] -> ", &api_key, LogLevel::WARN);

        let response = client
            .get(api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error sending request.")?;

        // Validate response status.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            LOGGER!("[RESPONSE] -> ", format!("{:#?}", data), LogLevel::SUCCESS);

            Ok(results)
        } else {
            LOGGER!("[RESPONSE STATUS] -> ", "FAILED", LogLevel::CRITICAL);
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }

    /// Filtering for environments.
    async fn find_service_by_environment(service_env: &str, limit: &str) -> Result<String, Error> {
        /*****************************************************
         *
            curl --request GET \
                --url 'https://api.render.com/v1/services?env=docker&limit=20' \
                --header 'Accept: application/json' \
                --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}",
            BASE_URL, "/services?env=", service_env, "&limit=", limit
        );

        // [DEBUG] logs.
        LOGGER!("Processing [REQUEST] -> ", &api_url, LogLevel::WARN);
        // LOGGER!("Processing [REQUEST] -> ", &api_key, LogLevel::WARN);

        let response = client
            .get(api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error sending request.")?;

        // Validate response status.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            LOGGER!("[RESPONSE] -> ", format!("{:#?}", data), LogLevel::SUCCESS);

            Ok(results)
        } else {
            LOGGER!("[RESPONSE STATUS] -> ", "FAILED", LogLevel::CRITICAL);
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }

    /// Creating services.
    async fn create_service(deployment_config: Template) -> Result<String, Error> {
        /// Currently supported - Github(https://github.com/username/reponame.git)
        /******************************************************
         *
            curl --request POST \
                --url https://api.render.com/v1/services \
                --header 'Accept: application/json' \
                --header 'Content-Type: application/json' \
                --header 'Authorization: Bearer {{render_api_token_goes_here}}'
                --data '
                {
                    "type": "static_site",
                    "autoDeploy": "yes",
                    "serviceDetails": {
                        "pullRequestPreviewsEnabled": "no"
                },
                    "name": "test",
                    "ownerId": "test",
                    "repo": "httpe",
                    "rootDir": "./",
                    "envVars": [
                        {
                        "key": "EXAMPLE",
                        "value": "EXAMPLE"
                        }
                    ]
                }'

        **************************************************************/
        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!("{}{}", BASE_URL, "/services");
        // let payload = serde_json::to_string_pretty(&deployment_config).unwrap();
        let payload = Template {
            type_: deployment_config.type_,
            name: deployment_config.name,
            owner_id: Info::get_owner_id().await,
        }

        // [DEBUG] logs.
        LOGGER!("Processing [REQUEST] -> ", &api_url, LogLevel::WARN);
        // LOGGER!("Processing [REQUEST] -> ", &api_key, LogLevel::WARN);
        LOGGER!("[PAYLOAD] -> ", &payload, LogLevel::WARN);

        let response = client
            .post(api_url)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .body(payload)
            .send()
            .await
            .context("Error processing request.")?;

        // Validate response status.
        if response.status().is_success() {
            let result = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;

            LOGGER!("[RESPONSE] -> ", format!("{:#?}", data), LogLevel::SUCCESS);

            Ok(result)
        } else {
            LOGGER!("[RESPONSE STATUS] -> ", "FAILED", LogLevel::CRITICAL);
            Err(anyhow::anyhow!(
                "Request failed with status: {:?}",
                response
            ))
        }
    }
}
