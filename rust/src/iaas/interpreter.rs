use super::parser::*;

#[derive(Debug)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub async fn interpret(&self, nodes: Vec<ASTNode>) -> Vec<String> {
        let mut results = Vec::new();

        for node in nodes {
            match node {
                ASTNode::ProviderBlock { name, properties } => {
                    results.push(self.interpret_provider_block(name, properties).await);
                }
                ASTNode::ResourceBlock {
                    name,
                    resource_type,
                    properties,
                } => {
                    results.push(
                        self.interpret_resource_block(name, resource_type, properties)
                            .await,
                    );
                }
            }
        }
        results
    }

    pub async fn interpret_provider_block(
        &self,
        name: String,
        properties: Vec<(String, ASTValue)>,
    ) -> String {
        let mut result = format!("Provider: {}", name);

        for (key, value) in properties {
            match value {
                ASTValue::StringLiteral(val) => result.push_str(&format!("\n  {}: {}", key, val)),
                ASTValue::NumberLiteral(val) => result.push_str(&format!("\n  {}: {}", key, val)),
            }
        }
        result
    }

    pub async fn interpret_resource_block(
        &self,
        name: String,
        resource_type: String,
        properties: Vec<(String, ASTValue)>,
    ) -> String {
        let mut result = format!("Resource: {} {}", resource_type, name);

        for (key, value) in properties {
            match value {
                ASTValue::StringLiteral(val) => result.push_str(&format!("\n  {}: {}", key, val)),
                ASTValue::NumberLiteral(val) => result.push_str(&format!("\n  {}: {}", key, val)),
            }
        }
        result
    }
}
