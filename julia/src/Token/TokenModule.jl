module TokenModule

export Token, lookup_ident, TokenType


@enum TokenType begin
  ILLEGAL
  EOF
  IDENT
  INT
  ASSIGN
  PLUS
  MINUS
  BANG
  ASTERISK
  SLASH
  LT
  GT
  COMMA
  SEMICOLON
  LPAREN
  RPAREN
  LBRACE
  RBRACE

  EQ
  NOT_EQ

  # KEYWORDS
  FUNCTION
  LET
  TRUE
  FALSE
  IF
  ELSE
  RETURN
end

for s in instances(TokenType)
  @eval export $(Symbol(s))
end

const lookup_ident = Dict{String,TokenType}(
  "fn" => FUNCTION,
  "let" => LET,
  "true" => TRUE,
  "false" => FALSE,
  "else" => ELSE,
  "if" => IF,
  "return" => RETURN,
)

struct Token
  type::TokenType
  literal::String
end

Token(type::TokenType, ch::Char) = Token(type, string(ch))
Token(type::TokenType, ::Nothing) = Token(type, "")

end
