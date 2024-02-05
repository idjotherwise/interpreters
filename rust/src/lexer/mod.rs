use crate::token::{Literal, Token, TokenType};
// disable the dead code warning
#[allow(dead_code)]
#[derive(Debug)]
pub struct Error {
    message: String,
    line: usize,
    column: usize,
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    err: Option<Error>,
    start: u32,
    current: u32,
    line: usize,
    // column: i64,
}
impl Default for Scanner {
    fn default() -> Self {
        Self {
            source: Vec::new(),
            tokens: Vec::new(),
            err: None,
            start: 0,
            current: 0,
            line: 0,
            // column: -1,
        }
    }
}

impl Scanner {
    fn is_end(&self) -> bool {
        (self.current as usize) >= self.source.len()
    }

    pub fn scan_tokens(&mut self, input: String) -> Result<Vec<Token>, Error> {
        self.source = input.as_bytes().to_vec();
        let _tokens: Vec<Token> = vec![];
        while !self.is_end() || self.err.is_none() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            "EOF".as_bytes().to_vec(),
            None,
            self.line,
        ));
        match self.err.take() {
            Some(e) => Err(e),
            None => Ok(self.tokens.clone()),
        }
    }
    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c as char {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() {
                    self.identifier();
                }
            }
        }
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        let c = self.source[self.current as usize];
        c
    }
    fn add_token(&mut self, token_type: TokenType) {
        self.add_literal_token(token_type, None)
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start as usize..self.current as usize].to_vec();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let val: f64 =
            String::from_utf8(self.source[self.start as usize..self.current as usize].to_vec())
                .unwrap()
                .parse()
                .unwrap();

        self.add_literal_token(TokenType::Number, Some(Literal::Number(val)))
    }

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        } else {
            self.source[self.current as usize] as char
        }
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) as usize >= self.source.len() {
            return '\0';
        } else {
            self.source[(self.current + 1) as usize] as char
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let text = self.source[self.start as usize..self.current as usize].to_vec();
        let token_type = match text.as_slice() {
            b"var" => TokenType::Var,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type);
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Scanner;
    use crate::token::{Token, TokenType};

    const TEST_SOURCE: &str = "var x = 1";
    #[test]
    fn can_construct_scanner() {
        let scanner = &mut Scanner::default();
        // scanner.scan_tokens(TEST_SOURCE.to_string());
        // assert_eq!(scanner.source, TEST_SOURCE.as_bytes().to_vec());
    }
    #[test]
    fn can_scan_tokens() {
        let scanner = &mut Scanner::default();
        // TODO: fix this test - len out of bounds.
        // let _tokens = scanner.scan_tokens(TEST_SOURCE.to_string());
        let _expected_tokens = vec![
            Token::new(TokenType::Var, "var".as_bytes().to_vec(), None, 1),
            Token::new(TokenType::Identifier, "x".as_bytes().to_vec(), None, 1),
            Token::new(TokenType::Equal, "=".as_bytes().to_vec(), None, 1),
            Token::new(TokenType::Number, "1".as_bytes().to_vec(), None, 1),
        ];
        // assert_eq!(tokens, expected_tokens);
    }
}
