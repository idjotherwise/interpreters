use crate::token::{Literal, Token, TokenType};
use thiserror::Error;

#[derive(Debug)]
pub struct SourcePosition {
    line: usize,
    column: usize,
}

impl std::fmt::Display for SourcePosition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "line {} column {}", self.line, self.column)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unexpected character '{c}' at {pos}")]
    UnexpectedChar { c: char, pos: SourcePosition },
}

#[derive(Default)]
pub struct Scanner {
    source: Vec<u8>,
    start: u32,
    current: u32,
    byte_offset: usize,
}

impl Scanner {
    pub fn resolve_position(&self) -> SourcePosition {
        let mut line = 0;
        let mut column = 0;

        for c in self.source.iter().take(self.byte_offset) {
            if c == &b'\n' {
                column = 0;
                line += 1;
            } else {
                column += 1;
            }
        }
        SourcePosition { line, column }
    }
    fn is_end(&self) -> bool {
        (self.current as usize) >= self.source.len()
    }

    pub fn scan_tokens(&mut self, input: String) -> Result<Vec<Token>, Vec<Error>> {
        log::debug!("Scanning: {}", input);
        self.source = input.as_bytes().to_vec();

        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<Error> = vec![];

        while !self.is_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(Some(t)) => {
                    log::debug!("Added new token: {}", t);
                    tokens.push(t)
                }
                Ok(None) => {}
                Err(e) => errors.push(e),
            }
        }
        tokens.push(Token::new(TokenType::Eof, "EOF".as_bytes().to_vec(), None));
        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }
    pub fn scan_token(&mut self) -> Result<Option<Token>, Error> {
        let c = self.advance();
        log::debug!(
            "Current character: {} {}",
            c as char,
            self.resolve_position()
        );
        match c as char {
            '(' => Ok(Some(self.add_token(TokenType::LeftParen))),
            ')' => Ok(Some(self.add_token(TokenType::RightParen))),
            '{' => Ok(Some(self.add_token(TokenType::LeftBrace))),
            '}' => Ok(Some(self.add_token(TokenType::RightBrace))),
            ',' => Ok(Some(self.add_token(TokenType::Comma))),
            '.' => Ok(Some(self.add_token(TokenType::Dot))),
            '-' => Ok(Some(self.add_token(TokenType::Minus))),
            '+' => Ok(Some(self.add_token(TokenType::Plus))),
            '=' => Ok(Some(self.add_token(TokenType::Equal))),
            ';' => Ok(Some(self.add_token(TokenType::Semicolon))),
            '*' => Ok(Some(self.add_token(TokenType::Star))),
            ' ' | '\n' | '\t' | '\r' => Ok(None),
            _ => {
                if c.is_ascii_digit() {
                    Ok(Some(self.number()))
                } else if c.is_ascii_alphabetic() {
                    Ok(Some(self.identifier()))
                } else {
                    Err(Error::UnexpectedChar {
                        c: c as char,
                        pos: self.resolve_position(),
                    })
                }
            }
        }
    }

    fn advance(&mut self) -> u8 {
        let c = self.source[self.current as usize];
        self.current += 1;
        self.byte_offset += 1;
        log::debug!("Advanced to {} {}", self.current, c as char);
        c
    }
    fn add_token(&mut self, token_type: TokenType) -> Token {
        self.add_literal_token(token_type, None)
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<Literal>) -> Token {
        let text = self.source[self.start as usize..self.current as usize].to_vec();
        Token::new(token_type, text, literal)
    }

    fn number(&mut self) -> Token {
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
        log::debug!("Peeking from {} to {}", self.current, self.current + 1);
        if self.is_end() {
            '\0'
        } else {
            self.source[self.current as usize] as char
        }
    }

    fn peek_next(&self) -> char {
        log::debug!("Peeking next from {} to {}", self.current, self.current + 1);
        if (self.current + 1) as usize >= self.source.len() {
            '\0'
        } else {
            self.source[(self.current + 1) as usize] as char
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let text = self.source[self.start as usize..self.current as usize].to_vec();
        let token_type = match text.as_slice() {
            b"var" => TokenType::Var,
            b"fn" => TokenType::Fn,
            b"return" => TokenType::Return,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Scanner;
    use crate::token::{Literal, Token, TokenType};

    const TEST_SOURCE: &str = "var x = 1";
    #[test]
    fn can_construct_scanner() {
        let scanner = &mut Scanner::default();
        let _ = scanner.scan_tokens(TEST_SOURCE.to_string());
        assert_eq!(scanner.source, TEST_SOURCE.as_bytes().to_vec());
    }
    #[test]
    fn can_scan_tokens() {
        let scanner = &mut Scanner::default();
        let tokens = scanner.scan_tokens(TEST_SOURCE.to_string());

        let expected_tokens = vec![
            Token::new(TokenType::Var, "var".as_bytes().to_vec(), None),
            Token::new(TokenType::Identifier, "x".as_bytes().to_vec(), None),
            Token::new(TokenType::Equal, "=".as_bytes().to_vec(), None),
            Token::new(
                TokenType::Number,
                "1".as_bytes().to_vec(),
                Some(Literal::Number(1.0)),
            ),
            Token::new(TokenType::Eof, "EOF".as_bytes().to_vec(), None),
        ];
        assert_eq!(tokens.unwrap(), expected_tokens);
    }

    #[test]
    fn can_identify_functions() {
        const FUNCTION_SOURCE: &str = "fn double(x) { return 2x }";
        let scanner = &mut Scanner::default();
        let tokens = scanner.scan_tokens(FUNCTION_SOURCE.to_string());

        let expected_tokens = vec![
            Token::new(TokenType::Fn, "fn".as_bytes().to_vec(), None),
            Token::new(TokenType::Identifier, "double".as_bytes().to_vec(), None),
            Token::new(TokenType::LeftParen, "(".as_bytes().to_vec(), None),
            Token::new(TokenType::Identifier, "x".as_bytes().to_vec(), None),
            Token::new(TokenType::RightParen, ")".as_bytes().to_vec(), None),
            Token::new(TokenType::LeftBrace, "{".as_bytes().to_vec(), None),
            Token::new(TokenType::Return, "return".as_bytes().to_vec(), None),
            Token::new(
                TokenType::Number,
                "2".as_bytes().to_vec(),
                Some(Literal::Number(2.)),
            ),
            Token::new(TokenType::Identifier, "x".as_bytes().to_vec(), None),
            Token::new(TokenType::RightBrace, "}".as_bytes().to_vec(), None),
            Token::new(TokenType::Eof, "EOF".as_bytes().to_vec(), None),
        ];
        assert_eq!(tokens.unwrap(), expected_tokens);
    }
}
