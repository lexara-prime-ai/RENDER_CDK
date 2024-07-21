#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConf {
    pub plan: String,
}
