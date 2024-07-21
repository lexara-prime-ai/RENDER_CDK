#![allow(non_snake_case)]
use serde::Serialize;

pub trait Stringify {
    fn CONVERT_TO_JSON_STRING(&self) -> String;
}

impl<T: Serialize> Stringify for T {
    fn CONVERT_TO_JSON_STRING(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
