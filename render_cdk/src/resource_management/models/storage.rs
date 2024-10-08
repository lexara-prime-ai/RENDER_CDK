#![allow(missing_docs)]
#![allow(unused)]
#![allow(non_snake_case)]
// [JSON] parsing.
use serde::{Deserialize, Serialize};

// [DEBUG] utils.
use colored::Colorize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConf {
    pub databaseName: Option<String>,
    pub databaseUser: Option<String>,
    pub enableHighAvailability: bool,
    pub plan: String,
    pub version: String,
    pub name: Option<String>,
    pub cidrBlocks: Vec<PostgresCidrAllowList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostgresCidrAllowList {
    pub cidrBlock: String,
    pub description: String,
}
