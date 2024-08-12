#![allow(missing_docs)]
#![allow(non_snake_case)]
// [JSON] parsing.
use serde::Serialize;

pub trait Stringify {
    fn stringify(&self) -> String;
}

impl<T: Serialize> Stringify for T {
    fn stringify(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
