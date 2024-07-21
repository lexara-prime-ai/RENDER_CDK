#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConf {
    pub plan: String,
}
