#![allow(missing_docs)]
#![allow(unused)]
#![allow(non_snake_case)]
// [JSON] parsing.
use serde::{Deserialize, Serialize};

// [DEBUG] utils.
use colored::Colorize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConf {
    pub name: Option<String>,
    pub plan: String,
    pub cidrBlocks: Vec<RedisCidrAllowList>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisCidrAllowList {
    pub cidrBlock: String,
    pub description: String,
}
