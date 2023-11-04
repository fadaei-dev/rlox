use std::collections::HashMap;

use super::*;

impl<'a> Scanner<'a> {
    pub fn string_literal(&mut self) -> Result<(), Report> {
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
            .substring(self.start + 1, self.current - 1)
            .into();

        self.add_token_literal(TokenType::STRING, Some(LiteralValue::StringValue(trimmed)));

        Ok(())
    }

    // 5.6v8 dropping v8, TODO: fix scanner
    pub fn number_literal(&mut self) -> Result<(), Report> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // TODO: match trimmed to return Report error on fail
        let trimmed: String = self.source.substring(self.start, self.current).into();

        if let Ok(parsed_float) = trimmed.parse::<f64>() {
            let number = LiteralValue::NumberValue(parsed_float);
            self.add_token_literal(TokenType::NUMBER, Some(number));
        } else {
            return Err(Report::new(
                self.line,
                String::new(),
                "Invalid number literal".into(),
            ));
        }

        Ok(())
    }

    pub fn reserved_literal(&mut self) -> Result<(), Report> {
        let ident: HashMap<&str, TokenType> = HashMap::from([
            ("and", TokenType::AND),
            ("class", TokenType::CLASS),
            ("else", TokenType::ELSE),
            ("false", TokenType::FALSE),
            ("for", TokenType::FOR),
            ("fun", TokenType::FUN),
            ("if", TokenType::IF),
            ("nil", TokenType::NIL),
            ("or", TokenType::OR),
            ("print", TokenType::PRINT),
            ("return", TokenType::RETURN),
            ("super", TokenType::SUPER),
            ("this", TokenType::THIS),
            ("true", TokenType::TRUE),
            ("var", TokenType::VAR),
            ("while", TokenType::WHILE),
        ]);

        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let trimmed = self.source.substring(self.start, self.current);

        let _type = ident.get(&trimmed);

        if let Some(_type) = ident.get(&trimmed) {
            self.add_token_literal(
                TokenType::IDENTIFIER,
                Some(LiteralValue::IdentifierValue(_type.clone())),
            );
        } else {
            self.add_token_literal(
                TokenType::IDENTIFIER,
                Some(LiteralValue::IdentifierValue(TokenType::IDENTIFIER)),
            );
        }

        Ok(())
    }
}
