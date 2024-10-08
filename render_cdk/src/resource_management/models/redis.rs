#![allow(missing_docs)]
#![allow(unused)]
#![allow(non_snake_case)]
// [JSON] parsing.
use serde::{Deserialize, Serialize};

// [render_cdk] modules.
use super::caching::RedisCidrAllowList;

// [DEBUG] utils.
use colored::Colorize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConf {
    pub name: Option<String>,
    pub plan: String,
    pub ownerId: String,
    pub ipAllowList: Option<Vec<RedisCidrAllowList>>,
}
