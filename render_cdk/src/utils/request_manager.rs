// [JSON] parsing.
use serde_json::Value;

// Idiomatic [ERROR] handling.
use anyhow::{Context, Error, Ok, Result};

// HTTP.
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

// [DEBUG] utils.
use crate::logger::prelude::*;
use crate::utils::stringify::Stringify;
use crate::{create_get_request, create_post_request, LOGGER};
use colored::Colorize;

/// Creates and sends a `GET` request using the provided HTTP client.
///
/// # Parameters
///
/// * `$client`: The HTTP client instance used to send the request.
/// * `$api_url`: The URL to which the request is sent.
/// * `$api_key`: The API key for authorization, which is used as a Bearer token.
///
/// # Example
///
/// ```ignore
/// let response = create_get_request!(client, "https://api.example.com/data", api_key).await?;
/// ```
///
/// This macro sets the required headers (`ACCEPT` and `AUTHORIZATION`) and sends a GET request.
#[macro_export]
macro_rules! create_get_request {
    ($client: expr, $api_url: expr, $api_key: expr) => {
        $client
            .get($api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", $api_key))
            .send()
            .await
            .context("Error sending request.")
    };
}

/// Creates and sends a `POST` request with a payload using the provided HTTP client.
///
/// # Parameters
///
/// * `$client`: The HTTP client instance used to send the request.
/// * `$api_url`: The URL to which the request is sent.
/// * `$api_key`: The API key for authorization, which is used as a Bearer token.
/// * `$payload`: The payload data to be sent with the request.
/// * `$process`: A string or identifier used to identify the process, for logging and error context.
///
/// # Example
///
/// ```ignore
/// let response = create_post_request!(client, "https://api.example.com/data", api_key, payload, "create_service").await?;
/// ```
///
/// This macro sets the required headers (`ACCEPT`, `CONTENT_TYPE`, and `AUTHORIZATION`), adds the provided payload, and sends a POST request.
#[macro_export]
macro_rules! create_post_request {
    ($client: expr, $api_url: expr, $api_key: expr, $payload: expr, $process: expr) => {
        $client
            .post($api_url)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", $api_key))
            .body($payload)
            .send()
            .await
            .context(format!("Error sending request :: {:?}", $process))
    };
}

/// Creates and sends a `DELETE` request using the provided HTTP client.
///
/// # Parameters
///
/// * `$client`: The HTTP client instance used to send the request.
/// * `$api_url`: The URL to which the request is sent.
/// * `$api_key`: The API key for authorization, which is used as a Bearer token.
///
/// # Example
///
/// ```ignore
/// let response = create_delete_request!(client, "https://api.example.com/data", api_key).await?;
/// ```
///
/// This macro sets the required headers (`ACCEPT` and `AUTHORIZATION`) and sends a DELETE request.
#[macro_export]
macro_rules! create_delete_request {
    ($client: expr, $api_url: expr, $api_key: expr) => {
        $client
            .delete($api_url)
            .header(ACCEPT, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", $api_key))
            .send()
            .await
            .context("Error sending request.")
    };
}

/// This macro handles the response from an HTTP request, logging the response or any errors.
///
/// # Parameters
///
/// * `$response`: The HTTP response to be processed. It is expected to be an asynchronous result of the HTTP request.
/// * `$process`: A string slice representing the process name or a message to be logged in case of an error.
///
/// # Example
///
/// ```ignore
/// let response = some_function_that_returns_response().await;
/// let result = handle_response!(response, "<create_service>");
/// ```
///
/// If the response status is successful, the result is logged and returned as JSON data. Otherwise, an error is logged
/// with a custom message including the provided process name.
#[macro_export]
macro_rules! handle_response {
    ($response: expr, $process: expr) => {
        if $response.status().is_success() {
            let result = $response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;

            LOGGER!("<response> -> ", format!("{:#?}", data), LogLevel::SUCCESS);

            Ok(data)
        } else {
            let result = $response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;
            let message = data["message"]
                .as_str()
                .unwrap_or(concat!("An error occurred :: Process -> ", $process));

            LOGGER!(
                "<response status> -> ",
                format!("{:#?}", message),
                LogLevel::CRITICAL
            );

            Err(anyhow::anyhow!("<Error>: {:#?}", data))
        }
    };
}

#[macro_export]
macro_rules! handle_response_data {
    ($response: expr, $process: expr) => {
        if $response.status().is_success() {
            let result = $response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;

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
            let result = $response.text().await.context("Error parsing response.")?;
            let data: Value = serde_json::from_str(&result)?;
            let message = data["message"]
                .as_str()
                .unwrap_or(concat!("An error occurred :: Process -> ", $process));

            LOGGER!(
                "<response status> -> ",
                format!("{:#?}", message),
                LogLevel::CRITICAL
            );

            Err(anyhow::anyhow!("<Error>: {:#?}", data))
        }
    };
}
