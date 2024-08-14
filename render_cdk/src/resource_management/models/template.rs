#![allow(missing_docs)]
#![allow(unused)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "autoDeploy")]
    pub auto_deploy: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "autoDeploy")]
    pub auto_deploy: Option<String>,

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "numInstances")]
    pub num_instances: Option<i32>,

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

#[cfg(test)]
mod model_tests {
    use super::*;

    #[test]
    fn test_template_model() {
        let service_details = ServiceDetails {
            region: Some("oregon".to_owned()),
            plan: Some("starter".to_owned()),
            build_command: Some("yarn".to_owned()),
            pre_deploy_command: None,
            headers: vec![],
            publish_path: Some("./".to_owned()),
            pull_request_previews_enabled: Some("yes".to_owned()),
            runtime: Some("node".to_owned()),
            num_instances: Some(1),
            routes: vec![],
            env_specific_details: Some(EnvSpecificDetails {
                build_command: Some("yarn".to_owned()),
                start_command: Some("npm start".to_owned()),
            }),
        };

        let image = Image {
            owner_id: "usr-a1b2c3d4".to_owned(),
            registry_credential_id: "33ef574274c011e4bea40242ac11001b".to_owned(),
            image_path: "/var/lib/docker".to_owned(),
        };

        let autoscaling = AutoScaling {
            min: 1,
            max: 2,
            criteria: Some(Criteria {
                cpu: Some(Cpu { percentage: 60 }),
                memory: Some(Memory { percentage: 60 }),
            }),
        };

        let template = Template {
            type_: "web_service".to_owned(),
            name: "test_web_service".to_owned(),
            repo: "https://github.com/<username>/<repo>".to_owned(),
            auto_deploy: Some("yes".to_owned()),
            branch: Some("master".to_owned()),
            image: Some(image),
            build_filter: None,
            root_dir: Some("./".to_owned()),
            env_vars: vec![],
            secret_files: vec![],
            service_details: Some(service_details),
            health_check_path: Some("".to_owned()),
            autoscaling: Some(autoscaling),
        };

        assert_eq!(template.type_, "web_service".to_owned());
        assert_eq!(template.name, "test_web_service".to_owned());
        assert_eq!(template.auto_deploy, Some("yes".to_owned()));
        assert_eq!(
            template.repo,
            "https://github.com/<username>/<repo>".to_owned()
        );
    }
}
