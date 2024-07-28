#![allow(unused)]
use anyhow::Error;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

use super::caching::CacheConf;
use super::storage::DatabaseConf;

// [DEBUG] utils.
use crate::logger::info::*;
use crate::utils::random::*;
use crate::utils::stringify::Stringify;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conf {
    pub database: Option<DatabaseConf>,
    pub redis: Option<CacheConf>,
}

impl Conf {
    fn populate_blank_values(config: &mut Conf) {
        // Validate [postgres] config.
        if let Some(database) = config.database.as_mut() {
            if database.databaseName.as_deref() == Some("") {
                database.databaseName = Some(format!("{}", GENERATE_UNIQUE_NAME()));
            }

            if database.databaseUser.as_deref() == Some("") {
                database.databaseUser = Some(format!("{}", GENERATE_UNIQUE_NAME()));
            }

            if database.name.as_deref() == Some("") {
                database.name = Some(format!("{}", GENERATE_UNIQUE_NAME()));
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
            .expect(format!("Unable to READ configuration: {config_path:?}").as_str());

        // Parse config. file.
        let mut config: Conf = toml::from_str(&contents)
            .expect(format!("Unable to PARSE configuration: {config_path:?}").as_str());

        // Validate config. file.
        if config.database.is_none() && config.redis.is_none() {
            LOGGER::INFO(
                "\nFound empty configuration file -> ",
                &config.CONVERT_TO_JSON_STRING(),
                LogLevel::CRITICAL,
            );
            return Err(anyhow::anyhow!("Found empty configuration file!"));
        }

        // Populate any <blank>/"" fields.
        Self::populate_blank_values(&mut config);

        ////////////////////////
        // Debug logs.
        ///////////////////////
        LOGGER::INFO(
            "\n -> Reading [CONFIG]\n\n",
            &config.CONVERT_TO_JSON_STRING(),
            LogLevel::SUCCESS,
        );

        Ok(Self {
            database: config.database,
            redis: config.redis,
        })
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
    fn test_generate_unique_name() {
        // Validate that the output is NOT empty.
        let config = Conf::read_configuration_file(&CONFIG_PATH).unwrap();
        let result = GENERATE_UNIQUE_NAME();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_conf_to_json_string() {
        let config = Conf::read_configuration_file(&CONFIG_PATH).unwrap();
        let result = config.CONVERT_TO_JSON_STRING();
        // Validate that the output is a String.
        assert_eq!(std::any::type_name_of_val(&result), "alloc::string::String");
    }
}
