use crate::{
    error::Report,
    parser_core::token::{Token, TokenType},
};

use super::token::LiteralValue;

pub struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<Report>> {
        let mut errors: Vec<Report> = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(report) => errors.push(report),
            }
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".into(), None, self.line));

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(self.tokens.clone())
    }

    // TODO: Error for None Char
    fn scan_token(&mut self) -> Result<(), Report> {
        let c = self.advance();

        if let Some(char) = c {
            match char {
                '{' => self.add_token(TokenType::LEFT_BRACE),
                '}' => self.add_token(TokenType::RIGHT_BRACE),
                ',' => self.add_token(TokenType::COMMA),
                '.' => self.add_token(TokenType::DOT),
                '-' => self.add_token(TokenType::MINUS),
                '+' => self.add_token(TokenType::PLUS),
                ';' => self.add_token(TokenType::SEMICOLON),
                '*' => self.add_token(TokenType::STAR),

                // equality operators need to check and consume next char
                '!' => {
                    let token = if self._match('=') {
                        TokenType::BANG_EQUAL
                    } else {
                        TokenType::BANG
                    };

                    self.add_token(token)
                }
                '=' => {
                    let token = if self._match('=') {
                        TokenType::EQUAL_EQUAL
                    } else {
                        TokenType::EQUAL
                    };

                    self.add_token(token)
                }
                '<' => {
                    let token = if self._match('=') {
                        TokenType::LESS_EQUAL
                    } else {
                        TokenType::LESS
                    };

                    self.add_token(token)
                }
                '>' => {
                    let token = if self._match('=') {
                        TokenType::GREATER_EQUAL
                    } else {
                        TokenType::GREATER
                    };

                    self.add_token(token)
                }
                '/' => {
                    if self._match('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::SLASH);
                    }
                }
                _ => {
                    return Err(Report::new(
                        self.line,
                        String::new(),
                        String::from("Unexpected character."),
                    ))
                }
            }
        }

        todo!()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect();

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn advance(&mut self) -> Option<char> {
        let r = self.source.chars().nth(self.current);

        self.current += 1;
        r
    }

    fn _match(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Some(m) = self.source.chars().nth(self.current) {
            if m != c {
                return false;
            }
        }

        self.current += 1;
        return true;
    }
}
