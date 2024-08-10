#![allow(missing_docs)]
// [JSON] parsing.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Template {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    pub repo: String,
    #[serde(rename = "autoDeploy")]
    pub auto_deploy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(rename = "buildFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_filter: Option<BuildFilter>,
    #[serde(rename = "rootDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "envVars")]
    pub env_vars: Vec<EnvVar>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "secretFiles")]
    pub secret_files: Vec<SecretFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "serviceDetails")]
    pub service_details: Option<ServiceDetails>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "registryCredentialId")]
    pub registry_credential_id: String,
    #[serde(rename = "imagePath")]
    pub image_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildFilter {
    pub paths: Vec<String>,
    #[serde(rename = "ignoredPaths")]
    pub ignored_paths: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvVar {
    pub key: String,
    pub value: Option<String>,
    #[serde(rename = "generateValue")]
    pub generate_value: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecretFile {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buildCommand")]
    pub build_command: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<Header>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publishPath")]
    pub publish_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pullRequestPreviewsEnabled")]
    pub pull_request_previews_enabled: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<Route>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Header {
    pub path: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Route {
    #[serde(rename = "type")]
    pub type_: String,
    pub source: String,
    pub destination: String,
    pub priority: u32,
}
