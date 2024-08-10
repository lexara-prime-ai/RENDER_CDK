#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(unused)]
use colored::Colorize;

#[macro_export]
macro_rules! LOGGER {
    ($identifier: expr, $message: expr, $level: expr) => {
        match $level {
            LogLevel::CRITICAL => {
                let info = $message.red();
                println!("{}{}\n", $identifier, info);
            }
            LogLevel::SUCCESS => {
                let info = $message.green();
                println!("{}{}\n", $identifier, info);
            }
            LogLevel::WARN => {
                let info = $message.yellow();
                println!("{}{}\n", $identifier, info);
            }
        }
    };
}

#[derive(Debug)]
pub enum LogLevel {
    CRITICAL,
    SUCCESS,
    WARN,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::CRITICAL => "CRITICAL",
            LogLevel::SUCCESS => "SUCCESS",
            LogLevel::WARN => "WARN",
        }
    }
}
