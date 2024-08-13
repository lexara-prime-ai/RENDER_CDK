#![allow(missing_docs)]
// [JSON] parsing.
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Base {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub repo: String,

    #[serde(rename = "ownerId")]
    pub owner_id: String,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "healthCheckPath")]
    pub health_check_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscaling: Option<AutoScaling>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Template {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "healthCheckPath")]
    pub health_check_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscaling: Option<AutoScaling>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "ownerId")]
    pub owner_id: String,

    #[serde(rename = "registryCredentialId")]
    pub registry_credential_id: String,

    #[serde(rename = "imagePath")]
    pub image_path: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct BuildFilter {
    pub paths: Vec<String>,

    #[serde(rename = "ignoredPaths")]
    pub ignored_paths: Vec<String>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct EnvVar {
    pub key: String,
    pub value: Option<String>,

    #[serde(rename = "generateValue")]
    pub generate_value: bool,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SecretFile {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ServiceDetails {
    pub region: String,
    pub plan: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buildCommand")]
    pub build_command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preDeployCommand")]
    pub pre_deploy_command: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub headers: Vec<Header>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publishPath")]
    pub publish_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pullRequestPreviewsEnabled")]
    pub pull_request_previews_enabled: Option<String>,

    pub runtime: String,

    #[serde(rename = "numInstances")]
    pub num_instances: i32,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<Route>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "envSpecificDetails")]
    pub env_specific_details: Option<EnvSpecificDetails>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct EnvSpecificDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buildCommand")]
    pub build_command: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "startCommand")]
    pub start_command: Option<String>,
}

// Autoscaling properties.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct AutoScaling {
    min: u32,
    max: u32,
    criteria: Option<Criteria>,
}

// Autoscaling criteria.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Criteria {
    pub cpu: Option<Cpu>,
    pub memory: Option<Memory>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Cpu {
    pub percentage: u32,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Memory {
    pub percentage: u32,
}

// Additional configuration i.e header, route etc.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Header {
    pub path: String,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Route {
    #[serde(rename = "type")]
    pub type_: String,
    pub source: String,
    pub destination: String,
    pub priority: u32,
}
