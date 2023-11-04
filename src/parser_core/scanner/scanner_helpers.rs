use super::*;

impl<'a> Scanner<'a> {
    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    pub fn add_token_literal(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text: String = self.source.substring(self.start, self.current).into();

        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    pub fn advance(&mut self) -> Option<char> {
        let r = self.source.chars().nth(self.current);

        self.current += 1;
        r
    }

    pub fn _match(&mut self, c: char) -> bool {
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
