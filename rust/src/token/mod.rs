use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, literal: &str, line: usize) -> Self {
        Self {
            ttype,
            literal: literal.to_string(),
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Illegal = 0,
    Eof,
    Identifier,
    Number,
    Str,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    BangEqual,

    Function,
    Let,
    True,
    False,
    If,
    Else,
}

impl From<TokenType> for &'static str {
    fn from(ttype: TokenType) -> &'static str {
        match ttype {
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF ",
            TokenType::Identifier => "IDENTIFIER ",
            TokenType::Number => "NUMBER ",
            TokenType::Str => "STR ",
            TokenType::Assign => "ASSIGN ",
            TokenType::Plus => "PLUS ",
            TokenType::Minus => "MINUS ",
            TokenType::Bang => "BANG ",
            TokenType::Asterisk => "ASTERISK ",
            TokenType::Slash => "SLASH ",
            TokenType::Less => "LESS ",
            TokenType::LessEqual => "LESSEQUAL ",
            TokenType::Greater => "GREATER ",
            TokenType::GreaterEqual => "GREATEREQUAL ",
            TokenType::Equal => "EQUAL ",
            TokenType::BangEqual => "BANGEQUAL ",
            TokenType::Function => "FUNCTION ",
            TokenType::Let => "LET ",
            TokenType::True => "TRUE ",
            TokenType::False => "FALSE ",
            TokenType::If => "IF ",
            TokenType::Else => "ELSE ",
        }
    }
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str = (*self).into();
        write!(f, "{}", s)
    }
}
