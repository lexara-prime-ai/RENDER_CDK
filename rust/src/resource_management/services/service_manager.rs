#![allow(unused)]
use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::environment_management::prelude::EnvironmentManager;
use crate::state::state::State;

const BASE_URL: &str = "https://api.render.com/v1";

#[derive(Debug)]
pub struct ServiceManager;

impl ServiceManager {
    pub async fn list_all_services(limit: &str) -> Result<String, Error> {
        /*****************************************************
         *
            curl --request GET \
            --url 'https://api.render.com/v1/services?limit=20' \
            --header 'Accept: application/json' \
            --header 'Authorization: Bearer {{render_api_token_goes_here}}'

        *****************************************************************/

        //////////////////////////////
        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!("{}{}{}", BASE_URL, "/services?limit=", limit);

        //////////////////////////////
        ////// [DEBUG] logs. /////////
        //////////////////////////////
        // println!("[REQUEST] -> {}", api_url);
        // println!("[REQUEST] -> {}", api_key.clone());
        //////////////////////////////

        let response = client
            .get(api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error sending request.")?;

        //////////////////////////////
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            println!("{}", results);
            Ok(results)
        } else {
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }

    /// Finding services by type.
    /// Reqquired arguments: <service_type>
    pub async fn find_service_by_name_and_type(
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

        //////////////////////////////
        let client = State::init().await.CLIENT;
        let api_key = State::init().await.API_KEY;
        let api_url = format!(
            "{}{}{}{}{}",
            BASE_URL, "/services?name=", service_name, "&type=", service_type
        );

        //////////////////////////////
        ////// [DEBUG] logs. /////////
        //////////////////////////////
        // println!("[REQUEST] -> {}", api_url);
        // println!("[REQUEST] -> {}", api_key.clone());
        //////////////////////////////

        let response = client
            .get(api_url)
            .header("ACCEPT", "application/json")
            .header("AUTHORIZATION", format!("Bearer {}", api_key))
            .send()
            .await
            .context("Error sending request.")?;

        //////////////////////////////
        if response.status().is_success() {
            let results = response.text().await.context("Error parsing response.")?;
            println!("{}", results);
            Ok(results)
        } else {
            Err(anyhow::anyhow!(
                "Request failed with status: {}",
                response.status()
            ))
        }
    }
}
