#![allow(unused)]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DatabaseConf {
    pub name: Option<String>,
    pub user: Option<String>,
    pub enable_high_availability: bool,
    pub plan: String,
    pub version: String,
}