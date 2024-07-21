#![allow(unused)]
use anyhow::Error;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

use super::db::DatabaseConf;
use super::redis::RedisConf;

// [DEBUG] utils.
use crate::logger::info::*;
use crate::utils::random::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conf {
    pub database: Option<DatabaseConf>,
    pub redis: Option<RedisConf>,
}

impl Conf {
    fn populate_blank_values(config: &mut Conf) {
        // Validate [postgres] config.
        if let Some(database) = config.database.as_mut() {
            if database.databaseName.as_deref() == Some("") {
                database.databaseName = Some(format!("db_{}", GENERATE_RANDOM_STRING(10)));
            }

            if database.databaseUser.as_deref() == Some("") {
                database.databaseUser = Some(format!("user_{}", GENERATE_RANDOM_STRING(10)));
            }
        }

        // Validate [redis] config.
        if let Some(redis) = config.redis.as_mut() {
            if redis.plan == "" {
                redis.plan = "starter".to_owned();
            }
        }
    }

    pub fn read_configuration_file(config_path: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(config_path)
            .expect(format!("Unable to read configuration: {config_path:?}").as_str());

        // Parse config. file.
        let mut config: Conf = toml::from_str(&contents)
            .expect(format!("Unable to parse configuration: {config_path:?}").as_str());

        // Populate any <black>/"" fields.
        Self::populate_blank_values(&mut config);

        ////////////////////////
        // Debug logs.
        ///////////////////////
        // LOGGER::INFO("\n -> Reading [CONFIG]\n\n", &config.to_json_string(), LogLevel::WARN);

        Ok(Self {
            database: config.database,
            redis: config.redis,
        })
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod config_test {
    // use crate::iaas::Conf;
    use super::*;

    // Constants.
    const CONFIG_PATH: &str = "./samples/sample.conf";

    /////////////////////////////////
    // Configuration Initialization.
    ////////////////////////////////
    #[test]
    fn test_read_configuration_file() {
        // Validate that the result is Ok().
        let config = Conf::read_configuration_file(&CONFIG_PATH);
        assert!(config.is_ok());
    }

    #[test]
    fn test_generate_random_string() {
        // Validate that the output is NOT empty.
        let config = Conf::read_configuration_file(&CONFIG_PATH).unwrap();
        let result = GENERATE_RANDOM_STRING(10);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_conf_to_json_string() {
        let config = Conf::read_configuration_file(&CONFIG_PATH).unwrap();
        let result = config.to_json_string();
        // Validate that the output is a String.
        assert_eq!(std::any::type_name_of_val(&result), "alloc::string::String");
    }
}
