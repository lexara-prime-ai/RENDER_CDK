#![allow(non_snake_case)]
#![allow(unused)]

#[derive(Debug, Clone)]
pub struct ServiceConf {
    pub service_type: String,
    pub service_name: String,
    pub service_runtime: String,
    pub service_plan: String,
}

pub struct Blueprints {
    pub base_conf: ServiceConf,
}
