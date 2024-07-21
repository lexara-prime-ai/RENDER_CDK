#![allow(non_snake_case)]
use crate::state_management::prelude::*;

use anyhow::{Context, Error, Ok, Result};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use super::config::Conf;
use super::db::DatabaseConf;
use super::redis::RedisConf;

// [DEBUG] utils.
use crate::logger::prelude::*;
use crate::utils::stringify::Stringify;

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
            "\n -> [Successfully] retrieved [postgres] [CONFIG] file\n\n",
            &config.database.CONVERT_TO_JSON_STRING(),
            LogLevel::SUCCESS,
        );

        LOGGER::INFO(
            "\n -> [Successfully] retrieved [redis] [CONFIG] file\n\n",
            &config.redis.CONVERT_TO_JSON_STRING(),
            LogLevel::SUCCESS,
        );
        ///////////////////////
    }
}
