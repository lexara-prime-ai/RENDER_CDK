#![allow(missing_docs)]
#![allow(unused)]
#![allow(non_snake_case)]
// [JSON] parsing.
extern crate serde;
extern crate serde_json;
use serde_json::Value;

// Idiomatic [ERROR] handling.
use anyhow::{Context, Error, Ok, Result};

// HTTP.
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

// [render_cdk] modules.
use crate::authentication::owner::Info;
use crate::authentication::owner::*;
use crate::environment_management::prelude::EnvironmentManager;
use crate::resource_management::models::postgres::PostgresConf;
use crate::resource_management::models::prelude::*;
use crate::resource_management::models::redis::RedisConf;
use crate::state_management::state::{Owner, State};
use crate::utils::config::Conf;
use crate::utils::request_manager;

// [DEBUG] utils.
use crate::logger::prelude::*;
use crate::utils::stringify::Stringify;
use crate::{
    create_delete_request, create_get_request, create_post_request, handle_response,
    handle_response_data, LOGGER,
};
use colored::Colorize;

// Predefined [CONSTANTS].
const BASE_URL: &str = "https://api.render.com/v1";

#[derive(Debug)]
pub struct ServiceManager;

pub trait ServiceManagerOperations {
    /// Querying services.
    /// List all services.
    fn list_all_services(
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// List all postgres instances.
    fn list_postgres_instances(
        include_replicas: bool,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// List all redis instances.
    fn find_redis_instance_by_name(
        name: &str,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// List the postgres instance matching the specified name.
    fn find_postgres_instance_by_name(
        name: &str,
        include_replicas: bool,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// List all postgres instances with the specified status.
    fn find_postgres_instance_with_status(
        status: &str,
        include_replicas: bool,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// List all services with the specified status.
    fn list_services_with_status(
        service_status: &str,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// Find a service by name and type.
    fn find_service_by_name_and_type(
        service_name: &str,
        service_type: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// List services that match specified region.
    fn find_service_by_region(
        service_region: &str,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// List services that match specified environment.
    fn find_service_by_environment(
        service_env: &str,
        limit: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// Creating services.
    /// Create and deploy a static site.
    fn create_service(
        deployment_config: Template,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// Deploy configuration.
    fn deploy_configuration(
        config_path: &str,
    ) -> impl std::future::Future<Output = Result<String, Error>> + Send;

    /// Deleting a service.
    fn delete_service(
        service_name: &str,
        service_type: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;

    /// Delete postgres instance.
    fn delete_postgres_instance(
        name: &str,
    ) -> impl std::future::Future<Output = Result<Value, Error>> + Send;
}

impl ServiceManagerOperations for ServiceManager {
    /// List all resources.
    async fn list_all_services(limit: &str) -> Result<Value, Error> {
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
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;

        // Validate <response> status.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            // Check if the response contains a list of services.
            if data.is_array() && data.as_array().unwrap().is_empty() {
                LOGGER!(
                    "<reponse> -> ",
                    "⚙️ :: No <services> found.",
                    LogLevel::WARN
                );
            } else {
                LOGGER!("<response> -> ", format!("{:#?}", data), LogLevel::SUCCESS);
            }

            Ok(data)
        } else {
            let result = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;
            let message = data["message"]
                .as_str()
                .unwrap_or("An error occured :: Process -> <list_all_services>");

            LOGGER!(
                "<response status> -> ",
                format!("{:#?}", message),
                LogLevel::CRITICAL
            );

            Err(anyhow::anyhow!("<Error>: {:#?}", data))
        }
    }

    async fn list_postgres_instances(include_replicas: bool, limit: &str) -> Result<Value, Error> {
        /*****************************************************
         *
            curl --request GET \
            --url 'https://api.render.com/v1/postgres?includeReplicas=true&limit=20' \
            --header 'Accept: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}",
            BASE_URL, "/postgres?includeReplicas=", include_replicas, "&limit=", limit
        );

        // [DEBUG] logs.
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;

        // Validate response.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            // Check if the response contains a list of databases.
            if data.is_array() && data.as_array().unwrap().is_empty() {
                LOGGER!(
                    "<response> ->",
                    "🛢 :: No <postgres> instances found.",
                    LogLevel::WARN
                );
            } else {
                LOGGER!("<response> ->", format!("{:#?}", data), LogLevel::SUCCESS);
            }

            Ok(data)
        } else {
            let result = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;
            let message = data["message"]
                .as_str()
                .unwrap_or("An error occured :: Process -> <list_postgres_instances>");

            LOGGER!(
                "<response status> -> ",
                format!("{:#?}", message),
                LogLevel::CRITICAL
            );

            Err(anyhow::anyhow!("<Error>: {:#?}", data))
        }
    }

    async fn find_postgres_instance_by_name(
        name: &str,
        include_replicas: bool,
        limit: &str,
    ) -> Result<Value, Error> {
        /*****************************************************
         *
            curl --request GET \
            --url 'https://api.render.com/v1/postgres?name=mydb&includeReplicas=true&limit=20' \
            --header 'Accept: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}{}{}",
            BASE_URL,
            "/postgres?name=",
            name,
            "&includeReplicas=",
            include_replicas,
            "&limit=",
            limit
        );

        // [DEBUG] logs.
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;

        // Validate response.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            // Check if response contains a list of databases.
            if data.is_array() && data.as_array().unwrap().is_empty() {
                LOGGER!(
                    "<reponse> -> ",
                    "🛢 :: No <postgres> instances found.",
                    LogLevel::WARN
                )
            } else {
                LOGGER!("<response> -> ", format!("{:#?}", data), LogLevel::SUCCESS);
            }

            Ok(data)
        } else {
            let result = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;
            let message = data["message"]
                .as_str()
                .unwrap_or("An error occured :: Process -> <find_postgres_instance_by_name>");

            LOGGER!(
                "<response status> -> ",
                format!("{:#?}", message),
                LogLevel::CRITICAL
            );

            Err(anyhow::anyhow!("<Error>: {:#?}", data))
        }
    }

    async fn find_postgres_instance_with_status(
        status: &str,
        include_replicas: bool,
        limit: &str,
    ) -> Result<Value, Error> {
        /*****************************************************
         *
            curl --request GET \
            --url  'https://api.render.com/v1/postgres?suspended=suspended&includeReplicas=true&limit=20' \
            --header 'Accept: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}{}{}",
            BASE_URL,
            "/postgres?suspended=",
            status,
            "&includeReplicas=",
            include_replicas,
            "&limit=",
            limit
        );

        // [DEBUG] logs.
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;

        // Validate response.
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&results)?;

            if data.is_array() && data.as_array().unwrap().is_empty() {
                LOGGER!(
                    "<reponse> -> ",
                    "🛢 :: No <postgres> instances found.",
                    LogLevel::WARN
                );
            } else {
                LOGGER!("<request> -> ", format!("{:#?}", data), LogLevel::SUCCESS);
            }

            Ok(data)
        } else {
            let result = response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;
            let message = data["message"]
                .as_str()
                .unwrap_or("An error occured :: Process -> ");

            LOGGER!(
                "<response status> -> ",
                format!("{:#?}", message),
                LogLevel::CRITICAL
            );

            Err(anyhow::anyhow!("<Error>: {:#?}", data))
        }
    }

    async fn find_redis_instance_by_name(name: &str, limit: &str) -> Result<Value, Error> {
        /*****************************************************
         *
            curl --request GET \
            --url  https://api.render.com/v1/redis?name=<instance_name>&limit=20' \
            --header 'Accept: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}",
            BASE_URL, "/redis?name=", name, "&limit=", limit
        );

        // [DEBUG] logs.
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;
        handle_response_data!(response, "<find_redis_instance_by_name>")
    }

    /// Finding all suspended services.
    /// Reqquired arguments: <service_status> i.e suspended/not_suspended.
    async fn list_services_with_status(service_status: &str, limit: &str) -> Result<Value, Error> {
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
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;
        handle_response_data!(response, "<list_services_with_status>")
    }

    /// Finding services by type.
    /// Reqquired arguments: <service_type>
    async fn find_service_by_name_and_type(
        service_name: &str,
        service_type: &str,
    ) -> Result<Value, Error> {
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
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;
        handle_response_data!(response, "<find_service_by_name_and_type>")
    }

    /// Finding services by region.
    async fn find_service_by_region(service_region: &str, limit: &str) -> Result<Value, Error> {
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
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;
        handle_response_data!(response, "<find_service_by_region>")
    }

    /// Filtering for environments.
    async fn find_service_by_environment(service_env: &str, limit: &str) -> Result<Value, Error> {
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
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);

        let response = create_get_request!(client, api_url, api_key)?;
        handle_response_data!(response, "<find_service_by_environment>")
    }

    /// Creating services.
    async fn create_service(deployment_config: Template) -> Result<Value, Error> {
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
        let payload = Base {
            type_: deployment_config.type_,
            name: deployment_config.name,
            repo: deployment_config.repo,
            owner_id: Info::get_owner_id().await,
            auto_deploy: deployment_config.auto_deploy,
            branch: deployment_config.branch,
            image: deployment_config.image,
            build_filter: deployment_config.build_filter,
            root_dir: deployment_config.root_dir,
            env_vars: deployment_config.env_vars,
            secret_files: deployment_config.secret_files,
            service_details: deployment_config.service_details,
            health_check_path: deployment_config.health_check_path,
            autoscaling: deployment_config.autoscaling,
        }
        .stringify();

        // [DEBUG] logs.
        LOGGER!("\nProcessing <request> -> ", &api_url, LogLevel::WARN);
        LOGGER!("[PAYLOAD] -> ", &payload, LogLevel::WARN);

        let response = create_post_request!(client, api_url, api_key, payload, "<create_service>")?;
        handle_response!(response, "<create_service>")
    }

    async fn deploy_configuration(config_path: &str) -> Result<String, Error> {
        let state = State::init().await;
        let client = state.CLIENT;
        let api_key = state.API_KEY;
        let CONFIG = Conf::read_configuration_file(config_path).unwrap();

        // Authorization.
        let owner_id = Info::get_owner_id().await;

        // [CONFIG] validation.
        let mut results = Vec::new();

        // [POSTGRES]
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
            .stringify();

            LOGGER!(
                "\n<request> :: Creating request -> ",
                &api_url,
                LogLevel::WARN
            );

            LOGGER!("[PAYLOAD] :: -> ", &payload, LogLevel::WARN);

            let response = create_post_request!(client, api_url, api_key, payload, "<postgres>")?;

            if response.status().is_success() {
                let result = response.text().await.context("Error parsing response.")?;

                let data: Value = serde_json::from_str(&result)?;
                let dashboard_url = data
                    .get("dashboardUrl")
                    .unwrap()
                    .as_str()
                    .expect("Failed to extract [dashboardUrl] from <response>.");

                LOGGER!(
                    "[POSTGRES] :: Deployment successful. -> ",
                    format!("{:#?}", dashboard_url),
                    LogLevel::SUCCESS
                );

                results.push(Ok(result));
            } else {
                let result = response
                    .text()
                    .await
                    .context("Error parsing response :: <deploy_configuration>")?;

                let data: Value = serde_json::from_str(&result)?;

                let message = data["message"]
                    .as_str()
                    .unwrap_or("An error occured :: Process -> <list_all_services>");

                LOGGER!(
                    "[POSTGRES] :: Deployment status :: -> ",
                    format!("{:#?}", message),
                    LogLevel::CRITICAL
                );

                results.push(Err(anyhow::anyhow!("<Error>: {:#?}", message)));
            }
        }

        // [REDIS]
        if CONFIG.redis.is_some() {
            let api_url = format!("{}{}", BASE_URL, "/redis");

            let payload = RedisConf {
                name: CONFIG.redis.clone().unwrap().name,
                ownerId: owner_id.clone(),
                plan: CONFIG.redis.clone().unwrap().plan,
                ipAllowList: Some(CONFIG.redis.clone().unwrap().cidrBlocks),
            }
            .stringify();

            LOGGER!(
                "\n<request> :: Creating request -> ",
                &api_url,
                LogLevel::WARN
            );

            LOGGER!("[PAYLOAD] :: -> ", &payload, LogLevel::WARN);

            let response = create_post_request!(client, api_url, api_key, payload, "<redis>")?;

            if response.status().is_success() {
                let result = response.text().await.context("Error parsing response.")?;

                let data: Value = serde_json::from_str(&result)?;

                LOGGER!(
                    "[REDIS] :: Deployment successful. -> ",
                    format!("{:#?}", data),
                    LogLevel::SUCCESS
                );
                results.push(Ok(result));
            } else {
                let result = response
                    .text()
                    .await
                    .context("Error parsing response :: <deploy_configuration>")?;

                let data: Value = serde_json::from_str(&result)?;

                let message = data["message"]
                    .as_str()
                    .unwrap_or("An error occured :: Process -> <list_all_services>");

                LOGGER!(
                    "[REDIS] :: Deployment status :: -> ",
                    format!("{:#?}", message),
                    LogLevel::CRITICAL
                );

                results.push(Err(anyhow::anyhow!("<Error>: {:#?}", message)));
            }
        }

        if results.is_empty() {
            LOGGER!(
                "[INFO] :: No configuration to process. -> ",
                "SKIPPED",
                LogLevel::WARN
            );
            Err(anyhow::anyhow!("No configuration to process."))
        } else {
            results
                .into_iter()
                .find(|r| r.is_err())
                .unwrap_or_else(|| Ok("All deployments successful".to_string()))
        }
    }

    /// Deleting services.
    async fn delete_service(service_name: &str, service_type: &str) -> Result<Value, Error> {
        /*****************************************************
         *
            curl --request DELETE \
             --url https://api.render.com/v1/services/serviceId \
             --header 'accept: application/json' \
             --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let service =
            ServiceManager::find_service_by_name_and_type(service_name, service_type).await?;

        // Retrieve <service_id>.
        let service_id = service[0]["service"]["id"].as_str();

        match service_id {
            Some(id) => {
                let client = State::init().await.CLIENT;
                let api_key = State::init().await.API_KEY;
                let service_url = format!("{}{}{}", BASE_URL, "/services/", id);

                // [DEBUG] logs.
                LOGGER!(
                    "\nProcessing <request> :: <delete> -> ",
                    &service_url,
                    LogLevel::WARN
                );

                let response = create_delete_request!(client, service_url, api_key)?;
                handle_response!(response, "<delete_service>")
            }
            None => Err(anyhow::anyhow!("Service Id not found.")),
        }
    }

    /// Deleting postgres instances.
    async fn delete_postgres_instance(name: &str) -> Result<Value, Error> {
        /*****************************************************
         *
            curl --request DELETE \
             --url https://api.render.com/v1/postgres/postgresId \
             --header 'accept: application/json' \
             --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        let postgres_instance =
            ServiceManager::find_postgres_instance_by_name(name, true, "100").await?;

        // Retrieve <postgres_id>.
        let postgres_id = postgres_instance[0]["postgres"]["id"].as_str();

        match postgres_id {
            Some(id) => {
                let client = State::init().await.CLIENT;
                let api_key = State::init().await.API_KEY;
                let postgres_url = format!("{}{}{}", BASE_URL, "/postgres/", id);

                // [DEBUG] logs.
                LOGGER!(
                    "\nProcessing <request> :: <delete> -> ",
                    &postgres_url,
                    LogLevel::WARN
                );

                let response = create_delete_request!(client, postgres_url, api_key)?;
                handle_response!(response, "<delete_postgres_instance>")
            }
            None => Err(anyhow::anyhow!("Postgres Id not found.")),
        }
    }
}
