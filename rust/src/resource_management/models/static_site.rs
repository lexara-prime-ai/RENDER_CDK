use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StaticSite {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub owner_id: String,
    pub repo: String,
    pub auto_deploy: bool,
    pub branch: String,
    pub image: Image,
    pub build_filter: BuildFilter,
    pub root_dir: String,
    pub env_vars: Vec<EnvVar>,
    pub secret_files: Vec<SecretFile>,
    pub service_details: ServiceDetails,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub owner_id: String,
    pub registry_credential_id: String,
    pub image_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildFilter {
    pub paths: Vec<String>,
    pub ignored_paths: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvVar {
    pub key: String,
    pub value: Option<String>,
    pub generate_value: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecretFile {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceDetails {
    pub build_command: String,
    pub headers: Vec<Header>,
    pub publish_path: String,
    pub pull_request_previews_enabled: bool,
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
    pub type_: String,
    pub source: String,
    pub destination: String,
    pub priority: u32,
}

// fn main() {
//     // Example instantiation
//     let site = StaticSite {
//         type_: "static_site".to_string(),
//         name: "My Static Site".to_string(),
//         owner_id: "owner123".to_string(),
//         repo: "https://github.com/render-examples/flask-hello-world".to_string(),
//         auto_deploy: true,
//         branch: "main".to_string(),
//         image: Image {
//             owner_id: "owner123".to_string(),
//             registry_credential_id: "credential123".to_string(),
//             image_path: "path/to/image".to_string(),
//         },
//         build_filter: BuildFilter {
//             paths: vec!["src".to_string()],
//             ignored_paths: vec!["tests".to_string()],
//         },
//         root_dir: "root".to_string(),
//         env_vars: vec![
//             EnvVar {
//                 key: "ENV_KEY".to_string(),
//                 value: Some("value".to_string()),
//                 generate_value: false,
//             },
//             EnvVar {
//                 key: "GENERATE_KEY".to_string(),
//                 value: None,
//                 generate_value: true,
//             },
//         ],
//         secret_files: vec![SecretFile {
//             name: "secret1".to_string(),
//             content: "content1".to_string(),
//         }],
//         service_details: ServiceDetails {
//             build_command: "build".to_string(),
//             headers: vec![Header {
//                 path: "/static/*".to_string(),
//                 name: "Cache-Control".to_string(),
//                 value: "public, max-age=604800".to_string(),
//             }],
//             publish_path: "publish".to_string(),
//             pull_request_previews_enabled: false,
//             routes: vec![Route {
//                 type_: "redirect".to_string(),
//                 source: "/:bar/foo".to_string(),
//                 destination: "/foo/:bar".to_string(),
//                 priority: 0,
//             }],
//         },
//     };

//     println!("{:?}", site);
// }
