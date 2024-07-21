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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conf {
    pub database: DatabaseConf,
    pub redis: RedisConf,
}

impl Conf {
    fn generate_random_string(&self, length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    fn populate_blank_values(config: &mut Conf) {
        if config.database.name.as_deref() == Some("") {
            config.database.name = Some(format!("db_{}", config.generate_random_string(10)));
        }

        if config.database.user.as_deref() == Some("") {
            config.database.user = Some(format!("user_{}", config.generate_random_string(10)));
        }
    }

    pub fn read_configuration_file(config_path: &str) -> Result<Self, Error> {
        let contents = fs::read_to_string(config_path)
            .expect(format!("Unable to read configuration: <{config_path:?}>").as_str());

        // Parse config. file.
        let mut config: Conf = toml::from_str(&contents)
            .expect(format!("Unable to parse configuration: <{config_path:?}>").as_str());

        // Populate any <black>/"" fields.
        Self::populate_blank_values(&mut config);

        ////////////////////////
        // Debug logs.
        ///////////////////////
        LOGGER::INFO("\n -> Readng [CONFIG]\n\n", &config.to_json_string(), LogLevel::WARN);

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
        let result = config.generate_random_string(10);
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
