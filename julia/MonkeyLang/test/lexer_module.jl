using Test
using MonkeyLang.TokenModule
using MonkeyLang.LexerModule

function test_lex(inp::String, exp)
  l = Lexer(inp)
  for (i, t) in exp
    @test next_token!(l) == Token(i, !isnothing(t) ? t : "")
  end
end

@testset "Simple Lexer input" begin
  input = "=+(){},;"
  expected = [
    (ASSIGN, '='),
    (PLUS, '+'),
    (LPAREN, '('),
    (RPAREN, ')'),
    (LBRACE, '{'),
    (RBRACE, '}'),
    (COMMA, ','),
    (SEMICOLON, ';'),
    (EOF, nothing),
  ]

  test_lex(input, expected)
end
@testset "Lexing a simple MonkeyLang program" begin

  @testset "Simple let statements" begin
    test_lex("let five = 5; let ten = 10;",
      [
        (LET, "let"),
        (IDENT, "five"),
        (ASSIGN, "="),
        (INT, "5"),
        (SEMICOLON, ";"),
        (LET, "let"),
        (IDENT, "ten"),
        (ASSIGN, "="),
        (INT, "10"),
        (SEMICOLON, ";"),
        (EOF, nothing),
      ])
  end

  @testset "Simple functions" begin
    test_lex("""let add = fn(x, y) {
      x + y;
    };
    let result = add(five, ten);""", [(LET, "let"),
        (IDENT, "add"),
        (ASSIGN, "="),
        (FUNCTION, "fn"),
        (LPAREN, "("),
        (IDENT, "x"),
        (COMMA, ","),
        (IDENT, "y"),
        (RPAREN, ")"),
        (LBRACE, "{"),
        (IDENT, "x"),
        (PLUS, "+"),
        (IDENT, "y"),
        (SEMICOLON, ";"),
        (RBRACE, "}"),
        (SEMICOLON, ";"),
        (LET, "let"),
        (IDENT, "result"),
        (ASSIGN, "="),
        (IDENT, "add"),
        (LPAREN, "("),
        (IDENT, "five"),
        (COMMA, ","),
        (IDENT, "ten"),
        (RPAREN, ")"),
        (SEMICOLON, ";"),
        (EOF, nothing),
      ])
  end

  @testset "Gibberish single token symbols" begin
    input = """
    !-/*5;
    5 < 10 > 5;
    """
    expected = [(BANG, "!"),
      (MINUS, "-"),
      (SLASH, "/"),
      (ASTERISK, "*"),
      (INT, "5"),
      (SEMICOLON, ";"),
      (INT, "5"),
      (LT, "<"),
      (INT, "10"),
      (GT, ">"),
      (INT, "5"),
      (SEMICOLON, ";"),
      (EOF, nothing),
    ]

    test_lex(input, expected)
  end

  @testset "If else return statements" begin
    inp = """
    if (5 < 10) {
      return false;
    } else {
      return false;
    }
    """

    expected = [
      (IF, "if"),
      (LPAREN, "("),
      (INT, "5"),
      (LT, "<"),
      (INT, "10"),
      (RPAREN, ")"),
      (LBRACE, "{"),
      (RETURN, "return"),
      (FALSE, "false"),
      (SEMICOLON, ";"),
      (RBRACE, "}"),
      (ELSE, "else"),
      (LBRACE, "{"),
      (RETURN, "return"),
      (FALSE, "false"),
      (SEMICOLON, ";"),
      (EOF, nothing),
    ]
  end

  @testset "Boolean operators" begin
    test_lex("10 == 10; 10 != 9;",
      [
        (INT, "10"),
        (EQ, "=="),
        (INT, "10"),
        (SEMICOLON, ";"),
        (INT, "10"),
        (NOT_EQ, "!="),
        (INT, "9"),
        (SEMICOLON, ";"),
        (EOF, nothing),
      ])
  end

end
