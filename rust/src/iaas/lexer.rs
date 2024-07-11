#![allow(unused)]

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Provider,
    Resource,
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(i64),
    Equals,
    LeftBrace,
    RightBrace,
    EOF,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Lexer<'l> {
    pub input: &'l str,
    pub position: usize,
    pub current_char: Option<char>,
}

impl<'l> Lexer<'l> {
    pub fn new(input: &'l str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            current_char: None,
        };

        lexer
    }

    pub fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input[self.position..].chars().next().unwrap());
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn peek_keyword(&self, keyword: &str) -> bool {
        self.input[self.position - 1..].starts_with(keyword)
    }

    pub fn consume_keyword(&mut self, keyword: &str) {
        for _ in 0..keyword.len() {
            self.advance();
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    pub fn read_string(&mut self) -> String {
        self.advance();
        let mut string = String::new();
        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            } else {
                string.push(c);
                self.advance();
            }
        }
        self.advance();
        string
    }

    pub fn read_number(&mut self) -> i64 {
        let mut number = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            Some('p') if self.peek_keyword("provider") => {
                self.consume_keyword("provider");
                Token::Provider
            }

            Some('r') if self.peek_keyword("resource") => {
                self.consume_keyword("resource");
                Token::Resource
            }

            Some(c) if c.is_alphabetic() => Token::Identifier(self.read_identifier()),
            Some('"') => Token::StringLiteral(self.read_string()),
            Some('=') => {
                self.advance();
                Token::Equals
            }
            Some('{') => {
                self.advance();
                Token::LeftBrace
            }
            Some('}') => {
                self.advance();
                Token::RightBrace
            }
            Some(c) if c.is_digit(10) => Token::NumberLiteral(self.read_number()),
            None => Token::EOF,
            _ => panic!("Unexpected character: {:?}", self.current_char),
        };

        token
    }
}
