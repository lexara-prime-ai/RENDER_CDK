#![allow(missing_docs)]
#![allow(unused)]
// [JSON] parsing.
use serde::{Deserialize, Serialize};

// Idiomatic [ERROR] handling.
use anyhow::Error;

// Randomization.
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

// File parsing.
use std::fs;
use toml;

// [render_cdk] modules.
use super::caching::{CacheConf, RedisCidrAllowList};
use super::storage::{DatabaseConf, PostgresCidrAllowList};

// [DEBUG] utils.
use crate::logger::prelude::*;
use crate::utils::random::*;
use crate::utils::stringify::Stringify;
use crate::LOGGER;
use colored::Colorize;

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

            // Provide <default> CIDR block.
            if database.cidrBlocks.is_empty() {
                database.cidrBlocks.push(PostgresCidrAllowList {
                    cidrBlock: "0.0.0.0/0".to_string(),
                    description: "Everywhere".to_string(),
                });
            }
        }

        // Validate [redis] config.
        if let Some(redis) = config.redis.as_mut() {
            if redis.name.as_deref() == Some("") {
                redis.name = Some(format!("{}", GENERATE_UNIQUE_NAME()));
            }

            // Provide <default> CIDR block.
            if redis.cidrBlocks.is_empty() {
                redis.cidrBlocks.push(RedisCidrAllowList {
                    cidrBlock: "0.0.0.0/0".to_string(),
                    description: "Everywhere".to_string(),
                });
            }

            if redis.plan.is_empty() {
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
            LOGGER!(
                "\nFound empty configuration file -> ",
                &config.CONVERT_TO_JSON_STRING(),
                LogLevel::CRITICAL
            );
            return Err(anyhow::anyhow!("Found empty configuration file!"));
        }

        // Populate any <blank>/"" fields.
        Self::populate_blank_values(&mut config);

        ////////////////////////
        // Debug logs.
        ///////////////////////
        LOGGER!(
            "\n -> Reading [CONFIG]\n\n",
            &config.CONVERT_TO_JSON_STRING(),
            LogLevel::SUCCESS
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
