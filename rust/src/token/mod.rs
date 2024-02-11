use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
}
impl Token {
    pub fn new(ttype: TokenType, lexeme: Vec<u8>, literal: Option<Literal>) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let real_string = std::str::from_utf8(&self.lexeme);
        match real_string {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Illegal,
    Eof,
}

impl From<TokenType> for &'static str {
    fn from(ttype: TokenType) -> &'static str {
        match ttype {
            TokenType::LeftParen => "LPAREN",
            TokenType::RightParen => "RPAREN",
            TokenType::LeftBrace => "LBRACE",
            TokenType::RightBrace => "RBRACE",
            TokenType::Illegal => "ILLEGAL",
            TokenType::Eof => "EOF ",
            TokenType::Identifier => "IDENTIFIER ",
            TokenType::Number => "NUMBER ",
            TokenType::String => "STR ",
            TokenType::Equal => "ASSIGN ",
            TokenType::Plus => "PLUS ",
            TokenType::Minus => "MINUS ",
            TokenType::Bang => "BANG ",
            TokenType::Star => "ASTERISK ",
            TokenType::Slash => "SLASH ",
            TokenType::Less => "LESS ",
            TokenType::LessEqual => "LESSEQUAL ",
            TokenType::Greater => "GREATER ",
            TokenType::GreaterEqual => "GREATEREQUAL ",
            TokenType::BangEqual => "BANGEQUAL ",
            TokenType::Fn => "FUNCTION ",
            TokenType::Var => "VAR ",
            TokenType::True => "TRUE ",
            TokenType::False => "FALSE ",
            TokenType::If => "IF ",
            TokenType::Else => "ELSE ",
            TokenType::Comma => "COMMA",
            TokenType::Dot => "DOT",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::EqualEqual => "EQUALEQUAL",
            TokenType::And => "&&",
            TokenType::Class => "CLASS ",
            TokenType::For => "FOR ",
            TokenType::Nil => "NIL ",
            TokenType::Or => "OR ",
            TokenType::Print => "PRINT ",
            TokenType::Return => "RETRUN ",
            TokenType::Super => "SUPER ",
            TokenType::This => "THIS ",
            TokenType::While => "FOR ",
        }
    }
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: &'static str = (*self).into();
        write!(f, "{}", s)
    }
}
