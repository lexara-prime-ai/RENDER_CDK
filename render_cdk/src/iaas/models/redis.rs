#![allow(unused)]
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConf {
    pub plan: String,
    pub ownerId: String,
}
