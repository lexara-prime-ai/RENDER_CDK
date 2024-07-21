use crate::state_management::prelude::*;

use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use super::config::Conf;
use super::db::DatabaseConf;
use super::redis::RedisConf;

// [DEBUG] utils.
use crate::logger::prelude::*;

const BASE_URL: &str = "https://api.render.com/v1";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Deploy {
    config: Conf,
}

impl Deploy {
    pub async fn deploy_configuration(config_path: &str) {
        let config = Conf::read_configuration_file(config_path).unwrap();

        ////////////////////////
        // Debug logs.
        ///////////////////////
        LOGGER::INFO(
            "\n -> [Successfully] retrieved [CONFIG] file\n\n",
            &config.to_json_string(),
            LogLevel::SUCCESS,
        );
    }
}
