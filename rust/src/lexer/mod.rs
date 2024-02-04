use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut line = 1;
        let tokens = self
            .source
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|t| {
                if *t == "\\n" {
                    line += 1;
                }
                Token::new(TokenType::Identifier, *t, line)
            })
            .collect::<Vec<Token>>();
        tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Scanner;

    const TEST_SOURCE: &str = "This is some input";
    #[test]
    fn can_construct_scanner() {
        let scanner = Scanner::new(TEST_SOURCE);
        assert_eq!(scanner.source, TEST_SOURCE);
    }
    #[test]
    fn can_scan_tokens() {
        let scanner = Scanner::new(TEST_SOURCE);
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens[1].literal, "is");
    }
}
