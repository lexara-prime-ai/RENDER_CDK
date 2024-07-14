use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct StaticSite {
    #[serde(rename = "type")]
    type_: String,
    name: String,
    owner_id: String,
    repo: String,
    auto_deploy: bool,
    branch: String,
    image: Image,
    build_filter: BuildFilter,
    root_dir: String,
    env_vars: Vec<EnvVar>,
    secret_files: Vec<SecretFile>,
    service_details: ServiceDetails,
}

#[derive(Debug, Deserialize, Serialize)]
struct Image {
    owner_id: String,
    registry_credential_id: String,
    image_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct BuildFilter {
    paths: Vec<String>,
    ignored_paths: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EnvVar {
    key: String,
    value: Option<String>,
    generate_value: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct SecretFile {
    name: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ServiceDetails {
    build_command: String,
    headers: Vec<Header>,
    publish_path: String,
    pull_request_previews_enabled: bool,
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Header {
    path: String,
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Route {
    type_: String,
    source: String,
    destination: String,
    priority: u32,
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
