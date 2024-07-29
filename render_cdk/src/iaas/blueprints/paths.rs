#![allow(missing_docs)]
#![allow(unused)]
#![allow(non_snake_case)]

const BLUEPRINT_OUTPUT_PATH: &str = "../blueprint_output";
const BLUEPRINT_TEMPLATES_PATH: &str = "../blueprint_templates";

#[derive(Debug, Clone)]
pub struct BlueprintPaths<'b> {
    pub BLUEPRINT_OUTPUT_PATH: &'b str,
    pub BLUEPRINT_TEMPLATES_PATH: &'b str,
}

pub trait OnInit {
    fn new(&self) -> Self;
}

impl<'b> OnInit for BlueprintPaths<'b> {
    fn new(&self) -> Self {
        Self {
            BLUEPRINT_OUTPUT_PATH,
            BLUEPRINT_TEMPLATES_PATH,
        }
    }
}
