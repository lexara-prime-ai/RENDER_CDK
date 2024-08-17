// HTTP.
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

#[macro_export]
macro_rules! create_request {
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

#[macro_export]
macro_rules! create_request_with_body {
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
