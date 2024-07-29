#![allow(unused)]
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

use crate::iaas::storage::PostgresCidrAllowList;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostgresConf {
    pub databaseName: Option<String>,
    pub databaseUser: Option<String>,
    pub enableHighAvailability: bool,
    pub plan: String,
    pub version: String,
    pub name: Option<String>,
    pub ownerId: String,
    pub ipAllowList: Option<Vec<PostgresCidrAllowList>>,
}
