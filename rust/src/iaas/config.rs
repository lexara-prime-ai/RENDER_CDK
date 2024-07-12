#![allow(unused)]
use anyhow::Error;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::fs;
use toml;

use super::db::DatabaseConf;
use super::redis::RedisConf;

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub database: DatabaseConf,
    pub redis: RedisConf,
}

impl Conf {
    pub fn generate_random_string(&self, length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub fn populate_blank_values(config: &mut Conf) {
        if config.database.name.as_deref() == Some("") {
            config.database.name = Some(format!("db_{}", config.generate_random_string(10)));
        }

        if config.database.user.as_deref() == Some("") {
            config.database.user = Some(format!("user_{}", config.generate_random_string(10)));
        }
    }

    pub fn read_configuration_file() -> Result<Self, Error> {
        let conf_path = "./samples/sample.conf";
        let contents = fs::read_to_string(conf_path)
            .expect(format!("Unable to read configuration: <{conf_path:?}>").as_str());

        // Parse config. file.
        let mut config: Conf = toml::from_str(&contents)
            .expect(format!("Unable to parse configuration: <{conf_path:?}>").as_str());

        // Populate any <black>/"" fields.
        Self::populate_blank_values(&mut config);

        ////////////////////////
        // Debug logs.
        ///////////////////////
        // println!("[DEBUG] -> {:?}", config);

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

    /////////////////////////////////
    // Configuration Initialization.
    ////////////////////////////////
    #[test]
    fn test_read_configuration_file() {
        // Validate that the result is Ok().
        let config = Conf::read_configuration_file();
        assert!(config.is_ok());
    }

    #[test]
    fn test_generate_random_string() {
        // Validate that the output is NOT empty.
        let config = Conf::read_configuration_file().unwrap();
        let result = config.generate_random_string(10);
        assert!(!result.is_empty());
    }
}
