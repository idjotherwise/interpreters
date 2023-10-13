module LexerModule

using Expronicon: @match

using ..TokenModule

export Lexer, read_char, read_char!, peek_char, next_token!

"""
Mutable struct to hold the current state of the Lexer. It contains
  the input string to the Lexer, the current `position` in the input (the current character)
  then `readPosition`, which is the position after the current one. And finally, the `ch` which is
  the current char under examination. 
"""
mutable struct Lexer
  input::String
  next::Union{Tuple{Char,Int},Nothing}
end

function Lexer(input::String)::Lexer
  Lexer(input, iterate(input))
end

"""
Non-mutating read of the next character (i.e, `peek`)
"""
function read_char(l::Lexer)::Union{Char,Nothing}
  if !isnothing(l.next)
    first(l.next)
  end
end

function peek_char(l::Lexer)
  if !isnothing(l.next)
    _, state = l.next
    next = iterate(l.input, state)
    if !isnothing(next)
      ch, _ = next
      ch
    end
  end
end

"""
Mutating read of the next character (i.e, `read` then `move`)
"""
function read_char!(l::Lexer)
  if !isnothing(l.next)
    ch, state = l.next
    l.next = iterate(l.input, state)
    ch
  end
end

function skipwhitespace!(l::Lexer)
  while isspace(read_char(l))
    read_char!(l)
  end
end

Base.isspace(::Nothing) = false
function read_double_token!(tt::TokenType, l::Lexer)::Token
  this_ch = read_char!(l)
  literal = join([this_ch, read_char(l)], "")
  Token(tt, literal)
end

"""Get the next token, using MLStyle pattern matching to get the right token"""
function next_token!(l::Lexer)
  skipwhitespace!(l)
  ch = read_char(l)
  @debug "Trying to match $ch"
  tok = @match ch begin
    '=' => begin
      if peek_char(l) == '='
        read_double_token!(EQ, l)
      else
        Token(ASSIGN, ch)
      end
    end
    ';' => Token(SEMICOLON, ch)
    '(' => Token(LPAREN, ch)
    ')' => Token(RPAREN, ch)
    ',' => Token(COMMA, ch)
    '+' => Token(PLUS, ch)
    '{' => Token(LBRACE, ch)
    '!' => begin
      if peek_char(l) == '='
        read_double_token!(NOT_EQ, l)
      else
        Token(BANG, ch)
      end
    end
    '>' => Token(GT, ch)
    '<' => Token(LT, ch)
    '-' => Token(MINUS, ch)
    '/' => Token(SLASH, ch)
    '*' => Token(ASTERISK, ch)
    '}' => Token(RBRACE, ch)
    nothing => begin
      return Token(EOF, "")
    end # Note that EOF is sent in the REPL when `ctrl+d` is sent
    _ => begin
      if isvalidletter(ch)
        return read_identifier!(l)
      elseif isdigit(ch)
        return read_number!(l)
      else
        Token(ILLEGAL, ch)
      end
    end
  end
  read_char!(l)
  tok
end

function isvalidletter(ch::Char)::Bool
  ch ∈ vcat('a':'z', 'A':'Z', ['_'])
end

isvalidletter(::Nothing)::Bool = false

function read_identifier!(l::Lexer)
  chars = Char[]
  while isvalidletter(read_char(l))
    push!(chars, read_char(l))
    read_char!(l)
  end
  literal = join(chars, "")
  Token(get(lookup_ident, literal, IDENT), literal)
end

function read_number!(l::Lexer)
  chars = Char[]
  while isdigit(read_char(l))
    push!(chars, read_char(l))
    read_char!(l)
  end
  literal = join(chars, "")
  Token(INT, literal)
end

Base.isdigit(::Nothing)::Bool = false

end
