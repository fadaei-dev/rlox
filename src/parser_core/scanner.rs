use crate::{
    error::Report,
    parser_core::token::{Token, TokenType},
};

use super::token::LiteralValue;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
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
                '"' => self.string_literal(),

                // special characters
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,

                // Report errors to scan_tokens
                _ => {
                    return Err(Report::new(
                        self.line,
                        String::new(),
                        String::from("Unexpected character."),
                    ))
                }
            }
        }

        Ok(())
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
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

// literals impl
impl Scanner {
    fn string_literal(&mut self) -> Result<(), Report> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Report::new(
                self.line,
                String::new(),
                "Unterminated string.".into(),
            ));
        }

        self.advance();

        let trimmed: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 1)
            .collect();

        self.add_token_literal(TokenType::STRING, Some(LiteralValue::StringValue(trimmed)));

        Ok(())
    }
}

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
}
