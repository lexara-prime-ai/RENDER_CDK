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
    pub cidrBlocks: Vec<CidrBlock>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CidrBlock {
    pub cidrBlock: String,
    pub description: String,
}
