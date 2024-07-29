#![allow(missing_docs)]
#![allow(non_snake_case)]
#![allow(unused)]

#[derive(Debug, Clone)]
pub struct BaseServiceConf {
    pub service_type: String,
    pub service_name: String,
    pub service_runtime: String,
    pub service_plan: String,
}

// #[derive(Debug, Clone)]
// pub struct Image

#[derive(Debug, Clone)]
pub struct Blueprints {
    pub base_conf: BaseServiceConf,
}
