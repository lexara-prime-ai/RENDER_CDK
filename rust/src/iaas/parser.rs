#![allow(unused)]

use super::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum ASTValue {
    StringLiteral(String),
    NumberLiteral(i64),
}

#[derive(Debug)]
pub enum ASTNode {
    ProviderBlock {
        name: String,
        properties: Vec<(String, ASTValue)>,
    },
    ResourceBlock {
        name: String,
        resource_type: String,
        properties: Vec<(String, ASTValue)>,
    },
}

#[derive(Debug)]
pub struct Parser<'l> {
    lexer: Lexer<'l>,
    current_token: Token,
}

impl<'l> Parser<'l> {
    pub fn new(mut lexer: Lexer<'l>) -> Self {
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_identifier(&mut self) -> String {
        match self.current_token {
            Token::Identifier(ref name) => {
                let name = name.clone();
                self.next_token();
                name
            }
            _ => panic!("Expected identifier, found: {:?}", self.current_token),
        }
    }

    pub fn expect_token(&mut self, expected: Token) {
        if self.current_token == expected {
            self.next_token();
        } else {
            panic!(
                "Expected token {:?}, found {:?}",
                expected, self.current_token
            );
        }
    }

    pub fn parse_properties(&mut self) -> Vec<(String, ASTValue)> {
        let mut properties = Vec::new();
        while self.current_token != Token::RightBrace {
            let key = self.parse_identifier();
            self.expect_token(Token::Equals);
            let value = self.parse_value();
            properties.push((key, value));
        }
        properties
    }

    pub fn parse_value(&mut self) -> ASTValue {
        match self.current_token {
            Token::StringLiteral(ref value) => {
                let value = value.clone();
                self.next_token();
                ASTValue::StringLiteral(value)
            }
            Token::NumberLiteral(value) => {
                self.next_token();
                ASTValue::NumberLiteral(value)
            }
            _ => panic!("Expected value, found: {:?}", self.current_token),
        }
    }

    pub fn parse_provider_block(&mut self) -> ASTNode {
        self.next_token();
        let name = self.parse_identifier();
        self.expect_token(Token::LeftBrace);
        let properties = self.parse_properties();
        self.expect_token(Token::RightBrace);
        ASTNode::ProviderBlock { name, properties }
    }

    pub fn parse_resource_block(&mut self) -> ASTNode {
        self.next_token();
        let resource_type = self.parse_identifier();
        let name = self.parse_identifier();
        self.expect_token(Token::LeftBrace);
        let properties = self.parse_properties();
        self.expect_token(Token::RightBrace);
        ASTNode::ResourceBlock {
            name,
            resource_type,
            properties,
        }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            match self.current_token {
                Token::Provider => {
                    nodes.push(self.parse_provider_block());
                }
                Token::Resource => {
                    nodes.push(self.parse_resource_block());
                }
                _ => panic!("Unexpected token: {:?}", self.current_token),
            }
        }
        nodes
    }
}
