#![allow(unused)]
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConf {
    pub databaseName: Option<String>,
    pub databaseUser: Option<String>,
    pub enableHighAvailability: bool,
    pub plan: String,
    pub version: String,
    pub name: Option<String>,
    pub cidrBlock: String,
    pub accessLevelDescription: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IpAllowList {
    pub cidrBlock: String,
    pub description: String,
}
