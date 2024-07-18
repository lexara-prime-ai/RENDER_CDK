#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisConf {
    pub plan: String,
}
