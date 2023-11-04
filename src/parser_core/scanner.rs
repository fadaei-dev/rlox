use crate::{
    error::Report,
    parser_core::token::{LiteralValue, Token, TokenType},
};

use substring::Substring;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    pub line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
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

        if let Some(_char) = c {
            match _char {
                '(' => self.add_token(TokenType::LEFT_PAREN),
                ')' => self.add_token(TokenType::RIGHT_PAREN),
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

                // string literal
                '"' => self.string_literal()?,

                // number literal
                _ if _char.is_ascii_digit() => self.number_literal()?,

                _ if _char.is_ascii_alphabetic() => self.reserved_literal()?,

                // special characters
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,

                // Report errors to scan_tokens
                _ => {
                    return Err(Report::new(
                        self.line,
                        String::new(),
                        String::from("Unexpected character."),
                    ));
                }
            }
        }

        Ok(())
    }
}

// helpers impl
mod scanner_helpers;
// literals impl
mod scanner_literals;

// tests clone literals multiple times, literals are low cost structs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( ))";
        let mut scanner = Scanner::new(source.into());

        let _ = scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LEFT_PAREN);
        assert_eq!(scanner.tokens[1].token_type, TokenType::LEFT_PAREN);
        assert_eq!(scanner.tokens[2].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(scanner.tokens[3].token_type, TokenType::RIGHT_PAREN);
        assert_eq!(scanner.tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source.into());

        let _ = scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::BANG);
        assert_eq!(scanner.tokens[1].token_type, TokenType::BANG_EQUAL);
        assert_eq!(scanner.tokens[2].token_type, TokenType::EQUAL_EQUAL);
        assert_eq!(scanner.tokens[3].token_type, TokenType::GREATER_EQUAL);
        assert_eq!(scanner.tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn handle_comment_tokens() {
        let source = "// this (/<-ignore) is a comment\n";

        let mut scanner = Scanner::new(source.into());

        let _ = scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 1);
    }

    #[test]
    fn handle_string_tokens() {
        let source = "\"this is a string\"";
        let mut scanner = Scanner::new(source.into());

        let _ = scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::STRING);
        assert_eq!(scanner.tokens[0].literal.is_some(), true);
        assert_eq!(
            scanner.tokens[0].literal.clone().is_some_and(|lit| {
                if let LiteralValue::StringValue(s) = lit {
                    return "this is a string" == s;
                } else {
                    return false;
                }
            }),
            true
        );
    }

    #[test]
    fn handle_number_tokens() {
        let source = "7 5.43 52 1.9";
        let mut scanner = Scanner::new(source.into());

        let _ = scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::NUMBER);

        assert_eq!(
            scanner.tokens[0].literal.clone().is_some_and(|lit| {
                if let LiteralValue::NumberValue(n) = lit {
                    return 7.0 == n;
                } else {
                    false
                }
            }),
            true
        );

        assert_eq!(
            scanner.tokens[1].literal.clone().is_some_and(|lit| {
                if let LiteralValue::NumberValue(n) = lit {
                    dbg!(n);
                    return 5.43 == n;
                } else {
                    false
                }
            }),
            true
        );
    }
}
